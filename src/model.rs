//! Source-agnostic, full-fidelity workbook model. Every backend (Google Sheets,
//! xlsx, csv) converts into this; analysis and rendering only ever see this.

use crate::a1::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Workbook {
    pub source: Source,
    pub sheets: Vec<Sheet>,
    pub named_ranges: Vec<NamedRange>,
    /// Deduplicated cell styles, referenced by `Cell::s`.
    pub styles: Vec<Style>,
    /// Deduplicated data-validation rules, referenced by `Cell::dv`.
    pub validations: Vec<Validation>,
    /// Macros, Apps Script projects, VBA modules.
    pub scripts: Vec<Script>,
    /// Connected Sheets / BigQuery / Looker data sources, kept raw.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_sources: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub developer_metadata: Vec<serde_json::Value>,
    /// Other workbooks this one references (Excel external links). Formulas
    /// name them as `'[file]Sheet'!A1`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_links: Vec<ExternalLink>,
    /// Anything the backend could not extract, so consumers know what is missing.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Source {
    /// "gsheet" | "xlsx" | "xlsm" | "xls" | "ods" | "csv"
    pub kind: String,
    /// Spreadsheet id or file path.
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Sheet {
    pub name: String,
    pub index: usize,
    #[serde(default, skip_serializing_if = "is_false")]
    pub hidden: bool,
    /// GRID, OBJECT (chart sheet), DATA_SOURCE
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,
    /// Allocated grid size (not the used range).
    pub grid_rows: u32,
    pub grid_cols: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub frozen_rows: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub frozen_cols: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tab_color: Option<String>,
    /// Sparse, sorted by (row, col).
    pub cells: Vec<Cell>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub merges: Vec<Rect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cond_formats: Vec<CondFormat>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Filter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protected: Vec<Protected>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charts: Vec<Chart>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pivots: Vec<Pivot>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_rows: Vec<(u32, u32)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_cols: Vec<(u32, u32)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub banded: Vec<Rect>,
    /// Excel tables (ListObjects); formulas reach them as `Name[Column]`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<Table>,
    /// Rows summarized instead of kept, when the workbook was too big to hold
    /// whole (see [`crate::xlsx::CELL_BUDGET`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elided: Option<Elided>,
}

/// A run of rows whose cells are not in [`Sheet::cells`] (formulas excepted, up
/// to a cap), with every column's values totalled instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Elided {
    /// First and last summarized row, 0-based and inclusive.
    pub rows: (u32, u32),
    /// Non-empty cells in those rows that are not in `cells`.
    pub cells: u64,
    /// Of those, formula cells (only once the cap on kept formulas is reached).
    pub formulas: u64,
    pub cols: Vec<ColStats>,
}

/// Running totals for one column of summarized rows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ColStats {
    pub col: u32,
    pub filled: u64,
    pub formulas: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nums: Option<Span>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<Span>,
    /// Text values with how often each appears, most frequent first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texts: Vec<(String, u64)>,
    pub text_cells: u64,
    /// More distinct texts appeared than `texts` lists; their cells are still
    /// counted in `text_cells`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub more_texts: bool,
    pub bools: u64,
    pub errors: u64,
}

/// Count and extremes of a column's numbers or dates, with how each extreme displays.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Span {
    pub n: u64,
    pub lo: f64,
    pub lo_text: String,
    pub hi: f64,
    pub hi_text: String,
}

impl Span {
    pub fn add(span: &mut Option<Span>, v: f64, text: impl FnOnce() -> String) {
        match span {
            None => {
                let t = text();
                *span = Some(Span { n: 1, lo: v, lo_text: t.clone(), hi: v, hi_text: t });
            }
            Some(s) => {
                s.n += 1;
                if v < s.lo {
                    s.lo = v;
                    s.lo_text = text();
                } else if v > s.hi {
                    s.hi = v;
                    s.hi_text = text();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Cell {
    pub r: u32,
    pub c: u32,
    #[serde(default, skip_serializing_if = "Value::is_empty")]
    pub v: Value,
    /// Formula text including the leading '='.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Display text, only stored when it differs from the plain rendering of `v`
    /// (dates, currency, percentages...).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl Cell {
    pub fn is_blank(&self) -> bool {
        self.v.is_empty() && self.f.is_none()
    }
    /// What a person would see in the cell.
    pub fn display(&self) -> String {
        match &self.t {
            Some(t) => t.clone(),
            None => self.v.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(untagged)]
pub enum Value {
    #[default]
    Empty,
    Num(f64),
    Bool(bool),
    Str(String),
    Err { err: String },
}

impl Value {
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => Ok(()),
            Value::Num(n) => write!(f, "{}", fmt_num(*n)),
            Value::Bool(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),
            Value::Str(s) => write!(f, "{s}"),
            Value::Err { err } => write!(f, "{err}"),
        }
    }
}

pub fn fmt_num(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        let s = format!("{:.6}", n);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Visual formatting, flattened to the properties that carry meaning for a reader.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Style {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_fmt: Option<String>,
    /// DATE, TIME, DATE_TIME, CURRENCY, PERCENT, NUMBER, TEXT, SCIENTIFIC
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub strike: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub underline: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub borders: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_align: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub wrap: bool,
}

impl Style {
    pub fn is_date(&self) -> bool {
        matches!(self.num_type.as_deref(), Some("DATE" | "DATE_TIME" | "TIME"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Validation {
    /// e.g. ONE_OF_LIST, ONE_OF_RANGE, NUMBER_BETWEEN, CUSTOM_FORMULA, BOOLEAN (checkbox)
    pub kind: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub strict: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub dropdown: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NamedRange {
    pub name: String,
    /// A1 reference including sheet, e.g. `'Revenue Input'!A1:R100`.
    pub range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CondFormat {
    pub ranges: Vec<Rect>,
    /// Human-readable rule, e.g. `NUMBER_LESS 0 → fg#FF0000` or `=$C2>100 → bg#F4CCCC`.
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Filter {
    pub range: Rect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Protected {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Rect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub warning_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Chart {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    pub kind: String,
    /// Source ranges as A1 strings (may be on other sheets).
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pivot {
    pub anchor: (u32, u32),
    pub source: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cols: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Table {
    pub name: String,
    /// Whole table including the header (and totals) rows.
    pub range: Rect,
    pub columns: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub totals: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalLink {
    /// File name as it appears in formulas, e.g. `Budget 2026.xlsx`.
    pub name: String,
    /// Link target as stored (path or URL).
    pub target: String,
    /// Sheet names cached from the other workbook.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sheets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Script {
    /// "vba" | "apps_script" | "macro"
    pub kind: String,
    pub name: String,
    pub source: String,
}

fn is_false(b: &bool) -> bool {
    !*b
}
fn is_zero(n: &u32) -> bool {
    *n == 0
}

/// Make the stored form independent of fetch order (parallel chunks arrive in
/// any order): renumber styles and validations by first use in row-major cell
/// order, and sort per-sheet lists.
pub fn canonicalize(wb: &mut Workbook) {
    fn remap<T: Clone>(items: &mut Vec<T>, ids: Vec<Option<u32>>) -> Vec<Option<u32>> {
        let mut map: Vec<Option<u32>> = vec![None; items.len()];
        let mut order: Vec<usize> = Vec::new();
        let out = ids
            .into_iter()
            .map(|id| {
                id.map(|i| {
                    let i = i as usize;
                    *map[i].get_or_insert_with(|| {
                        order.push(i);
                        (order.len() - 1) as u32
                    })
                })
            })
            .collect();
        *items = order.iter().map(|&i| items[i].clone()).collect();
        out
    }
    for sh in &mut wb.sheets {
        sh.cells.sort_by_key(|c| (c.r, c.c));
        sh.pivots.sort_by_key(|p| p.anchor);
    }
    let s_ids: Vec<Option<u32>> = wb.sheets.iter().flat_map(|sh| sh.cells.iter().map(|c| c.s)).collect();
    let d_ids: Vec<Option<u32>> = wb.sheets.iter().flat_map(|sh| sh.cells.iter().map(|c| c.dv)).collect();
    let s_new = remap(&mut wb.styles, s_ids);
    let d_new = remap(&mut wb.validations, d_ids);
    let mut i = 0;
    for sh in &mut wb.sheets {
        for c in &mut sh.cells {
            c.s = s_new[i];
            c.dv = d_new[i];
            i += 1;
        }
    }
}

/// Dedup table used by backends while building `styles`/`validations`.
#[derive(Default)]
pub struct Interner<T: std::hash::Hash + Eq + Clone> {
    pub items: Vec<T>,
    map: std::collections::HashMap<T, u32>,
}

impl<T: std::hash::Hash + Eq + Clone> Interner<T> {
    pub fn intern(&mut self, t: T) -> u32 {
        if let Some(&i) = self.map.get(&t) {
            return i;
        }
        let i = self.items.len() as u32;
        self.items.push(t.clone());
        self.map.insert(t, i);
        i
    }
}

/// Excel/Sheets serial date → ISO date (time appended when present).
pub fn serial_to_iso(serial: f64) -> String {
    let days = serial.floor() as i64;
    // 1899-12-30 is serial 0 in both Sheets and (post-1900-leap-bug) Excel.
    let z = days - 25569 + 719468; // days since 0000-03-01, via unix epoch offset
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let secs = ((serial - serial.floor()) * 86400.0).round() as i64;
    if secs == 0 {
        format!("{y:04}-{m:02}-{d:02}")
    } else {
        format!("{y:04}-{m:02}-{d:02} {:02}:{:02}", secs / 3600, (secs / 60) % 60)
    }
}

/// ISO 8601 date or date-time text ("2024-06-01", "2024-05-31T20:00:00.000Z") →
/// serial. A zone suffix is ignored and the clock time kept as written, as Excel
/// and openpyxl read it. None for anything else, including dates that don't exist.
pub fn iso_to_serial(s: &str) -> Option<f64> {
    let b = s.as_bytes();
    if b.len() < 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let num = |r: std::ops::Range<usize>| s.get(r).filter(|x| x.bytes().all(|c| c.is_ascii_digit()))?.parse::<i64>().ok();
    let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Days since 0000-03-01, then shifted to serial 0 = 1899-12-30 (see serial_to_iso).
    let (y_mar, m_mar) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = y_mar.div_euclid(400);
    let yoe = y_mar - era * 400;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + (153 * m_mar + 2) / 5 + d - 1;
    let date = (era * 146097 + doe - 719468 + 25569) as f64;
    if serial_to_iso(date) != s[..10] {
        return None; // e.g. 2024-02-31
    }
    let rest = &s[10..];
    if rest.is_empty() {
        return Some(date);
    }
    let time = rest.strip_prefix(['T', ' '])?.trim_end_matches('Z');
    let time = match time.rfind(['+', '-']) {
        Some(i) if i >= 5 => &time[..i], // "+02:00" / "-0400"
        _ => time,
    };
    let mut hms = time.splitn(3, ':');
    let h: f64 = hms.next()?.parse().ok()?;
    let min: f64 = hms.next()?.parse().ok()?;
    let sec: f64 = hms.next().map_or(Some(0.0), |x| x.parse().ok())?;
    if !(0.0..24.0).contains(&h) || !(0.0..60.0).contains(&min) || !(0.0..61.0).contains(&sec) {
        return None;
    }
    Some(date + (h * 3600.0 + min * 60.0 + sec) / 86400.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn serial_dates() {
        assert_eq!(super::serial_to_iso(45000.0), "2023-03-15");
        assert_eq!(super::serial_to_iso(1.0), "1899-12-31");
        assert_eq!(super::serial_to_iso(45000.5), "2023-03-15 12:00");
    }

    #[test]
    fn iso_dates() {
        use super::{iso_to_serial, serial_to_iso};
        assert_eq!(iso_to_serial("2023-03-15"), Some(45000.0));
        assert_eq!(iso_to_serial("2023-03-15T12:00:00"), Some(45000.5));
        assert_eq!(iso_to_serial("2023-03-15 12:00"), Some(45000.5));
        let shown = |s: &str| iso_to_serial(s).map(serial_to_iso);
        assert_eq!(shown("2024-05-31T20:00:00.000Z").as_deref(), Some("2024-05-31 20:00"));
        assert_eq!(shown("2024-05-31T20:00:00+02:00").as_deref(), Some("2024-05-31 20:00"));
        assert_eq!(shown("2024-02-29").as_deref(), Some("2024-02-29"));
        assert_eq!(shown("1999-12-31").as_deref(), Some("1999-12-31"));
        for bad in ["2024-02-30", "2023-13-01", "2024-06-00", "2024-6-1", "2024-06-01x", "2024-06-01 nan:00", "06/01/2024", "02134", ""] {
            assert_eq!(iso_to_serial(bad), None, "{bad}");
        }
    }
}
