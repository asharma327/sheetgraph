//! What `extract` does with sources that aren't local files: a mistyped path is a
//! clear "no such file" for that source only, a Google login problem only fails
//! the Google sources, and the other files are still extracted. No network.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/features.xlsx");
/// Google's public sample spreadsheet; never fetched here.
const SHEET_ID: &str = "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms";

fn out_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sheetgraph-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Runs `sheetgraph extract <args> --out <out>`; returns the exit code and stderr.
fn extract(args: &[&str], out: &Path) -> (i32, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_sheetgraph"))
        .arg("extract")
        .args(args)
        .arg("--out")
        .arg(out)
        .env_remove("SHEETGRAPH_TOKEN")
        .env_remove("GOOGLE_APPLICATION_CREDENTIALS")
        .output()
        .expect("run sheetgraph");
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stderr).into_owned())
}

/// Workbooks saved under `out` (one folder with a REPORT.md each).
fn extracted(out: &Path) -> usize {
    std::fs::read_dir(out)
        .map(|d| d.filter_map(|e| e.ok()).filter(|e| e.path().join("REPORT.md").exists()).count())
        .unwrap_or(0)
}

#[test]
fn google_sheet_or_file() {
    assert!(sheetgraph::is_google_sheet(&format!("https://docs.google.com/spreadsheets/d/{SHEET_ID}/edit#gid=0")));
    assert!(sheetgraph::is_google_sheet(SHEET_ID));
    for path in ["typo.xlsx", "data/report.xlsx", r"C:\books\q3.csv", "budget", "https://example.com/report.xlsx"] {
        assert!(!sheetgraph::is_google_sheet(path), "{path}");
    }
}

#[test]
fn missing_file_is_named_not_a_credentials_error() {
    let err = sheetgraph::load("typo.xlsx", None, false).unwrap_err().to_string();
    assert!(err.starts_with("no such file"), "{err}");
    assert!(!err.contains("credentials"), "{err}");
}

#[test]
fn missing_file_does_not_stop_the_others() {
    let out = out_dir("missing");
    let (code, stderr) = extract(&["typo.xlsx", FIXTURE], &out);
    assert_eq!(code, 1, "a failed source still fails the run\n{stderr}");
    assert!(stderr.contains("typo.xlsx: no such file"), "{stderr}");
    assert!(!stderr.contains("credentials"), "no Google login for a mistyped path\n{stderr}");
    assert_eq!(extracted(&out), 1, "the real file is still extracted\n{stderr}");
    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn google_login_failure_does_not_stop_local_files() {
    let out = out_dir("no-login");
    let creds = out.join("no-such-credentials.json");
    let (code, stderr) = extract(&[SHEET_ID, FIXTURE, "--creds", creds.to_str().unwrap()], &out);
    assert_eq!(code, 1, "{stderr}");
    assert!(stderr.contains("google auth failed"), "{stderr}");
    assert!(stderr.contains("Google credentials needed"), "{stderr}");
    assert_eq!(extracted(&out), 1, "the local file is still extracted\n{stderr}");
    let _ = std::fs::remove_dir_all(&out);
}
