//! End-to-end check of the xlsx reader on `tests/fixtures/features.xlsx`
//! (built by `tests/fixtures/make_features.py`).

use sheetgraph::a1::parse_rect;
use sheetgraph::analyze::{analyze, find_cell, sheet_index};
use sheetgraph::model::{Sheet, Workbook};

fn load() -> Workbook {
    sheetgraph::load(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/features.xlsx"), None, false).expect("fixture loads")
}

fn sheet<'a>(wb: &'a Workbook, name: &str) -> &'a Sheet {
    &wb.sheets[sheet_index(wb, name).expect(name)]
}

fn at<'a>(sh: &'a Sheet, a1: &str) -> &'a sheetgraph::model::Cell {
    let r = parse_rect(a1).unwrap();
    find_cell(sh, r.r0, r.c0).unwrap_or_else(|| panic!("no cell {a1}"))
}

#[test]
fn styles_and_number_formats() {
    let wb = load();
    let data = sheet(&wb, "Data");
    let header = &wb.styles[at(data, "A1").s.expect("header styled") as usize];
    assert!(header.bold && header.borders && header.wrap);
    assert_eq!(header.fg.as_deref(), Some("#FFFFFF"));
    assert_eq!(header.bg.as_deref(), Some("#376092"), "theme accent1, darker 25%");
    assert_eq!(header.h_align.as_deref(), Some("CENTER"));
    let price = &wb.styles[at(data, "E2").s.unwrap() as usize];
    assert_eq!((price.num_fmt.as_deref(), price.num_type.as_deref()), (Some("$#,##0.00"), Some("CURRENCY")));
    let date = &wb.styles[at(data, "A2").s.unwrap() as usize];
    assert_eq!(date.num_type.as_deref(), Some("DATE"));
}

#[test]
fn validations_clamped_to_data() {
    let wb = load();
    let data = sheet(&wb, "Data");
    let dv = |a1: &str| at(data, a1).dv.map(|d| &wb.validations[d as usize]);
    let region = dv("B21").expect("list on B");
    assert_eq!((region.kind.as_str(), region.values.join(",")), ("ONE_OF_LIST", "North,South,East,West".into()));
    assert!(region.dropdown);
    // B2:B1048576 is applied as far as the sheet has rows, not a million cells.
    assert!(data.cells.iter().all(|c| c.r <= 20));
    assert_eq!(dv("C2").unwrap().values, vec!["Lists!A1:A3"]);
    let units = dv("D2").unwrap();
    assert_eq!(units.kind, "WHOLE_NUMBER_BETWEEN");
    assert!(units.strict && units.help.as_deref() == Some("Units: Whole units, 0-1000"));
}

#[test]
fn conditional_formats() {
    let wb = load();
    let rules: Vec<&str> = sheet(&wb, "Data").cond_formats.iter().map(|c| c.rule.as_str()).collect();
    assert!(rules.contains(&"cell lessThan 0 → bg#FFC7CE fg#9C0006"), "{rules:?}");
    assert!(rules.contains(&"color scale min:#F8696B → max:#63BE7B"), "{rules:?}");
    assert!(rules.contains(&"=$D2>150 → bold"), "{rules:?}");
    assert!(rules.contains(&"data bar #638EC6"), "{rules:?}");
}

#[test]
fn sheet_view_notes_links_filters() {
    let wb = load();
    let data = sheet(&wb, "Data");
    assert_eq!((data.frozen_rows, data.frozen_cols), (1, 0));
    assert_eq!(data.hidden_cols, vec![(6, 6)]);
    assert_eq!(at(data, "A1").note.as_deref().map(|n| n.contains("ERP export")), Some(true));
    let summary = sheet(&wb, "Summary");
    assert_eq!(summary.tab_color.as_deref(), Some("#FF0000"));
    assert_eq!(summary.hidden_rows, vec![(7, 7)]);
    assert_eq!(at(summary, "A10").link.as_deref(), Some("https://example.com/docs"));
    assert_eq!(at(summary, "A11").link.as_deref(), Some("#Data!A1"));
    let log = sheet(&wb, "Log");
    assert_eq!(log.filters[0].criteria, vec!["A: show [open, blocked]"]);
    assert_eq!(sheet(&wb, "Lists").protected.len(), 1);
    assert!(!wb.named_ranges.iter().any(|n| n.name.contains("_FilterDatabase")));
}

#[test]
fn tables_charts_pivots() {
    let wb = load();
    let t = &sheet(&wb, "Data").tables[0];
    assert_eq!((t.name.as_str(), t.range.a1(), t.columns.len()), ("Sales", "A1:F21".to_string(), 6));
    let ch = &sheet(&wb, "Summary").charts[0];
    assert_eq!((ch.kind.as_str(), ch.title.as_str()), ("column", "Revenue by week"));
    assert!(ch.sources.iter().any(|s| s.ends_with("!F2:F21")), "{:?}", ch.sources);
    let p = &sheet(&wb, "Pivot").pivots[0];
    assert_eq!(p.anchor, (2, 0));
    assert_eq!(p.source, "Data!A1:F21");
    assert_eq!((p.rows.clone(), p.values.clone(), p.filters.clone()), (vec!["Region".into()], vec!["SUM(Revenue)".into()], vec!["Product".into()]));
}

#[test]
fn links_to_other_workbooks() {
    let wb = load();
    let link = &wb.external_links[0];
    assert_eq!(link.name, "Rates.xlsx");
    assert_eq!(link.sheets, vec!["Rates", "FX rates"]);
    let summary = sheet(&wb, "Summary");
    assert_eq!(at(summary, "B4").f.as_deref(), Some("='[Rates.xlsx]Rates'!B2*2"));
    assert_eq!(at(summary, "B5").f.as_deref(), Some("='[Rates.xlsx]FX rates'!$C$3+B1"));

    let a = analyze(&wb);
    let ranges: Vec<_> = a.externals.iter().filter(|e| e.func == "workbook link").map(|e| (e.target.as_str(), e.range.as_deref())).collect();
    assert_eq!(ranges, vec![("Rates.xlsx", Some("Rates!B2")), ("Rates.xlsx", Some("'FX rates'!C3"))]);
    assert!(a.broken_refs.is_empty(), "links aren't missing sheets: {:?}", a.broken_refs);
    // Structured ref Sales[Revenue] reads the Data sheet.
    let (data, summary) = (sheet_index(&wb, "Data").unwrap(), sheet_index(&wb, "Summary").unwrap());
    assert!(a.sheet_edges.contains_key(&(summary, data)));
}
