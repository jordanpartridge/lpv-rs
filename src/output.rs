//! A minimal stand-in for Symfony Console's output layer: verbosity levels,
//! `<info>`/`<error>`/`<comment>` tags (stripped when not decorated, ANSI
//! colours when decorated) and Symfony-style error blocks.

use std::io::{IsTerminal, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Quiet = 0,
    Normal = 1,
    Verbose = 2,
    VeryVerbose = 3,
    Debug = 4,
}

pub struct Output {
    pub verbosity: Verbosity,
    pub decorated: bool,
    stdout: Vec<u8>,
}

impl Output {
    pub fn new(verbosity: Verbosity, ansi: Option<bool>) -> Self {
        let decorated = match ansi {
            Some(flag) => flag,
            None => std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal(),
        };
        Self {
            verbosity,
            decorated,
            stdout: Vec::new(),
        }
    }

    pub fn write(&mut self, message: &str) {
        self.write_at(message, false, Verbosity::Normal);
    }

    pub fn writeln(&mut self, message: &str) {
        self.write_at(message, true, Verbosity::Normal);
    }

    pub fn verbose(&mut self, message: &str) {
        self.write_at(message, true, Verbosity::Verbose);
    }

    pub fn debug(&mut self, message: &str) {
        self.write_at(message, true, Verbosity::Debug);
    }

    fn write_at(&mut self, message: &str, newline: bool, level: Verbosity) {
        if level > self.verbosity || self.verbosity == Verbosity::Quiet {
            return;
        }
        let formatted = format_tags(message, self.decorated);
        self.stdout.extend_from_slice(formatted.as_bytes());
        if newline {
            self.stdout.push(b'\n');
        }
    }

    /// A runtime warning on stderr (shown at every verbosity, like PHP's).
    pub fn warn(&mut self, message: &str) {
        let mut err = std::io::stderr().lock();
        let _ = err.write_all(message.as_bytes());
        let _ = err.write_all(b"\n");
        let _ = err.flush();
    }

    /// Writes raw text (already formatted, e.g. JSON) regardless of tags.
    pub fn raw_line(&mut self, message: &str) {
        if self.verbosity == Verbosity::Quiet {
            return;
        }
        self.stdout.extend_from_slice(message.as_bytes());
        self.stdout.push(b'\n');
    }

    pub fn flush(&mut self) {
        let mut handle = std::io::stdout().lock();
        // Ignore broken pipes: there is nobody left to tell.
        let _ = handle.write_all(&self.stdout);
        let _ = handle.flush();
        self.stdout.clear();
    }
}

fn style_codes(tag: &str) -> Option<(&'static str, &'static str)> {
    match tag {
        "info" => Some(("\x1b[32m", "\x1b[39m")),
        "comment" => Some(("\x1b[33m", "\x1b[39m")),
        "error" => Some(("\x1b[37;41m", "\x1b[39;49m")),
        "question" => Some(("\x1b[30;46m", "\x1b[39;49m")),
        _ => None,
    }
}

/// Applies (or strips) the Symfony console tags we emit ourselves.
pub fn format_tags(message: &str, decorated: bool) -> String {
    let mut out = String::with_capacity(message.len());
    let mut stack: Vec<&str> = Vec::new();
    let mut rest = message;
    let push_text = |out: &mut String, text: &str, stack: &[&str]| {
        if text.is_empty() {
            return;
        }
        match stack
            .last()
            .and_then(|t| style_codes(t))
            .filter(|_| decorated)
        {
            Some((open, close)) => {
                out.push_str(open);
                out.push_str(text);
                out.push_str(close);
            }
            None => out.push_str(text),
        }
    };
    while let Some(pos) = rest.find('<') {
        let (before, after) = rest.split_at(pos);
        let Some(end) = after.find('>') else {
            break;
        };
        let tag = &after[1..end];
        let (closing, name) = match tag.strip_prefix('/') {
            Some(n) => (true, n),
            None => (false, tag),
        };
        let known = (closing && name.is_empty()) || style_codes(name).is_some();
        if !known {
            push_text(&mut out, &rest[..pos + 1], &stack);
            rest = &rest[pos + 1..];
            continue;
        }
        push_text(&mut out, before, &stack);
        if closing {
            stack.pop();
        } else {
            stack.push(name);
        }
        rest = &after[end + 1..];
    }
    push_text(&mut out, rest, &stack);
    out
}

/// PHP's `(int)` cast of a numeric-prefixed string.
pub fn php_int(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut end = 0;
    if end < bytes.len() && (bytes[end] == b'-' || bytes[end] == b'+') {
        end += 1;
    }
    while end < bytes.len() && bytes[end].is_ascii_digit() {
        end += 1;
    }
    s[..end].parse().unwrap_or(0)
}

/// Renders an error the way Symfony's `Application::renderThrowable()` does
/// for non-verbose runs, followed by the command synopsis when known.
pub fn render_error_block(title: Option<&str>, message: &str, synopsis: Option<&str>) -> String {
    // Terminal::getWidth(): (int) COLUMNS when set, 80 otherwise; Symfony
    // then uses width - 1 and wraps the message at width - 4 of that.
    let width: i64 = match std::env::var("COLUMNS") {
        Ok(c) => php_int(c.trim()),
        Err(_) => 80,
    };
    let chunk = if width == 0 {
        usize::MAX
    } else {
        usize::try_from(width - 1 - 4).unwrap_or(1).max(1)
    };
    let mut lines: Vec<String> = Vec::new();
    for raw in message.trim().split('\n') {
        let raw = raw.strip_suffix('\r').unwrap_or(raw);
        let chars: Vec<char> = raw.chars().collect();
        if chars.is_empty() {
            lines.push(String::new());
        }
        for piece in chars.chunks(chunk) {
            lines.push(piece.iter().collect());
        }
    }
    let len = lines
        .iter()
        .map(|l| l.chars().count() + 4)
        .max()
        .unwrap_or(4);
    let pad = " ".repeat(len);
    let mut s = String::from("\n");
    if let Some(t) = title {
        s.push_str(t);
        s.push('\n');
    }
    s.push_str(&pad);
    s.push('\n');
    for line in &lines {
        s.push_str("  ");
        s.push_str(line);
        s.push_str("  ");
        s.push_str(&" ".repeat(len - (line.chars().count() + 4)));
        s.push('\n');
    }
    s.push_str(&pad);
    s.push_str("\n\n");
    if let Some(syn) = synopsis {
        s.push_str(syn);
        s.push_str("\n\n");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_known_tags_only() {
        assert_eq!(
            format_tags("The file is <info>valid</info> <foo>.", false),
            "The file is valid <foo>."
        );
        assert_eq!(
            format_tags("<error>Use <info>classic</info>.</error>", false),
            "Use classic."
        );
    }

    #[test]
    fn colours_when_decorated() {
        assert_eq!(format_tags("<info>ok</info>", true), "\x1b[32mok\x1b[39m");
    }

    #[test]
    fn error_block_shape() {
        let b = render_error_block(None, "x", Some("syn"));
        assert_eq!(b, "\n     \n  x  \n     \n\nsyn\n\n");
    }
}
