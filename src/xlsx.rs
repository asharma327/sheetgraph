//! Local files: xlsx/xlsm/xlsb/xls/ods via calamine (values, formulas, defined
//! names, merges, VBA), csv/tsv via the csv crate.

use crate::model::*;
use anyhow::{Context, Result};
use calamine::{open_workbook_auto, Data, Reader, SheetVisible, Sheets};
use std::collections::BTreeMap;
use std::path::Path;

pub fn read(path: &Path) -> Result<Workbook> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("workbook").to_string();
    let mut wb = Workbook {
        source: Source { kind: ext.clone(), id: path.display().to_string(), title, ..Default::default() },
        ..Default::default()
    };
    if ext == "csv" || ext == "tsv" {
        wb.sheets.push(read_csv(path, if ext == "tsv" { b'\t' } else { b',' })?);
        return Ok(wb);
    }

    let mut book = open_workbook_auto(path).with_context(|| format!("opening {}", path.display()))?;
    let metas = book.sheets_metadata().to_vec();
    for (index, meta) in metas.iter().enumerate() {
        let name = meta.name.clone();
        let mut cells: BTreeMap<(u32, u32), Cell> = BTreeMap::new();
        if let Ok(range) = book.worksheet_range(&name) {
            let (r0, c0) = range.start().unwrap_or((0, 0));
            for (r, c, d) in range.used_cells() {
                let (r, c) = (r0 + r as u32, c0 + c as u32);
                let (v, t) = convert(d);
                if v.is_empty() {
                    continue;
                }
                cells.insert((r, c), Cell { r, c, v, t, ..Default::default() });
            }
        }
        if let Ok(fr) = book.worksheet_formula(&name) {
            let (r0, c0) = fr.start().unwrap_or((0, 0));
            for (r, c, f) in fr.used_cells() {
                if f.is_empty() {
                    continue;
                }
                let (r, c) = (r0 + r as u32, c0 + c as u32);
                let cell = cells.entry((r, c)).or_insert_with(|| Cell { r, c, ..Default::default() });
                cell.f = Some(if f.starts_with('=') { f.clone() } else { format!("={f}") });
            }
        }
        let (rows, cols) = cells.keys().fold((0, 0), |(mr, mc), &(r, c)| (mr.max(r + 1), mc.max(c + 1)));
        let mut sh = Sheet {
            name: name.clone(),
            index,
            hidden: meta.visible != SheetVisible::Visible,
            kind: "GRID".into(),
            grid_rows: rows,
            grid_cols: cols,
            cells: cells.into_values().collect(),
            ..Default::default()
        };
        if let Sheets::Xlsx(x) = &mut book {
            if let Ok(ms) = x.merge_cells_by_sheet_name(&name) {
                sh.merges = ms.iter().map(|d| crate::a1::Rect::new(d.start.0, d.start.1, d.end.0, d.end.1)).collect();
            }
        }
        wb.sheets.push(sh);
    }
    for (name, formula) in book.defined_names() {
        wb.named_ranges.push(NamedRange { name: name.clone(), range: formula.trim_start_matches('=').to_string() });
    }
    if let Ok(Some(vba)) = book.vba_project() {
        for m in vba.get_module_names() {
            if let Ok(src) = vba.get_module(m) {
                wb.scripts.push(Script { kind: "vba".into(), name: m.to_string(), source: src });
            }
        }
    }
    wb.warnings.push(format!(
        "{ext}: cell formatting, data validation, conditional formats and charts are not extracted yet for local files"
    ));
    Ok(wb)
}

fn convert(d: &Data) -> (Value, Option<String>) {
    match d {
        Data::Empty => (Value::Empty, None),
        Data::Int(i) => (Value::Num(*i as f64), None),
        Data::Float(f) => (Value::Num(*f), None),
        Data::String(s) => (Value::Str(s.clone()), None),
        Data::Bool(b) => (Value::Bool(*b), None),
        Data::DateTime(dt) => (Value::Num(dt.as_f64()), Some(serial_to_iso(dt.as_f64()))),
        Data::DateTimeIso(s) | Data::DurationIso(s) => (Value::Str(s.clone()), None),
        Data::Error(e) => (Value::Err { err: e.to_string() }, None),
    }
}

fn read_csv(path: &Path, delim: u8) -> Result<Sheet> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).flexible(true).delimiter(delim).from_path(path)?;
    let mut cells = Vec::new();
    let (mut rows, mut cols) = (0u32, 0u32);
    for (r, rec) in rdr.records().enumerate() {
        let rec = rec?;
        rows = r as u32 + 1;
        for (c, field) in rec.iter().enumerate() {
            if field.is_empty() {
                continue;
            }
            cols = cols.max(c as u32 + 1);
            let v = match field.parse::<f64>() {
                Ok(n) if field.trim() == field => Value::Num(n),
                _ => Value::Str(field.to_string()),
            };
            cells.push(Cell { r: r as u32, c: c as u32, v, ..Default::default() });
        }
    }
    Ok(Sheet {
        name: path.file_stem().and_then(|s| s.to_str()).unwrap_or("Sheet1").to_string(),
        kind: "GRID".into(),
        grid_rows: rows,
        grid_cols: cols,
        cells,
        ..Default::default()
    })
}
