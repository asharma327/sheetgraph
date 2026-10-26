//! Compact text rendering for LLM consumption. Everything here is a summary;
//! exact cells come from `query` against workbook.json.

use crate::a1::{col_name, Rect};
use crate::analyze::{cells_in, cover, find_cell, fingerprint, Analysis, Group, SheetAnalysis};
use crate::model::{Cell, Sheet, Style, Validation, Value, Workbook};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Opts {
    /// Formula lines per sheet before truncating.
    pub max_formula_lines: usize,
    /// Column/row profile lines per block.
    pub max_profile_lines: usize,
    /// Blocks described per sheet.
    pub max_blocks: usize,
    pub sample_rows: usize,
    pub sample_cols: usize,
    pub cell_width: usize,
    /// Blocks with at most this many cells are dumped verbatim.
    pub small_block: u32,
    pub max_formula_chars: usize,
    /// Render structural clones of an earlier sheet in full instead of as a diff.
    pub expand_clones: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            max_formula_lines: 40,
            max_profile_lines: 40,
            max_blocks: 12,
            sample_rows: 3,
            sample_cols: 16,
            cell_width: 24,
            small_block: 40,
            max_formula_chars: 260,
            expand_clones: false,
        }
    }
}

impl Opts {
    pub fn full() -> Self {
        Opts {
            max_formula_lines: usize::MAX,
            max_profile_lines: usize::MAX,
            max_blocks: usize::MAX,
            sample_rows: 5,
            sample_cols: 40,
            cell_width: 40,
            small_block: 200,
            max_formula_chars: 2000,
            expand_clones: true,
        }
    }
}

pub fn trunc(s: &str, n: usize) -> String {
    let s = s.replace(['\n', '\r'], "⏎");
    if s.chars().count() <= n {
        s
    } else {
        format!("{}…", s.chars().take(n.saturating_sub(1)).collect::<String>())
    }
}

fn k(n: u64) -> String {
    match n {
        0..=9_999 => n.to_string(),
        10_000..=999_999 => format!("{:.0}k", n as f64 / 1e3),
        _ => format!("{:.1}M", n as f64 / 1e6),
    }
}

/// Rough token estimate: ~4 chars/token for ASCII, ~1 token/char otherwise (Thai, CJK).
pub fn approx_tokens(s: &str) -> usize {
    let (mut ascii, mut other) = (0usize, 0usize);
    for ch in s.chars() {
        if ch.is_ascii() { ascii += 1 } else { other += 1 }
    }
    ascii / 4 + other * 9 / 10
}

fn rect_list(rs: &[Rect], max: usize) -> String {
    let mut s = rs.iter().take(max).map(|r| r.a1()).collect::<Vec<_>>().join(", ");
    if rs.len() > max {
        let _ = write!(s, " +{}", rs.len() - max);
    }
    s
}

// ---------------------------------------------------------------- workbook

pub fn render(wb: &Workbook, a: &Analysis, o: &Opts) -> String {
    let mut s = String::new();
    let src = &wb.source;
    let _ = writeln!(s, "# {}", src.title);
    let mut meta = vec![format!("{} {}", src.kind, src.id)];
    meta.extend(src.locale.clone());
    meta.extend(src.time_zone.clone());
    if let Some(m) = &src.modified {
        meta.push(format!("modified {}", &m[..m.len().min(10)]));
    }
    if let Some(ow) = &src.owner {
        meta.push(format!("owner {ow}"));
    }
    let _ = writeln!(s, "{}", meta.join(" · "));
    let cells: u64 = a.sheets.iter().map(|x| x.n_cells).sum();
    let nf: u64 = a.sheets.iter().map(|x| x.n_formulas).sum();
    let ng: usize = a.sheets.iter().map(|x| x.groups.len()).sum();
    let ne: u64 = a.sheets.iter().map(|x| x.n_errors).sum();
    let _ = writeln!(
        s,
        "{} sheets · {} non-empty cells · {} formulas in {} copy-groups{}",
        wb.sheets.len(),
        k(cells),
        k(nf),
        k(ng as u64),
        if ne > 0 { format!(" · {} error cells", k(ne)) } else { String::new() }
    );
    let _ = writeln!(
        s,
        "Notation: ƒN = formula copy-group N of a sheet; a group's formula is shown at its top-left cell and is copied (relative refs shift) over its range. ↓ filled down, → across, ▦ both."
    );

    let _ = writeln!(s, "\n## Sheets");
    for (si, sh) in wb.sheets.iter().enumerate() {
        let sa = &a.sheets[si];
        let mut line = format!("{}. {}", si + 1, sh.name);
        if sh.hidden {
            line.push_str(" [hidden]");
        }
        if sh.kind != "GRID" {
            let _ = write!(line, " [{}]", sh.kind);
        }
        match sa.used {
            Some(u) => {
                let _ = write!(line, " — {} · {} cells", u, k(sa.n_cells));
            }
            None => line.push_str(" — empty"),
        }
        if sa.n_formulas > 0 {
            let _ = write!(line, " · {} ƒ in {} groups", k(sa.n_formulas), sa.groups.len());
        }
        let reads: Vec<String> = a.sheet_edges.keys().filter(|(r, t)| *r == si && *t != si).map(|(_, t)| wb.sheets[*t].name.clone()).collect();
        if !reads.is_empty() {
            let _ = write!(line, " · reads {}", reads.join(", "));
        }
        let _ = writeln!(s, "{line}");
    }

    if !a.sheet_edges.is_empty() {
        let _ = writeln!(s, "\n## Data flow (source → readers, by formula refs)");
        let mut by_src: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for ((r, t), (g, _)) in &a.sheet_edges {
            by_src.entry(*t).or_default().push(format!("{} ({}g)", wb.sheets[*r].name, g));
        }
        for (t, readers) in by_src {
            let _ = writeln!(s, "{} → {}", wb.sheets[t].name, readers.join(", "));
        }
    }

    // Cross-sheet references from charts, pivots, validations feed the map too.
    let mut other_links = Vec::new();
    for sh in &wb.sheets {
        for ch in &sh.charts {
            other_links.push(format!("chart \"{}\" on {} ← {}", trunc(&ch.title, 40), sh.name, ch.sources.join(", ")));
        }
        for p in &sh.pivots {
            other_links.push(format!("pivot at {}!{} ← {}", sh.name, crate::a1::cell_name(p.anchor.0, p.anchor.1), p.source));
        }
    }
    if !other_links.is_empty() {
        let _ = writeln!(s, "\n## Charts & pivots");
        for l in other_links.iter().take(30) {
            let _ = writeln!(s, "{l}");
        }
        if other_links.len() > 30 {
            let _ = writeln!(s, "+{} more", other_links.len() - 30);
        }
    }

    if !a.externals.is_empty() || !a.broken_refs.is_empty() || !wb.data_sources.is_empty() {
        let _ = writeln!(s, "\n## External links");
        // One line per (function, target): the imported ranges and where they land.
        let mut by_target: Vec<((String, String), Vec<String>)> = Vec::new();
        for e in &a.externals {
            let g = &a.sheets[e.sheet].groups[e.group];
            let item = format!(
                "{}→{}!{}",
                e.range.as_ref().map(|r| format!("\"{r}\"")).unwrap_or_default(),
                crate::a1::quote_sheet(&wb.sheets[e.sheet].name),
                crate::a1::cell_name(g.rect.r0, g.rect.c0)
            );
            let key = (e.func.clone(), e.target.clone());
            match by_target.iter_mut().find(|(k, _)| *k == key) {
                Some((_, v)) => {
                    if !v.contains(&item) {
                        v.push(item)
                    }
                }
                None => by_target.push((key, vec![item])),
            }
        }
        for ((func, target), items) in &by_target {
            let more = if items.len() > 12 { format!(" +{}", items.len() - 12) } else { String::new() };
            let _ = writeln!(s, "{func} {} ({}): {}{more}", trunc(target, 80), items.len(), items.iter().take(12).cloned().collect::<Vec<_>>().join(", "));
        }
        for (n, cells) in &a.broken_refs {
            let _ = writeln!(s, "refs to missing sheet '{n}' in {} cells (#REF! or deleted sheet)", cells);
        }
        for d in &wb.data_sources {
            let spec = &d["spec"];
            let _ = writeln!(s, "data source {}: {}", d["dataSourceId"].as_str().unwrap_or("?"), trunc(&spec.to_string(), 200));
        }
    }

    if !wb.named_ranges.is_empty() {
        let _ = writeln!(s, "\n## Named ranges");
        for n in &wb.named_ranges {
            let _ = writeln!(s, "{} = {}", n.name, n.range);
        }
    }

    // Function census: custom first (they are code living outside the grid).
    if !a.custom_funcs.is_empty() {
        let _ = writeln!(s, "\n## Non-builtin functions (Apps Script custom functions, Named Functions, VBA UDFs, or LET/LAMBDA names)");
        let _ = writeln!(s, "{}", a.custom_funcs.iter().map(|(f, n)| format!("{f} ({n})")).collect::<Vec<_>>().join(" · "));
    }
    if !a.func_cells.is_empty() {
        let mut v: Vec<_> = a.func_cells.iter().collect();
        v.sort_by(|x, y| y.1.cmp(x.1));
        let _ = writeln!(s, "\n## Functions (cells using)");
        let _ = writeln!(s, "{}", v.iter().take(30).map(|(f, n)| format!("{f} {}", k(**n))).collect::<Vec<_>>().join(" · "));
        let flag = |list: &[&str]| -> Vec<String> { v.iter().filter(|(f, _)| list.contains(&f.as_str())).map(|(f, n)| format!("{f} {n}")).collect() };
        let dynamic = flag(crate::formula::DYNAMIC_FUNCS);
        let volatile = flag(crate::formula::VOLATILE_FUNCS);
        if !dynamic.is_empty() {
            let _ = writeln!(s, "dynamic refs (graph edges incomplete): {}", dynamic.join(", "));
        }
        if !volatile.is_empty() {
            let _ = writeln!(s, "volatile: {}", volatile.join(", "));
        }
    }

    if !wb.scripts.is_empty() {
        let _ = writeln!(s, "\n## Scripts");
        for sc in &wb.scripts {
            let procs = procedures(&sc.source);
            let _ = writeln!(s, "{} {} ({} lines): {}", sc.kind, sc.name, sc.source.lines().count(), trunc(&procs.join(", "), 400));
        }
    }
    if !wb.developer_metadata.is_empty() {
        let _ = writeln!(s, "\n## Developer metadata ({} entries — set by add-ons/integrations)", wb.developer_metadata.len());
        for d in wb.developer_metadata.iter().take(10) {
            let _ = writeln!(s, "{} = {}", d["metadataKey"].as_str().unwrap_or("?"), trunc(d["metadataValue"].as_str().unwrap_or(""), 80));
        }
    }
    if !wb.warnings.is_empty() {
        let _ = writeln!(s, "\n## Extraction notes");
        for w in &wb.warnings {
            let _ = writeln!(s, "- {w}");
        }
    }

    for si in 0..wb.sheets.len() {
        s.push('\n');
        s.push_str(&render_sheet(wb, a, si, o));
    }
    s
}

fn procedures(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in src.lines() {
        let t = line.trim_start();
        let lower = t.to_ascii_lowercase();
        for kw in ["function ", "sub ", "public sub ", "private sub ", "public function ", "private function ", "async function "] {
            if lower.starts_with(kw) {
                let name: String = t[kw.len()..].chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    out.push(name);
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------- sheet

pub fn render_sheet(wb: &Workbook, a: &Analysis, si: usize, o: &Opts) -> String {
    let sh = &wb.sheets[si];
    let sa = &a.sheets[si];
    let mut s = String::new();
    let mut head = format!("## {}. {}", si + 1, sh.name);
    match sa.used {
        Some(u) => {
            let _ = write!(head, " · {} · {} cells", u, k(sa.n_cells));
        }
        None => head.push_str(" · empty"),
    }
    if sh.hidden {
        head.push_str(" · hidden");
    }
    if sh.frozen_rows + sh.frozen_cols > 0 {
        let _ = write!(head, " · frozen {}r{}c", sh.frozen_rows, sh.frozen_cols);
    }
    if let Some(t) = &sh.tab_color {
        let _ = write!(head, " · tab {t}");
    }
    let _ = writeln!(s, "{head}");
    if sa.used.is_none() && sh.charts.is_empty() {
        return s;
    }

    let reads: Vec<String> = a.sheet_edges.iter().filter(|((r, t), _)| *r == si && *t != si).map(|((_, t), (g, _))| format!("{} ({g}g)", wb.sheets[*t].name)).collect();
    let readers: Vec<String> = a.sheet_edges.keys().filter(|(r, t)| *t == si && *r != si).map(|(r, _)| wb.sheets[*r].name.clone()).collect();
    if !reads.is_empty() {
        let _ = writeln!(s, "reads: {}", reads.join(", "));
    }
    if !readers.is_empty() {
        let _ = writeln!(s, "read by: {}", readers.join(", "));
    }

    if let (Some(p), false) = (sa.clone_of, o.expand_clones) {
        render_clone(&mut s, wb, a, si, p, o);
        return s;
    }

    // Formula group lookup per cell (for profiles).
    let group_at = |r: u32, c: u32| sa.groups.iter().position(|g| g.rect.contains(r, c));

    // ---- blocks
    let mut order: Vec<usize> = (0..sa.blocks.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(sa.blocks[i].cells));
    let shown: HashSet<usize> = order.iter().take(o.max_blocks).copied().collect();
    for (bi, b) in sa.blocks.iter().enumerate() {
        if !shown.contains(&bi) {
            continue;
        }
        if b.cells <= o.small_block {
            let _ = writeln!(s, "▸ {} ({} cells)", b.rect, b.cells);
            dump_block(&mut s, sh, &b.rect, &group_at, o);
        } else {
            profile_block(&mut s, wb, sh, &b.rect, b.cells, &group_at, o);
        }
    }
    if sa.blocks.len() > o.max_blocks {
        let rest: Vec<Rect> = order.iter().skip(o.max_blocks).map(|&i| sa.blocks[i].rect).collect();
        let cells: u32 = order.iter().skip(o.max_blocks).map(|&i| sa.blocks[i].cells).sum();
        let _ = writeln!(s, "▸ +{} smaller blocks ({} cells): {}", rest.len(), cells, rect_list(&rest, 8));
    }

    // ---- formulas
    if !sa.groups.is_empty() {
        let _ = writeln!(s, "ƒ formulas ({} cells, {} groups, {} families)", k(sa.n_formulas), sa.groups.len(), sa.families.len());
        let mut lines = 0;
        let mut hidden_groups = 0usize;
        let mut hidden_cells = 0u64;
        for fam in sa.families.iter().filter(|f| !sa.groups[f[0]].constant) {
            if lines >= o.max_formula_lines {
                hidden_groups += fam.len();
                hidden_cells += fam.iter().map(|&g| sa.groups[g].cells as u64).sum::<u64>();
                continue;
            }
            lines += 1;
            s.push_str(&family_lines(sh, sa, fam, o));
        }
        if hidden_groups > 0 {
            let _ = writeln!(s, "+{hidden_groups} more groups ({hidden_cells} cells) — run `sheetgraph sheet <dir> \"{}\" --full`", sh.name);
        }
        s.push_str(&typed_arithmetic(sa));
    }

    // ---- validations
    let dv_pts: Vec<(u32, u32, u32)> = sh.cells.iter().filter_map(|c| c.dv.map(|d| (d, c.r, c.c))).collect();
    if !dv_pts.is_empty() {
        let mut by: BTreeMap<u32, Vec<Rect>> = BTreeMap::new();
        for (d, r) in cover(dv_pts) {
            by.entry(d).or_default().push(r);
        }
        let _ = writeln!(s, "✓ validation");
        for (d, rs) in by.iter().take(15) {
            let _ = writeln!(s, "{}: {}", rect_list(rs, 4), validation_desc(&wb.validations[*d as usize]));
        }
        if by.len() > 15 {
            let _ = writeln!(s, "+{} more rules", by.len() - 15);
        }
    }

    // ---- conditional formats
    if !sh.cond_formats.is_empty() {
        let _ = writeln!(s, "◐ conditional formats");
        for cf in sh.cond_formats.iter().take(12) {
            let _ = writeln!(s, "{}: {}", rect_list(&cf.ranges, 3), trunc(&cf.rule, 160));
        }
        if sh.cond_formats.len() > 12 {
            let _ = writeln!(s, "+{} more", sh.cond_formats.len() - 12);
        }
    }

    // ---- styles (visual only; number formats are in the profiles)
    let mut desc_ids: HashMap<String, u32> = HashMap::new();
    let mut descs: Vec<String> = Vec::new();
    let mut pts = Vec::new();
    let used = sa.used.unwrap_or_default();
    for c in sh.cells.iter().filter(|c| used.contains(c.r, c.c)) {
        if let Some(st) = c.s {
            let d = style_desc(&wb.styles[st as usize]);
            if d.is_empty() {
                continue;
            }
            let n = descs.len() as u32;
            let id = *desc_ids.entry(d.clone()).or_insert_with(|| {
                descs.push(d);
                n
            });
            pts.push((id, c.r, c.c));
        }
    }
    if !pts.is_empty() {
        let mut by: BTreeMap<u32, (u64, Vec<Rect>)> = BTreeMap::new();
        for (d, r) in cover(pts) {
            let e = by.entry(d).or_default();
            e.0 += r.area();
            e.1.push(r);
        }
        let mut v: Vec<_> = by.into_iter().collect();
        v.sort_by_key(|(_, (n, _))| std::cmp::Reverse(*n));
        let _ = writeln!(s, "◆ formatting");
        for (d, (n, rs)) in v.iter().take(8) {
            let _ = writeln!(s, "{} ({} cells): {}", descs[*d as usize], n, rect_list(rs, 4));
        }
        if v.len() > 8 {
            let _ = writeln!(s, "+{} more styles", v.len() - 8);
        }
    }

    // ---- misc sheet features
    let mut misc = Vec::new();
    if !sh.merges.is_empty() {
        misc.push(format!("merged: {}", rect_list(&sh.merges, 5)));
    }
    for f in &sh.filters {
        let mut t = format!("filter{} {}", f.title.as_ref().map(|t| format!(" \"{t}\"")).unwrap_or_default(), f.range);
        if !f.criteria.is_empty() {
            let _ = write!(t, " [{}]", f.criteria.join("; "));
        }
        misc.push(t);
    }
    for p in &sh.protected {
        misc.push(format!(
            "protected {}{}{}",
            p.range.map(|r| r.a1()).unwrap_or_else(|| "sheet".into()),
            p.description.as_ref().map(|d| format!(" \"{}\"", trunc(d, 60))).unwrap_or_default(),
            if p.warning_only { " (warning only)" } else { "" }
        ));
    }
    if !sh.hidden_rows.is_empty() {
        misc.push(format!("hidden rows: {}", spans(&sh.hidden_rows, |i| (i + 1).to_string())));
    }
    if !sh.hidden_cols.is_empty() {
        misc.push(format!("hidden cols: {}", spans(&sh.hidden_cols, col_name)));
    }
    if !sh.banded.is_empty() {
        misc.push(format!("banded: {}", rect_list(&sh.banded, 4)));
    }
    for ch in &sh.charts {
        misc.push(format!("chart {} \"{}\" ← {}", ch.kind, trunc(&ch.title, 50), ch.sources.join(", ")));
    }
    for p in &sh.pivots {
        misc.push(format!(
            "pivot at {} ← {} rows[{}] cols[{}] values[{}]{}",
            crate::a1::cell_name(p.anchor.0, p.anchor.1),
            p.source,
            p.rows.join(","),
            p.cols.join(","),
            p.values.join(", "),
            if p.filters.is_empty() { String::new() } else { format!(" filters[{}]", p.filters.join(",")) }
        ));
    }
    for m in misc {
        let _ = writeln!(s, "{m}");
    }
    let notes: Vec<&Cell> = sh.cells.iter().filter(|c| c.note.is_some()).collect();
    if !notes.is_empty() {
        let _ = writeln!(s, "✎ notes ({})", notes.len());
        for c in notes.iter().take(20) {
            let _ = writeln!(s, "{}: {}", crate::a1::cell_name(c.r, c.c), trunc(c.note.as_deref().unwrap_or(""), 140));
        }
        if notes.len() > 20 {
            let _ = writeln!(s, "+{} more notes", notes.len() - 20);
        }
    }
    let links: Vec<&Cell> = sh.cells.iter().filter(|c| c.link.is_some()).collect();
    if !links.is_empty() {
        let mut hosts: BTreeMap<String, usize> = BTreeMap::new();
        for c in &links {
            let l = c.link.as_deref().unwrap_or("");
            let host = l.split("//").nth(1).unwrap_or(l).split('/').next().unwrap_or("").to_string();
            *hosts.entry(if host.is_empty() { "#internal".into() } else { host }).or_default() += 1;
        }
        let _ = writeln!(s, "⇗ {} hyperlinks: {}", links.len(), hosts.iter().take(6).map(|(h, n)| format!("{h} ({n})")).collect::<Vec<_>>().join(", "));
    }
    s
}

/// Render one formula family (groups sharing a shape) as one or two lines.
fn family_lines(sh: &Sheet, sa: &SheetAnalysis, fam: &[usize], o: &Opts) -> String {
    let mut out = String::new();
    let g0 = &sa.groups[fam[0]];
    let ids = if fam.len() == 1 {
        format!("ƒ{}", fam[0] + 1)
    } else if fam.windows(2).all(|w| w[1] == w[0] + 1) {
        format!("ƒ{}–{}", fam[0] + 1, fam[fam.len() - 1] + 1)
    } else {
        format!("ƒ{}…", fam[0] + 1)
    };
    let cells: u32 = fam.iter().map(|&g| sa.groups[g].cells).sum();
    let rects: Vec<Rect> = fam.iter().map(|&g| sa.groups[g].rect).collect();
    if fam.len() == 1 {
        let _ = writeln!(out, "{}", group_line(fam[0], g0, o));
    } else if fam.iter().all(|&g| sa.groups[g].key == g0.key) {
        // One formula copied over several ranges: the breaks between them are overrides.
        let _ = writeln!(
            out,
            "{ids} {} {} {}{} — same formula in {} ranges ({} cells): {}{}",
            g0.rect,
            dir(g0),
            trunc(&g0.formula, o.max_formula_chars),
            flags(g0),
            fam.len(),
            cells,
            rect_list(&rects[1..], 6),
            gaps(sh, &rects)
        );
    } else if fam.len() == 2 {
        for &gi in fam {
            let _ = writeln!(out, "{}", group_line(gi, &sa.groups[gi], o));
        }
    } else {
        let _ = writeln!(
            out,
            "{ids} {} similar groups ({} cells; same shape, refs/constants differ) e.g. {} {} {}{}; others at {}",
            fam.len(),
            cells,
            g0.rect,
            dir(g0),
            trunc(&g0.formula, o.max_formula_chars),
            flags(g0),
            rect_list(&rects[1..], 6)
        );
    }
    out
}

/// Ref-free formulas (hand-entered sums) are data entry, not logic: one line.
fn typed_arithmetic(sa: &SheetAnalysis) -> String {
    let consts: Vec<&Group> = sa.groups.iter().filter(|g| g.constant).collect();
    if consts.is_empty() {
        return String::new();
    }
    let cells: u32 = consts.iter().map(|g| g.cells).sum();
    let mut cols: Vec<u32> = consts.iter().map(|g| g.rect.c0).collect();
    cols.sort_unstable();
    cols.dedup();
    let eg: Vec<String> = consts.iter().take(3).map(|g| format!("`{}`", trunc(&g.formula, 40))).collect();
    format!(
        "typed arithmetic: {cells} cells hold ref-free formulas (hand-entered sums) e.g. {} in columns {}\n",
        eg.join(" "),
        trunc(&cols.iter().map(|c| col_name(*c)).collect::<Vec<_>>().join(","), 80)
    )
}

fn spans(v: &[(u32, u32)], f: impl Fn(u32) -> String) -> String {
    let mut parts: Vec<String> = v.iter().take(8).map(|&(a, b)| if a == b { f(a) } else { format!("{}-{}", f(a), f(b)) }).collect();
    if v.len() > 8 {
        parts.push(format!("+{}", v.len() - 8));
    }
    parts.join(", ")
}

fn dir(g: &Group) -> &'static str {
    match (g.rect.rows() > 1, g.rect.cols() > 1) {
        (true, false) => "↓",
        (false, true) => "→",
        (true, true) => "▦",
        _ => "",
    }
}

/// Cells sitting between ranges of the same copied formula, in the same
/// column(s) (or row(s)): usually hardcoded overrides or deliberate exceptions.
fn gaps(sh: &Sheet, rects: &[Rect]) -> String {
    let mut v: Vec<Rect> = rects.to_vec();
    v.sort_by_key(|r| (r.c0, r.c1, r.r0));
    let mut gap_rects = Vec::new();
    for w in v.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a.c0 == b.c0 && a.c1 == b.c1 && b.r0 > a.r1 + 1 {
            gap_rects.push(Rect { r0: a.r1 + 1, c0: a.c0, r1: b.r0 - 1, c1: a.c1 });
        }
    }
    v.sort_by_key(|r| (r.r0, r.r1, r.c0));
    for w in v.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a.r0 == b.r0 && a.r1 == b.r1 && b.c0 > a.c1 + 1 {
            gap_rects.push(Rect { r0: a.r0, c0: a.c1 + 1, r1: a.r1, c1: b.c0 - 1 });
        }
    }
    if gap_rects.is_empty() {
        return String::new();
    }
    // Keep gaps that actually interrupt the formula (hold something, or are tiny);
    // long blank stretches between separate regions are not overrides.
    let (mut vals, mut other_f, mut blank) = (0u64, 0u64, 0u64);
    let mut kept = Vec::new();
    for g in &gap_rects {
        let (mut v, mut f) = (0u64, 0u64);
        for c in cells_in(sh, g) {
            if c.f.is_some() {
                f += 1;
            } else if !c.v.is_empty() {
                v += 1;
            }
        }
        let b = g.area() - v - f;
        if v + f == 0 && b > 3 {
            continue;
        }
        vals += v;
        other_f += f;
        blank += b;
        kept.push(*g);
    }
    if kept.is_empty() {
        return String::new();
    }
    let mut what = Vec::new();
    if vals > 0 {
        what.push(format!("{vals} hardcoded values"));
    }
    if other_f > 0 {
        what.push(format!("{other_f} different formulas"));
    }
    if blank > 0 {
        what.push(format!("{blank} blank"));
    }
    format!("; breaks at {} = {}", rect_list(&kept, 5), what.join(", "))
}

fn flags(g: &Group) -> String {
    let mut f = String::new();
    if g.formula.contains("#REF!") {
        f.push_str(" [#REF!]");
    }
    if g.external {
        f.push_str(" [external]");
    }
    if g.dynamic {
        f.push_str(" [dynamic ref]");
    }
    if g.volatile {
        f.push_str(" [volatile]");
    }
    if g.spill {
        f.push_str(" [array/spills]");
    }
    if g.errors > 0 {
        let _ = write!(f, " [{} errors]", g.errors);
    }
    f
}

fn group_line(gi: usize, g: &Group, o: &Opts) -> String {
    format!("ƒ{} {} {} {}{}", gi + 1, g.rect, dir(g), trunc(&g.formula, o.max_formula_chars), flags(g)).replace("  ", " ")
}

fn validation_desc(v: &Validation) -> String {
    let vals = trunc(&v.values.join(", "), 200);
    let mut d = match v.kind.as_str() {
        "ONE_OF_LIST" => format!("dropdown [{vals}]"),
        "ONE_OF_RANGE" => format!("dropdown from {vals}"),
        "BOOLEAN" => "checkbox".to_string(),
        "CUSTOM_FORMULA" => format!("custom {vals}"),
        k => format!("{k} {vals}").trim_end().to_string(),
    };
    if v.strict {
        d.push_str(" (strict)");
    }
    if let Some(h) = &v.help {
        let _ = write!(d, " help: \"{}\"", trunc(h, 60));
    }
    d
}

fn style_desc(st: &Style) -> String {
    let mut p = Vec::new();
    if let Some(b) = &st.bg {
        p.push(format!("bg{b}"));
    }
    if let Some(f) = &st.fg {
        p.push(format!("fg{f}"));
    }
    for (on, name) in [(st.bold, "bold"), (st.italic, "italic"), (st.strike, "strike"), (st.underline, "underline"), (st.borders, "borders")] {
        if on {
            p.push(name.to_string());
        }
    }
    p.join(" ")
}

// ---------------------------------------------------------------- blocks

fn cell_text(c: &Cell, w: usize) -> String {
    let t = c.display();
    trunc(&t, w)
}

/// Small blocks: every non-empty cell, one line per row.
fn dump_block(s: &mut String, sh: &Sheet, rect: &Rect, group_at: &dyn Fn(u32, u32) -> Option<usize>, o: &Opts) {
    // (row, content) per non-empty row; identical consecutive rows are merged.
    let mut rows: Vec<(u32, String)> = Vec::new();
    for c in cells_in(sh, rect) {
        if c.is_blank() {
            continue;
        }
        if rows.last().map(|r| r.0) != Some(c.r) {
            rows.push((c.r, String::new()));
        }
        let line = &mut rows.last_mut().unwrap().1;
        let _ = write!(line, " {}=", col_name(c.c));
        if c.f.is_some() {
            if let Some(g) = group_at(c.r, c.c) {
                let _ = write!(line, "ƒ{}→", g + 1);
            }
        }
        line.push_str(&cell_text(c, o.cell_width));
    }
    let mut i = 0;
    while i < rows.len() {
        let mut j = i;
        while j + 1 < rows.len() && rows[j + 1].1 == rows[i].1 && rows[j + 1].0 == rows[j].0 + 1 {
            j += 1;
        }
        if j > i {
            let _ = writeln!(s, "  {}–{}:{}", rows[i].0 + 1, rows[j].0 + 1, rows[i].1);
        } else {
            let _ = writeln!(s, "  {}:{}", rows[i].0 + 1, rows[i].1);
        }
        i = j + 1;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Orient {
    Cols,
    Rows,
}

fn is_text(c: &Cell) -> bool {
    matches!(c.v, Value::Str(_)) && c.f.is_none()
}

/// Large blocks: detect header + orientation, profile each column (or row).
fn profile_block(s: &mut String, wb: &Workbook, sh: &Sheet, rect: &Rect, ncells: u32, group_at: &dyn Fn(u32, u32) -> Option<usize>, o: &Opts) {
    let cells: Vec<&Cell> = cells_in(sh, rect).filter(|c| !c.is_blank()).collect();
    let at: HashMap<(u32, u32), &Cell> = cells.iter().map(|c| ((c.r, c.c), *c)).collect();
    let row_cells = |r: u32| -> Vec<&Cell> { (rect.c0..=rect.c1).filter_map(|c| at.get(&(r, c)).copied()).collect() };
    let col_cells = |c: u32| -> Vec<&Cell> { (rect.r0..=rect.r1).filter_map(|r| at.get(&(r, c)).copied()).collect() };

    // Header row: first of the top 3 rows that is mostly text, reasonably full,
    // and whose labels don't recur as values in their column just below.
    let mut header_row = None;
    for r in rect.r0..=(rect.r0 + 2).min(rect.r1) {
        let rc = row_cells(r);
        let texts: Vec<&&Cell> = rc.iter().filter(|c| is_text(c)).collect();
        if rc.len() < 2 || texts.len() * 10 < rc.len() * 6 || (rc.len() as u64) * 2 < rect.cols().min(40) {
            continue;
        }
        let below = (r + 1)..=(r + 30).min(rect.r1);
        let repeats = texts
            .iter()
            .filter(|h| {
                let hv = h.display();
                below.clone().any(|rr| at.get(&(rr, h.c)).is_some_and(|x| x.display() == hv))
            })
            .count();
        if repeats * 4 <= texts.len() {
            header_row = Some(r);
            break;
        }
    }
    // Row labels: first column mostly text while the header row isn't.
    let first_col = col_cells(rect.c0);
    let col_texty = first_col.iter().filter(|c| is_text(c)).count();
    let wide = rect.cols() >= 6 && rect.cols() * 2 >= rect.rows();
    let orient = if wide && header_row.is_none() && first_col.len() as u64 * 2 >= rect.rows() && col_texty * 10 >= first_col.len() * 7 {
        Orient::Rows
    } else {
        Orient::Cols
    };

    let (nr, nc) = (rect.rows(), rect.cols());
    match orient {
        Orient::Cols => {
            let _ = writeln!(
                s,
                "▸ {} table {}×{} ({} cells){}",
                rect,
                nr,
                nc,
                ncells,
                header_row.map(|h| format!(", header row {}", h + 1)).unwrap_or_else(|| ", no header row".into())
            );
        }
        Orient::Rows => {
            let _ = writeln!(s, "▸ {} row-oriented {}×{} ({} cells), labels in column {}", rect, nr, nc, ncells, col_name(rect.c0));
        }
    }

    // Build lines: (index label, header text, data cells)
    let data_start = header_row.map(|h| h + 1).unwrap_or(rect.r0);
    // (index label, header text, header is text, data cells)
    let mut lines: Vec<(String, String, bool, Vec<&Cell>)> = Vec::new();
    match orient {
        Orient::Cols => {
            // Labels above the header row (often merged across a column group,
            // e.g. an item name over its in/out/used columns) prefix the header
            // wherever they change.
            let above = |c: u32| -> String {
                let Some(h) = header_row else { return String::new() };
                let mut parts = Vec::new();
                for r in rect.r0..h {
                    let cell = at.get(&(r, c)).copied().or_else(|| {
                        sh.merges.iter().find(|m| m.contains(r, c)).and_then(|m| find_cell(sh, m.r0, m.c0))
                    });
                    if let Some(x) = cell.filter(|x| !x.is_blank()) {
                        parts.push(cell_text(x, 30));
                    }
                }
                parts.join(" / ")
            };
            let mut last_above = String::new();
            for c in rect.c0..=rect.c1 {
                let hc = header_row.and_then(|h| at.get(&(h, c)));
                let mut head = hc.map(|x| cell_text(x, 30)).unwrap_or_default();
                let sup = above(c);
                if sup != last_above {
                    if !sup.is_empty() {
                        head = format!("[{sup}] {head}");
                    }
                    last_above = sup;
                }
                let data: Vec<&Cell> = (data_start..=rect.r1).filter_map(|r| at.get(&(r, c)).copied()).collect();
                if data.is_empty() && head.is_empty() {
                    continue;
                }
                lines.push((col_name(c), head, hc.is_some_and(|x| is_text(x)), data));
            }
        }
        Orient::Rows => {
            // Column headers for row-oriented blocks often sit in the first rows; show them as a profile line too.
            for r in rect.r0..=rect.r1 {
                let hc = at.get(&(r, rect.c0));
                let head = hc.map(|x| cell_text(x, 30)).unwrap_or_default();
                let data: Vec<&Cell> = (rect.c0 + 1..=rect.c1).filter_map(|c| at.get(&(r, c)).copied()).collect();
                if data.is_empty() && head.is_empty() {
                    continue;
                }
                lines.push(((r + 1).to_string(), head, hc.is_some_and(|x| is_text(x)), data));
            }
        }
    }

    // Profile + collapse runs of identical signature.
    let profiled: Vec<(String, String, bool, String, String)> = lines
        .iter()
        .map(|(idx, head, texty, data)| {
            let (sig, prof) = profile(wb, data, group_at);
            (idx.clone(), head.clone(), *texty, sig, prof)
        })
        .collect();
    let mut i = 0;
    let mut out_lines = 0;
    while i < profiled.len() {
        let mut j = i;
        while j + 1 < profiled.len() && profiled[j + 1].3 == profiled[i].3 {
            j += 1;
        }
        // Collapse runs that are series (date/number/blank headers, e.g. one column
        // per day). Distinctly-labelled text columns stay separate unless the run is long.
        let run = j - i + 1;
        let texty_heads = profiled[i..=j].iter().any(|p| p.2);
        let texty_vals = profiled[i].3.contains('t');
        if run < 4 || ((texty_heads || texty_vals) && run < 12) {
            j = i;
        }
        if out_lines >= o.max_profile_lines {
            let _ = writeln!(s, "  +{} more {}", profiled.len() - i, if orient == Orient::Cols { "columns" } else { "rows" });
            break;
        }
        let (idx, head, _, _, prof) = &profiled[i];
        if j > i {
            let (idx2, head2, ..) = &profiled[j];
            let heads = if texty_heads {
                let all: Vec<String> = profiled[i..=j].iter().map(|p| format!("\"{}\"", p.1)).collect();
                format!(" {}", trunc(&all.join(" "), 240))
            } else if head.is_empty() && head2.is_empty() {
                String::new()
            } else {
                format!(" \"{head}\" … \"{head2}\"")
            };
            let _ = writeln!(s, "  {idx}…{idx2} ({} alike){heads}: {prof}", j - i + 1);
        } else {
            let _ = writeln!(s, "  {idx} {}: {prof}", if head.is_empty() { "-".to_string() } else { format!("\"{head}\"") });
        }
        i = j + 1;
        out_lines += 1;
    }

    // Sample rows (column orientation only; row-oriented blocks are described by their rows already).
    if orient == Orient::Cols && o.sample_rows > 0 {
        let rows_with_data: Vec<u32> = {
            let mut v: Vec<u32> = cells.iter().filter(|c| c.r >= data_start).map(|c| c.r).collect();
            v.dedup();
            v
        };
        let mut pick: Vec<u32> = rows_with_data.iter().take(o.sample_rows).copied().collect();
        if let Some(&last) = rows_with_data.last() {
            if !pick.contains(&last) {
                pick.push(last);
            }
        }
        let cols: Vec<u32> = (rect.c0..=rect.c1).take(o.sample_cols).collect();
        let _ = writeln!(s, "  sample ({}{}):", cols.iter().map(|c| col_name(*c)).collect::<Vec<_>>().first().cloned().unwrap_or_default(), if rect.cols() > o.sample_cols as u64 { format!("…{} of {}", col_name(rect.c0 + o.sample_cols as u32 - 1), rect.cols()) } else { format!("…{}", col_name(rect.c1)) });
        for r in pick {
            let vals: Vec<String> = cols.iter().map(|c| at.get(&(r, *c)).map(|x| cell_text(x, o.cell_width)).unwrap_or_default()).collect();
            let _ = writeln!(s, "  {}| {}", r + 1, vals.join(" | "));
        }
    }
}

/// Returns (signature for run-collapsing, human profile).
fn profile(wb: &Workbook, data: &[&Cell], group_at: &dyn Fn(u32, u32) -> Option<usize>) -> (String, String) {
    if data.is_empty() {
        return ("empty".into(), "empty".into());
    }
    let n = data.len();
    let mut nums: Vec<&Cell> = Vec::new();
    let mut dates: Vec<&Cell> = Vec::new();
    let mut texts: Vec<&str> = Vec::new();
    let (mut bools, mut errs, mut formulas) = (0, 0, 0);
    let mut groups: Vec<usize> = Vec::new();
    let mut fmts: HashMap<&str, usize> = HashMap::new();
    for c in data {
        if c.f.is_some() {
            formulas += 1;
            if let Some(g) = group_at(c.r, c.c) {
                if !groups.contains(&g) {
                    groups.push(g);
                }
            }
        }
        let st = c.s.map(|s| &wb.styles[s as usize]);
        if let Some(f) = st.and_then(|s| s.num_fmt.as_deref()) {
            *fmts.entry(f).or_default() += 1;
        }
        match &c.v {
            Value::Num(_) if st.is_some_and(|s| s.is_date()) || c.t.as_deref().is_some_and(looks_date) => dates.push(c),
            Value::Num(_) => nums.push(c),
            Value::Str(t) => texts.push(t),
            Value::Bool(_) => bools += 1,
            Value::Err { .. } => errs += 1,
            Value::Empty => {}
        }
    }
    let mut parts: Vec<String> = Vec::new();
    let mut sig = String::new();
    if formulas * 2 >= n {
        let ids: Vec<String> = groups.iter().take(3).map(|g| format!("ƒ{}", g + 1)).collect();
        parts.push(format!("{}{}", ids.join(","), if groups.len() > 3 { format!(" +{}", groups.len() - 3) } else { String::new() }));
        sig.push_str("f:");
        // Family-level signature so neighbouring columns of the same computed shape collapse.
        sig.push_str(&groups.len().min(2).to_string());
    }
    let span = |v: &Vec<&Cell>| -> String {
        let mut v = v.clone();
        v.sort_by(|a, b| num(a).partial_cmp(&num(b)).unwrap_or(std::cmp::Ordering::Equal));
        let (lo, hi) = (v[0], v[v.len() - 1]);
        if num(lo) == num(hi) { lo.display() } else { format!("{} … {}", lo.display(), hi.display()) }
    };
    if !dates.is_empty() {
        parts.push(format!("date {}", span(&dates)));
        sig.push('d');
    }
    if !nums.is_empty() {
        let mut p = format!("num {}", span(&nums));
        if let Some((f, _)) = fmts.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))) {
            if dates.is_empty() {
                let _ = write!(p, " fmt {f}");
            }
        }
        parts.push(p);
        sig.push('n');
    }
    if !texts.is_empty() {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for t in &texts {
            *counts.entry(t).or_default() += 1;
        }
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        if v.len() <= 8 && texts.len() >= v.len() * 2 && texts.len() > 3 {
            parts.push(format!("enum {}", v.iter().map(|(t, c)| format!("{}({c})", trunc(t, 24))).collect::<Vec<_>>().join(" ")));
        } else if v.len() == 1 {
            parts.push(format!("text \"{}\"", trunc(v[0].0, 30)));
        } else {
            let eg: Vec<String> = v.iter().take(3).map(|(t, _)| format!("\"{}\"", trunc(t, 24))).collect();
            parts.push(format!("text {} distinct e.g. {}", v.len(), eg.join(", ")));
        }
        sig.push('t');
    }
    if bools > 0 {
        parts.push(format!("bool {bools}"));
        sig.push('b');
    }
    if errs > 0 {
        parts.push(format!("{errs} errors"));
        sig.push('e');
    }
    if n < 3 && formulas * 2 < n {
        // too few cells to profile meaningfully: show them
        let vals: Vec<String> = data.iter().map(|c| format!("{}={}", crate::a1::cell_name(c.r, c.c), trunc(&c.display(), 24))).collect();
        return (format!("few:{}", vals.join(",")), vals.join(" "));
    }
    let _ = n;
    (sig, format!("{} ({} filled)", parts.join("; "), n))
}

fn num(c: &Cell) -> f64 {
    match c.v {
        Value::Num(n) => n,
        _ => f64::NAN,
    }
}

fn looks_date(t: &str) -> bool {
    let b = t.as_bytes();
    let seps = b.iter().filter(|c| **c == b'/' || **c == b'-').count();
    seps == 2 && b.iter().filter(|c| c.is_ascii_digit()).count() >= 4 && t.len() <= 20
}

/// A sheet with the same layout as an earlier one: say so, then only what differs.
fn render_clone(s: &mut String, wb: &Workbook, a: &Analysis, si: usize, p: usize, o: &Opts) {
    let (sh, sa) = (&wb.sheets[si], &a.sheets[si]);
    let (ps, pa) = (&wb.sheets[p], &a.sheets[p]);
    let _ = writeln!(
        s,
        "same layout as \"{}\" ({:.0}% shared formulas/labels), different data — full section: `sheetgraph sheet <dir> \"{}\"`",
        ps.name,
        sa.clone_similarity * 100.0,
        sh.name
    );
    let mut blocks: Vec<&crate::analyze::Block> = sa.blocks.iter().collect();
    blocks.sort_by_key(|b| std::cmp::Reverse(b.cells));
    let bl: Vec<String> = blocks.iter().take(6).map(|b| format!("{} ({} cells)", b.rect, b.cells)).collect();
    let _ = writeln!(s, "blocks: {}{}", bl.join(", "), if blocks.len() > 6 { format!(" +{}", blocks.len() - 6) } else { String::new() });
    let proto_keys: HashSet<&String> = pa.family_keys.iter().collect();
    let new_fams: Vec<&Vec<usize>> = sa.families.iter().enumerate().filter(|(fi, f)| !sa.groups[f[0]].constant && !proto_keys.contains(&sa.family_keys[*fi])).map(|(_, f)| f).collect();
    let _ = writeln!(s, "ƒ {} cells in {} groups; {} families not in \"{}\"{}", k(sa.n_formulas), sa.groups.len(), new_fams.len(), ps.name, if new_fams.is_empty() { "" } else { ":" });
    for fam in new_fams.iter().take(o.max_formula_lines.min(10)) {
        s.push_str(&family_lines(sh, sa, fam, o));
    }
    if new_fams.len() > 10 {
        let _ = writeln!(s, "+{} more", new_fams.len() - 10);
    }
    s.push_str(&typed_arithmetic(sa));
    let (fp, pfp) = (fingerprint(sh, sa), fingerprint(ps, pa));
    let mut added: Vec<String> = fp.difference(&pfp).filter_map(|x| x.strip_prefix("t:")).map(|x| {
        let (c, t) = x.split_once(':').unwrap_or(("", x));
        format!("{}:\"{}\"", col_name(c.parse().unwrap_or(0)), trunc(t, 24))
    }).collect();
    added.sort();
    if !added.is_empty() {
        let _ = writeln!(s, "labels not in \"{}\": {}", ps.name, trunc(&added.join(" "), 300));
    }
    let notes: Vec<&Cell> = sh.cells.iter().filter(|c| c.note.is_some()).collect();
    for c in notes.iter().take(5) {
        let _ = writeln!(s, "✎ {}: {}", crate::a1::cell_name(c.r, c.c), trunc(c.note.as_deref().unwrap_or(""), 100));
    }
    if notes.len() > 5 {
        let _ = writeln!(s, "✎ +{} more notes", notes.len() - 5);
    }
}
