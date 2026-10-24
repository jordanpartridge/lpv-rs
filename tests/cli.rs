//! End-to-end tests: run the built `lpv-rs` binary on copies of the fixtures.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

fn fixture(name: &str) -> (tempfile::TempDir, PathBuf) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dest = tmp.path().join(name);
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
        &dest,
    );
    for (from, to) in [
        ("_gitattributes", ".gitattributes"),
        ("_gitignore", ".gitignore"),
    ] {
        if dest.join(from).exists() {
            std::fs::rename(dest.join(from), dest.join(to)).expect("rename");
        }
    }
    (tmp, dest)
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("mkdir");
    for entry in std::fs::read_dir(from).expect("read_dir") {
        let entry = entry.expect("entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("copy");
        }
    }
}

fn git_commit(dir: &Path) {
    for args in [
        vec!["init", "-q"],
        vec!["add", "-A"],
        vec![
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@example.com",
            "commit",
            "-q",
            "-m",
            "fixture",
        ],
    ] {
        let status = Command::new("git")
            .args(&args)
            .current_dir(dir)
            .status()
            .expect("git");
        assert!(status.success(), "git {args:?} failed");
    }
}

fn lpv(dir: &Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_lpv-rs"));
    cmd.args(args).current_dir(dir).env_clear();
    for key in ["PATH", "HOME"] {
        if let Some(value) = std::env::var_os(key) {
            cmd.env(key, value);
        }
    }
    cmd.output().expect("run lpv-rs")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn json(out: &Output) -> Value {
    serde_json::from_slice(&out.stdout).expect("valid JSON on stdout")
}

#[test]
fn valid_fixture_validates() {
    let (_tmp, dir) = fixture("valid");
    let out = lpv(&dir, &["validate"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        stdout(&out),
        "The present .gitattributes file is considered valid.\n"
    );
}

#[test]
fn missing_ignores_are_reported() {
    let (_tmp, dir) = fixture("missing-ignores");
    let out = lpv(&dir, &["validate"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stdout(&out).contains("considered invalid"));

    let report = json(&lpv(&dir, &["validate", "--format", "json"]));
    assert_eq!(report["valid"], Value::Bool(false));
    assert_eq!(report["exit_code"], 1);
    let missing: Vec<&str> = report["missing_export_ignores"]
        .as_array()
        .expect("array")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(missing.contains(&"tests/"), "{missing:?}");
    assert!(
        report["gitattributes_path"]
            .as_str()
            .is_some_and(|p| p.ends_with(".gitattributes"))
    );
}

#[test]
fn stale_ignores_are_reported() {
    let (_tmp, dir) = fixture("stale-ignores");
    let report = json(&lpv(
        &dir,
        &[
            "validate",
            "--report-stale-export-ignores",
            "--format",
            "json",
        ],
    ));
    let stale: Vec<&str> = report["stale_export_ignores"]
        .as_array()
        .expect("array")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        stale.contains(&"docs/") && stale.contains(&"CHANGELOG.md"),
        "{stale:?}"
    );
}

#[test]
fn create_then_validate() {
    let (_tmp, dir) = fixture("no-gitattributes");
    let out = lpv(&dir, &["create"]);
    assert_eq!(out.status.code(), Some(0), "{}", stdout(&out));
    let written = std::fs::read_to_string(dir.join(".gitattributes")).expect("written");
    assert!(written.contains("* text=auto eol=lf"));
    assert!(written.contains(".gitattributes export-ignore"));
    assert_eq!(lpv(&dir, &["validate"]).status.code(), Some(0));
    // A second create refuses to overwrite.
    assert_eq!(lpv(&dir, &["create"]).status.code(), Some(1));
}

#[test]
fn update_fixes_missing_ignores() {
    let (_tmp, dir) = fixture("missing-ignores");
    assert_eq!(lpv(&dir, &["update"]).status.code(), Some(0));
    assert_eq!(lpv(&dir, &["validate"]).status.code(), Some(0));
}

#[test]
fn negated_update_keeps_strategy() {
    let (_tmp, dir) = fixture("negated-bin");
    let out = lpv(&dir, &["update", "--dry-run"]);
    assert_eq!(out.status.code(), Some(0));
    let text = stdout(&out);
    assert!(text.contains("* export-ignore\n"));
    assert!(
        text.contains("bin/ -export-ignore\nbin/** -export-ignore\nbin/acme -export-ignore\n"),
        "{text}"
    );
}

#[test]
fn archive_leaks_are_found() {
    let (_tmp, dir) = fixture("archive-leaks");
    git_commit(&dir);
    let out = lpv(&dir, &["validate", "--validate-git-archive"]);
    assert_eq!(out.status.code(), Some(1));
    let report = json(&lpv(
        &dir,
        &["validate", "--validate-git-archive", "--format", "json"],
    ));
    assert_eq!(report["archive_validated"], Value::Bool(true));
    let leaks = report["archive_leaks"].as_array().expect("array");
    assert!(!leaks.is_empty());
}

#[test]
fn unknown_and_ambiguous_commands() {
    let (_tmp, dir) = fixture("valid");
    let out = lpv(&dir, &["frobnicate"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("Command \"frobnicate\" is not defined.")
    );
    let out = lpv(&dir, &["val"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("is ambiguous"));
    // Unique abbreviations resolve like Symfony's.
    assert_eq!(lpv(&dir, &["upd", "--dry-run"]).status.code(), Some(0));
}

#[test]
fn agent_json_when_ai_agent_is_set() {
    let (_tmp, dir) = fixture("valid");
    let out = Command::new(env!("CARGO_BIN_EXE_lpv-rs"))
        .arg("validate")
        .current_dir(&dir)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("AI_AGENT", "test-suite")
        .output()
        .expect("run");
    let report = json(&out);
    assert_eq!(report["command"], "validate");
    assert_eq!(report["status"], "success");
}
