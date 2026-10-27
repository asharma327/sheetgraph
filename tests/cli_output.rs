//! The command line's output side: files with the same name get their own
//! folders, and piping a long answer into a reader that stops early (`| head`)
//! ends quietly instead of panicking. No network.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sheetgraph"))
}

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sheetgraph-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Workbook folders under `out` (each holds a REPORT.md).
fn folders(out: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(out)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("REPORT.md").exists())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

fn extract(sources: &[&Path], out: &Path) {
    let status = bin().arg("extract").args(sources).arg("--out").arg(out).status().unwrap();
    assert!(status.success());
}

#[test]
fn same_name_files_get_their_own_folders() {
    let dir = temp_dir("same-name");
    let (jan, feb) = (dir.join("jan"), dir.join("feb"));
    for (d, who) in [(&jan, "Acme"), (&feb, "Globex")] {
        std::fs::create_dir_all(d).unwrap();
        std::fs::write(d.join("orders.csv"), format!("id,customer\n1,{who}\n")).unwrap();
    }
    let (a, b) = (jan.join("orders.csv"), feb.join("orders.csv"));
    let out = dir.join("out");
    extract(&[&a, &b], &out);
    assert_eq!(folders(&out), ["orders", "orders-csv"], "neither overwrites the other");
    let index = std::fs::read_to_string(out.join("INDEX.md")).unwrap();
    assert_eq!(index.matches("`orders").count(), 2, "{index}");
    for (folder, who) in [("orders", "Acme"), ("orders-csv", "Globex")] {
        let report = std::fs::read_to_string(out.join(folder).join("REPORT.md")).unwrap();
        assert!(report.contains(who), "{folder} holds its own file\n{report}");
    }
    // Running again reuses each file's folder rather than adding more.
    extract(&[&a, &b], &out);
    assert_eq!(folders(&out), ["orders", "orders-csv"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn piping_into_head_ends_quietly() {
    let dir = temp_dir("pipe");
    let csv = dir.join("pipecheck.csv");
    let mut text = String::from("id,customer,total\n");
    for i in 0..20_000 {
        text += &format!("{i},Customer {},{}.50\n", i % 10, i % 900);
    }
    std::fs::write(&csv, text).unwrap();
    let out = dir.join("out");
    extract(&[&csv], &out);

    // About 2 MB of output, far more than a pipe holds, so the writer is still
    // going when the reader stops after the first bytes, as `| head` does.
    let mut child = bin()
        .arg("cells")
        .arg(out.join("pipecheck"))
        .args(["pipecheck!A1:C20001", "--limit", "100000"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut first = [0u8; 64];
    child.stdout.as_mut().unwrap().read_exact(&mut first).unwrap();
    drop(child.stdout.take());
    let done = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&done.stderr);
    assert!(done.status.success(), "exit {:?}\n{stderr}", done.status.code());
    assert!(!stderr.contains("panicked"), "{stderr}");
    let _ = std::fs::remove_dir_all(&dir);
}
