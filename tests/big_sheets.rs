//! A sheet over the cell budget keeps its first and last rows and totals the
//! rows between; the report's column profiles must still count every row.

use sheetgraph::{analyze, query, report, xlsx};

#[test]
fn summarized_rows_still_count_in_profiles() {
    // 50,000 data rows × 2 columns against a 20,000-cell budget.
    let path = std::env::temp_dir().join(format!("sheetgraph-big-{}.csv", std::process::id()));
    let mut csv = String::from("status,amount\n");
    for i in 1..=50_000 {
        csv.push_str(&format!("{},{i}\n", ["open", "closed", "pending"][i % 3]));
    }
    std::fs::write(&path, csv).unwrap();
    let wb = xlsx::read_limited(&path, 20_000).unwrap();
    std::fs::remove_file(&path).ok();

    let sh = &wb.sheets[0];
    let e = sh.elided.as_ref().expect("rows summarized");
    assert_eq!(e.rows, (10_000, 49_000));
    assert_eq!(sh.cells.len() as u64 + e.cells, 100_002);

    let a = analyze::analyze(&wb);
    let rep = report::render(&wb, &a, &report::Opts::default());
    assert!(rep.contains("rows 10001–49001 summarized"), "{rep}");
    assert!(rep.contains(r#"A "status": enum closed(16667) pending(16667) open(16666) (50000 filled)"#), "{rep}");
    assert!(rep.contains(r#"B "amount": num 1 … 50000 (50000 filled)"#), "{rep}");
    assert!(rep.contains("table 50001×2 (100002 cells)"), "{rep}");

    let name = sheetgraph::a1::quote_sheet(&sh.name);
    let found = query::find(&wb, &a, "pending", 5);
    assert!(found.contains("more text matches in column A (13000)"), "{found}");
    let cells = query::cells(&wb, &a, &format!("{name}!A20000:B20001"), 10).unwrap();
    assert!(cells.contains("rows 10001–49001 are summarized"), "{cells}");
}
