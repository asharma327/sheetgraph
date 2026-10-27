//! CSV values and ISO dates the way a person reads them: semicolon CSVs split
//! into columns, codes like "02134" and words like "NaN" stay text, and dates
//! written as ISO text (in CSV, or as xlsx `t="d"` cells) are real dates.

use sheetgraph::model::{Cell, Value, Workbook};
use std::io::Write;
use std::path::PathBuf;

fn temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("sheetgraph-{}-{name}", std::process::id()))
}

fn load_csv(name: &str, text: impl AsRef<[u8]>) -> Workbook {
    let path = temp(name);
    std::fs::write(&path, text).unwrap();
    let wb = sheetgraph::load(path.to_str().unwrap(), None, false).expect("csv loads");
    let _ = std::fs::remove_file(&path);
    wb
}

fn at(wb: &Workbook, r: u32, c: u32) -> &Cell {
    wb.sheets[0].cells.iter().find(|x| x.r == r && x.c == c).unwrap_or_else(|| panic!("no cell r{r} c{c}"))
}

fn text(s: &str) -> Value {
    Value::Str(s.into())
}

fn report(wb: &Workbook) -> String {
    let a = sheetgraph::analyze::analyze(wb);
    sheetgraph::report::render(wb, &a, &sheetgraph::report::Opts::default())
}

#[test]
fn semicolon_csv_splits_into_columns() {
    let wb = load_csv("semi.csv", "order_id;customer;total\n97000;Acme Corp;12.50\n97001;\"Globex; Inc\";13.50\n");
    assert_eq!(wb.sheets[0].grid_cols, 3);
    assert_eq!(at(&wb, 1, 1).v, text("Acme Corp"));
    assert_eq!(at(&wb, 2, 1).v, text("Globex; Inc"));
    assert_eq!(at(&wb, 2, 2).v, Value::Num(13.5));
}

#[test]
fn comma_csv_with_semicolons_in_quotes_stays_comma() {
    let wb = load_csv("comma.csv", "\"name; full\",note\n\"Acme; Corp\",\"a;b;c\"\n");
    assert_eq!(wb.sheets[0].grid_cols, 2);
    assert_eq!(at(&wb, 1, 1).v, text("a;b;c"));
}

#[test]
fn codes_and_words_stay_text() {
    let wb = load_csv("values.csv", "zip,name,n\n02134,Nan,0\n00501,NaN,0.5\n90210,inf,-0.25\n0,Infinity,1e3\n");
    assert_eq!(at(&wb, 1, 0).v, text("02134"), "leading zero kept");
    assert_eq!(at(&wb, 2, 0).v, text("00501"));
    assert_eq!(at(&wb, 3, 0).v, Value::Num(90210.0));
    assert_eq!(at(&wb, 4, 0).v, Value::Num(0.0));
    for (r, name) in [(1, "Nan"), (2, "NaN"), (3, "inf"), (4, "Infinity")] {
        assert_eq!(at(&wb, r, 1).v, text(name), "{name} is a name, not a number");
    }
    for (r, n) in [(1, 0.0), (2, 0.5), (3, -0.25), (4, 1000.0)] {
        assert_eq!(at(&wb, r, 2).v, Value::Num(n));
    }
}

#[test]
fn long_ids_keep_every_digit() {
    let wb = load_csv(
        "ids.csv",
        "account,amount\n1234567890123456789,1.10\n1234567890123456790,12.50\n123456789012345,-0.5\n",
    );
    assert_eq!(at(&wb, 1, 0).v, text("1234567890123456789"), "19 digits don't fit a number");
    assert_eq!(at(&wb, 2, 0).v, text("1234567890123456790"));
    assert_eq!(at(&wb, 3, 0).v, Value::Num(123456789012345.0), "15 digits still a number");
    // Still numbers, shown as written.
    assert_eq!((&at(&wb, 1, 1).v, at(&wb, 1, 1).display()), (&Value::Num(1.1), "1.10".to_string()));
    assert_eq!(at(&wb, 2, 1).display(), "12.50");
    assert_eq!(at(&wb, 3, 1).display(), "-0.5");
}

#[test]
fn excel_windows_csv_reads_accents() {
    // Excel's "CSV (Comma delimited)" on Windows writes Windows-1252, not UTF-8.
    let wb = load_csv("cp1252.csv", b"name,city,note\r\nJos\xe9,K\xf6ln,\x80 5 \x96 \x93ok\x94\r\nAna,Lyon,plain\r\n");
    assert_eq!(at(&wb, 1, 0).v, text("José"));
    assert_eq!(at(&wb, 1, 1).v, text("Köln"));
    assert_eq!(at(&wb, 1, 2).v, text("€ 5 – “ok”"));
    assert_eq!(at(&wb, 2, 1).v, text("Lyon"));
}

#[test]
fn utf16_and_bom_csvs_read() {
    // Excel's "Unicode Text": UTF-16 LE with a BOM, tab-separated.
    let mut utf16 = vec![0xFF, 0xFE];
    utf16.extend("name\tcity\r\nJosé\tKöln\r\n".encode_utf16().flat_map(u16::to_le_bytes));
    let wb = load_csv("utf16.csv", &utf16);
    assert_eq!(wb.sheets[0].grid_cols, 2);
    assert_eq!(at(&wb, 1, 0).v, text("José"));
    assert_eq!(at(&wb, 1, 1).v, text("Köln"));
    // Excel's "CSV UTF-8": the BOM isn't part of the first header.
    let wb = load_csv("bom.csv", "\u{feff}order_id,customer\n1,Acme\n");
    assert_eq!(at(&wb, 0, 0).v, text("order_id"));
}

#[test]
fn csv_iso_dates_are_dates() {
    // Enough rows (over Opts::small_block cells) for the report to profile the columns.
    let mut csv = String::from("order_id,order_date\n");
    for day in 1..=28 {
        csv += &format!("{day},2024-06-{day:02}\n");
    }
    csv += "29,2024-06-01T09:30:00\n";
    let wb = load_csv("dates.csv", &csv);
    let d = at(&wb, 1, 1);
    assert!(matches!(d.v, Value::Num(_)), "{:?}", d.v);
    assert_eq!(d.display(), "2024-06-01");
    assert_eq!(at(&wb, 29, 1).display(), "2024-06-01 09:30");
    let rep = report(&wb);
    assert!(rep.contains("date 2024-06-01 … 2024-06-28"), "{rep}");
}

/// A one-sheet xlsx whose dates are `t="d"` ISO text, as SheetJS writes them:
/// order ids in A, 2024-05-31T20:00Z … 2024-06-27T20:00Z in B, and a plain
/// date in the last row.
fn iso_date_xlsx(path: &PathBuf) {
    let mut rows = String::from(r#"<row r="1"><c r="A1" t="inlineStr"><is><t>order_id</t></is></c><c r="B1" t="inlineStr"><is><t>order_date</t></is></c></row>"#);
    let stamps = std::iter::once("2024-05-31T20:00:00.000Z".to_string())
        .chain((1..=27).map(|d| format!("2024-06-{d:02}T20:00:00.000Z")))
        .chain(std::iter::once("2024-06-10".to_string()));
    for (i, stamp) in stamps.enumerate() {
        let r = i + 2;
        rows += &format!(r#"<row r="{r}"><c r="A{r}"><v>{i}</v></c><c r="B{r}" t="d"><v>{stamp}</v></c></row>"#);
    }
    let sheet = format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{rows}</sheetData></worksheet>"#);
    let parts = [
        (
            "[Content_Types].xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
        ),
        (
            "_rels/.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
        ),
        (
            "xl/workbook.xml",
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Orders" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
        ),
        (
            "xl/_rels/workbook.xml.rels",
            r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
        ),
        ("xl/worksheets/sheet1.xml", sheet.as_str()),
    ];
    let mut zip = zip::ZipWriter::new(std::fs::File::create(path).unwrap());
    for (name, xml) in parts {
        zip.start_file(name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(xml.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

#[test]
fn xlsx_iso_text_dates_are_dates() {
    let path = temp("iso-dates.xlsx");
    iso_date_xlsx(&path);
    let wb = sheetgraph::load(path.to_str().unwrap(), None, false).expect("xlsx loads");
    let _ = std::fs::remove_file(&path);
    let d = at(&wb, 1, 1);
    assert!(matches!(d.v, Value::Num(_)), "{:?}", d.v);
    // The clock time as written, zone ignored: what Excel and openpyxl show.
    assert_eq!(d.display(), "2024-05-31 20:00");
    assert_eq!(at(&wb, 29, 1).display(), "2024-06-10");
    let rep = report(&wb);
    assert!(rep.contains("date 2024-05-31 20:00 … 2024-06-27 20:00"), "{rep}");
}
