//! The parts of an xlsx/xlsm package calamine doesn't read: cell styles, data
//! validation, conditional formats, comments, hyperlinks, sheet view (frozen
//! panes, tab colour, hidden rows/cols), filters, protection, tables, charts,
//! pivots and links to other workbooks. Read straight from the package XML.
//!
//! Best-effort by design: a part that fails to parse becomes a warning, never a
//! failed extraction. Values and formulas still come from calamine.

use crate::a1::{col_name, parse_pt, parse_rect, quote_sheet, Rect, OPEN};
use crate::model::{
    Cell, Chart, CondFormat, ExternalLink, Filter, Interner, Pivot, Protected, Sheet, Style, Table, Validation, Workbook,
};
use anyhow::{anyhow, Result};
use quick_xml::events::{BytesRef, BytesStart, Event};
use quick_xml::Reader;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use zip::ZipArchive;

/// Add everything calamine skipped to `wb`, which calamine already filled
/// from the same file.
pub fn enrich(path: &Path, wb: &mut Workbook) -> Result<()> {
    let mut pkg = Pkg::open(path)?;
    let wb_part = pkg.rels("").into_values().find(|r| r.kind == "officeDocument").map(|r| r.target).unwrap_or_else(|| "xl/workbook.xml".into());
    let root = pkg.node(&wb_part).ok_or_else(|| anyhow!("no workbook part {wb_part}"))?;
    let rels = pkg.rels(&wb_part);
    let part_of = |kind: &str| rels.values().find(|r| r.kind == kind).map(|r| r.target.clone());

    let theme = part_of("theme").and_then(|p| pkg.node(&p)).map(|n| theme_colors(&n)).unwrap_or_else(default_theme);
    let styles = Styles::parse(&part_of("styles").and_then(|p| pkg.node(&p)).unwrap_or_default(), theme);
    let mut interner = Interner::<Style>::default();
    let xf_ids: Vec<Option<u32>> =
        styles.xfs.iter().map(|s| if *s == Style::default() { None } else { Some(interner.intern(s.clone())) }).collect();
    let mut dvs = Interner::<Validation>::default();
    let mut skipped: BTreeSet<&str> = BTreeSet::new();

    wb.external_links = external_links(&mut pkg, &root, &rels);

    for s in root.child("sheets").map(|s| s.children("sheet").collect::<Vec<_>>()).unwrap_or_default() {
        let (Some(name), Some(rel)) = (s.attr("name"), s.attr("id").and_then(|id| rels.get(id))) else { continue };
        let Some(si) = wb.sheets.iter().position(|x| x.name == name) else { continue };
        let mut cx = SheetCx { styles: &styles, xf_ids: &xf_ids, dvs: &mut dvs, skipped: &mut skipped };
        if let Err(e) = read_sheet(&mut pkg, &rel.target, &rel.kind, &mut wb.sheets[si], &mut cx) {
            wb.warnings.push(format!("{name}: {e:#}"));
        }
    }
    wb.styles = interner.items;
    wb.validations = dvs.items;

    // Formulas store other workbooks as `[1]Sheet!A1`; name them.
    if !wb.external_links.is_empty() {
        let links = wb.external_links.clone();
        for sh in &mut wb.sheets {
            for c in &mut sh.cells {
                if let Some(f) = c.f.as_mut().filter(|f| f.contains('[')) {
                    *f = name_external_refs(f, &links);
                }
            }
            for ch in &mut sh.charts {
                for src in &mut ch.sources {
                    *src = name_external_refs(src, &links);
                }
            }
        }
        for nr in &mut wb.named_ranges {
            nr.range = name_external_refs(&nr.range, &links);
        }
    }

    for (prefix, what) in [("xl/media/", "images"), ("xl/slicers/", "slicers"), ("xl/timelines/", "timelines")] {
        if pkg.names.keys().any(|n| n.starts_with(prefix)) {
            skipped.insert(what);
        }
    }
    if !skipped.is_empty() {
        wb.warnings.push(format!("not extracted: {}", skipped.into_iter().collect::<Vec<_>>().join(", ")));
    }
    Ok(())
}

// ---------------------------------------------------------------- package

struct Rel {
    /// Last segment of the relationship type, e.g. `worksheet`, `comments`.
    kind: String,
    /// Part name inside the package, or the link as written when external.
    target: String,
}

struct Pkg {
    zip: ZipArchive<BufReader<File>>,
    /// Lower-cased part name → stored name (part names are case-insensitive).
    names: HashMap<String, String>,
}

impl Pkg {
    fn open(path: &Path) -> Result<Pkg> {
        let zip = ZipArchive::new(BufReader::new(File::open(path)?))?;
        let names = zip.file_names().flatten().map(|n| (n.to_ascii_lowercase(), n.into_owned())).collect();
        Ok(Pkg { zip, names })
    }

    fn reader(&mut self, part: &str) -> Option<Reader<impl BufRead + '_>> {
        let name = self.names.get(&part.to_ascii_lowercase())?.clone();
        let f = self.zip.by_name(&name).ok()?;
        let mut r = Reader::from_reader(BufReader::new(f));
        let c = r.config_mut();
        c.check_end_names = false;
        c.check_comments = false;
        c.expand_empty_elements = false;
        c.trim_text(false);
        Some(r)
    }

    fn node(&mut self, part: &str) -> Option<Node> {
        parse(&mut self.reader(part)?).ok()
    }

    /// Relationships of a part (`""` for the package root), by id.
    fn rels(&mut self, part: &str) -> BTreeMap<String, Rel> {
        let (dir, file) = match part.rfind('/') {
            Some(i) => (&part[..=i], &part[i + 1..]),
            None => ("", part),
        };
        let mut out = BTreeMap::new();
        let Some(root) = self.node(&format!("{dir}_rels/{file}.rels")) else { return out };
        for r in root.children("Relationship") {
            let (Some(id), Some(target)) = (r.attr("Id"), r.attr("Target")) else { continue };
            let external = r.attr("TargetMode") == Some("External");
            out.insert(
                id.to_string(),
                Rel {
                    kind: r.attr("Type").unwrap_or("").rsplit('/').next().unwrap_or("").to_string(),
                    target: if external { target.to_string() } else { resolve(dir, target) },
                },
            );
        }
        out
    }
}

/// Resolve a relationship target against the source part's folder.
fn resolve(dir: &str, target: &str) -> String {
    let joined = match target.strip_prefix('/') {
        Some(abs) => abs.to_string(),
        None => format!("{dir}{target}"),
    };
    let mut parts: Vec<&str> = Vec::new();
    for p in joined.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(p),
        }
    }
    parts.join("/")
}

// ---------------------------------------------------------------- xml

/// A parsed element. Names are local (namespace prefixes dropped), so `r:id`
/// is `id` and `x14:formula1` is `formula1`.
#[derive(Debug, Default)]
struct Node {
    name: String,
    attrs: Vec<(String, String)>,
    kids: Vec<Node>,
    text: String,
}

impl Node {
    fn attr(&self, k: &str) -> Option<&str> {
        self.attrs.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str())
    }
    fn flag(&self, k: &str) -> bool {
        matches!(self.attr(k), Some("1" | "true"))
    }
    fn num(&self, k: &str) -> Option<u32> {
        self.attr(k).and_then(|v| v.parse::<f64>().ok()).filter(|v| *v >= 0.0).map(|v| v as u32)
    }
    fn child(&self, name: &str) -> Option<&Node> {
        self.kids.iter().find(|n| n.name == name)
    }
    fn children<'a, 's>(&'a self, name: &'s str) -> impl Iterator<Item = &'a Node> + use<'a, 's> {
        self.kids.iter().filter(move |n| n.name == name)
    }
    /// Every descendant with this name, in document order.
    fn all(&self, name: &str) -> Vec<&Node> {
        fn walk<'a>(n: &'a Node, name: &str, out: &mut Vec<&'a Node>) {
            for k in &n.kids {
                if k.name == name {
                    out.push(k);
                }
                walk(k, name, out);
            }
        }
        let mut out = Vec::new();
        walk(self, name, &mut out);
        out
    }
    /// Concatenated text of every `<t>` below (rich-text runs).
    fn runs(&self) -> String {
        self.all("t").iter().map(|t| t.text.as_str()).collect()
    }
}

fn local(e: &BytesStart) -> String {
    e.local_name().as_ref().to_string()
}

fn attrs_of(e: &BytesStart) -> Vec<(String, String)> {
    let mut it = e.attributes();
    it.with_checks(false);
    it.flatten()
        .map(|a| {
            let v = quick_xml::escape::unescape(&a.value).map(|v| v.into_owned()).unwrap_or_else(|_| a.value.to_string());
            (a.key.local_name().as_ref().to_string(), v)
        })
        .collect()
}

fn push_entity(r: &BytesRef, out: &mut String) {
    if let Some(s) = quick_xml::escape::resolve_xml_entity(r.as_ref()) {
        out.push_str(s);
    } else if let Ok(Some(c)) = r.resolve_char_ref() {
        out.push(c);
    }
}

fn parse<R: BufRead>(rd: &mut Reader<R>) -> Result<Node> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match rd.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let (n, a) = (local(&e), attrs_of(&e));
                return read_children(rd, n, a, &mut buf);
            }
            Event::Empty(e) => return Ok(Node { name: local(&e), attrs: attrs_of(&e), ..Default::default() }),
            Event::Eof => return Err(anyhow!("empty part")),
            _ => {}
        }
    }
}

fn read_children<R: BufRead>(rd: &mut Reader<R>, name: String, attrs: Vec<(String, String)>, buf: &mut Vec<u8>) -> Result<Node> {
    let mut node = Node { name, attrs, ..Default::default() };
    loop {
        buf.clear();
        match rd.read_event_into(buf)? {
            Event::Start(e) => {
                let (n, a) = (local(&e), attrs_of(&e));
                let kid = read_children(rd, n, a, buf)?;
                node.kids.push(kid);
            }
            Event::Empty(e) => node.kids.push(Node { name: local(&e), attrs: attrs_of(&e), ..Default::default() }),
            Event::Text(t) => node.text.push_str(&t.xml10_content()),
            Event::CData(t) => node.text.push_str(&t.xml10_content()),
            Event::GeneralRef(r) => push_entity(&r, &mut node.text),
            Event::End(_) | Event::Eof => return Ok(node),
            _ => {}
        }
    }
}

// ---------------------------------------------------------------- sheets

struct SheetCx<'a> {
    styles: &'a Styles,
    xf_ids: &'a [Option<u32>],
    dvs: &'a mut Interner<Validation>,
    skipped: &'a mut BTreeSet<&'static str>,
}

/// One pass over a worksheet part: cell style ids and hidden rows are taken
/// from `sheetData` as it streams by; every other top-level element is kept.
#[derive(Default)]
struct SheetXml {
    styled: Vec<(u32, u32, u32)>,
    hidden_rows: Vec<(u32, u32)>,
    nodes: Vec<Node>,
    /// Corrected text for shared formulas whose defining cell isn't the top-left
    /// of their range (see [`shared_formula`]).
    shared_fixes: Vec<(u32, u32, String)>,
    /// Summarized rows ([`crate::model::Elided`]): their cell styles are skipped.
    gap: Option<(u32, u32)>,
}

fn read_sheet_xml<R: BufRead>(rd: &mut Reader<R>, gap: Option<(u32, u32)>) -> Result<SheetXml> {
    let mut out = SheetXml { gap, ..Default::default() };
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match rd.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = local(&e);
                match name.as_str() {
                    "worksheet" | "chartsheet" | "dialogsheet" => {}
                    "sheetData" => read_sheet_data(rd, &mut out, &mut buf)?,
                    _ => {
                        let a = attrs_of(&e);
                        out.nodes.push(read_children(rd, name, a, &mut buf)?);
                    }
                }
            }
            Event::Empty(e) => out.nodes.push(Node { name: local(&e), attrs: attrs_of(&e), ..Default::default() }),
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

fn read_sheet_data<R: BufRead>(rd: &mut Reader<R>, out: &mut SheetXml, buf: &mut Vec<u8>) -> Result<()> {
    // `r` is optional on rows and cells; position then follows the previous one.
    let (mut row, mut next_col, mut started) = (0u32, 0u32, false);
    let (mut cell, mut masters) = ((0u32, 0u32), HashMap::new());
    loop {
        buf.clear();
        match rd.read_event_into(buf)? {
            Event::Start(e) if e.local_name().as_ref() == "f" => {
                let f = read_children(rd, "f".into(), attrs_of(&e), buf)?;
                shared_formula(&f, cell, &mut masters, &mut out.shared_fixes);
            }
            Event::Empty(e) if e.local_name().as_ref() == "f" => {
                let f = Node { name: "f".into(), attrs: attrs_of(&e), ..Default::default() };
                shared_formula(&f, cell, &mut masters, &mut out.shared_fixes);
            }
            Event::Start(e) | Event::Empty(e) => match e.local_name().as_ref() {
                "row" => {
                    let (mut r, mut hidden) = (None, false);
                    for a in e.attributes().flatten() {
                        match a.key.local_name().as_ref() {
                            "r" => r = a.value.parse::<u32>().ok().filter(|r| *r > 0),
                            "hidden" => hidden = matches!(a.value.as_ref(), "1" | "true"),
                            _ => {}
                        }
                    }
                    row = r.map(|r| r - 1).unwrap_or(if started { row + 1 } else { 0 });
                    started = true;
                    next_col = 0;
                    if hidden {
                        push_span(&mut out.hidden_rows, row);
                    }
                }
                "c" => {
                    let (mut pos, mut s) = (None, None);
                    for a in e.attributes().flatten() {
                        match a.key.local_name().as_ref() {
                            "r" => pos = parse_pt(&a.value).and_then(|p| Some((p.row?, p.col?))),
                            "s" => s = a.value.parse::<u32>().ok(),
                            _ => {}
                        }
                    }
                    let (r, c) = pos.unwrap_or((row, next_col));
                    next_col = c + 1;
                    cell = (r, c);
                    if let Some(s) = s.filter(|s| *s != 0)
                        && !out.gap.is_some_and(|(a, b)| (a..=b).contains(&r))
                    {
                        out.styled.push((r, c, s));
                    }
                }
                _ => {}
            },
            Event::End(e) if e.local_name().as_ref() == "sheetData" => return Ok(()),
            Event::Eof => return Ok(()),
            _ => {}
        }
    }
}

/// Shared formula index → its defining cell and text, or `None` when that cell
/// is the top-left of the formula's range.
type Masters = HashMap<String, Option<((u32, u32), String)>>;

/// calamine shifts a shared formula's copies from the top-left of its `ref`
/// range, but Excel shifts them from the cell that holds the formula text, and
/// the two differ when another cell in the range has its own formula. Record
/// the right text for those copies; every other shared formula is left alone.
fn shared_formula(f: &Node, at: (u32, u32), masters: &mut Masters, fixes: &mut Vec<(u32, u32, String)>) {
    let (Some("shared"), Some(si)) = (f.attr("t"), f.attr("si")) else { return };
    match f.attr("ref") {
        Some(r) => {
            let start = parse_rect(r).map(|x| (x.r0, x.c0));
            masters.insert(si.to_string(), (start.is_some_and(|s| s != at)).then(|| (at, f.text.clone())));
        }
        None => {
            if let Some(Some((from, text))) = masters.get(si)
                && let Ok(x) = calamine::expand_shared_formula(text, *from, at)
            {
                fixes.push((at.0, at.1, format!("={x}")));
            }
        }
    }
}

fn push_span(spans: &mut Vec<(u32, u32)>, i: u32) {
    match spans.last_mut() {
        Some(last) if last.1 + 1 == i => last.1 = i,
        _ => spans.push((i, i)),
    }
}

fn read_sheet(pkg: &mut Pkg, part: &str, kind: &str, sh: &mut Sheet, cx: &mut SheetCx) -> Result<()> {
    let sx = {
        let mut rd = pkg.reader(part).ok_or_else(|| anyhow!("missing part {part}"))?;
        read_sheet_xml(&mut rd, sh.elided.as_ref().map(|e| e.rows))?
    };
    let rels = pkg.rels(part);
    if kind == "chartsheet" {
        sh.kind = "OBJECT".into();
    }
    // cell_at binary-searches by position (xlsx::read already sorts).
    if !sh.cells.is_sorted_by_key(|c| (c.r, c.c)) {
        sh.cells.sort_by_key(|c| (c.r, c.c));
    }
    let mut at = Added { gap: sx.gap, ..Default::default() };

    for &(r, c, xf) in &sx.styled {
        if let Some(Some(id)) = cx.xf_ids.get(xf as usize)
            && let Some(x) = cell_at(&mut sh.cells, &mut at, r, c)
        {
            x.s = Some(*id);
        }
    }
    for (r, c, f) in sx.shared_fixes {
        if let Some(x) = cell_at(&mut sh.cells, &mut at, r, c) {
            x.f = Some(f);
        }
    }
    sh.hidden_rows = sx.hidden_rows;
    // Rules often cover whole columns; apply them only as far as the sheet has cells.
    let ext = sh.cells.iter().chain(&at.cells).fold((0u32, 0u32), |(r, c), x| (r.max(x.r), c.max(x.c)));
    let clamp = |rect: Rect| Rect { r1: rect.r1.min(ext.0.max(rect.r0)), c1: rect.c1.min(ext.1.max(rect.c0)), ..rect };

    // x14 rules that only extend a main rule (data bars, icon sets) are skipped.
    let main_ids: Vec<String> = sx
        .nodes
        .iter()
        .filter(|n| n.name == "conditionalFormatting")
        .flat_map(|n| n.all("id"))
        .map(|n| n.text.trim().to_string())
        .collect();

    for n in &sx.nodes {
        match n.name.as_str() {
            "sheetPr" => sh.tab_color = n.child("tabColor").and_then(|c| cx.styles.colors.of(c)),
            "sheetViews" => {
                if let Some(p) = n.child("sheetView").and_then(|v| v.child("pane")).filter(|p| p.attr("state").is_some_and(|s| s.starts_with("frozen"))) {
                    sh.frozen_rows = p.num("ySplit").unwrap_or(0);
                    sh.frozen_cols = p.num("xSplit").unwrap_or(0);
                }
            }
            "cols" => {
                for col in n.children("col").filter(|c| c.flag("hidden")) {
                    if let (Some(a), Some(b)) = (col.num("min"), col.num("max")) {
                        sh.hidden_cols.push((a.saturating_sub(1), b.saturating_sub(1)));
                    }
                }
            }
            "dataValidations" => {
                for dv in n.children("dataValidation") {
                    let f = |k: &str| dv.child(k).map(|x| x.text.clone());
                    apply_validation(sh, &mut at, cx, dv, f("formula1"), f("formula2"), dv.attr("sqref").unwrap_or(""), &clamp);
                }
            }
            "conditionalFormatting" => {
                let ranges = sqref(n.attr("sqref").unwrap_or(""));
                for rule in n.children("cfRule") {
                    sh.cond_formats.push(CondFormat { ranges: ranges.clone(), rule: cf_rule(rule, cx.styles) });
                }
            }
            "hyperlinks" => {
                for h in n.children("hyperlink") {
                    let target = h.attr("id").and_then(|id| rels.get(id)).map(|r| r.target.clone());
                    let link = match (target, h.attr("location")) {
                        (Some(t), Some(l)) => format!("{t}#{l}"),
                        (Some(t), None) => t,
                        (None, Some(l)) => format!("#{l}"),
                        (None, None) => continue,
                    };
                    for rect in sqref(h.attr("ref").unwrap_or("")) {
                        for_cells(clamp(rect), |r, c| {
                            if let Some(x) = cell_at(&mut sh.cells, &mut at, r, c) {
                                x.link = Some(link.clone());
                            }
                        });
                    }
                }
            }
            "autoFilter" => sh.filters.extend(auto_filter(n)),
            "sheetProtection" if n.flag("sheet") => sh.protected.push(Protected::default()),
            "tableParts" => {
                for tp in n.children("tablePart") {
                    let Some(rel) = tp.attr("id").and_then(|id| rels.get(id)) else { continue };
                    let Some(t) = pkg.node(&rel.target) else { continue };
                    if let Some(tbl) = table(&t) {
                        if let Some(f) = t.child("autoFilter").and_then(auto_filter).filter(|f| !f.criteria.is_empty()) {
                            sh.filters.push(f);
                        }
                        sh.tables.push(tbl);
                    }
                }
            }
            "drawing" => {
                if let Some(rel) = n.attr("id").and_then(|id| rels.get(id)) {
                    let target = rel.target.clone();
                    sh.charts.extend(charts(pkg, &target));
                }
            }
            "extLst" => {
                for dv in n.all("dataValidation") {
                    let f = |k: &str| dv.child(k).and_then(|x| x.child("f")).map(|x| x.text.clone());
                    let refs = dv.child("sqref").map(|s| s.text.clone()).unwrap_or_default();
                    apply_validation(sh, &mut at, cx, dv, f("formula1"), f("formula2"), &refs, &clamp);
                }
                for cf in n.all("conditionalFormatting") {
                    let ranges = sqref(&cf.child("sqref").map(|s| s.text.clone()).unwrap_or_default());
                    for rule in cf.children("cfRule").filter(|r| !r.attr("id").is_some_and(|id| main_ids.iter().any(|m| m == id))) {
                        sh.cond_formats.push(CondFormat { ranges: ranges.clone(), rule: cf_rule(rule, cx.styles) });
                    }
                }
                if !n.all("sparklineGroup").is_empty() {
                    cx.skipped.insert("sparklines");
                }
            }
            _ => {}
        }
    }

    // Notes: legacy comments, replaced by threaded comments where a cell has them.
    let mut notes: BTreeMap<(u32, u32), String> = BTreeMap::new();
    for rel in rels.values().filter(|r| r.kind == "comments") {
        let Some(n) = pkg.node(&rel.target) else { continue };
        for c in n.all("comment") {
            if let Some(p) = c.attr("ref").and_then(cell_pos) {
                notes.insert(p, c.runs().trim().to_string());
            }
        }
    }
    let mut threads: BTreeMap<(u32, u32), Vec<String>> = BTreeMap::new();
    for rel in rels.values().filter(|r| r.kind == "threadedComment") {
        let Some(n) = pkg.node(&rel.target) else { continue };
        for c in n.all("threadedComment") {
            if let Some(p) = c.attr("ref").and_then(cell_pos) {
                threads.entry(p).or_default().push(c.child("text").map(|t| t.text.trim().to_string()).unwrap_or_default());
            }
        }
    }
    for (p, msgs) in threads {
        notes.insert(p, msgs.join("\n↳ "));
    }
    // Notes are few and say things nothing else does: keep them in summarized rows too.
    at.gap = None;
    for ((r, c), text) in notes {
        if let Some(x) = cell_at(&mut sh.cells, &mut at, r, c) {
            x.note = Some(text);
        }
    }

    for rel in rels.values().filter(|r| r.kind == "pivotTable") {
        let target = rel.target.clone();
        if let Some(p) = pivot(pkg, &target) {
            sh.pivots.push(p);
        }
    }
    sh.cells.reserve_exact(at.cells.len());
    sh.cells.append(&mut at.cells);
    Ok(())
}

/// Cells `read_sheet` creates (styled blanks, notes, links...), kept apart and
/// appended once at the end: pushing them one by one onto a sheet's millions
/// of cells would double that Vec's allocation.
#[derive(Default)]
struct Added {
    cells: Vec<Cell>,
    at: HashMap<(u32, u32), usize>,
    /// Summarized rows, where no cell is added.
    gap: Option<(u32, u32)>,
}

/// The cell at (r, c), found in the sheet's sorted cells or added (never in
/// summarized rows, whose values are not kept).
fn cell_at<'a>(cells: &'a mut [Cell], added: &'a mut Added, r: u32, c: u32) -> Option<&'a mut Cell> {
    if let Ok(i) = cells.binary_search_by_key(&(r, c), |x| (x.r, x.c)) {
        return Some(&mut cells[i]);
    }
    if added.gap.is_some_and(|(a, b)| (a..=b).contains(&r)) {
        return None;
    }
    let i = *added.at.entry((r, c)).or_insert_with(|| {
        added.cells.push(Cell { r, c, ..Default::default() });
        added.cells.len() - 1
    });
    Some(&mut added.cells[i])
}

fn for_cells(rect: Rect, mut f: impl FnMut(u32, u32)) {
    for r in rect.r0..=rect.r1 {
        for c in rect.c0..=rect.c1 {
            f(r, c);
        }
    }
}

fn cell_pos(a1: &str) -> Option<(u32, u32)> {
    let p = parse_pt(a1)?;
    Some((p.row?, p.col?))
}

/// Space-separated ranges; Excel's last row/column become open edges (`A2:A`).
fn sqref(s: &str) -> Vec<Rect> {
    s.split_whitespace()
        .filter_map(|r| parse_rect(&r.replace('$', "")))
        .map(|mut r| {
            if r.r1 >= 1_048_575 {
                r.r1 = OPEN;
            }
            if r.c1 >= 16_383 {
                r.c1 = OPEN;
            }
            r
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn apply_validation(
    sh: &mut Sheet,
    at: &mut Added,
    cx: &mut SheetCx,
    n: &Node,
    f1: Option<String>,
    f2: Option<String>,
    refs: &str,
    clamp: &dyn Fn(Rect) -> Rect,
) {
    let Some(v) = validation(n, f1, f2) else { return };
    let id = cx.dvs.intern(v);
    for rect in sqref(refs) {
        for_cells(clamp(rect), |r, c| {
            if let Some(x) = cell_at(&mut sh.cells, at, r, c) {
                x.dv = Some(id);
            }
        });
    }
}

fn validation(n: &Node, f1: Option<String>, f2: Option<String>) -> Option<Validation> {
    let ty = n.attr("type").unwrap_or("none");
    let (kind, values) = match ty {
        "list" => match f1.as_deref().map(str::trim) {
            Some(f) if f.starts_with('"') => ("ONE_OF_LIST".to_string(), f.trim_matches('"').split(',').map(|s| s.trim().to_string()).collect()),
            Some(f) => ("ONE_OF_RANGE".to_string(), vec![f.replace('$', "")]),
            None => ("ONE_OF_LIST".to_string(), Vec::new()),
        },
        "custom" => ("CUSTOM_FORMULA".to_string(), f1.map(|f| format!("={f}")).into_iter().collect()),
        "whole" | "decimal" | "date" | "time" | "textLength" => {
            let pre = match ty {
                "whole" => "WHOLE_NUMBER",
                "decimal" => "NUMBER",
                "date" => "DATE",
                "time" => "TIME",
                _ => "TEXT_LENGTH",
            };
            let op = match n.attr("operator").unwrap_or("between") {
                "notBetween" => "NOT_BETWEEN",
                "equal" => "EQ",
                "notEqual" => "NOT_EQ",
                "greaterThan" => "GREATER",
                "greaterThanOrEqual" => "GREATER_THAN_EQ",
                "lessThan" => "LESS",
                "lessThanOrEqual" => "LESS_THAN_EQ",
                _ => "BETWEEN",
            };
            (format!("{pre}_{op}"), [f1, f2].into_iter().flatten().collect())
        }
        _ => ("ANY".to_string(), Vec::new()),
    };
    let help = match (n.attr("promptTitle"), n.attr("prompt")) {
        (Some(t), Some(p)) => Some(format!("{t}: {p}")),
        (None, Some(p)) => Some(p.to_string()),
        (Some(t), None) => Some(t.to_string()),
        (None, None) => None,
    };
    // An "any value" rule only matters when it shows an input message.
    if kind == "ANY" && help.is_none() {
        return None;
    }
    Some(Validation {
        strict: n.flag("showErrorMessage") && !matches!(n.attr("errorStyle"), Some("warning" | "information")),
        // showDropDown="1" *hides* the in-cell arrow.
        dropdown: ty == "list" && !n.flag("showDropDown"),
        kind,
        values,
        help,
    })
}

fn cf_rule(rule: &Node, st: &Styles) -> String {
    let ty = rule.attr("type").unwrap_or("?");
    // Main rules hold `<formula>`, x14 rules `<xm:f>`.
    let fs: Vec<&str> = rule.kids.iter().filter(|k| k.name == "formula" || k.name == "f").map(|k| k.text.as_str()).collect();
    let fmt = match rule.child("dxf") {
        Some(d) => st.dxf_brief(d),
        None => rule.attr("dxfId").and_then(|i| i.parse::<usize>().ok()).and_then(|i| st.dxfs.get(i)).cloned().unwrap_or_default(),
    };
    let cond = match ty {
        "cellIs" => format!("cell {} {}", rule.attr("operator").unwrap_or("?"), fs.join(" and ")),
        "expression" => format!("={}", fs.first().copied().unwrap_or("")),
        "colorScale" => {
            let Some(cs) = rule.child("colorScale") else { return "color scale".into() };
            let pts: Vec<String> = cs
                .children("cfvo")
                .zip(cs.children("color"))
                .map(|(v, c)| {
                    let val = v.attr("val").map(|x| x.to_string()).or_else(|| v.child("f").map(|f| f.text.clone()));
                    format!("{}{}:{}", v.attr("type").unwrap_or(""), val.map(|x| format!(" {x}")).unwrap_or_default(), st.colors.of(c).unwrap_or_default())
                })
                .collect();
            return format!("color scale {}", pts.join(" → "));
        }
        "dataBar" => {
            let color = rule.child("dataBar").and_then(|d| d.child("color").or_else(|| d.child("fillColor"))).and_then(|c| st.colors.of(c));
            return format!("data bar {}", color.unwrap_or_default()).trim_end().to_string();
        }
        "iconSet" => return format!("icon set {}", rule.child("iconSet").and_then(|i| i.attr("iconSet")).unwrap_or("3TrafficLights1")),
        "top10" => format!(
            "{} {}{}",
            if rule.flag("bottom") { "bottom" } else { "top" },
            rule.attr("rank").unwrap_or("10"),
            if rule.flag("percent") { "%" } else { "" }
        ),
        "aboveAverage" => (if rule.attr("aboveAverage") == Some("0") { "below average" } else { "above average" }).to_string(),
        "containsText" | "notContainsText" | "beginsWith" | "endsWith" => format!("{ty} \"{}\"", rule.attr("text").unwrap_or("")),
        "timePeriod" => format!("date {}", rule.attr("timePeriod").unwrap_or("?")),
        other => other.to_string(),
    };
    if fmt.is_empty() { cond } else { format!("{cond} → {fmt}") }
}

fn auto_filter(n: &Node) -> Option<Filter> {
    let range = parse_rect(&n.attr("ref")?.replace('$', ""))?;
    let mut criteria = Vec::new();
    for fc in n.children("filterColumn") {
        let col = range.c0 + fc.num("colId").unwrap_or(0);
        let bit = if let Some(f) = fc.child("filters") {
            let vals: Vec<&str> = f.children("filter").filter_map(|x| x.attr("val")).collect();
            let shown = if vals.len() <= 5 { format!("[{}]", vals.join(", ")) } else { format!("{} values", vals.len()) };
            format!("show {shown}{}", if f.flag("blank") { " + blanks" } else { "" })
        } else if let Some(cf) = fc.child("customFilters") {
            let join = if cf.flag("and") { " and " } else { " or " };
            cf.children("customFilter").map(|c| format!("{} {}", c.attr("operator").unwrap_or("equal"), c.attr("val").unwrap_or(""))).collect::<Vec<_>>().join(join)
        } else if let Some(t) = fc.child("top10") {
            format!("{} {}{}", if t.attr("top") == Some("0") { "bottom" } else { "top" }, t.attr("val").unwrap_or("10"), if t.flag("percent") { "%" } else { "" })
        } else if let Some(d) = fc.child("dynamicFilter") {
            d.attr("type").unwrap_or("dynamic").to_string()
        } else if fc.child("colorFilter").is_some() {
            "by color".to_string()
        } else {
            continue;
        };
        criteria.push(format!("{}: {bit}", col_name(col)));
    }
    Some(Filter { range, title: None, criteria })
}

fn table(n: &Node) -> Option<Table> {
    Some(Table {
        name: n.attr("displayName").or(n.attr("name"))?.to_string(),
        range: parse_rect(&n.attr("ref")?.replace('$', ""))?,
        columns: n.child("tableColumns").map(|c| c.children("tableColumn").filter_map(|x| x.attr("name").map(String::from)).collect()).unwrap_or_default(),
        totals: n.num("totalsRowCount").unwrap_or(0) > 0,
    })
}

// ---------------------------------------------------------------- charts & pivots

/// Charts on a drawing part, in the order they're anchored.
fn charts(pkg: &mut Pkg, drawing: &str) -> Vec<Chart> {
    let rels = pkg.rels(drawing);
    let Some(d) = pkg.node(drawing) else { return Vec::new() };
    let mut out = Vec::new();
    for c in d.all("chart") {
        let Some(rel) = c.attr("id").and_then(|id| rels.get(id)).filter(|r| r.kind == "chart") else { continue };
        let target = rel.target.clone();
        if let Some(n) = pkg.node(&target) {
            let id = target.rsplit('/').next().unwrap_or(&target).trim_end_matches(".xml").to_string();
            out.push(chart(&n, id));
        }
    }
    out
}

fn chart(n: &Node, id: String) -> Chart {
    let c = n.child("chart");
    let title = c
        .and_then(|c| c.child("title"))
        .map(|t| {
            let text = t.runs();
            if text.is_empty() { t.all("f").first().map(|f| format!("={}", f.text)).unwrap_or_default() } else { text }
        })
        .unwrap_or_default();
    let plot = c.and_then(|c| c.child("plotArea"));
    let mut kinds: Vec<String> = Vec::new();
    for k in plot.map(|p| p.kids.iter().filter(|k| k.name.ends_with("Chart")).collect::<Vec<_>>()).unwrap_or_default() {
        let name = match k.name.as_str() {
            "barChart" | "bar3DChart" => {
                if k.child("barDir").and_then(|d| d.attr("val")) == Some("bar") { "bar".to_string() } else { "column".to_string() }
            }
            "ofPieChart" => "pie-of-pie".to_string(),
            other => other.trim_end_matches("Chart").trim_end_matches("3D").to_string(),
        };
        if !kinds.contains(&name) {
            kinds.push(name);
        }
    }
    let mut sources: Vec<String> = Vec::new();
    for f in plot.map(|p| p.all("f")).unwrap_or_default() {
        let s = f.text.replace('$', "");
        if !s.is_empty() && !sources.contains(&s) {
            sources.push(s);
        }
    }
    Chart { id, title, kind: if kinds.is_empty() { "chart".into() } else { kinds.join("+") }, sources }
}

fn pivot(pkg: &mut Pkg, part: &str) -> Option<Pivot> {
    let n = pkg.node(part)?;
    let cache_part = pkg.rels(part).into_values().find(|r| r.kind == "pivotCacheDefinition").map(|r| r.target);
    let cache = cache_part.and_then(|p| pkg.node(&p));
    let fields: Vec<String> = cache
        .as_ref()
        .and_then(|c| c.child("cacheFields"))
        .map(|f| f.children("cacheField").map(|x| x.attr("name").unwrap_or("?").to_string()).collect())
        .unwrap_or_default();
    let field = |x: Option<&str>| x.and_then(|v| v.parse::<usize>().ok()).and_then(|i| fields.get(i)).cloned();
    let source = cache
        .as_ref()
        .and_then(|c| c.child("cacheSource"))
        .map(|s| match s.child("worksheetSource") {
            Some(w) => match (w.attr("sheet"), w.attr("ref"), w.attr("name")) {
                (Some(sh), Some(r), _) => format!("{}!{}", quote_sheet(sh), r),
                (_, _, Some(name)) => name.to_string(),
                (None, Some(r), None) => r.to_string(),
                _ => "?".to_string(),
            },
            None => format!("{} source", s.attr("type").unwrap_or("?")),
        })
        .unwrap_or_else(|| "?".into());
    let loc = parse_rect(n.child("location")?.attr("ref")?)?;
    // `x="-2"` is the "Values" pseudo-field, not a column.
    let list = |k: &str, item: &str, key: &str| -> Vec<String> {
        n.child(k).map(|l| l.children(item).filter_map(|x| field(x.attr(key))).collect()).unwrap_or_default()
    };
    Some(Pivot {
        anchor: (loc.r0, loc.c0),
        source,
        rows: list("rowFields", "field", "x"),
        cols: list("colFields", "field", "x"),
        values: n
            .child("dataFields")
            .map(|d| {
                d.children("dataField")
                    .map(|x| format!("{}({})", x.attr("subtotal").unwrap_or("sum").to_ascii_uppercase(), field(x.attr("fld")).unwrap_or_else(|| "?".into())))
                    .collect()
            })
            .unwrap_or_default(),
        filters: list("pageFields", "pageField", "fld"),
    })
}

// ---------------------------------------------------------------- external links

/// Linked workbooks in `externalReferences` order: formulas say `[1]`, `[2]`, …
fn external_links(pkg: &mut Pkg, root: &Node, rels: &BTreeMap<String, Rel>) -> Vec<ExternalLink> {
    let Some(refs) = root.child("externalReferences") else { return Vec::new() };
    let mut out = Vec::new();
    for (i, e) in refs.children("externalReference").enumerate() {
        let mut link = ExternalLink::default();
        if let Some(part) = e.attr("id").and_then(|id| rels.get(id)).map(|r| r.target.clone()) {
            let lrels = pkg.rels(&part);
            if let Some(n) = pkg.node(&part) {
                if let Some(book) = n.child("externalBook") {
                    link.target = book.attr("id").and_then(|id| lrels.get(id)).map(|r| r.target.clone()).unwrap_or_default();
                    link.sheets = book.child("sheetNames").map(|s| s.children("sheetName").filter_map(|x| x.attr("val").map(String::from)).collect()).unwrap_or_default();
                } else if let Some(d) = n.child("ddeLink") {
                    link.target = format!("DDE {} {}", d.attr("ddeService").unwrap_or(""), d.attr("ddeTopic").unwrap_or(""));
                } else if n.child("oleLink").is_some() {
                    link.target = "OLE link".into();
                }
            }
        }
        let decoded = percent_decode(&link.target);
        link.name = decoded.rsplit(['/', '\\']).next().filter(|s| !s.is_empty()).map(String::from).unwrap_or_else(|| format!("link{}", i + 1));
        out.push(link);
    }
    out
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len()
            && let Some(v) = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b >= 0x80
}

/// Rewrite Excel's stored external references to the form Excel displays:
/// `[1]Data!A1` → `'[Budget.xlsx]Data'!A1`, `'[1]My Data'!A1` → `'[Budget.xlsx]My Data'!A1`,
/// `[1]!Rate` → `'[Budget.xlsx]'!Rate`. String literals and table refs are left alone.
pub fn name_external_refs(f: &str, links: &[ExternalLink]) -> String {
    let book = |idx: &str| -> Option<String> {
        let i = idx.parse::<usize>().ok().filter(|i| *i >= 1)?;
        links.get(i - 1).map(|l| l.name.replace('\'', "''"))
    };
    let b = f.as_bytes();
    let mut out = String::with_capacity(f.len() + 16);
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            q @ (b'"' | b'\'') => {
                let s = i;
                i += 1;
                while i < b.len() {
                    if b[i] == q {
                        if b.get(i + 1) == Some(&q) {
                            i += 2;
                            continue;
                        }
                        break;
                    }
                    i += 1;
                }
                i = (i + 1).min(b.len());
                let tok = &f[s..i];
                // '[1]Sheet name'
                if q == b'\'' && tok.len() > 3 && tok.as_bytes()[1] == b'['
                    && let Some(j) = tok[2..].find(']')
                        && let Some(name) = book(&tok[2..2 + j]) {
                            out.push_str(&format!("'[{name}]{}", &tok[2 + j + 1..]));
                            continue;
                        }
                out.push_str(tok);
            }
            b'[' => {
                let structured = i > 0 && (is_name_byte(b[i - 1]) || b[i - 1] == b']');
                let mut depth = 0;
                let mut j = i;
                while j < b.len() {
                    match b[j] {
                        b'[' => depth += 1,
                        b']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                let end = (j + 1).min(b.len());
                let name = if structured { None } else { book(&f[i + 1..j.min(b.len())]) };
                match name {
                    Some(name) if b.get(end) == Some(&b'!') => {
                        out.push_str(&format!("'[{name}]'!"));
                        i = end + 1;
                    }
                    Some(name) => {
                        let mut k = end;
                        while k < b.len() && is_name_byte(b[k]) {
                            k += 1;
                        }
                        if b.get(k) == Some(&b'!') {
                            out.push_str(&format!("'[{name}]{}'!", &f[end..k]));
                            i = k + 1;
                        } else {
                            out.push_str(&f[i..end]);
                            i = end;
                        }
                    }
                    None => {
                        out.push_str(&f[i..end]);
                        i = end;
                    }
                }
            }
            _ => {
                let ch = f[i..].chars().next().unwrap_or(' ');
                out.push(ch);
                i += ch.len_utf8();
            }
        }
    }
    out
}

// ---------------------------------------------------------------- styles

struct Styles {
    xfs: Vec<Style>,
    /// Differential formats (conditional formatting), already summarized.
    dxfs: Vec<String>,
    colors: Colors,
    fmts: HashMap<u32, String>,
}

impl Styles {
    fn parse(n: &Node, theme: Vec<String>) -> Styles {
        let mut colors = Colors { theme, indexed: PALETTE.iter().map(|s| s.to_string()).collect() };
        if let Some(ic) = n.child("colors").and_then(|c| c.child("indexedColors")) {
            let custom: Vec<String> = ic.children("rgbColor").filter_map(|c| c.attr("rgb")).map(hex6).collect();
            if !custom.is_empty() {
                colors.indexed = custom;
            }
        }
        let mut fmts: HashMap<u32, String> = BUILTIN_FMTS.iter().map(|(i, f)| (*i, f.to_string())).collect();
        for f in n.child("numFmts").map(|x| x.children("numFmt").collect::<Vec<_>>()).unwrap_or_default() {
            if let (Some(id), Some(code)) = (f.num("numFmtId"), f.attr("formatCode")) {
                fmts.insert(id, code.to_string());
            }
        }
        let list = |k: &str, item: &str| -> Vec<&Node> { n.child(k).map(|x| x.children(item).collect()).unwrap_or_default() };
        let (fonts, fills, borders) = (list("fonts", "font"), list("fills", "fill"), list("borders", "border"));
        let val = |f: Option<&Node>, k: &str| f.and_then(|f| f.child(k)).and_then(|x| x.attr("val")).map(String::from);
        let (def_name, def_size) = (val(fonts.first().copied(), "name"), val(fonts.first().copied(), "sz"));

        let mut st = Styles { xfs: Vec::new(), dxfs: Vec::new(), colors, fmts };
        for xf in list("cellXfs", "xf") {
            let id = |k: &str| xf.num(k).unwrap_or(0) as usize;
            let font = fonts.get(id("fontId")).copied();
            let num_fmt = st.fmts.get(&(id("numFmtId") as u32)).filter(|c| !c.eq_ignore_ascii_case("general")).cloned();
            let align = xf.child("alignment");
            st.xfs.push(Style {
                num_type: num_fmt.as_deref().and_then(num_type).map(String::from),
                num_fmt,
                bg: fills.get(id("fillId")).and_then(|f| st.fill_color(f)).filter(|c| c != "#FFFFFF"),
                fg: font.and_then(|f| f.child("color")).and_then(|c| st.colors.of(c)).filter(|c| c != "#000000"),
                bold: font.is_some_and(|f| on(f, "b")),
                italic: font.is_some_and(|f| on(f, "i")),
                strike: font.is_some_and(|f| on(f, "strike")),
                underline: font.is_some_and(|f| on(f, "u")),
                borders: borders.get(id("borderId")).is_some_and(|b| b.kids.iter().any(|s| s.attr("style").is_some_and(|v| v != "none"))),
                h_align: align.and_then(|a| a.attr("horizontal")).filter(|h| !matches!(*h, "general" | "left")).map(|h| h.to_ascii_uppercase()),
                font: val(font, "name").filter(|f| Some(f) != def_name.as_ref()),
                size: val(font, "sz").filter(|s| Some(s) != def_size.as_ref()).and_then(|s| s.parse::<f64>().ok()).map(|s| s.round() as u32),
                wrap: align.is_some_and(|a| a.flag("wrapText")),
            });
        }
        st.dxfs = list("dxfs", "dxf").into_iter().map(|d| st.dxf_brief(d)).collect();
        st
    }

    fn fill_color(&self, fill: &Node) -> Option<String> {
        let p = fill.child("patternFill")?;
        if p.attr("patternType") != Some("solid") {
            return None;
        }
        self.colors.of(p.child("fgColor")?)
    }

    /// `bg#FFC7CE fg#9C0006 bold fmt 0.0%`, the shape gsheets uses for rules.
    fn dxf_brief(&self, d: &Node) -> String {
        let mut out = Vec::new();
        // In a dxf a solid fill's colour is usually in bgColor.
        if let Some(p) = d.child("fill").and_then(|f| f.child("patternFill"))
            && let Some(c) = p.child("bgColor").or_else(|| p.child("fgColor")).and_then(|c| self.colors.of(c)) {
                out.push(format!("bg{c}"));
            }
        if let Some(f) = d.child("font") {
            if let Some(c) = f.child("color").and_then(|c| self.colors.of(c)) {
                out.push(format!("fg{c}"));
            }
            for (k, name) in [("b", "bold"), ("i", "italic"), ("strike", "strike"), ("u", "underline")] {
                if on(f, k) {
                    out.push(name.to_string());
                }
            }
        }
        if let Some(code) = d.child("numFmt").and_then(|n| n.attr("formatCode")) {
            out.push(format!("fmt {code}"));
        }
        out.join(" ")
    }
}

/// A boolean font property: `<b/>` is on, `<b val="0"/>` is off.
fn on(font: &Node, k: &str) -> bool {
    font.child(k).is_some_and(|x| !matches!(x.attr("val"), Some("0" | "false" | "none")))
}

/// Number format → the type names the Sheets API uses.
fn num_type(code: &str) -> Option<&'static str> {
    // First section only, without literals, escapes, padding and [colour]/[$-locale] tags.
    let mut s = String::new();
    let mut chars = code.chars().peekable();
    let mut currency = false;
    while let Some(ch) = chars.next() {
        match ch {
            ';' => break,
            '"' => {
                for c in chars.by_ref() {
                    if c == '"' {
                        break;
                    }
                }
            }
            '\\' | '_' | '*' => {
                let lit = chars.next();
                currency |= ch == '\\' && lit.is_some_and(|c| "$€£¥₹".contains(c));
            }
            '[' => {
                let tag: String = chars.by_ref().take_while(|c| *c != ']').collect();
                let t = tag.to_ascii_lowercase();
                if t.starts_with('$') && tag.len() > 1 && !tag[1..].starts_with('-') {
                    currency = true;
                } else if matches!(t.as_str(), "h" | "hh" | "m" | "mm" | "s" | "ss") {
                    s.push('h');
                }
            }
            '$' | '€' | '£' | '¥' | '₹' => currency = true,
            _ => s.push(ch.to_ascii_lowercase()),
        }
    }
    if s.trim() == "@" {
        return Some("TEXT");
    }
    let date = s.contains('y') || s.contains('d') || (s.contains('m') && !s.contains('h') && !s.contains('s') && !s.contains('0') && !s.contains('#'));
    let time = s.contains('h') || s.contains('s') || s.contains("am/pm") || s.contains("a/p");
    Some(match (date, time) {
        (true, true) => "DATE_TIME",
        (true, false) => "DATE",
        (false, true) => "TIME",
        _ if s.contains('%') => "PERCENT",
        _ if currency => "CURRENCY",
        _ if s.contains("e+") || s.contains("e-") => "SCIENTIFIC",
        _ => "NUMBER",
    })
}

const BUILTIN_FMTS: &[(u32, &str)] = &[
    (1, "0"),
    (2, "0.00"),
    (3, "#,##0"),
    (4, "#,##0.00"),
    (5, "$#,##0_);($#,##0)"),
    (6, "$#,##0_);[Red]($#,##0)"),
    (7, "$#,##0.00_);($#,##0.00)"),
    (8, "$#,##0.00_);[Red]($#,##0.00)"),
    (9, "0%"),
    (10, "0.00%"),
    (11, "0.00E+00"),
    (12, "# ?/?"),
    (13, "# ??/??"),
    (14, "m/d/yyyy"),
    (15, "d-mmm-yy"),
    (16, "d-mmm"),
    (17, "mmm-yy"),
    (18, "h:mm AM/PM"),
    (19, "h:mm:ss AM/PM"),
    (20, "h:mm"),
    (21, "h:mm:ss"),
    (22, "m/d/yyyy h:mm"),
    (37, "#,##0 ;(#,##0)"),
    (38, "#,##0 ;[Red](#,##0)"),
    (39, "#,##0.00;(#,##0.00)"),
    (40, "#,##0.00;[Red](#,##0.00)"),
    (45, "mm:ss"),
    (46, "[h]:mm:ss"),
    (47, "mmss.0"),
    (48, "##0.0E+0"),
    (49, "@"),
];

// ---------------------------------------------------------------- colours

struct Colors {
    /// RRGGBB by Excel's theme index: lt1, dk1, lt2, dk2, accent1–6, hlink, folHlink.
    theme: Vec<String>,
    indexed: Vec<String>,
}

impl Colors {
    /// `#RRGGBB` for an rgb / theme (+tint) / indexed colour element.
    fn of(&self, n: &Node) -> Option<String> {
        if n.flag("auto") {
            return None;
        }
        let base = if let Some(rgb) = n.attr("rgb") {
            hex6(rgb)
        } else if let Some(t) = n.num("theme") {
            self.theme.get(t as usize)?.clone()
        } else {
            let x = n.num("indexed")?;
            // 64/65 are the system foreground/background: "automatic".
            self.indexed.get(x as usize)?.clone()
        };
        let tint = n.attr("tint").and_then(|t| t.parse::<f64>().ok()).unwrap_or(0.0);
        Some(format!("#{}", if tint == 0.0 { base } else { apply_tint(&base, tint) }))
    }
}

fn hex6(argb: &str) -> String {
    argb[argb.len().saturating_sub(6)..].to_ascii_uppercase()
}

fn theme_colors(n: &Node) -> Vec<String> {
    let Some(scheme) = n.all("clrScheme").into_iter().next() else { return default_theme() };
    ["lt1", "dk1", "lt2", "dk2", "accent1", "accent2", "accent3", "accent4", "accent5", "accent6", "hlink", "folHlink"]
        .iter()
        .map(|k| {
            scheme
                .child(k)
                .and_then(|c| c.kids.first())
                .and_then(|c| if c.name == "sysClr" { c.attr("lastClr") } else { c.attr("val") })
                .map(hex6)
                .unwrap_or_else(|| "000000".into())
        })
        .collect()
}

/// Office 2013–2022 theme, for packages without a theme part.
fn default_theme() -> Vec<String> {
    ["FFFFFF", "000000", "E7E6E6", "44546A", "4472C4", "ED7D31", "A5A5A5", "FFC000", "5B9BD5", "70AD47", "0563C1", "954F72"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// Excel's tint: shift HLS lightness toward black (tint < 0) or white (tint > 0).
/// Floating point, so a channel can differ by one from Excel's integer math.
fn apply_tint(hex: &str, tint: f64) -> String {
    let v = u32::from_str_radix(hex, 16).unwrap_or(0);
    let ch = |s: u32| ((v >> s) & 255) as f64 / 255.0;
    let (h, l, s) = rgb_to_hls(ch(16), ch(8), ch(0));
    let l = if tint < 0.0 { l * (1.0 + tint) } else { l * (1.0 - tint) + tint };
    let (r, g, b) = hls_to_rgb(h, l.clamp(0.0, 1.0), s);
    let b8 = |x: f64| (x * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("{:02X}{:02X}{:02X}", b8(r), b8(g), b8(b))
}

fn rgb_to_hls(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let l = (max + min) / 2.0;
    if max == min {
        return (0.0, l, 0.0);
    }
    let d = max - min;
    let s = if l <= 0.5 { d / (max + min) } else { d / (2.0 - max - min) };
    let (rc, gc, bc) = ((max - r) / d, (max - g) / d, (max - b) / d);
    let h = if r == max {
        bc - gc
    } else if g == max {
        2.0 + rc - bc
    } else {
        4.0 + gc - rc
    };
    ((h / 6.0).rem_euclid(1.0), l, s)
}

fn hls_to_rgb(h: f64, l: f64, s: f64) -> (f64, f64, f64) {
    if s == 0.0 {
        return (l, l, l);
    }
    let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let m1 = 2.0 * l - m2;
    let v = |hue: f64| {
        let hue = hue.rem_euclid(1.0);
        if hue < 1.0 / 6.0 {
            m1 + (m2 - m1) * hue * 6.0
        } else if hue < 0.5 {
            m2
        } else if hue < 2.0 / 3.0 {
            m1 + (m2 - m1) * (2.0 / 3.0 - hue) * 6.0
        } else {
            m1
        }
    };
    (v(h + 1.0 / 3.0), v(h), v(h - 1.0 / 3.0))
}

/// Excel's legacy 64-colour palette (`indexed="n"`).
const PALETTE: [&str; 64] = [
    "000000", "FFFFFF", "FF0000", "00FF00", "0000FF", "FFFF00", "FF00FF", "00FFFF", "000000", "FFFFFF", "FF0000", "00FF00", "0000FF", "FFFF00", "FF00FF", "00FFFF",
    "800000", "008000", "000080", "808000", "800080", "008080", "C0C0C0", "808080", "9999FF", "993366", "FFFFCC", "CCFFFF", "660066", "FF8080", "0066CC", "CCCCFF",
    "000080", "FF00FF", "FFFF00", "00FFFF", "800080", "800000", "008080", "0000FF", "00CCFF", "CCFFFF", "CCFFCC", "FFFF99", "99CCFF", "FF99CC", "CC99FF", "FFCC99",
    "3366FF", "33CCCC", "99CC00", "FFCC00", "FF9900", "FF6600", "666699", "969696", "003366", "339966", "003300", "333300", "993300", "993366", "333399", "333333",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_format_types() {
        assert_eq!(num_type("0.00%"), Some("PERCENT"));
        assert_eq!(num_type("$#,##0.00_);($#,##0.00)"), Some("CURRENCY"));
        assert_eq!(num_type("[$€-407]#,##0.00"), Some("CURRENCY"));
        assert_eq!(num_type("[$-409]mmmm d, yyyy"), Some("DATE"));
        assert_eq!(num_type("yyyy-mm-dd hh:mm"), Some("DATE_TIME"));
        assert_eq!(num_type("[h]:mm:ss"), Some("TIME"));
        assert_eq!(num_type("mmm"), Some("DATE"));
        assert_eq!(num_type("#,##0.0\" days\""), Some("NUMBER"));
        assert_eq!(num_type("0.00E+00"), Some("SCIENTIFIC"));
        assert_eq!(num_type("@"), Some("TEXT"));
    }

    #[test]
    fn tints() {
        // Office "Blue, Accent 1, lighter 80%" and "darker 25%".
        assert_eq!(apply_tint("4472C4", 0.7999816888943144), "DAE3F3");
        assert_eq!(apply_tint("4472C4", -0.249977111117893), "2F5597");
    }

    #[test]
    fn shared_formulas_shift_from_their_defining_cell() {
        // O6 has its own formula, so the shared formula over O6:O9 is defined
        // in O7, and O8 must read row 8, not row 9.
        let sheet = |range: &str| {
            format!(
                r#"<worksheet><sheetData><row r="6"><c r="O6"><f>SUM(C6:N6)</f></c></row>
                <row r="7"><c r="O7"><f t="shared" ref="{range}" si="1">SUM(C7:N7)</f></c></row>
                <row r="8"><c r="O8"><f t="shared" si="1"/></c></row>
                <row r="9"><c r="O9"><f t="shared" si="1"/></c></row></sheetData></worksheet>"#
            )
        };
        let fixes = |range: &str| read_sheet_xml(&mut Reader::from_str(&sheet(range)), None).unwrap().shared_fixes;
        assert_eq!(fixes("O6:O9"), [(7, 14, "=SUM(C8:N8)".to_string()), (8, 14, "=SUM(C9:N9)".to_string())]);
        // The usual layout, which calamine already gets right, is left alone.
        assert!(fixes("O7:O9").is_empty());
    }

    #[test]
    fn part_paths() {
        assert_eq!(resolve("xl/worksheets/", "../drawings/drawing1.xml"), "xl/drawings/drawing1.xml");
        assert_eq!(resolve("xl/", "/xl/styles.xml"), "xl/styles.xml");
        assert_eq!(resolve("", "xl/workbook.xml"), "xl/workbook.xml");
    }

    #[test]
    fn external_refs_get_file_names() {
        let links = vec![
            ExternalLink { name: "Budget 2026.xlsx".into(), ..Default::default() },
            ExternalLink { name: "Rates.xlsx".into(), ..Default::default() },
        ];
        let f = "=[1]Data!B2+'[2]FX rates'!$C$3*[2]!Margin+Sales[Amount]+COUNTIF(A:A,\"[1]x\")";
        assert_eq!(
            name_external_refs(f, &links),
            "='[Budget 2026.xlsx]Data'!B2+'[Rates.xlsx]FX rates'!$C$3*'[Rates.xlsx]'!Margin+Sales[Amount]+COUNTIF(A:A,\"[1]x\")"
        );
        assert_eq!(name_external_refs("=Table1[[#This Row],[1]]", &links), "=Table1[[#This Row],[1]]");
    }

    #[test]
    fn percent_decoding() {
        assert_eq!(percent_decode("file:///C:/My%20Files/Q1%20Plan.xlsx"), "file:///C:/My Files/Q1 Plan.xlsx");
        assert_eq!(percent_decode("100%"), "100%");
    }
}
