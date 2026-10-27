//! sheetgraph: extract spreadsheets (Google Sheets, xlsx, csv) into a compact,
//! LLM-readable map plus a formula dependency graph.
//!
//! Pipeline: backend → [`model::Workbook`] (full fidelity) → [`analyze::analyze`]
//! (copy-groups, blocks, edges) → [`report::render`] (REPORT.md) and
//! [`graph::build`] (graph.json). [`query`] answers cell-level questions later.

pub mod a1;
pub mod analyze;
pub mod formula;
pub mod graph;
pub mod gsheets;
pub mod model;
pub mod ooxml;
pub mod query;
pub mod report;
pub mod store;
pub mod xlsx;

use anyhow::Result;
use std::path::Path;

/// Load a workbook from a local file path, or from a Google Sheets id / URL.
pub fn load(source: &str, auth: Option<&gsheets::Auth>, verbose: bool) -> Result<model::Workbook> {
    let p = Path::new(source);
    if p.exists() {
        return xlsx::read(p);
    }
    if !is_google_sheet(source) {
        anyhow::bail!("no such file (and not a Google Sheets id or URL)");
    }
    let auth = auth.ok_or_else(|| anyhow::anyhow!("not a file; Google credentials needed to read it as a Google Sheet"))?;
    gsheets::fetch(auth, source, verbose)
}

/// Whether a source that isn't a local file reads as a Google Sheets URL or id.
/// Anything else (a name with an extension, a path) is a file that wasn't found.
pub fn is_google_sheet(source: &str) -> bool {
    let s = source.trim();
    if s.starts_with("https://") || s.starts_with("http://") {
        return s.contains("/d/");
    }
    s.len() >= 25 && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
