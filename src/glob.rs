//! A port of the BSD-style `glob(3)` (with `GLOB_BRACE`) that PHP 8.5 bundles
//! and that lean-package-validator relies on, plus the glibc-style
//! `fnmatch(3)` PHP uses for `Glob::globArray()`.
//!
//! Matching semantics, result ordering (each brace alternative sorted on its
//! own, alternatives concatenated in pattern order) and the "initial dot must
//! be matched literally" rule are all preserved, because the reference
//! implementation's output order depends on them.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const PROTECT: u16 = 0x100;
const LBRACE: u16 = b'{' as u16;
const RBRACE: u16 = b'}' as u16;
const LBRACKET: u16 = b'[' as u16;
const RBRACKET: u16 = b']' as u16;
const COMMA: u16 = b',' as u16;
const COLON: u16 = b':' as u16;
const NOT: u16 = b'!' as u16;
const RANGE: u16 = b'-' as u16;
const STAR: u16 = b'*' as u16;
const QUESTION: u16 = b'?' as u16;
const QUOTE: u8 = b'\\';
const EOS: u16 = 0;

const CLASSES: [&str; 12] = [
    "alnum", "alpha", "blank", "cntrl", "digit", "graph", "lower", "print", "punct", "space",
    "upper", "xdigit",
];

#[derive(Debug, Clone)]
enum Item {
    Ch(u8),
    Range(u8, u8),
    Class(usize),
}

#[derive(Debug, Clone)]
enum Tok {
    Ch(u8),
    All,
    One,
    Set { negate: bool, items: Vec<Item> },
}

fn class_matches(idx: usize, c: u8) -> bool {
    match idx {
        0 => c.is_ascii_alphanumeric(),
        1 => c.is_ascii_alphabetic(),
        2 => c == b' ' || c == b'\t',
        3 => c.is_ascii_control(),
        4 => c.is_ascii_digit(),
        5 => c.is_ascii_graphic(),
        6 => c.is_ascii_lowercase(),
        7 => c.is_ascii_graphic() || c == b' ',
        8 => c.is_ascii_punctuation(),
        9 => matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c),
        10 => c.is_ascii_uppercase(),
        11 => c.is_ascii_hexdigit(),
        _ => false,
    }
}

/// Filesystem globber rooted at a base directory (the PHP code `chdir()`s
/// into the package directory before calling `glob()`).
pub struct Globber {
    base: PathBuf,
    cache: RefCell<HashMap<Vec<u8>, Option<Vec<String>>>>,
}

impl Globber {
    pub fn new(base: &Path) -> Self {
        Self {
            base: base.to_path_buf(),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// `glob($pattern, GLOB_BRACE)`.
    pub fn glob(&self, pattern: &str) -> Vec<String> {
        let mut chars: Vec<u16> = Vec::with_capacity(pattern.len());
        let bytes = pattern.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i];
            i += 1;
            if c == QUOTE {
                match bytes.get(i) {
                    Some(&next) => {
                        chars.push(u16::from(next) | PROTECT);
                        i += 1;
                    }
                    None => chars.push(u16::from(QUOTE) | PROTECT),
                }
            } else {
                chars.push(u16::from(c));
            }
        }
        let mut out = Vec::new();
        self.globexp1(&chars, &mut out);
        out
    }

    fn globexp1(&self, pattern: &[u16], out: &mut Vec<String>) {
        if pattern == [LBRACE, RBRACE] {
            self.glob0(pattern, out);
            return;
        }
        match pattern.iter().position(|&c| c == LBRACE) {
            Some(ptr) => self.globexp2(ptr, pattern, out),
            None => self.glob0(pattern, out),
        }
    }

    fn globexp2(&self, ptr: usize, pattern: &[u16], out: &mut Vec<String>) {
        let len = pattern.len();
        let prefix = &pattern[..ptr];
        let start = ptr + 1;

        // Find the balanced closing brace.
        let mut depth = 0i64;
        let mut pe = start;
        while pe < len {
            let c = pattern[pe];
            if c == LBRACKET {
                let pm = pe;
                pe += 1;
                while pe < len && pattern[pe] != RBRACKET {
                    pe += 1;
                }
                if pe >= len {
                    pe = pm;
                }
            } else if c == LBRACE {
                depth += 1;
            } else if c == RBRACE {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            pe += 1;
        }

        if depth != 0 || pe >= len {
            self.glob0(pattern, out);
            return;
        }

        let rest = &pattern[pe + 1..];
        let mut depth = 0i64;
        let mut pl = start;
        let mut pm = start;
        while pm <= pe {
            let c = pattern[pm];
            if c == LBRACKET {
                let save = pm;
                pm += 1;
                while pm < len && pattern[pm] != RBRACKET {
                    pm += 1;
                }
                if pm >= len {
                    pm = save;
                }
            } else if c == LBRACE {
                depth += 1;
            } else if c == RBRACE && depth > 0 {
                depth -= 1;
            } else if c == RBRACE || (c == COMMA && depth == 0) {
                let mut patbuf: Vec<u16> = Vec::with_capacity(len);
                patbuf.extend_from_slice(prefix);
                patbuf.extend_from_slice(&pattern[pl..pm]);
                patbuf.extend_from_slice(rest);
                self.globexp1(&patbuf, out);
                pl = pm + 1;
            }
            pm += 1;
        }
    }

    fn glob0(&self, pattern: &[u16], out: &mut Vec<String>) {
        let Some(toks) = compile(pattern) else {
            return;
        };
        let old = out.len();
        self.walk(Vec::new(), &toks, out);
        out[old..].sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    }

    fn resolve(&self, pathbuf: &[u8]) -> PathBuf {
        let rel = String::from_utf8_lossy(pathbuf).into_owned();
        if rel.is_empty() {
            self.base.clone()
        } else {
            self.base.join(rel)
        }
    }

    fn lstat_ok(&self, pathbuf: &[u8]) -> bool {
        if pathbuf.is_empty() {
            return false;
        }
        std::fs::symlink_metadata(self.resolve(pathbuf)).is_ok()
    }

    fn read_dir(&self, pathbuf: &[u8]) -> Option<Vec<String>> {
        if let Some(hit) = self.cache.borrow().get(pathbuf) {
            return hit.clone();
        }
        let listing = std::fs::read_dir(self.resolve(pathbuf)).ok().map(|rd| {
            let mut names = vec![".".to_string(), "..".to_string()];
            names.extend(
                rd.filter_map(Result::ok)
                    .map(|e| e.file_name().to_string_lossy().into_owned()),
            );
            names
        });
        self.cache
            .borrow_mut()
            .insert(pathbuf.to_vec(), listing.clone());
        listing
    }

    fn walk(&self, mut pathbuf: Vec<u8>, toks: &[Tok], out: &mut Vec<String>) {
        let mut rest = toks;
        loop {
            if rest.is_empty() {
                if self.lstat_ok(&pathbuf) {
                    out.push(String::from_utf8_lossy(&pathbuf).into_owned());
                }
                return;
            }
            let seg_end = rest
                .iter()
                .position(|t| matches!(t, Tok::Ch(b'/')))
                .unwrap_or(rest.len());
            let seg = &rest[..seg_end];
            let anymeta = seg.iter().any(|t| !matches!(t, Tok::Ch(_)));
            if !anymeta {
                for t in seg {
                    if let Tok::Ch(c) = t {
                        pathbuf.push(*c);
                    }
                }
                rest = &rest[seg_end..];
                while let Some(Tok::Ch(b'/')) = rest.first() {
                    pathbuf.push(b'/');
                    rest = &rest[1..];
                }
                continue;
            }

            let Some(names) = self.read_dir(&pathbuf) else {
                return;
            };
            let literal_dot = matches!(seg.first(), Some(Tok::Ch(b'.')));
            let after = &rest[seg_end..];
            for name in names {
                if name.starts_with('.') && !literal_dot {
                    continue;
                }
                if tok_match(name.as_bytes(), seg) {
                    let mut next = pathbuf.clone();
                    next.extend_from_slice(name.as_bytes());
                    self.walk(next, after, out);
                }
            }
            return;
        }
    }
}

fn get(pattern: &[u16], i: usize) -> u16 {
    pattern.get(i).copied().unwrap_or(EOS)
}

fn ch(c: u16) -> u8 {
    (c & 0xff) as u8
}

fn compile(pattern: &[u16]) -> Option<Vec<Tok>> {
    let n = pattern.len();
    let mut toks: Vec<Tok> = Vec::with_capacity(n);
    let mut q = 0usize;
    while q < n {
        let c = pattern[q];
        q += 1;
        match c {
            LBRACKET => {
                let c2 = get(pattern, q);
                if c2 == NOT {
                    q += 1;
                }
                let has_close = q < n && pattern[q + 1..].contains(&RBRACKET);
                if q >= n || !has_close {
                    toks.push(Tok::Ch(b'['));
                    if c2 == NOT {
                        q -= 1;
                    }
                    continue;
                }
                let negate = c2 == NOT;
                let mut items: Vec<Item> = Vec::new();
                let mut c = get(pattern, q);
                q += 1;
                loop {
                    if c == LBRACKET && get(pattern, q) == COLON {
                        let mut err;
                        loop {
                            err = charclass(pattern, &mut q, &mut items);
                            if err != 0 {
                                break;
                            }
                            c = get(pattern, q);
                            q += 1;
                            if !(c == LBRACKET && get(pattern, q) == COLON) {
                                break;
                            }
                        }
                        if err == -1 {
                            return None;
                        }
                        if c == RBRACKET {
                            break;
                        }
                    }
                    items.push(Item::Ch(ch(c)));
                    if get(pattern, q) == RANGE && get(pattern, q + 1) != RBRACKET {
                        let end = get(pattern, q + 1);
                        items.pop();
                        items.push(Item::Range(ch(c), ch(end)));
                        q += 2;
                    }
                    c = get(pattern, q);
                    q += 1;
                    if c == RBRACKET || q > n {
                        break;
                    }
                }
                toks.push(Tok::Set { negate, items });
            }
            QUESTION => toks.push(Tok::One),
            STAR => {
                if !matches!(toks.last(), Some(Tok::All)) {
                    toks.push(Tok::All);
                }
            }
            other => toks.push(Tok::Ch(ch(other))),
        }
    }
    Some(toks)
}

/// `g_charclass()`; `q` points at the ':' following '['.
fn charclass(pattern: &[u16], q: &mut usize, items: &mut Vec<Item>) -> i32 {
    let start = *q + 1;
    let Some(colon) = pattern
        .get(start..)
        .and_then(|s| s.iter().position(|&c| c == COLON))
        .map(|p| p + start)
    else {
        return 1;
    };
    if get(pattern, colon + 1) != RBRACKET {
        return 1;
    }
    let name = &pattern[start..colon];
    let found = CLASSES.iter().position(|cls| {
        cls.len() == name.len()
            && cls
                .bytes()
                .zip(name.iter())
                .all(|(a, &b)| u16::from(a) == b)
    });
    match found {
        Some(idx) => {
            items.push(Item::Class(idx));
            *q += name.len() + 3;
            0
        }
        None => -1,
    }
}

fn tok_match(name: &[u8], pat: &[Tok]) -> bool {
    let mut n = 0usize;
    let mut p = 0usize;
    while p < pat.len() {
        match &pat[p] {
            Tok::All => {
                while p < pat.len() && matches!(pat[p], Tok::All) {
                    p += 1;
                }
                if p == pat.len() {
                    return true;
                }
                loop {
                    if tok_match(&name[n..], &pat[p..]) {
                        return true;
                    }
                    if n >= name.len() {
                        return false;
                    }
                    n += 1;
                }
            }
            Tok::One => {
                if n >= name.len() {
                    return false;
                }
                n += 1;
            }
            Tok::Set { negate, items } => {
                let Some(&k) = name.get(n) else {
                    return false;
                };
                n += 1;
                let ok = items.iter().any(|item| match item {
                    Item::Ch(c) => *c == k,
                    Item::Range(a, b) => *a <= k && k <= *b,
                    Item::Class(idx) => class_matches(*idx, k),
                });
                if ok == *negate {
                    return false;
                }
            }
            Tok::Ch(c) => {
                if name.get(n) != Some(c) {
                    return false;
                }
                n += 1;
            }
        }
        p += 1;
    }
    n == name.len()
}

/// glibc `fnmatch($pattern, $string, FNM_CASEFOLD)`.
pub fn fnmatch_casefold(pattern: &str, string: &str) -> bool {
    fn_match(pattern.as_bytes(), string.as_bytes())
}

fn fold(c: u8) -> u8 {
    c.to_ascii_lowercase()
}

/// Parses a bracket expression starting right after '['. Returns the
/// match result and the index after the closing ']', or None when the
/// bracket is not terminated (then '[' is literal).
fn bracket(p: &[u8], c: u8) -> Option<(bool, usize)> {
    let mut i = 0;
    let negate = matches!(p.first(), Some(b'!') | Some(b'^'));
    if negate {
        i += 1;
    }
    let c = fold(c);
    let mut matched = false;
    let mut first = true;
    loop {
        let &pc = p.get(i)?;
        if pc == b']' && !first {
            i += 1;
            break;
        }
        first = false;
        if pc == b'[' && p.get(i + 1) == Some(&b':') {
            let rest = &p[i + 2..];
            if let Some(end) = rest.windows(2).position(|w| w == b":]") {
                let name = String::from_utf8_lossy(&rest[..end]);
                if let Some(idx) = CLASSES.iter().position(|cls| *cls == name) {
                    if class_matches(idx, c) || (idx == 6 || idx == 10) && class_matches(1, c) {
                        matched = true;
                    }
                    i += 2 + end + 2;
                    continue;
                }
            }
        }
        let mut lo = pc;
        i += 1;
        if lo == b'\\' {
            lo = *p.get(i)?;
            i += 1;
        }
        if p.get(i) == Some(&b'-') && p.get(i + 1).is_some_and(|&x| x != b']') {
            let mut hi = p[i + 1];
            i += 2;
            if hi == b'\\' {
                hi = *p.get(i)?;
                i += 1;
            }
            if fold(lo) <= c && c <= fold(hi) {
                matched = true;
            }
        } else if fold(lo) == c {
            matched = true;
        }
    }
    Some((matched != negate, i))
}

fn fn_match(p: &[u8], s: &[u8]) -> bool {
    let mut pi = 0;
    let mut si = 0;
    while pi < p.len() {
        match p[pi] {
            b'?' => {
                if si >= s.len() {
                    return false;
                }
                si += 1;
                pi += 1;
            }
            b'*' => {
                while pi < p.len() && p[pi] == b'*' {
                    pi += 1;
                }
                if pi == p.len() {
                    return true;
                }
                return (si..=s.len()).any(|k| fn_match(&p[pi..], &s[k..]));
            }
            b'[' => {
                let Some(&c) = s.get(si) else {
                    return false;
                };
                match bracket(&p[pi + 1..], c) {
                    Some((true, used)) => {
                        pi += 1 + used;
                        si += 1;
                    }
                    Some((false, _)) => return false,
                    None => {
                        if fold(c) != b'[' {
                            return false;
                        }
                        pi += 1;
                        si += 1;
                    }
                }
            }
            b'\\' if pi + 1 < p.len() => {
                if s.get(si).map(|&c| fold(c)) != Some(fold(p[pi + 1])) {
                    return false;
                }
                pi += 2;
                si += 1;
            }
            pc => {
                if s.get(si).map(|&c| fold(c)) != Some(fold(pc)) {
                    return false;
                }
                pi += 1;
                si += 1;
            }
        }
    }
    si == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        for d in [".cursor/sub", "Docs", "docs"] {
            fs::create_dir_all(root.join(d)).expect("mkdir");
        }
        for f in [
            "a",
            "b",
            "B",
            "_x",
            ".hid",
            ".cursor/z",
            ".cursor/A",
            ".cursor/.h",
            "Makefile",
            "makefile",
            "c,d",
            "e{f}",
            "abx",
        ] {
            fs::write(root.join(f), "").expect("write");
        }
        dir
    }

    #[test]
    fn brace_alternatives_are_sorted_individually() {
        let dir = fixture();
        let g = Globber::new(dir.path());
        let got = g.glob("{.*,{M,m}akefile,.cursor/**}*");
        assert_eq!(
            got,
            vec![
                ".",
                "..",
                ".cursor",
                ".hid",
                "Makefile",
                "makefile",
                ".cursor/A",
                ".cursor/sub",
                ".cursor/z"
            ]
        );
    }

    #[test]
    fn caret_is_literal_and_bang_negates() {
        let dir = fixture();
        let g = Globber::new(dir.path());
        assert_eq!(g.glob("[^a]"), vec!["a"]);
        assert!(!g.glob("[!a]*").contains(&"a".to_string()));
        assert!(!g.glob("[!a]*").contains(&".hid".to_string()));
    }

    #[test]
    fn escapes_trailing_slash_and_unclosed_braces() {
        let dir = fixture();
        let g = Globber::new(dir.path());
        assert_eq!(g.glob("{e\\{f\\},q}"), vec!["e{f}"]);
        assert_eq!(g.glob("docs/"), vec!["docs/"]);
        assert!(g.glob("a/").is_empty());
        assert!(g.glob("{a").is_empty());
        assert_eq!(g.glob("{a[bc]x,q}"), vec!["abx"]);
        assert_eq!(g.glob("[[:upper:]]*"), vec!["B", "Docs", "Makefile"]);
    }

    #[test]
    fn fnmatch_casefold_semantics() {
        assert!(fnmatch_casefold("license.*", "LICENSE.md"));
        assert!(fnmatch_casefold("docs*", "Docs/"));
        assert!(fnmatch_casefold("*", ".hidden/x"));
        assert!(fnmatch_casefold("[^a]b", "cb"));
        assert!(!fnmatch_casefold("readme.*", "README"));
    }
}
