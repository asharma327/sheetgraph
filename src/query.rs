//! Cell-level queries against an extracted workbook: exact cells, precedent /
//! dependent tracing, and text search. These are what an agent calls after
//! reading REPORT.md.

use crate::a1::{cell_name, parse_ref, quote_sheet, Rect};
use crate::analyze::{cells_in, cover, find_cell, sheet_index, sheet_span, Analysis, GRef, Group};
use crate::formula::{lex, Lexed, RefTok};
use crate::model::{Cell, Sheet, Workbook};
use crate::report::trunc;
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fmt::Write;
use std::path::Path;
use std::rc::Rc;

/// An extracted workbook, loaded and analyzed.
pub type Book = Rc<(Workbook, Analysis)>;

/// Other extracted workbooks that `trace` follows links into.
pub trait Linked {
    /// The workbook a link names: an Excel file name (`Budget.xlsx`) or a
    /// Google spreadsheet id. None when it wasn't extracted alongside.
    fn book(&self, target: &str) -> Option<Book>;
    /// The workbooks whose formulas link to `wb`.
    fn importers(&self, wb: &Workbook) -> Vec<Book>;
}

/// What other workbooks' formulas call `wb`: its file name, or its spreadsheet id.
pub fn link_name(wb: &Workbook) -> String {
    if wb.source.kind == "gsheet" {
        return wb.source.id.clone();
    }
    Path::new(&wb.source.id).file_name().and_then(|n| n.to_str()).unwrap_or(&wb.source.id).to_string()
}

/// The merge a cell sits in, if any.
fn merge_at(sh: &Sheet, r: u32, c: u32) -> Option<Rect> {
    sh.merges.iter().find(|m| m.area() > 1 && m.contains(r, c)).copied()
}

fn resolve(wb: &Workbook, a1: &str, default_sheet: Option<usize>) -> Result<(usize, Rect)> {
    let (sheet, rect) = parse_ref(a1).ok_or_else(|| anyhow!("cannot parse range {a1:?} (use Sheet!A1 or 'My Sheet'!A1:B9)"))?;
    let si = match sheet {
        Some(s) => sheet_index(wb, &s).ok_or_else(|| anyhow!("no sheet named {s:?}"))?,
        None => default_sheet.ok_or_else(|| anyhow!("range needs a sheet: Sheet!{a1}"))?,
    };
    let sh = &wb.sheets[si];
    Ok((si, rect.clamp_to(sh.grid_rows.max(1), sh.grid_cols.max(1))))
}

fn group_of(a: &Analysis, si: usize, r: u32, c: u32) -> Option<usize> {
    a.sheets[si].groups.iter().position(|g| g.rect.contains(r, c))
}

fn describe_cell(wb: &Workbook, a: &Analysis, si: usize, c: &Cell) -> String {
    let mut s = format!("{} = {}", cell_name(c.r, c.c), trunc(&c.display(), 80));
    if let Some(m) = merge_at(&wb.sheets[si], c.r, c.c) {
        let _ = write!(s, "  [merged {m}]");
    }
    if let Some(f) = &c.f {
        let g = group_of(a, si, c.r, c.c).map(|g| format!("ƒ{} ", g + 1)).unwrap_or_default();
        let _ = write!(s, "  {g}{}", trunc(f, 300));
    }
    if let Some(st) = c.s {
        let st = &wb.styles[st as usize];
        let mut bits = Vec::new();
        bits.extend(st.cell_style.clone().map(|n| format!("style \"{n}\"")));
        bits.extend(st.num_fmt.clone().map(|f| format!("fmt {f}")));
        bits.extend(st.bg.clone().map(|b| format!("bg{b}")));
        bits.extend(st.fg.clone().map(|b| format!("fg{b}")));
        for (on, name) in [(st.bold, "bold"), (st.italic, "italic"), (st.strike, "strike"), (st.unlocked, "unlocked"), (st.hidden_formula, "formula hidden")] {
            if on {
                bits.push(name.into());
            }
        }
        bits.extend(st.border.clone().map(|b| format!("border {b}")));
        if !bits.is_empty() {
            let _ = write!(s, "  [{}]", bits.join(" "));
        }
    }
    if !c.runs.is_empty() {
        let _ = write!(s, "  [rich text: {}]", trunc(&crate::report::runs_text(c), 200));
    }
    if let Some(o) = &c.obj {
        let mut t = o.kind.clone();
        if let Some(x) = &o.text {
            let _ = write!(t, " \"{}\"", trunc(x, 60));
        }
        if let Some(x) = &o.target {
            let _ = write!(t, " {}", trunc(x, 80));
        }
        if !o.props.is_empty() {
            let props: Vec<String> = o.props.iter().map(|(k, v)| format!("{k}={}", trunc(v, 40))).collect();
            let _ = write!(t, " {{{}}}", trunc(&props.join(", "), 300));
        }
        let _ = write!(s, "  [{t}]");
    }
    if let Some(d) = c.dv {
        let v = &wb.validations[d as usize];
        let _ = write!(s, "  [validation {} {}]", v.kind, trunc(&v.values.join(","), 80));
    }
    if let Some(n) = &c.note {
        let _ = write!(s, "  [note: {}]", trunc(n, 120));
    }
    if let Some(l) = &c.link {
        let _ = write!(s, "  [link {}]", trunc(l, 80));
    }
    s
}

/// Every non-empty cell in a range, one per line.
pub fn cells(wb: &Workbook, a: &Analysis, range: &str, limit: usize) -> Result<String> {
    let (si, rect) = resolve(wb, range, None)?;
    let sh = &wb.sheets[si];
    let mut s = format!("{}!{}\n", quote_sheet(&sh.name), rect);
    let mut n = 0;
    for c in cells_in(sh, &rect) {
        if c.is_blank() && c.note.is_none() && c.dv.is_none() {
            continue;
        }
        if n == limit {
            let rest = cells_in(sh, &rect).filter(|c| !c.is_blank()).count() - n;
            let _ = writeln!(s, "… +{rest} more cells (raise --limit)");
            break;
        }
        let _ = writeln!(s, "{}", describe_cell(wb, a, si, c));
        n += 1;
    }
    // Cells under a merge whose top-left is outside the range show that cell.
    for m in sh.merges.iter().filter(|m| m.area() > 1 && m.intersects(&rect) && !rect.contains(m.r0, m.c0)) {
        let part = Rect { r0: m.r0.max(rect.r0), c0: m.c0.max(rect.c0), r1: m.r1.min(rect.r1), c1: m.c1.min(rect.c1) };
        let shown = find_cell(sh, m.r0, m.c0).map(|c| trunc(&c.display(), 80)).unwrap_or_default();
        let _ = writeln!(s, "{part} inside merge {m}: shows {} = {shown}", cell_name(m.r0, m.c0));
    }
    if let Some(e) = sh.elided.as_ref().filter(|e| rect.r0 <= e.rows.1 && e.rows.0 <= rect.r1) {
        let _ = writeln!(s, "rows {}–{} are summarized, not kept: their cells aren't listed (REPORT.md has their column totals)", e.rows.0 + 1, e.rows.1 + 1);
    }
    Ok(s)
}

/// Summary of what sits in a range: one line.
fn describe_range(wb: &Workbook, a: &Analysis, si: usize, rect: &Rect) -> String {
    let sh = &wb.sheets[si];
    let used = a.sheets[si].used;
    let rect = match used {
        Some(u) if rect.intersects(&u) => Rect { r0: rect.r0.max(u.r0), c0: rect.c0.max(u.c0), r1: rect.r1.min(u.r1), c1: rect.c1.min(u.c1) },
        _ => return format!("{}!{} (empty)", quote_sheet(&sh.name), rect),
    };
    if rect.area() == 1 {
        return match (find_cell(sh, rect.r0, rect.c0), merge_at(sh, rect.r0, rect.c0)) {
            (Some(c), _) => format!("{}!{}", quote_sheet(&sh.name), describe_cell(wb, a, si, c)),
            (None, Some(m)) => {
                let shown = find_cell(sh, m.r0, m.c0).map(|c| trunc(&c.display(), 80)).unwrap_or_default();
                format!("{}!{} (blank: inside merge {m}, which shows {} = {shown})", quote_sheet(&sh.name), rect, cell_name(m.r0, m.c0))
            }
            (None, None) => format!("{}!{} (blank)", quote_sheet(&sh.name), rect),
        };
    }
    let (mut n, mut nf) = (0, 0);
    for c in cells_in(sh, &rect) {
        if !c.is_blank() {
            n += 1;
            if c.f.is_some() {
                nf += 1;
            }
        }
    }
    let gs: Vec<String> = a.sheets[si].groups.iter().enumerate().filter(|(_, g)| g.rect.intersects(&rect)).map(|(i, _)| format!("ƒ{}", i + 1)).collect();
    let mut s = format!("{}!{} ({n} cells", quote_sheet(&sh.name), rect);
    if nf > 0 {
        let _ = write!(s, ", {nf} formulas in {}{}", gs.iter().take(5).cloned().collect::<Vec<_>>().join(","), if gs.len() > 5 { format!(" +{}", gs.len() - 5) } else { String::new() });
    }
    s.push(')');
    s
}

/// Precedents ("what does this depend on") and dependents ("what uses this").
/// With `books`, links into other extracted workbooks are followed too.
pub fn trace(wb: &Workbook, a: &Analysis, cell: &str, depth: usize, up: bool, down: bool, books: Option<&dyn Linked>) -> Result<String> {
    let (si, rect) = resolve(wb, cell, None)?;
    let mut t = Tracer { books, home: wb.source.id.clone(), max: depth, seen: HashSet::new(), out: describe_range(wb, a, si, &rect) + "\n" };
    if up {
        let _ = writeln!(t.out, "precedents:");
        t.precedents(wb, a, si, &rect, 1);
    }
    if down {
        let _ = writeln!(t.out, "dependents:");
        t.seen.clear();
        t.dependents(wb, a, si, &rect, 1);
    }
    Ok(t.out)
}

struct Tracer<'l> {
    books: Option<&'l dyn Linked>,
    /// The workbook the trace started in; lines from others name theirs.
    home: String,
    max: usize,
    seen: HashSet<(String, usize, Rect)>,
    out: String,
}

/// A range in another extracted workbook.
type Remote = (Book, usize, Rect);

impl Tracer<'_> {
    fn tag(&self, wb: &Workbook) -> String {
        if wb.source.id == self.home { String::new() } else { format!("[{}] ", if wb.source.kind == "gsheet" { wb.source.title.clone() } else { link_name(wb) }) }
    }

    /// `[Book.xlsx]Sheet` / IMPORTRANGE target and range, found among the extracted workbooks.
    fn remote(&self, book: &str, sheet: &str, r: Rect) -> Option<Remote> {
        let o = self.books?.book(book)?;
        let ts = sheet_index(&o.0, sheet)?;
        Some((o, ts, r))
    }

    fn precedents(&mut self, wb: &Workbook, a: &Analysis, si: usize, rect: &Rect, d: usize) {
        if d > self.max {
            return;
        }
        let sh = &wb.sheets[si];
        let pad = "  ".repeat(d);
        let tag = self.tag(wb);
        // Exact refs of each formula cell in the range (shifted per cell), merged.
        let mut refs: Vec<(usize, Rect)> = Vec::new();
        let mut externals: Vec<String> = Vec::new();
        let mut remotes: Vec<Remote> = Vec::new();
        let mut cells_seen = 0;
        for c in cells_in(sh, rect) {
            let Some(f) = &c.f else { continue };
            cells_seen += 1;
            if cells_seen > 5000 {
                break;
            }
            let lx = lex(f);
            for fu in lx.funcs.iter().filter(|fu| crate::formula::EXTERNAL_FUNCS.contains(&fu.name.as_str())) {
                let args: Vec<String> = crate::formula::call_args(f, fu.open)
                    .iter()
                    .take(2)
                    .map(|a| crate::formula::unquote(a).map(|u| crate::formula::spreadsheet_id(&u)).unwrap_or_else(|| a.clone()))
                    .collect();
                if fu.name == "IMPORTRANGE"
                    && let [id, range] = args.as_slice()
                    && let Some((Some(sheet), r)) = parse_ref(range)
                    && let Some(x) = self.remote(id, &sheet, r)
                {
                    push_remote(&mut remotes, x);
                    continue;
                }
                let e = format!("{} {}", fu.name, args.join(" "));
                if !externals.contains(&e) {
                    externals.push(e);
                }
            }
            for t in &lx.refs {
                let Some(r) = t.rect() else { continue };
                if let Some(n) = t.sheet.as_deref().filter(|n| n.starts_with('[')) {
                    let (book, sheet) = n[1..].split_once(']').unwrap_or((&n[1..], ""));
                    match self.remote(book, sheet, r) {
                        Some(x) => push_remote(&mut remotes, x),
                        None => {
                            let e = format!("'{n}'!{r}");
                            if !externals.contains(&e) {
                                externals.push(e);
                            }
                        }
                    }
                    continue;
                }
                let ts = match &t.sheet {
                    None => vec![si],
                    Some(n) => sheet_span(wb, n),
                };
                for ts in ts {
                    merge_ref(&mut refs, ts, r);
                }
            }
            let lambdas = lx.funcs.iter().filter(|f| !crate::formula::is_builtin(&f.name)).map(|f| &f.name);
            for n in lx.names.iter().chain(lambdas) {
                for (ns, r) in a.name(Some(si), n).map(|d| d.targets.as_slice()).unwrap_or_default() {
                    merge_ref(&mut refs, ns.unwrap_or(si), *r);
                }
            }
        }
        for e in &externals {
            let _ = writeln!(self.out, "{pad}← external: {}", trunc(e, 200));
        }
        // What the traced range's dropdowns choose from and its rules read (not followed further).
        let mut ruled = Vec::new();
        for r in a.sheets[si].rules.iter().filter(|r| d == 1 && r.ranges.iter().any(|x| x.intersects(rect))) {
            for x in &r.refs {
                let line = format!("{pad}← {}: {tag}{}", rule_label(r.kind), gref_text(wb, si, x));
                if !ruled.contains(&line) {
                    ruled.push(line);
                }
            }
        }
        for l in ruled {
            let _ = writeln!(self.out, "{l}");
        }
        for (o, ts, r) in remotes {
            let _ = writeln!(self.out, "{pad}← {}{}", self.tag(&o.0), describe_range(&o.0, &o.1, ts, &r));
            if self.seen.insert((o.0.source.id.clone(), ts, r)) {
                self.precedents(&o.0, &o.1, ts, &r, d + 1);
            }
        }
        for (ts, r) in refs {
            let _ = writeln!(self.out, "{pad}← {tag}{}", describe_range(wb, a, ts, &r));
            if self.seen.insert((wb.source.id.clone(), ts, r)) {
                self.precedents(wb, a, ts, &r, d + 1);
            }
        }
    }

    fn dependents(&mut self, wb: &Workbook, a: &Analysis, si: usize, rect: &Rect, d: usize) {
        if d > self.max {
            return;
        }
        let pad = "  ".repeat(d);
        let tag = self.tag(wb);
        // Candidate groups via their union refs, then exact per-cell check.
        for (gs, sa) in a.sheets.iter().enumerate() {
            for (gi, g) in sa.groups.iter().enumerate() {
                if !g.refs.iter().any(|r| r.sheet == Some(si) && r.rect.intersects(rect)) {
                    continue;
                }
                let lx = lex(&g.formula);
                let hits = group_hits(g, &lx, |t, x| {
                    let on = match &t.sheet {
                        None => gs == si,
                        Some(n) => sheet_span(wb, n).contains(&si),
                    };
                    on && x.intersects(rect)
                }, |n| a.name(Some(gs), n).is_some_and(|d| d.targets.iter().any(|(ns, x)| ns.unwrap_or(gs) == si && x.intersects(rect))));
                for hr in hits {
                    if gs == si && hr == *rect {
                        continue;
                    }
                    let _ = writeln!(self.out, "{pad}→ {tag}{}!{} ƒ{} {}", quote_sheet(&wb.sheets[gs].name), hr, gi + 1, trunc(&g.formula, 160));
                    if self.seen.insert((wb.source.id.clone(), gs, hr)) {
                        self.dependents(wb, a, gs, &hr, d + 1);
                    }
                }
            }
        }
        // Charts, dropdowns and rules reading the range.
        for (cs, sh) in wb.sheets.iter().enumerate() {
            for ch in &sh.charts {
                for src in &ch.sources {
                    if let Some((Some(n), r)) = parse_ref(src)
                        && sheet_index(wb, &n) == Some(si) && r.intersects(rect) {
                            let _ = writeln!(self.out, "{pad}→ {tag}chart \"{}\" on {}", ch.title, wb.sheets[cs].name);
                        }
                }
            }
            for r in a.sheets[cs].rules.iter().filter(|r| r.refs.iter().any(|x| x.sheet == Some(si) && x.rect.intersects(rect))) {
                let ranges: Vec<String> = r.ranges.iter().take(3).map(|x| x.a1()).collect();
                let _ = writeln!(self.out, "{pad}→ {tag}{} on {}!{}", rule_label(r.kind), quote_sheet(&sh.name), ranges.join(","));
            }
        }
        // Formulas in other extracted workbooks that link here.
        let Some(books) = self.books else { return };
        let me = link_name(wb);
        let sheet = &wb.sheets[si].name;
        for o in books.importers(wb) {
            let (ow, oa) = (&o.0, &o.1);
            let otag = self.tag(ow);
            for (gs, sa) in oa.sheets.iter().enumerate() {
                for (gi, g) in sa.groups.iter().enumerate().filter(|(_, g)| g.external) {
                    let f = &g.formula;
                    let lx = lex(f);
                    // IMPORTRANGE("<this id>", "Sheet!A1:B9") reads the same range from every cell.
                    let imported = lx.funcs.iter().filter(|fu| fu.name == "IMPORTRANGE").any(|fu| {
                        let args: Vec<String> = crate::formula::call_args(f, fu.open).iter().filter_map(|x| crate::formula::unquote(x)).collect();
                        args.len() == 2
                            && crate::formula::spreadsheet_id(&args[0]) == wb.source.id
                            && parse_ref(&args[1]).is_some_and(|(s, r)| s.is_some_and(|s| s.eq_ignore_ascii_case(sheet)) && r.intersects(rect))
                    });
                    let hits = if imported {
                        vec![g.rect]
                    } else {
                        group_hits(g, &lx, |t, x| {
                            t.sheet.as_deref().and_then(|n| n.strip_prefix('[')).and_then(|n| n.split_once(']')).is_some_and(|(b, s)| {
                                b.eq_ignore_ascii_case(&me) && s.eq_ignore_ascii_case(sheet)
                            }) && x.intersects(rect)
                        }, |_| false)
                    };
                    for hr in hits {
                        let _ = writeln!(self.out, "{pad}→ {otag}{}!{} ƒ{} {}", quote_sheet(&ow.sheets[gs].name), hr, gi + 1, trunc(f, 160));
                        if self.seen.insert((ow.source.id.clone(), gs, hr)) {
                            self.dependents(ow, oa, gs, &hr, d + 1);
                        }
                    }
                }
            }
        }
    }
}

fn push_remote(v: &mut Vec<Remote>, x: Remote) {
    match v.iter_mut().find(|(o, s, _)| Rc::ptr_eq(o, &x.0) && *s == x.1) {
        Some(e) if e.2.intersects(&x.2) || e.2 == x.2 => e.2 = e.2.union(&x.2),
        _ => v.push(x),
    }
}

/// Cells of a formula group whose own (shifted) refs or names hit, as rectangles.
fn group_hits(g: &Group, lx: &Lexed, by_ref: impl Fn(&RefTok, Rect) -> bool, by_name: impl Fn(&str) -> bool) -> Vec<Rect> {
    let named = lx.names.iter().chain(lx.funcs.iter().map(|f| &f.name)).any(|n| by_name(n));
    let mut hits: Vec<((), u32, u32)> = Vec::new();
    let mut checked = 0;
    'cells: for r in g.rect.r0..=g.rect.r1 {
        for c in g.rect.c0..=g.rect.c1 {
            checked += 1;
            if checked > 200_000 {
                break 'cells;
            }
            let (dr, dc) = (r as i64 - g.rect.r0 as i64, c as i64 - g.rect.c0 as i64);
            if named || lx.refs.iter().any(|t| t.rect_at(dr, dc).is_some_and(|x| by_ref(t, x))) {
                hits.push(((), r, c));
            }
        }
    }
    cover(hits).into_iter().map(|(_, r)| r).collect()
}

fn rule_label(kind: &str) -> &'static str {
    match kind {
        "dropdown" => "dropdown choices",
        "validation" => "validation rule",
        _ => "conditional format",
    }
}

fn gref_text(wb: &Workbook, si: usize, x: &GRef) -> String {
    let t = x.sheet.unwrap_or(si);
    let at = format!("{}!{}", quote_sheet(&wb.sheets[t].name), x.rect);
    match &x.via_name {
        Some(n) => format!("{n} ({at})"),
        None => at,
    }
}

fn merge_ref(refs: &mut Vec<(usize, Rect)>, ts: usize, r: Rect) {
    // Collapse refs that are adjacent copies (A2, A3, A4 → A2:A4) on the same sheet.
    for e in refs.iter_mut() {
        if e.0 == ts {
            let o = e.1;
            let same_cols = o.c0 == r.c0 && o.c1 == r.c1 && (r.r0 <= o.r1.saturating_add(1) && o.r0 <= r.r1.saturating_add(1));
            let same_rows = o.r0 == r.r0 && o.r1 == r.r1 && (r.c0 <= o.c1.saturating_add(1) && o.c0 <= r.c1.saturating_add(1));
            if same_cols || same_rows || o == r {
                e.1 = o.union(&r);
                return;
            }
        }
    }
    refs.push((ts, r));
}

/// Case-insensitive search over values, formulas, notes and sheet names.
pub fn find(wb: &Workbook, a: &Analysis, needle: &str, limit: usize) -> String {
    let n = needle.to_lowercase();
    let mut s = String::new();
    let mut hits = 0;
    for (si, sh) in wb.sheets.iter().enumerate() {
        for c in &sh.cells {
            let hay = [c.display(), c.f.clone().unwrap_or_default(), c.note.clone().unwrap_or_default()];
            if hay.iter().any(|h| h.to_lowercase().contains(&n)) {
                hits += 1;
                if hits <= limit {
                    let _ = writeln!(s, "{}!{}", quote_sheet(&sh.name), describe_cell(wb, a, si, c));
                }
            }
        }
    }
    for (i, sc) in wb.scripts.iter().enumerate() {
        for (ln, line) in sc.source.lines().enumerate() {
            if line.to_lowercase().contains(&n) {
                hits += 1;
                if hits <= limit {
                    let p = a.macros.iter().find(|p| p.script == i && p.lines.0 <= ln + 1 && ln < p.lines.1);
                    let _ = writeln!(s, "{} {}:{}{}: {}", sc.kind, sc.name, ln + 1, p.map(|p| format!(" ({})", p.name)).unwrap_or_default(), trunc(line.trim(), 160));
                }
            }
        }
    }
    if hits > limit {
        let _ = writeln!(s, "… {} hits total (raise --limit)", hits);
    }
    // Summarized rows only have their texts tallied per column; count matches there.
    let mut summarized = 0;
    for sh in &wb.sheets {
        let Some(e) = &sh.elided else { continue };
        let mut cols = Vec::new();
        for st in &e.cols {
            let m: u64 = st.texts.iter().filter(|(t, _)| t.to_lowercase().contains(&n)).map(|(_, k)| k).sum();
            if m > 0 {
                summarized += m;
                cols.push(format!("{} ({m})", crate::a1::col_name(st.col)));
            }
        }
        let rows = format!("{}!{}:{} (summarized rows)", quote_sheet(&sh.name), e.rows.0 + 1, e.rows.1 + 1);
        if cols.is_empty() {
            let _ = writeln!(s, "{rows}: no tallied text matches; numbers, formulas and texts past each column's tally aren't searchable there");
        } else {
            let _ = writeln!(s, "{rows}: more text matches in column {}", cols.join(", "));
        }
    }
    if hits == 0 && summarized == 0 {
        s.push_str("no matches\n");
    }
    s
}

/// Without `name`, every module and procedure with what each touches. With
/// it, the code of a module, a procedure, or `Module.Procedure`, numbered.
pub fn macros(wb: &Workbook, a: &Analysis, name: Option<&str>) -> Result<String> {
    let mut s = String::new();
    let Some(name) = name else {
        if wb.scripts.is_empty() {
            return Ok("no macros or scripts in this workbook\n".into());
        }
        let mut empty = Vec::new();
        for (i, sc) in wb.scripts.iter().enumerate() {
            if !sc.has_code() {
                empty.push(sc.name.as_str());
                continue;
            }
            let _ = writeln!(s, "{} {} ({} lines)", sc.kind, sc.name, sc.source.lines().count());
            for p in a.macros.iter().filter(|p| p.script == i) {
                let d = crate::macros::describe(wb, a, p, usize::MAX).map(|d| format!(" · {d}")).unwrap_or_default();
                let _ = writeln!(s, "  {} {} lines {}-{}{d}", p.kind, p.name, p.lines.0, p.lines.1);
            }
        }
        if !empty.is_empty() {
            let _ = writeln!(s, "no code: {}", empty.join(", "));
        }
        return Ok(s);
    };
    let numbered = |s: &mut String, src: &str, from: usize, to: usize| {
        for (n, line) in src.lines().enumerate().skip(from - 1).take(to + 1 - from) {
            let _ = writeln!(s, "{:>5}  {}", n + 1, line);
        }
    };
    if let Some(sc) = wb.scripts.iter().find(|sc| sc.name.eq_ignore_ascii_case(name)) {
        let _ = writeln!(s, "{} {} ({} lines)", sc.kind, sc.name, sc.source.lines().count());
        numbered(&mut s, &sc.source, 1, sc.source.lines().count().max(1));
        return Ok(s);
    }
    let (module, proc) = match name.split_once('.') {
        Some((m, p)) => (Some(m), p),
        None => (None, name),
    };
    let found: Vec<_> = a
        .macros
        .iter()
        .filter(|p| p.name.eq_ignore_ascii_case(proc) && module.is_none_or(|m| wb.scripts[p.script].name.eq_ignore_ascii_case(m)))
        .collect();
    if found.is_empty() {
        let mut names: Vec<String> = wb.scripts.iter().filter(|sc| sc.has_code()).map(|sc| sc.name.clone()).collect();
        names.extend(a.macros.iter().map(|p| format!("{}.{}", wb.scripts[p.script].name, p.name)));
        return Err(anyhow!("no module or procedure {name:?}; have: {}", trunc(&names.join(", "), 600)));
    }
    for p in found {
        let sc = &wb.scripts[p.script];
        let d = crate::macros::describe(wb, a, p, usize::MAX).map(|d| format!("\n{d}")).unwrap_or_default();
        let _ = writeln!(s, "{} {}.{} lines {}-{}{d}", p.kind, sc.name, p.name, p.lines.0, p.lines.1);
        numbered(&mut s, &sc.source, p.lines.0, p.lines.1);
    }
    Ok(s)
}
