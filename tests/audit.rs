//! Coverage checklist on `tests/fixtures/audit.xlsx` (built by
//! `tests/fixtures/make_audit.py`): file-level settings, tabs, cells and the
//! dependency graph, each item checked in the model, the report or graph.json.

use sheetgraph::a1::parse_rect;
use sheetgraph::analyze::{analyze, find_cell, sheet_index};
use sheetgraph::model::{Cell, Sheet, Value, Workbook};

fn load() -> Workbook {
    sheetgraph::load(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/audit.xlsx"), None, false).expect("fixture loads")
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
    let calc = |a1: &str| {
        let r = parse_rect(a1).unwrap();
        let gi = a.sheets[2].groups.iter().position(|g| g.rect.contains(r.r0, r.c0)).expect(a1);
        sheetgraph::graph::group_id(2, gi)
    };
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
    ] {
        assert!(r.contains(want), "missing {want:?} in\n{r}");
    }
    assert!(!r.contains("hunter2"));
    assert!(!r.contains("not extracted: images"));
}
