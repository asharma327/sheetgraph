//! VBA macros: read from `tests/fixtures/macros.xlsm` (built by
//! `tests/fixtures/make_macros.py`), then what each procedure touches and
//! calls, in the report, graph.json and the `macro` / `find` commands.

use sheetgraph::a1::parse_rect;
use sheetgraph::analyze::analyze;
use sheetgraph::macros::{describe, formula_cells, At, Proc, Touch};
use sheetgraph::model::{NamedRange, Script, Sheet, Workbook};
use std::path::PathBuf;
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macros.xlsm");

fn load() -> Workbook {
    sheetgraph::load(FIXTURE, None, false).expect("fixture loads")
}

fn proc<'a>(procs: &'a [Proc], name: &str) -> &'a Proc {
    procs.iter().find(|p| p.name == name).unwrap_or_else(|| panic!("no procedure {name}"))
}

fn touch(sheet: usize, a1: Option<&str>, write: bool) -> Touch {
    Touch { at: At::Sheet(sheet), rect: a1.map(|r| parse_rect(r).unwrap()), write }
}

#[test]
fn modules_and_code_names() {
    let wb = load();
    let names: Vec<(&str, bool)> = wb.scripts.iter().map(|s| (s.name.as_str(), s.has_code())).collect();
    assert_eq!(names, [("Module1", true), ("Sheet1", false), ("Sheet2", false), ("ThisWorkbook", true)]);
    assert!(wb.scripts[0].source.contains("Public Sub RollUp()"));
    let code: Vec<_> = wb.sheets.iter().map(|s| s.code_name.as_deref()).collect();
    assert_eq!(code, [Some("Sheet1"), Some("Sheet2")]);
}

#[test]
fn what_each_procedure_touches() {
    let wb = load();
    let a = analyze(&wb);
    let (data, summary) = (0, 1);

    let roll = proc(&a.macros, "RollUp");
    assert_eq!((roll.kind, roll.lines), ("sub", (5, 14)));
    // With Worksheets("Summary") … .Range("A1:B1").Value = …, .Range("B2:B5").ClearContents,
    // and src (Set to Worksheets("Data")) … .Font.Bold = True.
    assert_eq!(
        roll.touches,
        [touch(data, Some("A1:F1"), true), touch(summary, Some("A1:B1"), true), touch(summary, Some("B2:B5"), true)]
    );
    assert_eq!(roll.calls, [a.macros.iter().position(|p| p.name == "Totals").unwrap()]);

    // Totals(ws, dest) is called with src and the code name Sheet2. The text
    // "Summary" in a comment doesn't count; Cells(i, 5) is somewhere on Data.
    let totals = proc(&a.macros, "Totals");
    assert_eq!(totals.touches, [touch(data, None, false), touch(data, Some("E10"), false), touch(summary, Some("B2"), true)]);
    assert_eq!(describe(&wb, &a, totals, 6).unwrap(), "writes Summary!B2 · uses Data!E10,…");

    let margin = proc(&a.macros, "Margin");
    assert_eq!((margin.kind, formula_cells(&a, margin)), ("function", 8));
    assert!(margin.touches.is_empty());

    let open = proc(&a.macros, "Workbook_Open");
    let t = open.trigger.as_ref().expect("an event procedure");
    assert_eq!((t.event.as_str(), t.sheet), ("Open", None));
    assert_eq!(describe(&wb, &a, open, 6).unwrap(), "runs on workbook Open · calls RollUp");
}

#[test]
fn report_and_graph() {
    let wb = load();
    let a = analyze(&wb);
    let report = sheetgraph::report::render(&wb, &a, &sheetgraph::report::Opts::default());
    assert!(report.contains("vba Module1 (27 lines): RollUp, Totals, Margin\n"), "{report}");
    assert!(report.contains("  RollUp: writes Data!A1:F1; Summary!A1:B1,B2:B5 · calls Totals\n"), "{report}");
    assert!(report.contains("  Margin: used in 8 formula cells\n"), "{report}");
    assert!(report.contains("2 modules with no code: Sheet1, Sheet2\n"), "{report}");
    assert!(!report.contains("vba Sheet1"), "{report}");

    let g = sheetgraph::graph::build(&wb, &a);
    let node = g["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "macro:ThisWorkbook.Workbook_Open").expect("macro node");
    assert_eq!((node["type"].as_str(), node["trigger"].as_str(), node["lines"].as_str()), (Some("macro"), Some("Open"), Some("11-13")));
    let edges: Vec<(String, String, String)> = g["edges"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["kind"] != "reads")
        .map(|e| (e["from"].as_str().unwrap().into(), e["to"].as_str().unwrap_or("-").into(), e["kind"].as_str().unwrap().into()))
        .collect();
    let has = |from: &str, to: &str, kind: &str| edges.iter().any(|e| e == &(from.into(), to.into(), kind.into()));
    assert!(has("macro:Module1.RollUp", "s2.b1", "writes"), "{edges:?}");
    assert!(has("macro:Module1.RollUp", "macro:Module1.Totals", "calls"), "{edges:?}");
    assert!(has("macro:ThisWorkbook.Workbook_Open", "macro:Module1.RollUp", "calls"), "{edges:?}");
    // Data!F2:F9 is =Margin(D2,3)
    assert!(has("s1.f2", "macro:Module1.Margin", "calls"), "{edges:?}");
}

#[test]
fn macro_and_find_commands() {
    let out: PathBuf = std::env::temp_dir().join(format!("sheetgraph-macros-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    let run = |args: &[&str]| {
        let o = Command::new(env!("CARGO_BIN_EXE_sheetgraph")).args(args).output().unwrap();
        assert!(o.status.success(), "{args:?}: {}", String::from_utf8_lossy(&o.stderr));
        String::from_utf8(o.stdout).unwrap()
    };
    run(&["extract", FIXTURE, "--out", out.to_str().unwrap()]);
    let dir = out.join("macros");
    let dir = dir.to_str().unwrap();

    let list = run(&["macro", dir]);
    assert!(list.contains("  sub Totals lines 16-23 · writes Summary!B2 · uses Data!E10,…\n"), "{list}");
    assert!(list.ends_with("no code: Sheet1, Sheet2\n"), "{list}");

    let code = run(&["macro", dir, "module1.totals"]);
    assert!(code.starts_with("sub Module1.Totals lines 16-23\n"), "{code}");
    assert!(code.contains("   16  Private Sub Totals(ws As Worksheet, dest As Worksheet)\n"), "{code}");
    assert!(code.trim_end().ends_with("23  End Sub"), "{code}");
    assert_eq!(run(&["macro", dir, "ThisWorkbook"]).lines().count(), 14, "header and 13 lines");

    let found = run(&["find", dir, "clearcontents"]);
    assert_eq!(found, "vba Module1:10 (RollUp): .Range(\"B2:B5\").ClearContents\n");
    let _ = std::fs::remove_dir_all(&out);
}

/// Patterns from real workbooks, on a hand-made workbook: string constants as
/// sheet names, a helper `As Worksheet` that creates a sheet, a sheet renamed
/// through its variable, `With` + `.Range(.Cells(…), .Cells(…))`, a single-line
/// `If … Then … Else …`, named ranges, and a sheet's own module.
#[test]
fn resolution_patterns() {
    let sheet = |name: &str, code: &str| Sheet { name: name.into(), code_name: Some(code.into()), ..Default::default() };
    let vba = |name: &str, source: &str| Script { kind: "vba".into(), name: name.into(), source: source.into() };
    let wb = Workbook {
        sheets: vec![sheet("Raw Data", "Sheet1"), sheet("Calc", "shCalc")],
        named_ranges: vec![NamedRange { name: "TaxRate".into(), range: "Calc!$B$2".into() }],
        scripts: vec![
            vba(
                "Module1",
                r#"Private Const RAW As String = "Raw Data"
Private Const OUT_SHEET As String = "Output"

Sub Build()
    Dim src As Worksheet, dst As Worksheet
    Set src = ThisWorkbook.Sheets(RAW)
    Set dst = FreshSheet(OUT_SHEET)
    With shCalc
        .Range(.Cells(2, 1), .Cells(9, "C")).Copy
        If src.Range("A1").Value = "" Then .Range("D1").Value = 0 Else .Range("D1").Value = 1
    End With
    dst.Range("A1").PasteSpecial
    Range("TaxRate").Value = 0.2
End Sub

Function FreshSheet(nm As String) As Worksheet
    Dim ws As Worksheet
    Set ws = ThisWorkbook.Worksheets.Add
    ws.Name = nm
    Set FreshSheet = ws
End Function
"#,
            ),
            vba("Sheet1", "Private Sub Worksheet_Change(ByVal Target As Range)\n    Range(\"A1\").Interior.Color = vbYellow\nEnd Sub\n"),
        ],
        ..Default::default()
    };
    let a = analyze(&wb);
    let (raw, calc) = (0, 1);
    let r = |s: &str| Some(parse_rect(s).unwrap());

    let build = proc(&a.macros, "Build");
    assert_eq!(
        build.touches,
        [
            Touch { at: At::Sheet(raw), rect: r("A1"), write: false },
            Touch { at: At::Sheet(calc), rect: r("D1"), write: true },
            Touch { at: At::Sheet(calc), rect: r("A2:C9"), write: false },
            Touch { at: At::Sheet(calc), rect: r("B2"), write: true },
            Touch { at: At::Missing("Output".into()), rect: r("A1"), write: true },
        ]
    );
    assert_eq!(
        describe(&wb, &a, build, 6).unwrap(),
        "writes Calc!D1,B2; Output!A1 (not in file) · uses 'Raw Data'!A1; Calc!A2:C9 · calls FreshSheet"
    );

    // Unqualified Range in a sheet's own module is that sheet.
    let change = proc(&a.macros, "Worksheet_Change");
    assert_eq!(change.trigger.as_ref().map(|t| (t.event.as_str(), t.sheet)), Some(("Change", Some(raw))));
    assert_eq!(change.touches, [Touch { at: At::Sheet(raw), rect: r("A1"), write: true }]);
}
