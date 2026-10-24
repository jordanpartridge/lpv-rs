//! `--validate-git-archive`: build `git archive HEAD` and inspect the
//! top-level entries, like `Stolt\LeanPackage\Archive` (which iterates the
//! root of a `PharData` built from `git archive -o .tar.gz HEAD`).
//!
//! Like the original, git runs in the process working directory (not in the
//! `directory` argument). Unlike the original, nothing is written to disk:
//! the tar stream is read straight from `git archive`'s stdout.

use std::path::Path;
use std::process::{Command, Stdio};

use crate::error::LpvError;
use crate::php;

pub struct ArchiveCheck {
    pub found_unexpected: Vec<String>,
}

fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

fn has_head(cwd: &Path) -> bool {
    Command::new("git")
        .args(["show-ref", "--head"])
        .current_dir(cwd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Top-level entries of `git archive HEAD` as `(name, is_dir)`, in archive order.
fn top_level_entries(cwd: &Path) -> Result<Vec<(String, bool)>, LpvError> {
    let output = Command::new("git")
        .args(["archive", "--format=tar", "HEAD"])
        .current_dir(cwd)
        .stderr(Stdio::null())
        .output()
        .map_err(|e| LpvError::Io(format!("failed to run git archive: {e}")))?;
    if !output.status.success() {
        return Err(LpvError::Io("git archive HEAD failed".to_string()));
    }
    let mut archive = tar::Archive::new(output.stdout.as_slice());
    let mut seen: Vec<(String, bool)> = Vec::new();
    let entries = archive
        .entries()
        .map_err(|e| LpvError::Io(format!("cannot read git archive: {e}")))?;
    for entry in entries {
        let entry = entry.map_err(|e| LpvError::Io(format!("cannot read git archive: {e}")))?;
        let kind = entry.header().entry_type();
        if kind.is_pax_global_extensions() || kind.is_pax_local_extensions() {
            continue;
        }
        let path_bytes = entry.path_bytes();
        let path = String::from_utf8_lossy(&path_bytes).into_owned();
        let path = path.trim_start_matches("./");
        let mut parts = path.splitn(2, '/');
        let Some(first) = parts.next().filter(|s| !s.is_empty()) else {
            continue;
        };
        let rest = parts.next().unwrap_or("");
        let is_dir = kind.is_dir() || !rest.is_empty();
        if !seen.iter().any(|(n, _)| n == first) {
            seen.push((first.to_string(), is_dir));
        } else if is_dir {
            for item in seen.iter_mut() {
                if item.0 == first {
                    item.1 = true;
                }
            }
        }
    }
    Ok(seen)
}

/// `Validator::validate()` + `Archive::compareArchive()`.
pub fn validate(
    cwd: &Path,
    unexpected_artifacts: &[String],
    should_have_license: bool,
) -> Result<ArchiveCheck, LpvError> {
    if !git_available() {
        return Err(LpvError::Uncaught {
            file: "Archive.php",
            line: 117,
            message: "The Git command is not available.".to_string(),
        });
    }
    if !has_head(cwd) {
        return Err(LpvError::Uncaught {
            file: "Archive.php",
            line: 135,
            message: "No Git HEAD present to create an archive from.".to_string(),
        });
    }
    let entries = top_level_entries(cwd)?;
    let mut found = Vec::new();
    let mut has_license = false;
    for (name, is_dir) in entries {
        if is_dir {
            let candidate = format!("{name}/");
            if unexpected_artifacts.contains(&candidate) {
                found.push(candidate);
            }
            continue;
        }
        if should_have_license && php::contains_ci(&name, "license") {
            has_license = true;
        }
        if unexpected_artifacts.contains(&name) {
            found.push(name);
        }
    }
    if should_have_license && !has_license {
        return Err(LpvError::NoLicenseFilePresent);
    }
    php::sort_case(&mut found);
    Ok(ArchiveCheck {
        found_unexpected: found,
    })
}
