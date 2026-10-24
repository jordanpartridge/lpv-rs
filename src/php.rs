//! Small, faithful re-implementations of the PHP built-ins whose exact
//! semantics the original lean-package-validator relies on.

use std::cmp::Ordering;

/// Characters stripped by PHP's `trim()` family by default.
const PHP_WS: &[char] = &[' ', '\t', '\n', '\r', '\0', '\x0B'];

pub fn trim(s: &str) -> &str {
    s.trim_matches(PHP_WS)
}

pub fn rtrim(s: &str) -> &str {
    s.trim_end_matches(PHP_WS)
}

pub fn ltrim(s: &str) -> &str {
    s.trim_start_matches(PHP_WS)
}

/// `zend_binary_strcasecmp` in the C locale (ASCII lower-casing, then length).
pub fn cmp_case(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    for (x, y) in a.iter().zip(b.iter()) {
        let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
        if x != y {
            return x.cmp(&y);
        }
    }
    a.len().cmp(&b.len())
}

/// `sort($array, SORT_STRING | SORT_FLAG_CASE)` (stable since PHP 8.0).
pub fn sort_case(v: &mut [String]) {
    v.sort_by(|a, b| cmp_case(a, b));
}

/// `array_values(array_unique($array))` keeping the first occurrence.
pub fn unique(v: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(v.len());
    for item in v {
        if !out.contains(&item) {
            out.push(item);
        }
    }
    out
}

/// Returns true when `array_unique` would have removed at least one element,
/// i.e. when the PHP array would end up with "holes" in its keys.
pub fn has_duplicates(v: &[String]) -> bool {
    v.iter().enumerate().any(|(i, item)| v[..i].contains(item))
}

/// `preg_split('/\r\n|\r|\n/', $content)`.
pub fn split_lines(content: &str) -> Vec<&str> {
    let bytes = content.as_bytes();
    let mut out = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' => {
                out.push(&content[start..i]);
                if bytes.get(i + 1) == Some(&b'\n') {
                    i += 1;
                }
                start = i + 1;
            }
            b'\n' => {
                out.push(&content[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(&content[start..]);
    out
}

/// `Str::detectEol()` of the original: the most frequent EOL wins, later
/// candidates win ties.
pub fn detect_eol(content: &str) -> &'static str {
    let mut max = 0usize;
    let mut preferred = "\n";
    for eol in ["\n", "\r", "\n\r", "\r\n"] {
        let count = content.matches(eol).count();
        if count < max {
            continue;
        }
        max = count;
        preferred = eol;
    }
    preferred
}

/// PHP's `basename()` for '/'-separated paths without a suffix argument.
pub fn basename(path: &str) -> &str {
    let trimmed = path.trim_end_matches('/');
    match trimmed.rfind('/') {
        Some(idx) => &trimmed[idx + 1..],
        None => trimmed,
    }
}

/// `substr($s, 1)` for strings whose first byte is ASCII.
pub fn drop_first_byte(s: &str) -> &str {
    s.get(1..).unwrap_or("")
}

/// `substr($s, 0, -1)` for strings whose last byte is ASCII.
pub fn drop_last_byte(s: &str) -> &str {
    if s.is_empty() {
        return "";
    }
    s.get(..s.len() - 1).unwrap_or("")
}

/// `substr($s, 1, -1)`.
pub fn inner(s: &str) -> &str {
    if s.len() < 2 {
        return "";
    }
    s.get(1..s.len() - 1).unwrap_or("")
}

/// `preg_match('/(License.*)/i', $s)`.
pub fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

/// `preg_match("/(\*\h*)(text\h*)(=\h*auto)/", $content)`.
pub fn has_text_auto_directive(content: &str) -> bool {
    let b = content.as_bytes();
    let is_h = |c: u8| c == b' ' || c == b'\t' || c == 0xA0;
    for start in 0..b.len() {
        if b[start] != b'*' {
            continue;
        }
        let mut i = start + 1;
        while i < b.len() && is_h(b[i]) {
            i += 1;
        }
        if !b[i..].starts_with(b"text") {
            continue;
        }
        i += 4;
        while i < b.len() && is_h(b[i]) {
            i += 1;
        }
        if i >= b.len() || b[i] != b'=' {
            continue;
        }
        i += 1;
        while i < b.len() && is_h(b[i]) {
            i += 1;
        }
        if b[i..].starts_with(b"auto") {
            return true;
        }
    }
    false
}

/// A tiny JSON value model that serialises exactly like PHP's
/// `json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES)`.
#[derive(Debug, Clone)]
pub enum PhpJson {
    Bool(bool),
    Str(String),
    List(Vec<String>),
}

pub fn php_json_encode(entries: &[(&str, PhpJson)]) -> String {
    if entries.is_empty() {
        return "{}".to_string();
    }
    let mut out = String::from("{\n");
    for (i, (key, value)) in entries.iter().enumerate() {
        out.push_str("    ");
        out.push_str(&php_json_string(key));
        out.push_str(": ");
        match value {
            PhpJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            PhpJson::Str(s) => out.push_str(&php_json_string(s)),
            PhpJson::List(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                } else {
                    out.push_str("[\n");
                    for (j, item) in items.iter().enumerate() {
                        out.push_str("        ");
                        out.push_str(&php_json_string(item));
                        if j + 1 < items.len() {
                            out.push(',');
                        }
                        out.push('\n');
                    }
                    out.push_str("    ]");
                }
            }
        }
        if i + 1 < entries.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push('}');
    out
}

fn php_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || (c as u32) >= 0x80 => {
                let mut buf = [0u16; 2];
                for unit in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{:04x}", unit));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sort_matches_php() {
        let mut v: Vec<String> = [
            "tests/",
            "README.md",
            ".gitattributes",
            "phpstan.neon.dist",
            "phpstan-baseline.neon",
            "CHANGELOG.md",
            "composer.lock",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        sort_case(&mut v);
        assert_eq!(
            v,
            vec![
                ".gitattributes",
                "CHANGELOG.md",
                "composer.lock",
                "phpstan-baseline.neon",
                "phpstan.neon.dist",
                "README.md",
                "tests/"
            ]
        );
    }

    #[test]
    fn eol_detection() {
        assert_eq!(detect_eol("a\nb\n"), "\n");
        assert_eq!(detect_eol("a\r\nb\r\n"), "\r\n");
        assert_eq!(detect_eol("no newline"), "\r\n");
    }

    #[test]
    fn line_split() {
        assert_eq!(split_lines("a\r\nb\rc\n"), vec!["a", "b", "c", ""]);
    }

    #[test]
    fn basenames() {
        assert_eq!(basename("tests/"), "tests");
        assert_eq!(basename(".claude/settings.json"), "settings.json");
    }

    #[test]
    fn text_auto() {
        assert!(has_text_auto_directive("* text=auto\n"));
        assert!(has_text_auto_directive("*\ttext = auto eol=lf"));
        assert!(!has_text_auto_directive("*.php text eol=lf"));
    }

    #[test]
    fn json_like_php() {
        let s = php_json_encode(&[
            ("command", PhpJson::Str("validate".into())),
            ("valid", PhpJson::Bool(false)),
            ("list", PhpJson::List(vec!["a/b".into()])),
            ("empty", PhpJson::List(vec![])),
        ]);
        assert_eq!(
            s,
            "{\n    \"command\": \"validate\",\n    \"valid\": false,\n    \"list\": [\n        \"a/b\"\n    ],\n    \"empty\": []\n}"
        );
        assert_eq!(php_json_string("é"), "\"\\u00e9\"");
    }
}
