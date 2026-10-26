//! Turns a cell-level workbook into region-level structure:
//! - formula groups: rectangles of copied formulas (same R1C1 form)
//! - families: groups whose formulas differ only in refs/numbers
//! - blocks: connected regions of non-empty cells (tables, label clusters)
//! - edges: group → referenced range, resolved to sheets / named ranges / externals

use crate::a1::{parse_ref, Rect};
use crate::formula::{self, call_args, lex, unquote, Lexed};
use crate::model::{Cell, Sheet, Value, Workbook};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct Analysis {
    pub sheets: Vec<SheetAnalysis>,
    /// (reader sheet, source sheet) → (groups, cells)
    pub sheet_edges: BTreeMap<(usize, usize), (usize, u64)>,
    pub externals: Vec<External>,
    /// Non-builtin function → formula cells using it.
    pub custom_funcs: BTreeMap<String, u64>,
    /// Builtin function → formula cells using it.
    pub func_cells: BTreeMap<String, u64>,
    /// References to sheets that do not exist in this workbook.
    pub broken_refs: BTreeMap<String, u64>,
    pub named: HashMap<String, (Option<usize>, Rect)>,
}

#[derive(Debug, Clone, Default)]
pub struct SheetAnalysis {
    pub used: Option<Rect>,
    pub n_cells: u64,
    pub n_formulas: u64,
    pub n_errors: u64,
    pub groups: Vec<Group>,
    pub blocks: Vec<Block>,
    /// family id → group indices (in id order)
    pub families: Vec<Vec<usize>>,
    /// family id → family key (refs and numbers erased), for cross-sheet comparison.
    pub family_keys: Vec<String>,
    /// Sheet index this sheet is a structural clone of (same layout, other data).
    pub clone_of: Option<usize>,
    pub clone_similarity: f32,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub rect: Rect,
    /// R1C1 key id: groups with the same key hold the exact same (copied) formula.
    pub key: u32,
    pub formula: String,
    pub cells: u32,
    pub family: usize,
    pub refs: Vec<GRef>,
    pub funcs: Vec<String>,
    pub errors: u32,
    pub dynamic: bool,
    pub volatile: bool,
    pub spill: bool,
    pub external: bool,
    /// No refs, names or functions: hand-typed arithmetic like `=3000+1577`.
    pub constant: bool,
}

#[derive(Debug, Clone)]
pub struct GRef {
    /// Resolved target sheet; None when the sheet is unknown (broken / other file).
    pub sheet: Option<usize>,
    pub sheet_name: Option<String>,
    /// Union of the ranges this ref touches across the whole group.
    pub rect: Rect,
    pub via_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub rect: Rect,
    pub cells: u32,
    pub formulas: u32,
}

#[derive(Debug, Clone)]
pub struct External {
    pub sheet: usize,
    pub group: usize,
    pub func: String,
    /// Spreadsheet id / URL / ticker, resolved from cell refs where possible.
    pub target: String,
    pub range: Option<String>,
}

pub fn analyze(wb: &Workbook) -> Analysis {
    let mut named = HashMap::new();
    for nr in &wb.named_ranges {
        if let Some((sh, rect)) = parse_ref(&nr.range) {
            let si = sh.as_deref().and_then(|s| sheet_index(wb, s));
            named.insert(nr.name.to_ascii_uppercase(), (si, rect));
        }
    }
    let mut a = Analysis {
        sheets: Vec::new(),
        sheet_edges: BTreeMap::new(),
        externals: Vec::new(),
        custom_funcs: BTreeMap::new(),
        func_cells: BTreeMap::new(),
        broken_refs: BTreeMap::new(),
        named,
    };
    for (si, sh) in wb.sheets.iter().enumerate() {
        let sa = analyze_sheet(wb, si, sh, &mut a);
        a.sheets.push(sa);
    }
    find_clones(wb, &mut a);
    a
}

/// Structural fingerprint: formula families (copied ≥2 cells, ignoring typed
/// arithmetic) plus the text labels in the top rows / first columns.
pub fn fingerprint(sh: &Sheet, sa: &SheetAnalysis) -> std::collections::HashSet<String> {
    let mut fp = std::collections::HashSet::new();
    for (fi, fam) in sa.families.iter().enumerate() {
        let cells: u32 = fam.iter().map(|&g| sa.groups[g].cells).sum();
        if cells >= 2 && !sa.groups[fam[0]].constant {
            fp.insert(format!("f:{}", sa.family_keys[fi]));
        }
    }
    for c in &sh.cells {
        if let (Value::Str(t), None) = (&c.v, &c.f) {
            if c.r < 4 || c.c < 2 {
                fp.insert(format!("t:{}:{}", c.c, t.trim()));
            }
        }
    }
    fp
}

fn find_clones(wb: &Workbook, a: &mut Analysis) {
    let fps: Vec<_> = wb.sheets.iter().zip(&a.sheets).map(|(sh, sa)| fingerprint(sh, sa)).collect();
    let mut protos: Vec<usize> = Vec::new();
    for i in 0..wb.sheets.len() {
        if fps[i].len() < 4 {
            continue;
        }
        let best = protos
            .iter()
            .map(|&p| {
                let inter = fps[i].intersection(&fps[p]).count() as f32;
                let uni = fps[i].union(&fps[p]).count() as f32;
                (p, inter / uni)
            })
            .max_by(|x, y| x.1.total_cmp(&y.1));
        match best {
            Some((p, sim)) if sim >= 0.6 => {
                a.sheets[i].clone_of = Some(p);
                a.sheets[i].clone_similarity = sim;
            }
            _ => protos.push(i),
        }
    }
}

pub fn sheet_index(wb: &Workbook, name: &str) -> Option<usize> {
    wb.sheets.iter().position(|s| s.name == name).or_else(|| {
        let l = name.to_lowercase();
        wb.sheets.iter().position(|s| s.name.to_lowercase() == l)
    })
}

/// Index of the cell at (r, c) in a sheet's sorted cell list.
pub fn find_cell(sh: &Sheet, r: u32, c: u32) -> Option<&Cell> {
    sh.cells.binary_search_by_key(&(r, c), |x| (x.r, x.c)).ok().map(|i| &sh.cells[i])
}

/// Cells within a rectangle (rows are a contiguous slice of the sorted list).
pub fn cells_in<'a>(sh: &'a Sheet, rect: &Rect) -> impl Iterator<Item = &'a Cell> + 'a {
    let lo = sh.cells.partition_point(|x| x.r < rect.r0);
    let hi = sh.cells.partition_point(|x| x.r <= rect.r1);
    let rect = *rect;
    sh.cells[lo..hi].iter().filter(move |x| x.c >= rect.c0 && x.c <= rect.c1)
}

fn analyze_sheet(wb: &Workbook, si: usize, sh: &Sheet, a: &mut Analysis) -> SheetAnalysis {
    let mut sa = SheetAnalysis::default();
    let mut used: Option<Rect> = None;
    // key id per formula cell
    let mut keys: HashMap<String, u32> = HashMap::new();
    let mut fam_keys: HashMap<String, usize> = HashMap::new();
    let mut key_family: Vec<usize> = Vec::new();
    let mut fcells: Vec<(u32, u32, u32)> = Vec::new(); // (c, r, key)
    for cell in &sh.cells {
        if cell.is_blank() {
            continue;
        }
        sa.n_cells += 1;
        let r = Rect::cell(cell.r, cell.c);
        used = Some(used.map_or(r, |u| u.union(&r)));
        if matches!(cell.v, Value::Err { .. }) {
            sa.n_errors += 1;
        }
        if let Some(f) = &cell.f {
            sa.n_formulas += 1;
            let lx = lex(f);
            for fu in &lx.funcs {
                let m = if formula::is_builtin(&fu.name) { &mut a.func_cells } else { &mut a.custom_funcs };
                *m.entry(fu.name.clone()).or_default() += 1;
            }
            let key = formula::r1c1_key(f, &lx, cell.r, cell.c);
            let n = keys.len() as u32;
            let k = *keys.entry(key).or_insert(n);
            if k == n {
                let fk = formula::family_key(f, &lx);
                let nf = fam_keys.len();
                key_family.push(*fam_keys.entry(fk).or_insert(nf));
            }
            fcells.push((cell.c, cell.r, k));
        }
    }
    sa.used = used;

    // Vertical runs per column, then merge runs horizontally into rectangles.
    fcells.sort_unstable();
    let mut rects: Vec<(u32, Rect)> = Vec::new();
    let mut active: HashMap<(u32, u32, u32), usize> = HashMap::new();
    let mut i = 0;
    while i < fcells.len() {
        let (c, r0, k) = fcells[i];
        let mut j = i;
        while j + 1 < fcells.len() && fcells[j + 1].0 == c && fcells[j + 1].1 == fcells[j].1 + 1 && fcells[j + 1].2 == k {
            j += 1;
        }
        let r1 = fcells[j].1;
        match active.get(&(k, r0, r1)) {
            Some(&idx) if rects[idx].1.c1 + 1 == c => rects[idx].1.c1 = c,
            _ => {
                active.insert((k, r0, r1), rects.len());
                rects.push((k, Rect { r0, c0: c, r1, c1: c }));
            }
        }
        i = j + 1;
    }

    // Order families by first appearance (row-major), groups within by position.
    rects.sort_by_key(|(_, r)| (r.r0, r.c0));
    let mut fam_order: HashMap<usize, usize> = HashMap::new();
    for (k, _) in &rects {
        let f = key_family[*k as usize];
        let n = fam_order.len();
        fam_order.entry(f).or_insert(n);
    }
    rects.sort_by_key(|(k, r)| (fam_order[&key_family[*k as usize]], r.r0, r.c0));
    sa.families = vec![Vec::new(); fam_order.len()];
    sa.family_keys = vec![String::new(); fam_order.len()];
    for (key, orig) in &fam_keys {
        if let Some(&new) = fam_order.get(orig) {
            sa.family_keys[new] = key.clone();
        }
    }

    for (k, rect) in rects {
        let family = fam_order[&key_family[k as usize]];
        let anchor = find_cell(sh, rect.r0, rect.c0).expect("anchor cell");
        let f = anchor.f.clone().unwrap_or_default();
        let lx = lex(&f);
        let gi = sa.groups.len();
        let mut g = Group {
            rect,
            key: k,
            formula: f.clone(),
            cells: rect.area() as u32,
            family,
            refs: Vec::new(),
            funcs: dedup(lx.funcs.iter().map(|f| f.name.clone()).collect()),
            errors: 0,
            dynamic: false,
            volatile: false,
            spill: false,
            external: false,
            constant: lx.refs.is_empty() && lx.names.is_empty() && lx.funcs.is_empty(),
        };
        for fu in &g.funcs {
            g.dynamic |= formula::DYNAMIC_FUNCS.contains(&fu.as_str());
            g.volatile |= formula::VOLATILE_FUNCS.contains(&fu.as_str());
            g.spill |= formula::SPILL_FUNCS.contains(&fu.as_str());
            g.external |= formula::EXTERNAL_FUNCS.contains(&fu.as_str());
        }
        g.errors = cells_in(sh, &rect).filter(|c| matches!(c.v, Value::Err { .. })).count() as u32;
        g.refs = resolve_refs(wb, si, &lx, &rect, a);
        let mut targets: Vec<usize> = g.refs.iter().filter_map(|r| r.sheet).filter(|t| *t != si).collect();
        targets.sort_unstable();
        targets.dedup();
        for t in targets {
            let e = a.sheet_edges.entry((si, t)).or_default();
            e.0 += 1;
            e.1 += g.cells as u64;
        }
        for r in &g.refs {
            if r.sheet.is_some() {
                continue;
            } else if let Some(n) = &r.sheet_name {
                *a.broken_refs.entry(n.clone()).or_default() += g.cells as u64;
            }
        }
        if g.external {
            collect_externals(sh, si, gi, &f, &lx, &rect, a);
        }
        sa.families[family].push(gi);
        sa.groups.push(g);
    }

    sa.blocks = find_blocks(sh);
    sa
}

fn dedup(mut v: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    v.retain(|x| seen.insert(x.clone()));
    v
}

fn resolve_refs(wb: &Workbook, si: usize, lx: &Lexed, rect: &Rect, a: &Analysis) -> Vec<GRef> {
    let (dr, dc) = (rect.r1 as i64 - rect.r0 as i64, rect.c1 as i64 - rect.c0 as i64);
    let mut out = Vec::new();
    for t in &lx.refs {
        let (Some(a0), Some(a1)) = (t.rect_at(0, 0), t.rect_at(dr, dc)) else { continue };
        let (sheet, sheet_name) = match &t.sheet {
            None => (Some(si), None),
            Some(n) => (sheet_index(wb, n), Some(n.clone())),
        };
        out.push(GRef { sheet, sheet_name, rect: a0.union(&a1), via_name: None });
    }
    for n in &lx.names {
        if let Some((sh, r)) = a.named.get(&n.to_ascii_uppercase()) {
            out.push(GRef { sheet: sh.or(Some(si)), sheet_name: None, rect: *r, via_name: Some(n.clone()) });
        }
    }
    out
}

fn collect_externals(sh: &Sheet, si: usize, gi: usize, f: &str, lx: &Lexed, rect: &Rect, a: &mut Analysis) {
    // Resolve an argument: string literal, or a same-sheet cell holding the value.
    let resolve = |arg: &str| -> String {
        if let Some(s) = unquote(arg) {
            return s;
        }
        if let Some((None, r)) = parse_ref(arg) {
            if let Some(c) = find_cell(sh, r.r0, r.c0) {
                return c.display();
            }
        }
        arg.to_string()
    };
    for fu in &lx.funcs {
        if !formula::EXTERNAL_FUNCS.contains(&fu.name.as_str()) {
            continue;
        }
        let args = call_args(f, fu.open);
        let target = args.first().map(|s| resolve(s)).unwrap_or_default();
        let (target, range) = match fu.name.as_str() {
            "IMPORTRANGE" => (formula::spreadsheet_id(&target), args.get(1).map(|s| resolve(s))),
            "IMPORTHTML" | "IMPORTXML" => (target, args.get(1).map(|s| resolve(s))),
            _ => (target, None),
        };
        // One entry per distinct (func, target, range) per group.
        if !a.externals.iter().any(|e| e.sheet == si && e.group == gi && e.func == fu.name && e.target == target && e.range == range) {
            a.externals.push(External { sheet: si, group: gi, func: fu.name.clone(), target, range });
        }
    }
    let _ = rect;
}

/// Connected regions of non-empty cells. Cells join when horizontally within
/// one blank column of each other, or vertically adjacent with overlapping spans.
fn find_blocks(sh: &Sheet) -> Vec<Block> {
    // Row segments.
    let mut segs: Vec<(u32, u32, u32)> = Vec::new(); // (r, c0, c1)
    for cell in sh.cells.iter().filter(|c| !c.is_blank()) {
        match segs.last_mut() {
            Some(s) if s.0 == cell.r && cell.c <= s.2 + 2 => s.2 = cell.c,
            _ => segs.push((cell.r, cell.c, cell.c)),
        }
    }
    let mut uf: Vec<usize> = (0..segs.len()).collect();
    fn find(uf: &mut [usize], mut x: usize) -> usize {
        while uf[x] != x {
            uf[x] = uf[uf[x]];
            x = uf[x];
        }
        x
    }
    let mut prev: (usize, usize) = (0, 0); // segment index range of previous row
    let mut i = 0;
    while i < segs.len() {
        let r = segs[i].0;
        let mut j = i;
        while j < segs.len() && segs[j].0 == r {
            j += 1;
        }
        if prev.1 > prev.0 && segs[prev.0].0 + 1 == r {
            let (mut p, mut q) = (prev.0, i);
            while p < prev.1 && q < j {
                let (a, b) = (segs[p], segs[q]);
                if a.1 <= b.2 && b.1 <= a.2 {
                    let (x, y) = (find(&mut uf, p), find(&mut uf, q));
                    uf[x] = y;
                }
                if a.2 < b.2 { p += 1 } else { q += 1 }
            }
        }
        prev = (i, j);
        i = j;
    }
    let mut comps: HashMap<usize, Rect> = HashMap::new();
    for (k, s) in segs.iter().enumerate() {
        let root = find(&mut uf, k);
        let r = Rect { r0: s.0, c0: s.1, r1: s.0, c1: s.2 };
        comps.entry(root).and_modify(|x| *x = x.union(&r)).or_insert(r);
    }
    let mut boxes: Vec<Rect> = comps.into_values().collect();
    // Merge overlapping bounding boxes until stable.
    if boxes.len() <= 4000 {
        loop {
            boxes.sort();
            let mut merged = false;
            let mut out: Vec<Rect> = Vec::with_capacity(boxes.len());
            'next: for b in boxes {
                for o in out.iter_mut() {
                    if o.intersects(&b) {
                        *o = o.union(&b);
                        merged = true;
                        continue 'next;
                    }
                }
                out.push(b);
            }
            boxes = out;
            if !merged {
                break;
            }
        }
    }
    boxes.sort_by_key(|b| (b.r0, b.c0));
    boxes
        .into_iter()
        .map(|rect| {
            let (mut cells, mut formulas) = (0, 0);
            for c in cells_in(sh, &rect) {
                if !c.is_blank() {
                    cells += 1;
                    if c.f.is_some() {
                        formulas += 1;
                    }
                }
            }
            Block { rect, cells, formulas }
        })
        .collect()
}

/// Cover a set of cells (with a key) by rectangles: vertical runs merged sideways.
pub fn cover<K: Copy + Eq + std::hash::Hash + Ord>(mut pts: Vec<(K, u32, u32)>) -> Vec<(K, Rect)> {
    // pts: (key, r, c)
    pts.sort_unstable_by_key(|&(k, r, c)| (c, r, k));
    let mut rects: Vec<(K, Rect)> = Vec::new();
    let mut active: HashMap<(K, u32, u32), usize> = HashMap::new();
    let mut i = 0;
    while i < pts.len() {
        let (k, r0, c) = pts[i];
        let mut j = i;
        while j + 1 < pts.len() && pts[j + 1].2 == c && pts[j + 1].1 == pts[j].1 + 1 && pts[j + 1].0 == k {
            j += 1;
        }
        let r1 = pts[j].1;
        match active.get(&(k, r0, r1)) {
            Some(&idx) if rects[idx].1.c1 + 1 == c => rects[idx].1.c1 = c,
            _ => {
                active.insert((k, r0, r1), rects.len());
                rects.push((k, Rect { r0, c0: c, r1, c1: c }));
            }
        }
        i = j + 1;
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Cell, Sheet, Value, Workbook};

    fn cell(r: u32, c: u32, v: f64, f: Option<&str>) -> Cell {
        Cell { r, c, v: Value::Num(v), f: f.map(String::from), ..Default::default() }
    }

    #[test]
    fn groups_copies_and_breaks() {
        // B1:B4 = A*2 except B3 hardcoded; C1:E1 = year-style fill right.
        let mut cells = vec![];
        for r in 0..4 {
            cells.push(cell(r, 0, r as f64, None));
            if r == 2 {
                cells.push(cell(r, 1, 99.0, None));
            } else {
                cells.push(cell(r, 1, 0.0, Some(&format!("=A{}*2", r + 1))));
            }
        }
        for c in 2..5 {
            cells.push(cell(0, c, 0.0, Some(&format!("=Data!{}1+$A$1", crate::a1::col_name(c)))));
        }
        cells.sort_by_key(|c| (c.r, c.c));
        let data = Sheet { name: "Data".into(), grid_rows: 10, grid_cols: 10, cells: vec![cell(0, 2, 1.0, None)], ..Default::default() };
        let main = Sheet { name: "Main".into(), grid_rows: 10, grid_cols: 10, cells, ..Default::default() };
        let wb = Workbook { sheets: vec![main, data], ..Default::default() };
        let a = analyze(&wb);
        let g: Vec<String> = a.sheets[0].groups.iter().map(|g| g.rect.a1()).collect();
        assert_eq!(g, vec!["B1:B2", "B4", "C1:E1"]);
        // B1:B2 and B4 are the same copied formula (same key) → one family.
        assert_eq!(a.sheets[0].groups[0].key, a.sheets[0].groups[1].key);
        assert_eq!(a.sheets[0].families.len(), 2);
        // Main reads Data via one group.
        assert_eq!(a.sheet_edges.get(&(0, 1)).map(|e| e.0), Some(1));
        // The ref union of C1:E1 covers Data!C1:E1.
        let r = &a.sheets[0].groups[2].refs[0];
        assert_eq!((r.sheet, r.rect.a1()), (Some(1), "C1:E1".to_string()));
    }
}
