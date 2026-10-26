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
pub mod query;
pub mod report;
pub mod store;
pub mod xlsx;

use anyhow::Result;
use std::path::Path;

/// Load a workbook from a local file path, or treat the argument as a Google
/// Sheets id / URL.
pub fn load(source: &str, auth: Option<&gsheets::Auth>, verbose: bool) -> Result<model::Workbook> {
    let p = Path::new(source);
    if p.exists() {
        return xlsx::read(p);
    }
    let auth = auth.ok_or_else(|| anyhow::anyhow!("{source} is not a file; Google credentials needed"))?;
    gsheets::fetch(auth, source, verbose)
}
