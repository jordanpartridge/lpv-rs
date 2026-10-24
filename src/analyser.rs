//! The export-ignore analysis: a faithful port of
//! `ClassicExportIgnoreAnalyser` (+ the bits of `AbstractExportIgnoreAnalyser`,
//! `Analyser` and `NegatedExportIgnoreAnalyser`).
//!
//! The state machine deliberately mirrors the PHP object (including flags that
//! are only set as a side effect of collecting expectations), because the
//! human output of the reference depends on those side effects.

use std::path::{Path, PathBuf};

use crate::error::LpvError;
use crate::glob::{self, Globber};
use crate::php;
use crate::presets;

pub const PLACEHOLDER: &str = "{{ export_ignores_placement }}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flavour {
    Classic,
    Negated,
}

#[derive(Debug)]
pub enum GlobFileError {
    NonExistent,
    Invalid,
}

pub struct Analyser {
    /// The directory exactly as PHP sees it (relative paths stay relative).
    pub directory: String,
    cwd: PathBuf,
    globber: Globber,
    pub glob_pattern: String,
    strict_order: bool,
    stale: bool,
    strict_alignment: bool,
    sort_dirs_to_files: bool,
    keep_license: bool,
    keep_readme: bool,
    keep_glob_pattern: String,
    align: bool,
    pub group_non_export_ignores: bool,
    preceding_slash: bool,
    text_auto: bool,
    mit_license: Option<bool>,
    preferred_eol: &'static str,
    /// Set once `Analyser::getExpectedGitattributesContent()` has swapped the
    /// classic analyser for a `NegatedExportIgnoreAnalyser`.
    negated_mode: bool,
}

fn guard_glob_pattern(pattern: &str) -> Result<(), ()> {
    let mut invalid = false;
    if pattern != "{" && !pattern.ends_with('}') && !pattern.ends_with("}*") {
        invalid = true;
    }
    let braces = php::trim(php::inner(pattern));
    if braces.is_empty() {
        invalid = true;
    }
    if braces.split(',').count() == 1 {
        invalid = true;
    }
    if invalid { Err(()) } else { Ok(()) }
}

impl Analyser {
    pub fn new(cwd: &Path) -> Self {
        let directory = cwd.to_string_lossy().into_owned();
        Self {
            globber: Globber::new(cwd),
            directory,
            cwd: cwd.to_path_buf(),
            glob_pattern: presets::default_glob_pattern(),
            strict_order: false,
            stale: false,
            strict_alignment: false,
            sort_dirs_to_files: false,
            keep_license: false,
            keep_readme: false,
            keep_glob_pattern: String::new(),
            align: false,
            group_non_export_ignores: false,
            preceding_slash: false,
            text_auto: false,
            mit_license: None,
            preferred_eol: "\n",
            negated_mode: false,
        }
    }

    pub fn dir_path(&self) -> PathBuf {
        self.cwd.join(&self.directory)
    }

    /// `setDirectory()`.
    pub fn set_directory(&mut self, directory: &str) -> Result<(), LpvError> {
        let path = self.cwd.join(directory);
        if directory.is_empty() || !path.is_dir() {
            return Err(LpvError::Uncaught {
                file: "AbstractExportIgnoreAnalyser.php",
                line: 216,
                message: format!("Directory {directory} doesn't exist."),
            });
        }
        self.directory = directory.to_string();
        self.globber = Globber::new(&path);
        Ok(())
    }

    pub fn gitattributes_file_path(&self) -> String {
        format!("{}/.gitattributes", self.directory)
    }

    fn gitattributes_fs_path(&self) -> PathBuf {
        self.dir_path().join(".gitattributes")
    }

    pub fn has_gitattributes_file(&self) -> bool {
        let path = self.gitattributes_fs_path();
        path.exists() && std::fs::File::open(&path).is_ok()
    }

    /// `FileRepository::getGitattributesContent()`.
    pub fn gitattributes_content(&self) -> String {
        std::fs::read(self.gitattributes_fs_path())
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default()
    }

    /// `getPresentGitAttributesContent()`.
    pub fn present_content(&self) -> String {
        if !self.has_gitattributes_file() {
            return String::new();
        }
        self.gitattributes_content()
    }

    // ----- option setters -------------------------------------------------

    pub fn enable_strict_order(&mut self) {
        self.strict_order = true;
    }
    pub fn enable_sort_dirs_to_files(&mut self) {
        self.sort_dirs_to_files = true;
    }
    pub fn enable_stale(&mut self) {
        self.stale = true;
    }
    pub fn enable_strict_alignment(&mut self) {
        self.strict_alignment = true;
    }
    pub fn enable_keep_license(&mut self) {
        self.keep_license = true;
    }
    pub fn enable_keep_readme(&mut self) {
        self.keep_readme = true;
    }
    pub fn enable_align(&mut self) {
        self.align = true;
    }

    pub fn set_keep_glob_pattern(&mut self, pattern: &str) -> Result<(), ()> {
        guard_glob_pattern(pattern)?;
        self.keep_glob_pattern = pattern.to_string();
        Ok(())
    }

    pub fn set_glob_pattern(&mut self, pattern: &str) -> Result<(), ()> {
        self.glob_pattern = php::trim(pattern).to_string();
        guard_glob_pattern(&self.glob_pattern)
    }

    pub fn set_glob_pattern_from_file(&mut self, file: &str) -> Result<(), GlobFileError> {
        let path = self.cwd.join(file);
        if file.is_empty() || !path.is_file() {
            return Err(GlobFileError::NonExistent);
        }
        let content = std::fs::read(&path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let patterns: Vec<&str> = php::split_lines(&content)
            .into_iter()
            .map(php::trim)
            .filter(|l| !l.is_empty())
            .collect();
        let pattern = format!("{{{}}}*", patterns.join(","));
        self.set_glob_pattern(&pattern)
            .map_err(|()| GlobFileError::Invalid)
    }

    pub fn set_glob_pattern_from_preset(&mut self, preset: &str) -> Result<(), ()> {
        let globs = presets::by_language_name(preset).ok_or(())?;
        self.glob_pattern = presets::brace_pattern(&globs);
        Ok(())
    }

    // ----- license / readme / text auto ----------------------------------

    pub fn is_keep_license_enabled(&mut self) -> bool {
        if self.keep_license {
            return true;
        }
        self.has_mit_license()
    }

    fn has_mit_license(&mut self) -> bool {
        if let Some(cached) = self.mit_license {
            return cached;
        }
        let mut found = false;
        if let Ok(entries) = std::fs::read_dir(self.dir_path()) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !name.to_ascii_lowercase().starts_with("license") {
                    continue;
                }
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                if let Ok(bytes) = std::fs::read(&path) {
                    let content = String::from_utf8_lossy(&bytes);
                    if php::contains_ci(&content, "MIT License")
                        || php::contains_ci(&content, "License: MIT")
                    {
                        found = true;
                        break;
                    }
                }
            }
        }
        self.mit_license = Some(found);
        found
    }

    pub fn has_preceding_slashes(&self) -> bool {
        self.preceding_slash
    }

    pub fn has_text_autoconfiguration(&mut self) -> bool {
        if self.gitattributes_content().contains("text=auto eol=lf") {
            self.text_auto = true;
        }
        self.text_auto
    }

    // ----- globbing helpers -----------------------------------------------

    fn gitignored_patterns(&self) -> Vec<String> {
        let path = self.dir_path().join(".gitignore");
        let Ok(bytes) = std::fs::read(path) else {
            return Vec::new();
        };
        let content = String::from_utf8_lossy(&bytes);
        let mut out = Vec::new();
        for line in php::split_lines(&content) {
            let line = php::trim(line);
            if line.is_empty() || line.contains('#') {
                continue;
            }
            if line.starts_with('/') {
                out.push(php::drop_first_byte(line).to_string());
            }
            if line.ends_with('/') {
                out.push(php::drop_last_byte(line).to_string());
            }
            out.push(line.to_string());
        }
        out
    }

    /// `patternHasMatch()`.
    pub fn pattern_has_match(&self, pattern: &str) -> bool {
        let trimmed = php::trim(pattern);
        let glob_pattern = if trimmed.starts_with('/') {
            php::trim(php::drop_first_byte(pattern)).to_string()
        } else if trimmed.ends_with('/') {
            php::trim(php::drop_last_byte(pattern)).to_string()
        } else {
            format!("{{{trimmed}}}*")
        };
        !self.globber.glob(&glob_pattern).is_empty()
    }

    /// `file_exists($this->directory . '/' . $relative)` (relative to the cwd).
    fn file_exists_in_dir(&self, relative: &str) -> bool {
        self.cwd
            .join(format!("{}/{}", self.directory, relative))
            .exists()
    }

    // ----- classic analysis ------------------------------------------------

    /// `ClassicExportIgnoreAnalyser::collectExpectedExportIgnores()`.
    pub fn collect_expected(&mut self) -> Vec<String> {
        let mut ignored: Vec<String> = [".", "..", ".git", ".DS_Store"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        ignored.extend(self.gitignored_patterns());

        let base = self.dir_path();
        let mut expected: Vec<String> = Vec::new();
        for filename in self.globber.glob(&self.glob_pattern) {
            if ignored.contains(&filename) {
                continue;
            }
            if base.join(&filename).is_dir() {
                expected.push(format!("{filename}/"));
            } else {
                expected.push(filename);
            }
        }

        if self.has_gitattributes_file() {
            let preserved = self.present_to_preserve(&expected);
            expected.extend(preserved);
        }

        if !self.strict_order {
            php::sort_case(&mut expected);
        }
        if self.is_keep_license_enabled() {
            expected.retain(|e| !php::contains_ci(e, "license"));
        }
        if self.keep_readme {
            expected.retain(|e| !php::contains_ci(e, "readme"));
        }
        if !self.keep_glob_pattern.is_empty() {
            let excludes = glob_array(&self.keep_glob_pattern, &expected);
            expected.retain(|e| !excludes.contains(e));
        }
        php::unique(expected)
    }

    /// `getPresentExportIgnoresToPreserve()`.
    fn present_to_preserve(&mut self, expected: &[String]) -> Vec<String> {
        let content = self.gitattributes_content();
        if php::has_text_auto_directive(&content) {
            self.text_auto = true;
        }
        let basenames: Vec<&str> = expected.iter().map(|e| php::basename(e)).collect();
        let mut preserve = Vec::new();
        for line in php::split_lines(&content) {
            if !(line.contains("export-ignore")
                && !line.contains("-export-ignore")
                && !line.contains('#'))
            {
                continue;
            }
            let mut pattern = line.split("export-ignore").next().unwrap_or("");
            if pattern.starts_with('/') {
                pattern = php::drop_first_byte(pattern);
                self.preceding_slash = true;
            }
            let matches = self.pattern_has_match(pattern);
            let pattern = php::trim(pattern);
            if matches
                && !expected.iter().any(|e| e == pattern)
                && !basenames.contains(&pattern)
                && self.file_exists_in_dir(pattern)
            {
                preserve.push(pattern.to_string());
            }
        }
        preserve
    }

    /// `ClassicExportIgnoreAnalyser::getPresentExportIgnores()` before
    /// `array_unique()`.
    pub fn present_raw(
        &self,
        apply_glob: bool,
        content: Option<&str>,
        strict: bool,
    ) -> Vec<String> {
        let owned;
        let content = match content {
            Some(c) if !c.is_empty() => c,
            _ => {
                if !self.has_gitattributes_file() {
                    return Vec::new();
                }
                owned = self.gitattributes_content();
                owned.as_str()
            }
        };
        let mut out = Vec::new();
        for line in php::split_lines(content) {
            let Some(pos) = line.find("export-ignore") else {
                continue;
            };
            let before = &line[..pos];
            if before.is_empty() || php::rtrim(before).ends_with('-') {
                continue;
            }
            let mut entry = before;
            if self.pattern_has_match(php::trim(entry)) {
                if entry.starts_with('/') {
                    entry = php::drop_first_byte(entry);
                }
                out.push(php::trim(entry).to_string());
            } else if !apply_glob {
                out.push(php::trim(entry).to_string());
            }
        }
        if !strict {
            php::sort_case(&mut out);
        }
        out
    }

    pub fn present(&self, apply_glob: bool) -> Vec<String> {
        php::unique(self.present_raw(apply_glob, None, false))
    }

    /// Present export-ignores that match nothing in the package directory.
    pub fn stale_entries(&self) -> Vec<String> {
        let filtered = self.present(true);
        self.present(false)
            .into_iter()
            .filter(|e| !filtered.contains(e))
            .collect()
    }

    fn aligned(artifacts: &[String]) -> Vec<String> {
        let longest = artifacts.iter().map(String::len).max().unwrap_or(0);
        artifacts
            .iter()
            .map(|a| format!("{a}{}", " ".repeat(longest - a.len())))
            .collect()
    }

    fn by_directories_to_files(artifacts: &[String]) -> Vec<String> {
        let dirs = artifacts
            .iter()
            .filter(|a| a.find('/').is_some_and(|p| p > 0))
            .cloned();
        let files = artifacts.iter().filter(|a| !a.contains('/')).cloned();
        dirs.chain(files).collect()
    }

    /// `ClassicExportIgnoreAnalyser::hasCompleteExportIgnores()`.
    pub fn has_complete_classic(&mut self) -> bool {
        let mut expected = self.collect_expected();
        if expected.is_empty() || !self.has_gitattributes_file() {
            return false;
        }
        let mut actual = self.present(true);
        let mut stale = Vec::new();
        if self.stale {
            for entry in self.present(false) {
                // PHP checks file_exists() relative to the process cwd here.
                if entry.is_empty() || !self.cwd.join(&entry).exists() {
                    stale.push(entry);
                }
            }
        }
        if self.strict_alignment {
            expected = Self::aligned(&expected);
        }
        if self.stale {
            actual.extend(stale);
        }
        expected == actual
    }

    /// `ClassicExportIgnoreAnalyser::getGitattributesContentToBe()`.
    fn content_to_be(&mut self, postfix: Vec<String>) -> String {
        let collected = self.collect_expected();
        let postfix =
            if postfix.len() == 1 && postfix[0] == ".gitattributes" && collected.is_empty() {
                Vec::new()
            } else {
                postfix
            };
        let mut list = php::unique(collected.into_iter().chain(postfix).collect());
        php::sort_case(&mut list);
        if list.is_empty() {
            return String::new();
        }
        if !self.sort_dirs_to_files && (self.align || self.strict_alignment) {
            list = Self::aligned(&list);
        }
        if self.sort_dirs_to_files {
            list = Self::by_directories_to_files(&list);
        }
        if self.is_keep_license_enabled() {
            list.retain(|e| !php::contains_ci(e, "license"));
        }
        if self.keep_readme {
            list.retain(|e| !php::contains_ci(e, "readme"));
        }
        if !self.keep_glob_pattern.is_empty() {
            let excludes = glob_array(&self.keep_glob_pattern, &list);
            list.retain(|e| !excludes.contains(e));
        }
        let eol = self.preferred_eol;
        let mut content = list.join(&format!(" export-ignore{eol}")) + " export-ignore" + eol;
        if self.has_gitattributes_file() {
            let export_ignores = php::rtrim(&content).to_string();
            content = self.non_export_ignores_content();
            if content.contains(PLACEHOLDER) {
                content = content.replace(PLACEHOLDER, &export_ignores);
            } else {
                content = format!("{content}{}{export_ignores}", self.preferred_eol.repeat(2));
            }
        } else {
            content = format!(
                "* text=auto eol=lf{}{content}",
                self.preferred_eol.repeat(2)
            );
        }
        content
    }

    /// `getPresentNonExportIgnoresContent()`.
    fn non_export_ignores_content(&mut self) -> String {
        if !self.has_gitattributes_file() {
            return String::new();
        }
        let content = self.gitattributes_content();
        let eol = php::detect_eol(&content);
        self.preferred_eol = eol;
        let lines = php::split_lines(&content);
        if self.group_non_export_ignores {
            let kept: Vec<&str> = lines
                .into_iter()
                .filter(|l| !l.contains("export-ignore") || l.contains('#'))
                .map(php::trim)
                .collect();
            let collapsed = collapse_and_trim_blank_lines(&kept);
            if collapsed.is_empty() {
                return PLACEHOLDER.to_string();
            }
            return collapsed.join(eol);
        }
        let mut kept: Vec<&str> = Vec::new();
        let mut placeholder_set = false;
        for line in lines {
            if !line.contains("export-ignore") || line.contains('#') {
                kept.push(php::trim(line));
            } else if !placeholder_set {
                placeholder_set = true;
                kept.push(PLACEHOLDER);
            }
        }
        kept.join(eol)
    }

    // ----- Analyser facade -------------------------------------------------

    pub fn uses_negated(content: &str) -> bool {
        php::split_lines(content)
            .into_iter()
            .any(|l| php::trim(l) == "* export-ignore")
    }

    pub fn file_uses_negated(&self) -> bool {
        if !self.has_gitattributes_file() {
            return false;
        }
        Self::uses_negated(&self.gitattributes_content())
    }

    /// `Analyser::getExpectedGitattributesContent()`.
    pub fn expected_content(&mut self, flavour: Flavour) -> Result<String, LpvError> {
        let mut postfix = Vec::new();
        if !self.has_gitattributes_file() && flavour == Flavour::Classic {
            postfix.push(".gitattributes".to_string());
        }
        if self.negated_mode || flavour == Flavour::Negated || self.file_uses_negated() {
            self.switch_to_negated();
            return self.negated_content_to_be();
        }
        Ok(self.content_to_be(postfix))
    }

    /// `Analyser::hasCompleteExportIgnores()`.
    pub fn has_complete(&mut self) -> bool {
        if self.file_uses_negated() {
            return self.negated_has_complete();
        }
        self.has_complete_classic()
    }

    /// `Analyser::hasCompleteExportIgnoresFromString()`.
    pub fn has_complete_from_string(&mut self, content: &str) -> bool {
        if Self::uses_negated(content) {
            return !self.negated_present(true, Some(content)).is_empty();
        }
        let mut expected = self.collect_expected();
        let strict = self.strict_order;
        let raw = self.present_raw(true, Some(content), strict);
        let had_dupes = php::has_duplicates(&raw);
        let mut present = php::unique(raw);
        php::sort_case(&mut expected);
        if strict {
            return !had_dupes && expected == present;
        }
        php::sort_case(&mut present);
        expected == present
    }

    // ----- negated generation ----------------------------------------------

    /// lpv replaces its classic analyser with a fresh `NegatedExportIgnoreAnalyser`
    /// built from the `ExportIgnoreConfiguration` only (directory, glob pattern,
    /// sort/keep/align/strict-order settings). Everything else (strict
    /// alignment, grouping, stale comparison, detected EOL, ...) starts over.
    fn switch_to_negated(&mut self) {
        if self.negated_mode {
            return;
        }
        self.negated_mode = true;
        self.strict_alignment = false;
        self.group_non_export_ignores = false;
        self.stale = false;
        self.preceding_slash = false;
        self.text_auto = false;
        self.mit_license = None;
        self.preferred_eol = "\n";
    }

    /// The process working directory as lpv sees it (it may have chdir()ed).
    pub fn process_cwd(&self) -> &Path {
        &self.cwd
    }

    fn fs_entry(&self, entry: &str) -> PathBuf {
        self.cwd.join(format!("{}/{}", self.directory, entry))
    }

    /// `NegatedExportIgnoreAnalyser::collectExpectedExportIgnores()`: every
    /// non-hidden top-level entry not matched by the glob pattern.
    fn negated_collect_expected(&mut self) -> Result<Vec<String>, LpvError> {
        // The PHP code chdir()s into the package directory as a side effect, so
        // every later relative path (including the directory itself) resolves
        // against it.
        self.cwd = self.dir_path();
        let mut matches = self.globber.glob(&self.glob_pattern);
        if !matches.is_empty() {
            if !self.keep_license && std::fs::read_dir(self.dir_path()).is_err() {
                // hasMitLicense(): new DirectoryIterator($this->directory).
                return Err(LpvError::Uncaught {
                    file: "AbstractExportIgnoreAnalyser.php",
                    line: 360,
                    message: format!(
                        "DirectoryIterator::__construct({}): Failed to open directory: No such file or directory",
                        self.directory
                    ),
                });
            }
            let keep_license = self.is_keep_license_enabled();
            let keep_readme = self.keep_readme;
            matches.retain(|f| {
                !(keep_license && php::contains_ci(f, "license"))
                    && !(keep_readme && php::contains_ci(f, "readme"))
            });
        }
        let all = self.globber.glob("{*}");
        Ok(all.into_iter().filter(|f| !matches.contains(f)).collect())
    }

    /// `buildExportIgnoresToNegate()`.
    fn negated_entries(&mut self) -> Result<Vec<String>, LpvError> {
        let mut entries = self.negated_collect_expected()?;
        php::sort_case(&mut entries);
        let entries: Vec<String> = entries
            .into_iter()
            .map(|e| {
                if self.fs_entry(&e).is_dir() {
                    format!("{e}/")
                } else {
                    e
                }
            })
            .collect();
        let globbed: Vec<String> = entries
            .iter()
            .filter(|e| self.should_be_globbed(e))
            .map(|e| format!("{e}**"))
            .collect();
        let entries = php::unique(entries.into_iter().chain(globbed).collect());
        let mut expanded = Vec::new();
        for entry in entries {
            let is_bin = entry.ends_with("bin/");
            expanded.push(entry.clone());
            if !is_bin || php::trim(&self.directory).is_empty() {
                continue;
            }
            if let Ok(dir) = std::fs::read_dir(self.fs_entry(&entry)) {
                for file in dir.flatten() {
                    if file.path().is_dir() {
                        continue;
                    }
                    expanded.push(format!("{entry}{}", file.file_name().to_string_lossy()));
                }
            }
        }
        let mut entries = php::unique(expanded);
        php::sort_case(&mut entries);
        Ok(entries)
    }

    /// `shouldBeGlobed()`: directories with content (a `bin/` needs two entries).
    fn should_be_globbed(&self, entry: &str) -> bool {
        if !entry.ends_with('/') {
            return false;
        }
        let count = std::fs::read_dir(self.fs_entry(entry))
            .map(|d| d.flatten().count())
            .unwrap_or(0);
        if entry.ends_with("bin/") {
            count >= 2
        } else {
            count > 0
        }
    }

    /// `NegatedExportIgnoreAnalyser::getGitattributesContentToBe()`.
    fn negated_content_to_be(&mut self) -> Result<String, LpvError> {
        let mut entries = self.negated_entries()?;
        if entries.is_empty() {
            return Ok(String::new());
        }
        if !self.sort_dirs_to_files && (self.align || self.strict_alignment) {
            entries = Self::aligned(&entries);
        }
        if self.sort_dirs_to_files {
            entries = Self::by_directories_to_files(&entries);
        }
        let eol = self.preferred_eol;
        let lines: Vec<String> = if self.align {
            Self::aligned(&entries)
                .into_iter()
                .map(|e| format!("{e} -export-ignore"))
                .collect()
        } else {
            entries
                .iter()
                .map(|e| format!("{e} -export-ignore"))
                .collect()
        };
        let content = format!("* export-ignore{eol}{eol}{}{eol}", lines.join(eol));
        if self.has_gitattributes_file() {
            let export_ignores = php::rtrim(&content).to_string();
            let existing = self.non_export_ignores_content();
            if existing.contains(PLACEHOLDER) {
                return Ok(existing.replace(PLACEHOLDER, &export_ignores));
            }
            return Ok(format!(
                "{existing}{}{export_ignores}",
                self.preferred_eol.repeat(2)
            ));
        }
        Ok(format!(
            "* text=auto eol=lf{}{content}",
            self.preferred_eol.repeat(2)
        ))
    }

    // ----- negated validation ----------------------------------------------

    fn negated_present(&self, apply_glob: bool, content: Option<&str>) -> Vec<String> {
        let owned;
        let content = match content {
            Some(c) if !c.is_empty() => c,
            _ => {
                if !self.has_gitattributes_file() {
                    return Vec::new();
                }
                owned = self.gitattributes_content();
                owned.as_str()
            }
        };
        let mut out = Vec::new();
        for line in php::split_lines(content) {
            if !line.contains("-export-ignore") || php::ltrim(line).starts_with('#') {
                continue;
            }
            let pattern = line.split("-export-ignore").next().unwrap_or("");
            let pattern = php::trim(pattern).trim_start_matches('/');
            if pattern.is_empty() {
                continue;
            }
            if !apply_glob || self.pattern_has_match(pattern) {
                out.push(pattern.to_string());
            }
        }
        php::sort_case(&mut out);
        php::unique(out)
    }

    fn negated_has_complete(&self) -> bool {
        if !self.has_gitattributes_file() {
            return false;
        }
        let present = self.negated_present(true, None);
        if present.is_empty() {
            return false;
        }
        if self.stale {
            let all = self.negated_present(false, None);
            if all.iter().any(|e| !present.contains(e)) {
                return false;
            }
        }
        has_double_star_for_directories(&present)
    }
}

fn has_double_star_for_directories(entries: &[String]) -> bool {
    for entry in entries {
        if !entry.ends_with('/') {
            continue;
        }
        let directory = entry.trim_end_matches('/');
        let expected = format!("{directory}/**");
        let prefix = format!("{directory}/");
        let has_direct_file = entries
            .iter()
            .any(|c| c.starts_with(&prefix) && !c.ends_with('/') && *c != expected);
        if has_direct_file {
            continue;
        }
        if !entries.contains(&expected) {
            return false;
        }
    }
    true
}

fn collapse_and_trim_blank_lines<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    let mut collapsed: Vec<&str> = Vec::new();
    let mut prev_blank = false;
    for &line in lines {
        if line.is_empty() {
            if !prev_blank {
                collapsed.push(line);
            }
            prev_blank = true;
        } else {
            collapsed.push(line);
            prev_blank = false;
        }
    }
    while collapsed.first().is_some_and(|l| l.is_empty()) {
        collapsed.remove(0);
    }
    while collapsed.last().is_some_and(|l| l.is_empty()) {
        collapsed.pop();
    }
    collapsed
}

/// `Glob::globArray()`: for each comma-separated part, only the *first*
/// matching element is excluded (a quirk of the original we keep).
pub fn glob_array(pattern: &str, values: &[String]) -> Vec<String> {
    let stripped: String = pattern.chars().filter(|c| *c != '{' && *c != '}').collect();
    let mut excludes = Vec::new();
    for part in stripped.split(',') {
        if let Some(first) = values.iter().find(|v| glob::fnmatch_casefold(part, v)) {
            excludes.push(first.clone());
        }
    }
    excludes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_pattern_guard() {
        assert!(guard_glob_pattern("{.*,*.md}").is_ok());
        assert!(guard_glob_pattern("{.*,*.md}*").is_ok());
        assert!(guard_glob_pattern("{.*}").is_err());
        assert!(guard_glob_pattern("1").is_err());
        assert!(guard_glob_pattern("").is_err());
    }

    #[test]
    fn glob_array_excludes_first_match_per_part() {
        let values: Vec<String> = ["LICENSE.md", "license.txt", "docs/", "README.md"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            glob_array("{LICENSE.*,docs*}", &values),
            vec!["LICENSE.md".to_string(), "docs/".to_string()]
        );
    }

    #[test]
    fn double_star_rule() {
        let v: Vec<String> = ["src/", "src/**", "composer.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(has_double_star_for_directories(&v));
        let v: Vec<String> = ["src/", "composer.json"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(!has_double_star_for_directories(&v));
    }
}
