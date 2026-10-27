use anyhow::Result;
use clap::{Parser, Subcommand};
use sheetgraph::{analyze, gsheets, query, report, store};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "sheetgraph", version, about = "Extract spreadsheets into a compact LLM-readable map + formula graph")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Extract Google Sheets (id or URL) and/or local files (xlsx, xlsm, xls, xlsb, ods, csv).
    Extract {
        sources: Vec<String>,
        #[arg(short, long, default_value = "sheetgraph-out")]
        out: PathBuf,
        /// Service-account or authorized-user JSON (default: $SHEETGRAPH_TOKEN, $GOOGLE_APPLICATION_CREDENTIALS, then gcloud ADC).
        #[arg(long)]
        creds: Option<String>,
        /// Apps Script project id(s) to include (needs user OAuth with script.projects.readonly).
        #[arg(long = "apps-script")]
        apps_script: Vec<String>,
        /// Render the report without truncation.
        #[arg(long)]
        full: bool,
    },
    /// Re-render one sheet's section (use --full for every formula group and column).
    Sheet {
        dir: PathBuf,
        sheet: String,
        #[arg(long)]
        full: bool,
    },
    /// Every non-empty cell in a range with value, formula, format, validation, note.
    Cells {
        dir: PathBuf,
        range: String,
        #[arg(long, default_value_t = 200)]
        limit: usize,
    },
    /// Precedents and dependents of a cell or range.
    Trace {
        dir: PathBuf,
        cell: String,
        #[arg(long, default_value_t = 2)]
        depth: usize,
        /// Only precedents.
        #[arg(long)]
        up: bool,
        /// Only dependents.
        #[arg(long)]
        down: bool,
    },
    /// Search values, formulas and notes.
    Find {
        dir: PathBuf,
        text: String,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Re-render REPORT.md / graph.json / meta.json / INDEX.md for every saved workbook under OUT (no network).
    Rebuild {
        #[arg(default_value = "sheetgraph-out")]
        out: PathBuf,
        #[arg(long)]
        full: bool,
    },
    /// Print the report for a saved extraction (e.g. with --full).
    Report {
        dir: PathBuf,
        #[arg(long)]
        full: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Extract { sources, out, creds, apps_script, full } => {
            std::fs::create_dir_all(&out)?;
            let needs_google = sources.iter().any(|s| !std::path::Path::new(s).exists());
            let auth = if needs_google { Some(gsheets::Auth::discover(creds.as_deref())?) } else { None };
            if let Some(a) = &auth {
                eprintln!("google auth: {}", a.identity());
            }
            let opts = if full { report::Opts::full() } else { report::Opts::default() };
            let mut failed = 0;
            for src in &sources {
                let t0 = Instant::now();
                let mut wb = match sheetgraph::load(src, auth.as_ref(), true) {
                    Ok(wb) => wb,
                    Err(e) => {
                        eprintln!("✗ {src}: {e:#}");
                        failed += 1;
                        continue;
                    }
                };
                for id in &apps_script {
                    match gsheets::fetch_apps_script(auth.as_ref().expect("google auth"), id) {
                        Ok(s) => wb.scripts.extend(s),
                        Err(e) => wb.warnings.push(format!("apps script {id}: {e}")),
                    }
                }
                let t1 = Instant::now();
                let a = analyze::analyze(&wb);
                let rep = report::render(&wb, &a, &opts);
                let (dir, meta) = store::save(&out, &wb, &a, &rep)?;
                eprintln!(
                    "✓ {} → {}  fetch {:.1}s analyze {:.2}s · {} cells, {} formulas → {} groups · REPORT ≈{} tokens vs raw dump ≈{} ({:.0}×)",
                    meta.title,
                    dir.display(),
                    (t1 - t0).as_secs_f32(),
                    t1.elapsed().as_secs_f32(),
                    meta.cells,
                    meta.formulas,
                    meta.groups,
                    meta.report_tokens,
                    meta.naive_tokens,
                    meta.naive_tokens as f64 / meta.report_tokens.max(1) as f64
                );
            }
            store::write_index(&out)?;
            eprintln!("index: {}", out.join("INDEX.md").display());
            if failed > 0 {
                std::process::exit(1);
            }
        }
        Cmd::Sheet { dir, sheet, full } => {
            let wb = store::load(&dir)?;
            let a = analyze::analyze(&wb);
            let si = analyze::sheet_index(&wb, &sheet).ok_or_else(|| anyhow::anyhow!("no sheet {sheet:?}"))?;
            let opts = if full { report::Opts::full() } else { report::Opts::default() };
            print!("{}", report::render_sheet(&wb, &a, si, &opts));
        }
        Cmd::Cells { dir, range, limit } => {
            let wb = store::load(&dir)?;
            let a = analyze::analyze(&wb);
            print!("{}", query::cells(&wb, &a, &range, limit)?);
        }
        Cmd::Trace { dir, cell, depth, up, down } => {
            let wb = store::load(&dir)?;
            let a = analyze::analyze(&wb);
            let (u, d) = if !up && !down { (true, true) } else { (up, down) };
            print!("{}", query::trace(&wb, &a, &cell, depth, u, d)?);
        }
        Cmd::Find { dir, text, limit } => {
            let wb = store::load(&dir)?;
            let a = analyze::analyze(&wb);
            print!("{}", query::find(&wb, &a, &text, limit));
        }
        Cmd::Rebuild { out, full } => {
            let opts = if full { report::Opts::full() } else { report::Opts::default() };
            for e in std::fs::read_dir(&out)? {
                let dir = e?.path();
                if !dir.join("workbook.json.gz").exists() {
                    continue;
                }
                let wb = store::load(&dir)?;
                let a = analyze::analyze(&wb);
                let rep = report::render(&wb, &a, &opts);
                let (_, meta) = store::save_derived(&dir, &wb, &a, &rep)?;
                eprintln!("✓ {} · REPORT ≈{} tokens", meta.title, meta.report_tokens);
            }
            store::write_index(&out)?;
        }
        Cmd::Report { dir, full } => {
            let wb = store::load(&dir)?;
            let a = analyze::analyze(&wb);
            let opts = if full { report::Opts::full() } else { report::Opts::default() };
            print!("{}", report::render(&wb, &a, &opts));
        }
    }
    Ok(())
}
