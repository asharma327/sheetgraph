//! On-disk layout of an extraction:
//!
//! ```text
//! out/
//!   INDEX.md                    all workbooks + cross-workbook links
//!   <slug>/REPORT.md            compact map for an LLM (start here)
//!   <slug>/graph.json           region-level dependency graph
//!   <slug>/workbook.json.gz     every cell, for `sheetgraph cells|trace|find`
//!   <slug>/meta.json            sizes and ids used by INDEX.md
//! ```

use crate::analyze::Analysis;
use crate::model::Workbook;
use crate::report::approx_tokens;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub title: String,
    pub id: String,
    pub kind: String,
    pub slug: String,
    pub sheets: usize,
    pub cells: u64,
    pub formulas: u64,
    pub groups: usize,
    pub report_bytes: usize,
    pub report_tokens: usize,
    /// Naive "dump every cell with its formula" size, for comparison.
    pub naive_tokens: usize,
    pub imports: Vec<String>,
    pub sheet_names: Vec<String>,
}

pub fn slug(wb: &Workbook) -> String {
    let base = Path::new(&wb.source.id).file_name().and_then(|s| s.to_str()).unwrap_or(&wb.source.id);
    let t: String = wb.source.title.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let t = t.split('-').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("-");
    let id: String = base.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.').take(10).collect();
    if t.is_empty() { id } else if wb.source.kind == "gsheet" { format!("{t}_{id}") } else { t }
}

pub fn naive_tokens(wb: &Workbook) -> usize {
    let mut s = 0;
    for sh in &wb.sheets {
        for c in &sh.cells {
            if c.is_blank() {
                continue;
            }
            // "Sheet!A1\tvalue\tformula\n"
            let line = format!("{}!{}\t{}\t{}\n", sh.name, crate::a1::cell_name(c.r, c.c), c.display(), c.f.as_deref().unwrap_or(""));
            s += approx_tokens(&line);
        }
    }
    s
}

pub fn save(out: &Path, wb: &Workbook, a: &Analysis, report: &str) -> Result<(PathBuf, Meta)> {
    let dir = out.join(slug(wb));
    fs::create_dir_all(&dir)?;
    let f = fs::File::create(dir.join("workbook.json.gz"))?;
    let mut gz = flate2::write::GzEncoder::new(BufWriter::new(f), flate2::Compression::fast());
    // Buffer in front of the encoder too: serde_json writes in tiny pieces,
    // which are slow to compress one at a time.
    let mut w = BufWriter::with_capacity(1 << 16, &mut gz);
    serde_json::to_writer(&mut w, wb)?;
    w.flush()?;
    drop(w);
    gz.finish()?.flush()?;
    save_derived(&dir, wb, a, report)
}

/// Write everything derived from the workbook (report, graph, meta) into `dir`.
pub fn save_derived(dir: &Path, wb: &Workbook, a: &Analysis, report: &str) -> Result<(PathBuf, Meta)> {
    let slug = dir.file_name().and_then(|s| s.to_str()).unwrap_or_default().to_string();
    fs::write(dir.join("REPORT.md"), report)?;
    let g = crate::graph::build(wb, a);
    fs::write(dir.join("graph.json"), serde_json::to_vec(&g)?)?;
    let mut imports: Vec<String> =
        a.externals.iter().filter(|e| e.func == "IMPORTRANGE" || e.func == "workbook link").map(|e| e.target.clone()).collect();
    imports.sort();
    imports.dedup();
    let meta = Meta {
        title: wb.source.title.clone(),
        id: wb.source.id.clone(),
        kind: wb.source.kind.clone(),
        slug: slug.clone(),
        sheets: wb.sheets.len(),
        cells: a.sheets.iter().map(|s| s.n_cells).sum(),
        formulas: a.sheets.iter().map(|s| s.n_formulas).sum(),
        groups: a.sheets.iter().map(|s| s.groups.len()).sum(),
        report_bytes: report.len(),
        report_tokens: approx_tokens(report),
        naive_tokens: naive_tokens(wb),
        imports,
        sheet_names: wb.sheets.iter().map(|s| s.name.clone()).collect(),
    };
    fs::write(dir.join("meta.json"), serde_json::to_vec_pretty(&meta)?)?;
    Ok((dir.to_path_buf(), meta))
}

/// Accepts either a workbook dir (containing workbook.json.gz) or the file itself.
pub fn load(path: &Path) -> Result<Workbook> {
    let p = if path.is_dir() { path.join("workbook.json.gz") } else { path.to_path_buf() };
    let f = fs::File::open(&p).with_context(|| format!("opening {}", p.display()))?;
    let gz = flate2::read::GzDecoder::new(BufReader::new(f));
    let mut wb: Workbook = serde_json::from_reader(BufReader::new(gz))?;
    crate::model::canonicalize(&mut wb);
    Ok(wb)
}

/// Rebuild INDEX.md from every meta.json under `out`.
pub fn write_index(out: &Path) -> Result<String> {
    let mut metas: Vec<Meta> = Vec::new();
    for e in fs::read_dir(out)? {
        let p = e?.path().join("meta.json");
        if let Ok(b) = fs::read(&p)
            && let Ok(m) = serde_json::from_slice::<Meta>(&b) {
                metas.push(m);
            }
    }
    metas.sort_by(|a, b| a.title.cmp(&b.title));
    let mut s = String::from("# Spreadsheet index\n");
    s.push_str("Each workbook has `<dir>/REPORT.md` (read first), `graph.json`, and cell-level queries: `sheetgraph cells|trace|find <dir> ...`.\n\n");
    for m in &metas {
        s.push_str(&format!(
            "- **{}** (`{}/`) {} {} · {} sheets · {} cells · {} formulas/{} groups · report ≈{} tokens (raw cell dump ≈{})\n  sheets: {}\n",
            m.title,
            m.slug,
            m.kind,
            m.id,
            m.sheets,
            m.cells,
            m.formulas,
            m.groups,
            m.report_tokens,
            m.naive_tokens,
            crate::report::trunc(&m.sheet_names.join(", "), 400)
        ));
    }
    let mut links = Vec::new();
    for m in &metas {
        for t in &m.imports {
            // Sheets ids match exactly; Excel links name the other file.
            let same_file = |x: &Meta| Path::new(&x.id).file_name().and_then(|n| n.to_str()).is_some_and(|n| n.eq_ignore_ascii_case(t));
            let target = metas
                .iter()
                .find(|x| &x.id == t || same_file(x))
                .map(|x| format!("**{}**", x.title))
                .unwrap_or_else(|| format!("`{t}` (not extracted)"));
            links.push(format!("- **{}** imports from {}", m.title, target));
        }
    }
    if !links.is_empty() {
        s.push_str("\n## Cross-workbook links (IMPORTRANGE and Excel external links; ranges are listed in each REPORT.md under External links)\n");
        s.push_str(&links.join("\n"));
        s.push('\n');
    }
    fs::write(out.join("INDEX.md"), &s)?;
    Ok(s)
}
