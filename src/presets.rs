//! Language presets: the default glob patterns of artifacts that should be
//! export-ignored, copied verbatim from lean-package-validator v6.3.2.

use crate::php;

const AI_GLOB: &[&str] = &[
    "AGENT.md",
    "AGENTS.md",
    "CLAUDE.md",
    "GEMINI.md",
    "AI.md",
    "AIDER.md",
    "CURSOR.md",
    "COPILOT.md",
    "CODEX.md",
    "QWEN.md",
    "WINDSURF.md",
    ".aiassistant",
    ".aider*",
    ".cursor",
    ".cursor/**",
    ".github/copilot-instructions.md",
    ".windsurf",
    ".windsurf/**",
    ".claude",
    ".claude/**",
    ".gemini",
    ".gemini/**",
    ".codex",
    ".codex/**",
    "llms.txt",
    "llms-full.txt",
];

const COMMON_GLOB: &[&str] = &[
    ".*",
    "*.txt",
    "*.{md,MD}",
    "*.rst",
    "*.toml",
    "*.xml",
    "*.yml",
    "*.dist.*",
    ".githooks",
    "*.dist",
    "{B,b}uild*",
    "{B,b}enchmark*",
    "{D,d}ist",
    "{D,d}oc*",
    "{A,a}rt*",
    "{A,a}sset*",
    "{T,t}ool*",
    "{T,t}est*",
    "{S,s}pec*",
    "{E,e}xample*",
    "LICENSE",
    "{M,m}ake",
    "*.{png,gif,jpeg,jpg,webp}",
];

const PHP_GLOB: &[&str] = &[
    "*.lock",
    "castor.php",
    "phpunit*",
    "appveyor.yml",
    "box.json",
    "composer-dependency-analyser*",
    "collision-detector*",
    "captainhook.json",
    "extension.neon",
    "peck.json",
    "infection*",
    "phpstan*",
    "sonar*",
    "rector*",
    "phpkg.con*",
    "package*",
    "pint.{json,php}",
    "renovate.json",
    "*debugbar.json",
    "phpinsights*",
    "ecs*",
    "RMT",
    "{{M,m}ake,{B,b}ox,{V,v}agrant,{P,p}hulp}file",
    "vendor*",
    "version-bumper.php",
];

const RUST_GLOB: &[&str] = &["benches/**", ".rustfmt.toml", ".clippy.toml"];

const PYTHON_GLOB: &[&str] = &[
    "*.py[cod]",
    "setup.*",
    "requirements*.txt",
    "Pipfile",
    "Pipfile.lock",
];

const GO_GLOB: &[&str] = &[
    "go.*",
    "cmd/**",
    "pkg/**",
    "internal/**",
    "*.go",
    "*_test.go",
];

const JAVASCRIPT_GLOB: &[&str] = &[
    "package.json",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "bun.lockb",
    "__tests__/**",
    "*.{test,spec}.{js,ts,jsx,tsx}",
    "tsconfig.json",
    "tsconfig.*.json",
    ".eslintrc*",
    ".prettierrc*",
    ".babelrc*",
    "vite.config.*",
    "webpack.config.*",
    "rollup.config.*",
    "jest.config.*",
    "node_modules",
];

/// Class-name stems of the presets shipped with v6.3.2 (Common excluded).
const AVAILABLE: &[&str] = &["Go", "JavaScript", "Php", "Python", "Rust"];

fn preset_glob(name: &str) -> Option<Vec<String>> {
    let specific: &[&str] = match name {
        "Go" => GO_GLOB,
        "JavaScript" => JAVASCRIPT_GLOB,
        "Php" => PHP_GLOB,
        "Python" => PYTHON_GLOB,
        "Rust" => RUST_GLOB,
        _ => return None,
    };
    let merged: Vec<String> = AI_GLOB
        .iter()
        .chain(COMMON_GLOB.iter())
        .chain(specific.iter())
        .map(|s| s.to_string())
        .collect();
    let mut globs = php::unique(merged);
    php::sort_case(&mut globs);
    Some(globs)
}

/// `Finder::getPresetGlobByLanguageName()`: `ucfirst(strtolower($name))`
/// must equal a preset class stem, so e.g. "javascript" never resolves to
/// the `JavaScript` preset (a quirk of the original we keep).
pub fn by_language_name(name: &str) -> Option<Vec<String>> {
    let lower = name.to_ascii_lowercase();
    let mut chars = lower.chars();
    let normalised = match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    };
    if !AVAILABLE.contains(&normalised.as_str()) {
        return None;
    }
    preset_glob(&normalised)
}

/// The brace glob pattern built from a preset: `'{' . implode(',', $globs) . '}*'`.
pub fn brace_pattern(globs: &[String]) -> String {
    format!("{{{}}}*", globs.join(","))
}

pub fn default_glob_pattern() -> String {
    brace_pattern(&preset_glob("Php").unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_lookup_mirrors_php_ucfirst_strtolower() {
        assert!(by_language_name("php").is_some());
        assert!(by_language_name("RUST").is_some());
        assert!(by_language_name("javascript").is_none());
        assert!(by_language_name("JavaScript").is_none());
        assert!(by_language_name("foo").is_none());
    }

    #[test]
    fn default_pattern_matches_reference_byte_for_byte() {
        // Captured from lean-package-validator 6.3.2 (Finder::getDefaultPreset()).
        let reference = "{*.dist,*.dist.*,*.lock,*.rst,*.toml,*.txt,*.xml,*.yml,*.{md,MD},*.{png,gif,jpeg,jpg,webp},*debugbar.json,.*,.aiassistant,.aider*,.claude,.claude/**,.codex,.codex/**,.cursor,.cursor/**,.gemini,.gemini/**,.githooks,.github/copilot-instructions.md,.windsurf,.windsurf/**,AGENT.md,AGENTS.md,AI.md,AIDER.md,appveyor.yml,box.json,captainhook.json,castor.php,CLAUDE.md,CODEX.md,collision-detector*,composer-dependency-analyser*,COPILOT.md,CURSOR.md,ecs*,extension.neon,GEMINI.md,infection*,LICENSE,llms-full.txt,llms.txt,package*,peck.json,phpinsights*,phpkg.con*,phpstan*,phpunit*,pint.{json,php},QWEN.md,rector*,renovate.json,RMT,sonar*,vendor*,version-bumper.php,WINDSURF.md,{A,a}rt*,{A,a}sset*,{B,b}enchmark*,{B,b}uild*,{D,d}ist,{D,d}oc*,{E,e}xample*,{M,m}ake,{S,s}pec*,{T,t}est*,{T,t}ool*,{{M,m}ake,{B,b}ox,{V,v}agrant,{P,p}hulp}file}*";
        assert_eq!(default_glob_pattern(), reference);
    }

    #[test]
    fn rust_preset_matches_reference() {
        let reference = "{*.dist,*.dist.*,*.rst,*.toml,*.txt,*.xml,*.yml,*.{md,MD},*.{png,gif,jpeg,jpg,webp},.*,.aiassistant,.aider*,.claude,.claude/**,.clippy.toml,.codex,.codex/**,.cursor,.cursor/**,.gemini,.gemini/**,.githooks,.github/copilot-instructions.md,.rustfmt.toml,.windsurf,.windsurf/**,AGENT.md,AGENTS.md,AI.md,AIDER.md,benches/**,CLAUDE.md,CODEX.md,COPILOT.md,CURSOR.md,GEMINI.md,LICENSE,llms-full.txt,llms.txt,QWEN.md,WINDSURF.md,{A,a}rt*,{A,a}sset*,{B,b}enchmark*,{B,b}uild*,{D,d}ist,{D,d}oc*,{E,e}xample*,{M,m}ake,{S,s}pec*,{T,t}est*,{T,t}ool*}*";
        assert_eq!(
            brace_pattern(&by_language_name("rust").unwrap_or_default()),
            reference
        );
    }
}
