//! Cell-level queries against an extracted workbook: exact cells, precedent /
//! dependent tracing, and text search. These are what an agent calls after
//! reading REPORT.md.

use crate::a1::{cell_name, parse_ref, quote_sheet, Rect};
use crate::analyze::{cells_in, cover, find_cell, sheet_index, Analysis};
use crate::formula::lex;
use crate::model::{Cell, Workbook};
use crate::report::trunc;
use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fmt::Write;

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
    if let Some(f) = &c.f {
        let g = group_of(a, si, c.r, c.c).map(|g| format!("ƒ{} ", g + 1)).unwrap_or_default();
        let _ = write!(s, "  {g}{}", trunc(f, 300));
    }
    if let Some(st) = c.s {
        let st = &wb.styles[st as usize];
        let mut bits = Vec::new();
        bits.extend(st.num_fmt.clone().map(|f| format!("fmt {f}")));
        bits.extend(st.bg.clone().map(|b| format!("bg{b}")));
        bits.extend(st.fg.clone().map(|b| format!("fg{b}")));
        if st.bold {
            bits.push("bold".into());
        }
        if !bits.is_empty() {
            let _ = write!(s, "  [{}]", bits.join(" "));
        }
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
        return match find_cell(sh, rect.r0, rect.c0) {
            Some(c) => format!("{}!{}", quote_sheet(&sh.name), describe_cell(wb, a, si, c)),
            None => format!("{}!{} (blank)", quote_sheet(&sh.name), rect),
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
pub fn trace(wb: &Workbook, a: &Analysis, cell: &str, depth: usize, up: bool, down: bool) -> Result<String> {
    let (si, rect) = resolve(wb, cell, None)?;
    let mut s = describe_range(wb, a, si, &rect) + "\n";
    if up {
        let _ = writeln!(s, "precedents:");
        let mut seen = HashSet::new();
        precedents(wb, a, si, &rect, 1, depth, &mut seen, &mut s);
    }
    if down {
        let _ = writeln!(s, "dependents:");
        let mut seen = HashSet::new();
        dependents(wb, a, si, &rect, 1, depth, &mut seen, &mut s);
    }
    Ok(s)
}

#[allow(clippy::too_many_arguments)]
fn precedents(wb: &Workbook, a: &Analysis, si: usize, rect: &Rect, d: usize, max: usize, seen: &mut HashSet<(usize, Rect)>, out: &mut String) {
    if d > max {
        return;
    }
    let sh = &wb.sheets[si];
    // Exact refs of each formula cell in the range (shifted per cell), merged.
    let mut refs: Vec<(usize, Rect)> = Vec::new();
    let mut externals: Vec<String> = Vec::new();
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
            let e = format!("{} {}", fu.name, args.join(" "));
            if !externals.contains(&e) {
                externals.push(e);
            }
        }
        for t in &lx.refs {
            let Some(r) = t.rect() else { continue };
            if let Some(n) = t.sheet.as_deref().filter(|n| n.starts_with('[')) {
                let e = format!("'{n}'!{r}");
                if !externals.contains(&e) {
                    externals.push(e);
                }
                continue;
            }
            let ts = match &t.sheet {
                None => Some(si),
                Some(n) => sheet_index(wb, n),
            };
            if let Some(ts) = ts {
                merge_ref(&mut refs, ts, r);
            }
        }
        for n in &lx.names {
            if let Some((ns, r)) = a.named.get(&n.to_ascii_uppercase()) {
                merge_ref(&mut refs, ns.unwrap_or(si), *r);
            }
        }
    }
    for e in &externals {
        let _ = writeln!(out, "{}← external: {}", "  ".repeat(d), trunc(e, 200));
    }
    for (ts, r) in refs {
        let pad = "  ".repeat(d);
        let _ = writeln!(out, "{pad}← {}", describe_range(wb, a, ts, &r));
        if seen.insert((ts, r)) {
            precedents(wb, a, ts, &r, d + 1, max, seen, out);
        }
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

#[allow(clippy::too_many_arguments)]
fn dependents(wb: &Workbook, a: &Analysis, si: usize, rect: &Rect, d: usize, max: usize, seen: &mut HashSet<(usize, Rect)>, out: &mut String) {
    if d > max {
        return;
    }
    // Candidate groups via their union refs, then exact per-cell check.
    for (gs, sa) in a.sheets.iter().enumerate() {
        for (gi, g) in sa.groups.iter().enumerate() {
            if !g.refs.iter().any(|r| r.sheet == Some(si) && r.rect.intersects(rect)) {
                continue;
            }
            let lx = lex(&g.formula);
            let mut hits: Vec<((), u32, u32)> = Vec::new();
            let mut checked = 0;
            for r in g.rect.r0..=g.rect.r1 {
                for c in g.rect.c0..=g.rect.c1 {
                    checked += 1;
                    if checked > 200_000 {
                        break;
                    }
                    let (dr, dc) = (r as i64 - g.rect.r0 as i64, c as i64 - g.rect.c0 as i64);
                    let hit = lx.refs.iter().any(|t| {
                        let ts = match &t.sheet {
                            None => Some(gs),
                            Some(n) => sheet_index(wb, n),
                        };
                        ts == Some(si) && t.rect_at(dr, dc).is_some_and(|x| x.intersects(rect))
                    }) || lx.names.iter().any(|n| a.named.get(&n.to_ascii_uppercase()).is_some_and(|(ns, x)| ns.unwrap_or(gs) == si && x.intersects(rect)));
                    if hit {
                        hits.push(((), r, c));
                    }
                }
            }
            for (_, hr) in cover(hits) {
                if gs == si && hr.intersects(rect) && hr == *rect {
                    continue;
                }
                let pad = "  ".repeat(d);
                let _ = writeln!(out, "{pad}→ {}!{} ƒ{} {}", quote_sheet(&wb.sheets[gs].name), hr, gi + 1, trunc(&g.formula, 160));
                if seen.insert((gs, hr)) {
                    dependents(wb, a, gs, &hr, d + 1, max, seen, out);
                }
            }
        }
    }
    // Charts and pivots reading the range.
    for (cs, sh) in wb.sheets.iter().enumerate() {
        for ch in &sh.charts {
            for src in &ch.sources {
                if let Some((Some(n), r)) = parse_ref(src)
                    && sheet_index(wb, &n) == Some(si) && r.intersects(rect) {
                        let _ = writeln!(out, "{}→ chart \"{}\" on {}", "  ".repeat(d), ch.title, wb.sheets[cs].name);
                    }
            }
        }
    }
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
