//! Local files: xlsx/xlsm/xlsb/xls/ods via calamine (values, formulas, defined
//! names, merges, VBA), csv/tsv via the csv crate. For xlsx/xlsm, [`crate::ooxml`]
//! adds formatting, validation, comments, charts, pivots and external links.
//!
//! A workbook keeps every cell up to [`CELL_BUDGET`]. Past it, a sheet keeps its
//! last rows in full and folds the rows before them into per-column totals
//! ([`Elided`]), so memory stays bounded however big the file is.

use crate::model::*;
use anyhow::{Context, Result};
use calamine::{open_workbook_auto, CellType, Data, Range, Reader, SheetVisible, Sheets, XlsxError};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Cells a workbook keeps in full: about 2–3 GB of memory.
pub const CELL_BUDGET: u64 = 10_000_000;
/// Cells every sheet keeps however much of the budget earlier sheets used.
const SHEET_FLOOR: u64 = 20_000;
/// Rows kept in full at the end of a summarized sheet (totals rows live there).
const TAIL_ROWS: usize = 1_000;
/// Formula cells kept inside summarized rows; past this they are only counted.
const GAP_FORMULAS: u64 = 1_000_000;
/// Distinct texts tallied per summarized column; past this, cells are still counted.
const GAP_TEXTS: usize = 10_000;

pub fn read(path: &Path) -> Result<Workbook> {
    read_limited(path, CELL_BUDGET)
}

/// [`read`] with a different cell budget.
pub fn read_limited(path: &Path, budget: u64) -> Result<Workbook> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("workbook").to_string();
    let mut wb = Workbook {
        source: Source { kind: ext.clone(), id: path.display().to_string(), title, ..Default::default() },
        ..Default::default()
    };
    let mut left = budget;
    if ext == "csv" || ext == "tsv" {
        let mut keep = Keep::new(&mut left);
        let (grid_rows, grid_cols) = read_csv(path, if ext == "tsv" { b'\t' } else { b',' }, &mut keep)?;
        let (cells, elided) = keep.finish();
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Sheet1").to_string();
        note_elided(&mut wb, &name, elided.as_ref(), budget);
        wb.sheets.push(Sheet { name, kind: "GRID".into(), grid_rows, grid_cols, cells, elided, ..Default::default() });
        return Ok(wb);
    }

    let mut book = open_workbook_auto(path).with_context(|| format!("opening {}", path.display()))?;
    let xml_bytes = sheet_xml_bytes(path);
    let metas = book.sheets_metadata().to_vec();
    for (index, meta) in metas.iter().enumerate() {
        let name = meta.name.clone();
        let mut keep = Keep::new(&mut left);
        if let Err(e) = read_cells(&mut book, &name, xml_bytes, &mut keep) {
            wb.warnings.push(format!("{name}: cells after the last one read are missing: {e:#}"));
        }
        let (cells, elided) = keep.finish();
        note_elided(&mut wb, &name, elided.as_ref(), budget);
        let mut sh = Sheet {
            name: name.clone(),
            index,
            hidden: meta.visible != SheetVisible::Visible,
            kind: "GRID".into(),
            cells,
            elided,
            ..Default::default()
        };
        if let Sheets::Xlsx(x) = &mut book
            && let Ok(ms) = x.merge_cells_by_sheet_name(&name) {
                sh.merges = ms.iter().map(|d| crate::a1::Rect::new(d.start.0, d.start.1, d.end.0, d.end.1)).collect();
            }
        wb.sheets.push(sh);
    }
    // _FilterDatabase is the hidden name Excel keeps for every autofilter.
    for (name, formula) in book.defined_names().iter().filter(|(n, _)| !n.ends_with("_FilterDatabase")) {
        wb.named_ranges.push(NamedRange { name: name.clone(), range: formula.trim_start_matches('=').to_string() });
    }
    if let Ok(Some(vba)) = book.vba_project() {
        for m in vba.get_module_names() {
            if let Ok(src) = vba.get_module(m) {
                wb.scripts.push(Script { kind: "vba".into(), name: m.to_string(), source: src });
            }
        }
    }
    if matches!(ext.as_str(), "xlsx" | "xlsm" | "xltx" | "xltm") {
        if let Err(e) = crate::ooxml::enrich(path, &mut wb) {
            wb.warnings.push(format!("{ext}: formatting, validation, comments, charts and pivots could not be read: {e:#}"));
        }
    } else {
        wb.warnings.push(format!(
            "{ext}: cell formatting, data validation, conditional formats, comments, charts and pivots are only extracted from xlsx/xlsm"
        ));
    }
    canonicalize(&mut wb);
    for sh in &mut wb.sheets {
        (sh.grid_rows, sh.grid_cols) = sh.cells.iter().fold((0, 0), |(mr, mc), c| (mr.max(c.r + 1), mc.max(c.c + 1)));
    }
    Ok(wb)
}

fn note_elided(wb: &mut Workbook, sheet: &str, e: Option<&Elided>, budget: u64) {
    if let Some(e) = e {
        wb.warnings.push(format!(
            "{sheet}: rows {}–{} ({} cells) are summarized, not kept, because the workbook has more than {budget} cells; \
             the report's column profiles still count every row",
            e.rows.0 + 1,
            e.rows.1 + 1,
            e.cells
        ));
    }
}

/// Unzipped size of all worksheet parts of an xlsx (0 for other formats).
fn sheet_xml_bytes(path: &Path) -> u64 {
    let Ok(mut zip) = File::open(path).map_err(anyhow::Error::from).and_then(|f| Ok(zip::ZipArchive::new(BufReader::new(f))?)) else {
        return 0;
    };
    (0..zip.len())
        .filter_map(|i| zip.by_index_raw(i).ok().filter(|e| e.name().is_ok_and(|n| n.starts_with("xl/worksheets/"))).map(|e| e.size()))
        .sum()
}

/// One sheet's values and formulas, in row order, into `keep`. xlsx is read
/// cell by cell; the other formats only come as whole ranges, which merge in
/// row order.
fn read_cells(book: &mut Sheets<BufReader<File>>, name: &str, xml_bytes: u64, keep: &mut Keep) -> Result<()> {
    let eq = |f: &str| if f.starts_with('=') { f.to_string() } else { format!("={f}") };
    if let Sheets::Xlsx(x) = book {
        let mut rd = match x.worksheet_cells_reader(name) {
            Ok(rd) => rd,
            Err(XlsxError::NotAWorksheet(_)) => return Ok(()),
            Err(e) => return Err(e.into()),
        };
        // Some writers declare every sheet as just `A1`; then bound the cell
        // count by the XML instead (a cell takes at least ~15 bytes of it).
        let d = rd.dimensions().len();
        keep.hint = Some(if d > 1 { d } else { xml_bytes / 15 });
        while let Some(c) = rd.next_cell_with_formula()? {
            let f = c.formula.as_deref().filter(|f| !f.is_empty()).map(eq);
            let (v, t) = convert(c.value.into());
            if !v.is_empty() || f.is_some() {
                keep.push(Cell { r: c.pos.0, c: c.pos.1, v, t, f, ..Default::default() });
            }
        }
        return Ok(());
    }

    fn used<T: CellType>(range: &Range<T>) -> impl Iterator<Item = ((u32, u32), &T)> {
        let (r0, c0) = range.start().unwrap_or((0, 0));
        range.used_cells().map(move |(r, c, x)| ((r0 + r as u32, c0 + c as u32), x))
    }
    let formulas = book.worksheet_formula(name).ok();
    let mut values = book.worksheet_range(name).ok();
    let fs = || formulas.iter().flat_map(used).filter(|(_, f)| !f.is_empty());
    keep.hint = Some(values.iter().flat_map(used).count() as u64 + fs().count() as u64);
    let mut fs = fs().peekable();
    let formula_only = |((r, c), f): ((u32, u32), &String)| Cell { r, c, f: Some(eq(f)), ..Default::default() };
    if let Some(range) = &mut values {
        let (r0, c0) = range.start().unwrap_or((0, 0));
        for i in 0..range.height() {
            for (j, d) in range[i].iter_mut().enumerate().filter(|(_, d)| **d != Data::Empty) {
                let (r, c) = (r0 + i as u32, c0 + j as u32);
                while let Some(x) = fs.next_if(|(p, _)| *p < (r, c)) {
                    keep.push(formula_only(x));
                }
                let f = fs.next_if(|(p, _)| *p == (r, c)).map(|(_, f)| eq(f));
                let (v, t) = convert(std::mem::take(d));
                if v.is_empty() && f.is_none() {
                    continue;
                }
                keep.push(Cell { r, c, v, t, f, ..Default::default() });
            }
        }
    }
    for x in fs {
        keep.push(formula_only(x));
    }
    Ok(())
}

/// Decides, cell by cell in row order, what a sheet keeps: everything while the
/// workbook's budget lasts (and at least [`SHEET_FLOOR`] cells), then only the
/// newest [`TAIL_ROWS`] rows, folding each older row into column totals.
struct Keep<'a> {
    left: &'a mut u64,
    kept: u64,
    cells: Vec<Cell>,
    tail: VecDeque<Cell>,
    tail_rows: usize,
    fold: Option<Fold>,
    /// The sheet's likely cell count (an upper bound), used to size `cells`
    /// once [`SIZE_AFTER_ROWS`] rows show how wide its rows are.
    hint: Option<u64>,
    rows: u32,
    row_start: usize,
    width: u64,
}

/// Rows read before `cells` is sized from [`Keep::hint`].
const SIZE_AFTER_ROWS: u32 = 100;

impl<'a> Keep<'a> {
    fn new(left: &'a mut u64) -> Self {
        Keep {
            left,
            kept: 0,
            cells: Vec::new(),
            tail: VecDeque::new(),
            tail_rows: 0,
            fold: None,
            hint: None,
            rows: 0,
            row_start: 0,
            width: 0,
        }
    }

    /// Size `cells` in one step, so a big sheet isn't grown by doubling: room
    /// for what the budget lets it keep, plus its tail rows if it will be
    /// summarized.
    fn size_up(&mut self) {
        let Some(hint) = self.hint.take() else { return };
        let head = hint.min((self.kept + *self.left).max(SHEET_FLOOR));
        let want = if head < hint { (head + self.width * TAIL_ROWS as u64).min(hint) } else { hint };
        self.cells.reserve_exact((want as usize).saturating_sub(self.cells.len()));
    }

    fn push(&mut self, c: Cell) {
        if self.fold.is_none() {
            match self.cells.last_mut() {
                // A position given twice: the later one wins, as in a calamine range.
                Some(last) if (last.r, last.c) == (c.r, c.c) => {
                    *last = c;
                    return;
                }
                Some(last) if last.r != c.r && self.kept >= SHEET_FLOOR && *self.left == 0 => {
                    self.fold = Some(Fold { rows: (c.r, c.r), ..Default::default() });
                }
                last => {
                    if last.is_some_and(|l| l.r != c.r) {
                        self.width = self.width.max((self.cells.len() - self.row_start) as u64);
                        self.row_start = self.cells.len();
                        self.rows += 1;
                        if self.rows == SIZE_AFTER_ROWS {
                            self.size_up();
                        }
                    }
                    self.kept += 1;
                    *self.left = self.left.saturating_sub(1);
                    self.cells.push(c);
                    return;
                }
            }
        }
        if self.tail.back().is_none_or(|l| l.r != c.r) {
            if self.tail_rows == TAIL_ROWS {
                let fold = self.fold.as_mut().expect("summarizing");
                let r = self.tail.front().map_or(c.r, |x| x.r);
                fold.rows.1 = r;
                while self.tail.front().is_some_and(|x| x.r == r) {
                    let x = self.tail.pop_front().expect("front cell");
                    fold.add(x, &mut self.cells);
                }
            } else {
                self.tail_rows += 1;
            }
        }
        self.tail.push_back(c);
    }

    fn finish(mut self) -> (Vec<Cell>, Option<Elided>) {
        self.cells.reserve_exact(self.tail.len());
        self.cells.extend(self.tail);
        // A sheet's declared size can be far bigger than what it holds.
        if self.cells.capacity() > self.cells.len() + self.cells.len() / 8 + 1024 {
            self.cells.shrink_to_fit();
        }
        let elided = self.fold.filter(|f| f.cells > 0).map(Fold::finish);
        (self.cells, elided)
    }
}

#[derive(Default)]
struct Fold {
    rows: (u32, u32),
    cells: u64,
    formulas: u64,
    kept_formulas: u64,
    cols: BTreeMap<u32, (ColStats, HashMap<String, u64>)>,
}

impl Fold {
    /// Total one cell of a summarized row; formulas are kept while the cap allows.
    fn add(&mut self, c: Cell, kept: &mut Vec<Cell>) {
        if c.f.is_some() && self.kept_formulas < GAP_FORMULAS {
            self.kept_formulas += 1;
            kept.push(c);
            return;
        }
        self.cells += 1;
        let (st, texts) = self.cols.entry(c.c).or_default();
        st.filled += 1;
        if c.f.is_some() {
            self.formulas += 1;
            st.formulas += 1;
        }
        match &c.v {
            // Same split as the report's column profiles.
            Value::Num(n) if c.t.as_deref().is_some_and(crate::report::looks_date) => Span::add(&mut st.dates, *n, || c.display()),
            Value::Num(n) => Span::add(&mut st.nums, *n, || c.display()),
            Value::Str(s) => {
                st.text_cells += 1;
                if let Some(k) = texts.get_mut(s) {
                    *k += 1;
                } else if texts.len() < GAP_TEXTS {
                    texts.insert(s.clone(), 1);
                } else {
                    st.more_texts = true;
                }
            }
            Value::Bool(_) => st.bools += 1,
            Value::Err { .. } => st.errors += 1,
            Value::Empty => {}
        }
    }

    fn finish(self) -> Elided {
        let cols = self
            .cols
            .into_iter()
            .map(|(col, (mut st, texts))| {
                st.col = col;
                st.texts = texts.into_iter().collect();
                st.texts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                st
            })
            .collect();
        Elided { rows: self.rows, cells: self.cells, formulas: self.formulas, cols }
    }
}

fn convert(d: Data) -> (Value, Option<String>) {
    match d {
        Data::Empty => (Value::Empty, None),
        Data::Int(i) => (Value::Num(i as f64), None),
        Data::Float(f) => (Value::Num(f), None),
        Data::String(s) => (Value::Str(s), None),
        Data::Bool(b) => (Value::Bool(b), None),
        Data::DateTime(dt) => (Value::Num(dt.as_f64()), Some(serial_to_iso(dt.as_f64()))),
        Data::DateTimeIso(s) | Data::DurationIso(s) => (Value::Str(s), None),
        Data::Error(e) => (Value::Err { err: e.to_string() }, None),
    }
}

/// Cells into `keep`; returns the grid size (rows read, widest row).
fn read_csv(path: &Path, delim: u8, keep: &mut Keep) -> Result<(u32, u32)> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).flexible(true).delimiter(delim).from_path(path)?;
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
            keep.push(Cell { r: r as u32, c: c as u32, v, ..Default::default() });
        }
    }
    Ok((rows, cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(r: u32, c: u32) -> Cell {
        Cell { r, c, v: Value::Num((r * 10 + c) as f64), ..Default::default() }
    }

    #[test]
    fn keeps_head_and_tail_and_totals_the_rest() {
        // 40k rows × 2 columns against a 30k-cell budget: the head fills the
        // budget, the last TAIL_ROWS rows are kept, the rows between are totalled,
        // and a formula among them is kept.
        let mut left = 30_000;
        let mut keep = Keep::new(&mut left);
        for r in 0..40_000 {
            keep.push(num(r, 0));
            let mut b = num(r, 1);
            if r == 20_000 {
                b.f = Some("=A20001*2".into());
            }
            keep.push(b);
        }
        let (cells, elided) = keep.finish();
        let e = elided.expect("rows summarized");
        assert_eq!(e.rows, (15_000, 40_000 - TAIL_ROWS as u32 - 1));
        assert_eq!(e.cells, 2 * (40_000 - 15_000 - TAIL_ROWS as u64) - 1);
        assert_eq!(cells.len() as u64 + e.cells, 80_000);
        assert!(cells.iter().any(|c| c.r == 20_000 && c.f.is_some()));
        assert!(cells.windows(2).all(|w| (w[0].r, w[0].c) < (w[1].r, w[1].c)));
        let a = &e.cols[0];
        assert_eq!((a.col, a.filled), (0, 40_000 - 15_000 - TAIL_ROWS as u64));
        let n = a.nums.as_ref().unwrap();
        assert_eq!((n.lo, n.hi), (150_000.0, 389_990.0));
    }

    #[test]
    fn small_sheets_keep_everything() {
        let mut left = CELL_BUDGET;
        let mut keep = Keep::new(&mut left);
        for r in 0..1_000 {
            keep.push(num(r, 0));
        }
        let (cells, elided) = keep.finish();
        assert_eq!((cells.len(), elided), (1_000, None));
    }
}
