//! graph.json: region-level dependency graph for programmatic consumers.

use crate::a1::{quote_sheet, Rect};
use crate::analyze::Analysis;
use crate::model::Workbook;
use serde_json::{json, Value as J};

pub const SCHEMA: &str = "sheetgraph/1";
/// Beyond this many target nodes per reference, link to the sheet instead.
const MAX_TARGETS: usize = 25;

pub fn group_id(si: usize, gi: usize) -> String {
    format!("s{}.f{}", si + 1, gi + 1)
}
pub fn block_id(si: usize, bi: usize) -> String {
    format!("s{}.b{}", si + 1, bi + 1)
}

pub fn build(wb: &Workbook, a: &Analysis) -> J {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (si, sh) in wb.sheets.iter().enumerate() {
        let sa = &a.sheets[si];
        nodes.push(json!({
            "id": format!("s{}", si + 1), "type": "sheet", "name": sh.name, "hidden": sh.hidden,
            "used": sa.used.map(|u| u.a1()), "cells": sa.n_cells, "formulas": sa.n_formulas,
        }));
        for (bi, b) in sa.blocks.iter().enumerate() {
            nodes.push(json!({
                "id": block_id(si, bi), "type": "block", "sheet": format!("s{}", si + 1),
                "range": b.rect.a1(), "cells": b.cells, "formulas": b.formulas,
            }));
        }
        for (gi, g) in sa.groups.iter().enumerate() {
            let mut flags = Vec::new();
            for (on, name) in [(g.external, "external"), (g.dynamic, "dynamic"), (g.volatile, "volatile"), (g.spill, "spill")] {
                if on {
                    flags.push(name);
                }
            }
            nodes.push(json!({
                "id": group_id(si, gi), "type": "formula", "sheet": format!("s{}", si + 1),
                "range": g.rect.a1(), "formula": g.formula, "cells": g.cells, "family": g.family,
                "funcs": g.funcs, "flags": flags, "errors": g.errors,
            }));
            for r in &g.refs {
                let range_txt = match (r.sheet, &r.sheet_name) {
                    (Some(t), _) => format!("{}!{}", quote_sheet(&wb.sheets[t].name), r.rect),
                    (None, Some(n)) => format!("{}!{}", quote_sheet(n), r.rect),
                    _ => r.rect.a1(),
                };
                let Some(t) = r.sheet else {
                    edges.push(json!({"from": group_id(si, gi), "to": null, "kind": "reads", "range": range_txt, "broken": true}));
                    continue;
                };
                let targets = targets(a, t, &r.rect, Some((si, gi)));
                if targets.len() > MAX_TARGETS {
                    edges.push(json!({"from": group_id(si, gi), "to": format!("s{}", t + 1), "kind": "reads", "range": range_txt, "via": r.via_name}));
                } else {
                    for to in targets {
                        edges.push(json!({"from": group_id(si, gi), "to": to, "kind": "reads", "range": range_txt, "via": r.via_name}));
                    }
                }
            }
        }
        for ch in &sh.charts {
            let id = format!("s{}.chart{}", si + 1, ch.id);
            nodes.push(json!({"id": id, "type": "chart", "sheet": format!("s{}", si + 1), "kind": ch.kind, "title": ch.title}));
            for src in &ch.sources {
                link_a1(wb, a, &id, src, "plots", &mut edges);
            }
        }
        for p in &sh.pivots {
            let id = format!("s{}.pivot{}", si + 1, crate::a1::cell_name(p.anchor.0, p.anchor.1));
            nodes.push(json!({"id": id, "type": "pivot", "sheet": format!("s{}", si + 1), "rows": p.rows, "cols": p.cols, "values": p.values}));
            link_a1(wb, a, &id, &p.source, "summarizes", &mut edges);
        }
    }
    for e in &a.externals {
        let id = format!("ext:{}:{}", e.func, e.target);
        if !nodes.iter().any(|n| n["id"] == id) {
            nodes.push(json!({"id": id, "type": "external", "func": e.func, "target": e.target}));
        }
        edges.push(json!({"from": group_id(e.sheet, e.group), "to": id, "kind": "imports", "range": e.range}));
    }
    for n in &wb.named_ranges {
        nodes.push(json!({"id": format!("name:{}", n.name), "type": "named_range", "range": n.range}));
    }
    json!({
        "schema": SCHEMA,
        "source": wb.source,
        "nodes": nodes,
        "edges": edges,
    })
}

fn link_a1(wb: &Workbook, a: &Analysis, from: &str, a1: &str, kind: &str, edges: &mut Vec<J>) {
    let Some((sheet, rect)) = crate::a1::parse_ref(a1) else { return };
    let Some(t) = sheet.as_deref().and_then(|s| crate::analyze::sheet_index(wb, s)) else { return };
    let ts = targets(a, t, &rect, None);
    if ts.len() > MAX_TARGETS {
        edges.push(json!({"from": from, "to": format!("s{}", t + 1), "kind": kind, "range": a1}));
    } else {
        for to in ts {
            edges.push(json!({"from": from, "to": to, "kind": kind, "range": a1}));
        }
    }
}

/// Nodes on sheet `t` overlapping `rect`: formula groups, and blocks that hold
/// input values (blocks made purely of formulas are represented by their groups).
fn targets(a: &Analysis, t: usize, rect: &Rect, skip: Option<(usize, usize)>) -> Vec<String> {
    let sa = &a.sheets[t];
    let mut out = Vec::new();
    for (gi, g) in sa.groups.iter().enumerate() {
        if g.rect.intersects(rect) && skip != Some((t, gi)) {
            out.push(group_id(t, gi));
        }
    }
    for (bi, b) in sa.blocks.iter().enumerate() {
        if b.cells > b.formulas && b.rect.intersects(rect) {
            out.push(block_id(t, bi));
        }
    }
    out
}
