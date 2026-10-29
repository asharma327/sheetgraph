//! Coverage checklist on `tests/fixtures/audit.xlsx` (built by
//! `tests/fixtures/make_audit.py`): file-level settings, tabs, cells and the
//! dependency graph, each item checked in the model, the report or graph.json.

use sheetgraph::a1::parse_rect;
use sheetgraph::analyze::{analyze, find_cell, sheet_index, Analysis};
use sheetgraph::model::{Cell, Sheet, Value, Workbook};

fn load() -> Workbook {
    load_fixture("audit.xlsx")
}

fn load_fixture(name: &str) -> Workbook {
    sheetgraph::load(&format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR")), None, false).expect("fixture loads")
}

fn edges(g: &serde_json::Value) -> Vec<(String, String, String, String)> {
    g["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            let s = |k: &str| e[k].as_str().unwrap_or("-").to_string();
            (s("from"), s("to"), s("kind"), s("range"))
        })
        .collect()
}

/// Group id of the formula at `a1` on Calc (sheet 3).
fn calc_group(a: &Analysis, a1: &str) -> String {
    let r = parse_rect(a1).unwrap();
    let gi = a.sheets[2].groups.iter().position(|g| g.rect.contains(r.r0, r.c0)).expect(a1);
    sheetgraph::graph::group_id(2, gi)
}

fn sheet<'a>(wb: &'a Workbook, name: &str) -> &'a Sheet {
    &wb.sheets[sheet_index(wb, name).expect(name)]
}

fn at<'a>(sh: &'a Sheet, a1: &str) -> &'a Cell {
    let r = parse_rect(a1).unwrap();
    find_cell(sh, r.r0, r.c0).unwrap_or_else(|| panic!("no cell {a1}"))
}

#[test]
fn file_level() {
    let wb = load();
    let prop = |k: &str| wb.properties.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
    assert_eq!(prop("title"), Some("Audit Workbook"));
    assert_eq!(prop("author"), Some("Audit Author"));
    assert_eq!(prop("description"), Some("Monthly close model"));
    assert_eq!(prop("Department"), Some("Finance"), "custom property");

    let calc = wb.calc.as_ref().expect("non-default calculation");
    assert_eq!((calc.mode.as_str(), calc.iterative, calc.max_iterations), ("manual", true, Some(100)));
    assert!(wb.structure_locked);

    let db = &wb.connections[0];
    assert_eq!((db.name.as_str(), db.kind.as_str(), db.refresh_on_open), ("Prices DB", "odbc", true));
    assert_eq!(db.source.as_deref(), Some("DRIVER={SQL Server};SERVER=sql01;DATABASE=Prices;UID=report"), "password removed");
    assert_eq!(db.command.as_deref(), Some("SELECT * FROM dbo.prices"));
    let pq = &wb.connections[1];
    assert_eq!((pq.kind.as_str(), pq.source.as_deref()), ("power query", Some("Prices")));

    let queries: Vec<(&str, &str)> = wb.scripts.iter().map(|s| (s.kind.as_str(), s.name.as_str())).collect();
    assert_eq!(queries, [("power_query", "Prices"), ("power_query", "Top Prices")]);
    assert!(wb.scripts[0].source.starts_with("let\n    Source = Sql.Database(\"sql01\", \"Prices\")"), "{}", wb.scripts[0].source);
    assert!(wb.scripts[1].source.ends_with("Table.FirstN(Sorted, 10)"), "{}", wb.scripts[1].source);
}

#[test]
fn tabs() {
    let wb = load();
    assert!(sheet(&wb, "Lookup").hidden && sheet(&wb, "Secret").hidden);
    assert!(sheet(&wb, "Secret").very_hidden && !sheet(&wb, "Lookup").very_hidden, "only macro code can unhide Secret");
    assert_eq!(sheet(&wb, "ChartSheet").kind, "OBJECT");
    let report = sheet(&wb, "Report");
    assert_eq!((report.frozen_rows, report.frozen_cols, report.tab_color.as_deref()), (1, 1, Some("#00B050")));
    assert_eq!((report.tables[0].name.as_str(), report.charts.len(), report.protected.len()), ("Sales", 1, 1));

    let shapes: Vec<_> = sheet(&wb, "Objects")
        .shapes
        .iter()
        .map(|x| (x.kind.as_str(), x.text.as_deref(), x.alt.as_deref(), x.on_click.as_deref(), x.anchor.map(|r| r.a1())))
        .collect();
    assert_eq!(
        shapes,
        [
            ("picture", None, Some("Company logo"), None, Some("D2:E6".into())),
            ("text box", Some("Rates update on the 1st of each month"), None, None, Some("G2:I4".into())),
            ("button", Some("Run roll-up"), None, Some("RollUp"), Some("G9:H10".into())),
        ]
    );
}

#[test]
fn cells() {
    let wb = load();
    let inp = sheet(&wb, "Inputs");
    // Numbers read as they show.
    for (a1, shown) in [("B2", "$12.50"), ("B3", "25%"), ("B4", "1,234,567"), ("C2", "2026-01-15")] {
        assert_eq!(at(inp, a1).display(), shown, "{a1}");
    }
    assert_eq!(at(inp, "B3").v, Value::Num(0.25), "the stored value stays");

    let style = |a1: &str| &wb.styles[at(inp, a1).s.expect(a1) as usize];
    assert!(style("A1").bold && style("A3").italic && style("A4").underline && style("A5").strike);
    assert_eq!((style("A6").font.as_deref(), style("A6").size, style("A6").fg.as_deref()), (Some("Courier New"), Some(14), Some("#FF0000")));
    assert!(style("L2").unlocked && !style("B2").unlocked);
    assert!(style("L3").hidden_formula);
    assert_eq!(style("J2").cell_style.as_deref(), Some("Input"));
    assert_eq!(style("A1").cell_style, None, "Normal isn't named");
    for (a1, border) in [("A1", "bottom thin"), ("N2", "box thin"), ("N3", "bottom double"), ("N4", "top thin, bottom double")] {
        assert_eq!(style(a1).border.as_deref(), Some(border), "{a1}");
    }

    let rich = &at(inp, "F2").runs;
    assert_eq!(rich.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(), ["Total ", "due"]);
    assert!(rich[1].f.bold && rich[1].f.fg.as_deref() == Some("#FF0000"));

    let photo = at(inp, "P2");
    let obj = photo.obj.as_ref().expect("picture in cell");
    assert_eq!((obj.kind.as_str(), obj.text.as_deref(), obj.target.as_deref()), ("image", Some("Product photo"), Some("xl/media/photo.png")));
    assert_eq!((&photo.v, photo.display().as_str()), (&Value::Empty, "[image: Product photo]"), "not #VALUE!");

    let stock = at(inp, "Q2");
    assert_eq!(stock.v, Value::Str("Microsoft Corp".into()));
    let props = &stock.obj.as_ref().expect("data type").props;
    assert!(props.contains(&("Price".into(), "412.5".into())) && props.contains(&("Exchange".into(), "NASDAQ".into())), "{props:?}");

    assert_eq!(at(inp, "G2").note.as_deref(), Some("Check with finance"));
    assert_eq!(at(inp, "H2").link.as_deref(), Some("https://example.com/policy"));
    assert!(at(inp, "I2").dv.is_some() && at(inp, "I3").dv.is_some());
    assert_eq!(inp.merges.iter().map(|r| r.a1()).collect::<Vec<_>>(), ["A8:C8"]);
}

#[test]
fn dependency_graph() {
    let wb = load();
    let a = analyze(&wb);
    let g = sheetgraph::graph::build(&wb, &a);
    let edges: Vec<(String, String, String)> = g["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e["from"].as_str().unwrap().into(), e["to"].as_str().unwrap_or("-").into(), e["range"].as_str().unwrap_or("").into()))
        .collect();
    let calc = |a1: &str| calc_group(&a, a1);
    let reads = |from: &str, range: &str| edges.iter().any(|(f, _, r)| f == from && r == range);

    // =SUM(Inputs:Report!Z1) reads every sheet in the span.
    assert!(reads(&calc("A5"), "Inputs!Z1") && reads(&calc("A5"), "Report!Z1"), "{edges:?}");
    // ='[Other.xlsx]Data'!A1 is an import from another workbook, not a broken ref.
    let b4 = calc("B4");
    assert!(edges.iter().any(|(f, t, _)| *f == b4 && t == "ext:workbook link:Other.xlsx"));
    assert!(!edges.iter().any(|(f, t, _)| *f == b4 && t == "-"), "{edges:?}");
    assert!(a.broken_refs.is_empty(), "{:?}", a.broken_refs);
    // Other references: cross-sheet, named range, table column, hidden sheet.
    assert!(reads(&calc("A1"), "Inputs!B2"));
    assert!(edges.iter().any(|(f, _, r)| *f == calc("A3") && r == "Inputs!B3"), "TaxRate");
    assert!(reads(&calc("A4"), "Report!A1:C6"), "Sales[Units]");
    assert!(reads(&calc("B6"), "Lookup!A1:B3"));
    // The macro button is a node that runs RollUp (no VBA here, so no edge).
    assert!(g["nodes"].as_array().unwrap().iter().any(|n| n["type"] == "shape" && n["text"] == "Run roll-up"));
}

#[test]
fn names() {
    let wb = load();
    let local: Vec<_> = wb.named_ranges.iter().filter(|n| n.name == "LocalRate").map(|n| (n.scope.as_deref(), n.range.as_str())).collect();
    assert_eq!(local, [(None, "Inputs!$B$4"), (Some("Calc"), "Calc!$A$1")]);
    let a = analyze(&wb);
    let calc = sheet_index(&wb, "Calc");
    assert_eq!(a.name_range(calc, "LocalRate").map(|(s, r)| (s, r.a1())), Some((calc, "A1".into())), "Calc's own LocalRate wins on Calc");
    assert_eq!(a.name_range(Some(0), "LocalRate").map(|(s, r)| (s, r.a1())), Some((Some(0), "B4".into())));
    assert!(a.name(None, "AddTax").is_some_and(|d| d.lambda));

    let g = sheetgraph::graph::build(&wb, &a);
    let e = edges(&g);
    let reads = |from: &str, range: &str| e.iter().any(|(f, _, k, r)| f == from && k == "reads" && r == range);
    // =LocalRate on Calc, =AddTax(10) (a LAMBDA), =GrossRate*2 (a formula name over TaxRate).
    assert!(reads(&calc_group(&a, "B3"), "Calc!A1") && !reads(&calc_group(&a, "B3"), "Inputs!B4"), "{e:?}");
    assert!(reads(&calc_group(&a, "B1"), "Inputs!B3"));
    assert!(reads(&calc_group(&a, "B8"), "Inputs!B3"));
    assert!(reads("name:AddTax", "Inputs!B3") && reads("name:GrossRate", "Inputs!B3"));
    assert!(g["nodes"].as_array().unwrap().iter().any(|n| n["id"] == "name:Calc!LocalRate" && n["scope"] == "Calc"));
}

#[test]
fn rules() {
    let wb = load();
    let a = analyze(&wb);
    let e = edges(&sheetgraph::graph::build(&wb, &a));
    let from = |kind: &str, range: &str| e.iter().filter(|(_, _, k, r)| k == kind && r == range).map(|(f, ..)| f.clone()).collect::<Vec<_>>();
    // Dropdown lists: same sheet, another sheet, through a name.
    assert_eq!(from("choices", "Inputs!K2:K4"), ["s1.dv1", "s1.dv3"]);
    assert_eq!(from("choices", "Lookup!A1:A2"), ["s1.dv2"]);
    // A validation formula and a conditional format reading other sheets.
    assert_eq!(from("reads", "Report!B2"), ["s1.dv4"]);
    assert_eq!(from("reads", "Inputs!B2").iter().filter(|f| f.starts_with("s3.cf")).count(), 1);
}

#[test]
fn merged_cells() {
    let wb = load();
    let a = analyze(&wb);
    let g = &a.sheets[2].groups[a.sheets[2].groups.iter().position(|g| g.formula == "=Inputs!B8").unwrap()];
    let c = g.covered_eg.as_ref().expect("reads a cell under a merge");
    assert_eq!((g.covered, c.sheet, c.cell, c.merge.a1()), (1, 0, (7, 1), "A8:C8".into()));
    assert!(a.sheets.iter().flat_map(|s| &s.groups).filter(|g| g.covered > 0).count() == 1, "only =Inputs!B8");

    let cells = sheetgraph::query::cells(&wb, &a, "Inputs!A8:C8", 50).unwrap();
    assert!(cells.contains("A8 = Merged title  [merged A8:C8]"), "{cells}");
    let covered = sheetgraph::query::cells(&wb, &a, "Inputs!B8:C8", 50).unwrap();
    assert!(covered.contains("B8:C8 inside merge A8:C8: shows A8 = Merged title"), "{covered}");
    let t = sheetgraph::query::trace(&wb, &a, "Calc!B7", 1, true, false, None).unwrap();
    assert!(t.contains("Inputs!B8 (blank: inside merge A8:C8, which shows A8 = Merged title)"), "{t}");
}

#[test]
fn linked_workbooks() {
    let out = std::env::temp_dir().join(format!("sheetgraph-audit-links-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    let mut dirs = Vec::new();
    for name in ["audit.xlsx", "Other.xlsx"] {
        let wb = load_fixture(name);
        let a = analyze(&wb);
        let rep = sheetgraph::report::render(&wb, &a, &sheetgraph::report::Opts::default());
        dirs.push(sheetgraph::store::save(&out, &wb, &a, &rep).unwrap().0);
    }
    let open = |i: usize| {
        let wb = sheetgraph::store::load(&dirs[i]).unwrap();
        let a = analyze(&wb);
        (wb, a, sheetgraph::store::Siblings::of(&dirs[i]))
    };
    // audit's Calc!B4 reads '[Other.xlsx]Data'!A1, which reads Data!B1.
    let (wb, a, books) = open(0);
    let up = sheetgraph::query::trace(&wb, &a, "Calc!B4", 3, true, false, Some(&books)).unwrap();
    assert!(up.contains("  ← [Other.xlsx] Data!A1 =   ƒ1 =B1*2\n    ← [Other.xlsx] Data!B1 = 21\n"), "{up}");
    // And from Other's side, its B1 feeds audit's Calc!B4.
    let (wb, a, books) = open(1);
    let down = sheetgraph::query::trace(&wb, &a, "Data!B1", 3, false, true, Some(&books)).unwrap();
    assert!(down.contains("→ [audit.xlsx] Calc!B4 ƒ8 ='[Other.xlsx]Data'!A1"), "{down}");
    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn report() {
    let wb = load();
    let a = analyze(&wb);
    let r = sheetgraph::report::render(&wb, &a, &sheetgraph::report::Opts::default());
    for want in [
        "properties: title: Audit Workbook · author: Audit Author",
        "calculation: manual (stored values may be stale until recalculated) · iterative: circular references allowed, up to 100 iterations or change < 0.001\n",
        "structure locked: sheets can't be added, deleted, renamed or unhidden\n",
        "connection \"Prices DB\" (odbc, refreshes on open): DRIVER={SQL Server};SERVER=sql01;DATABASE=Prices;UID=report · SELECT * FROM dbo.prices\n",
        "power_query Prices (5 lines): Sql.Database(\"sql01\", \"Prices\")\n",
        " B=$12.50 ",
        " P=[image: Product photo] Q=Microsoft Corp\n",
        "unlocked cells (editable when the sheet is protected): L2\n",
        "formulas hidden when protected: L3\n",
        "▣ pictures in cells (1): P2 \"Product photo\"\n",
        "▣ data types (linked records) (1): Q2 \"Microsoft Corp\" {Name=Microsoft Corporation, Price=412.5, Exchange=NASDAQ}\n",
        "F2: Total [due]{bold fg#FF0000}\n",
        "picture Logo at D2:E6 — Company logo\n",
        "text box \"Rates update on the 1st of each month\" at G2:I4\n",
        "button \"Run roll-up\" at G9:H10 → runs RollUp\n",
        "5. Secret [very hidden]",
        "## 5. Secret · A1 · 1 cells · very hidden (only macro code can unhide it)\n",
        "AddTax (LAMBDA function) = _xlfn.LAMBDA(_xlpm.x,_xlpm.x*(1+TaxRate))\n",
        "LocalRate (Calc only) = Calc!$A$1\n",
        "print area of Report = 'Report'!$A$1:$C$6\n",
        "ADDTAX (1, LAMBDA name)",
        "dropdown on Inputs!I4 ← Lookup!A1:A2\n",
        "validation on Inputs!I6 ← Report!B2\n",
        "conditional format on Calc!A1:A8 ← Inputs!B2\n",
        "style \"Input\" bg#FFCC99 fg#3F3F76 border box thin (1 cells): J2\n",
        "border top thin, bottom double (1 cells): N4\n",
        "=Inputs!B8 [reads B8 inside merge A8:C8: blank]\n",
        "merged: 3× column A, 2 rows each, rows 2–7; 4× 3 columns wide over row 1, B:M; O1:P2\n",
    ] {
        assert!(r.contains(want), "missing {want:?} in\n{r}");
    }
    assert!(!r.contains("hunter2"));
    assert!(!r.contains("not extracted: images"));
}
