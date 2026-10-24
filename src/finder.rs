//! Symfony Console's `Application::find()` for the command token: exact
//! names, unambiguous abbreviations, ambiguity suggestions and the "Did you
//! mean ...?" alternatives (`findAlternatives()`).

use crate::cli::{LPV_COMMANDS, LpvCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    Found(&'static str),
    /// `NamespaceNotFoundException` or an ambiguity: rendered as an error block.
    Error(String),
    /// `CommandNotFoundException`: rendered as an error block unless the run
    /// is interactive and there is exactly one alternative, in which case
    /// Symfony asks whether to run it instead.
    NotDefined {
        message: String,
        alternatives: Vec<&'static str>,
    },
}

fn command(name: &str) -> Option<&'static LpvCommand> {
    LPV_COMMANDS.iter().find(|c| c.name == name)
}

/// `Terminal::getWidth()`.
pub fn terminal_width() -> i64 {
    match std::env::var("COLUMNS") {
        Ok(c) => crate::output::php_int(c.trim()),
        Err(_) => 80,
    }
}

fn prefix_matches(name: &str, item: &str, case_insensitive: bool) -> bool {
    // `{^a[^:]*:b[^:]*}` against names that never contain a colon.
    if name.contains(':') || item.len() < name.len() {
        return false;
    }
    if case_insensitive {
        item.as_bytes()[..name.len()].eq_ignore_ascii_case(name.as_bytes())
    } else {
        item.starts_with(name)
    }
}

pub fn find(name: &str) -> Lookup {
    if let Some(cmd) = command(name) {
        return Lookup::Found(cmd.name);
    }
    let mut matches: Vec<&'static LpvCommand> = LPV_COMMANDS
        .iter()
        .filter(|c| prefix_matches(name, c.name, false))
        .collect();
    if matches.is_empty() {
        matches = LPV_COMMANDS
            .iter()
            .filter(|c| prefix_matches(name, c.name, true))
            .collect();
    }
    if matches.is_empty() {
        if let Some(pos) = name.rfind(':') {
            // findNamespace(): no lpv command lives in a namespace.
            return Lookup::Error(format!(
                "There are no commands defined in the \"{}\" namespace.",
                &name[..pos]
            ));
        }
        let mut message = format!("Command \"{name}\" is not defined.");
        let alternatives: Vec<&'static str> = find_alternatives(name)
            .into_iter()
            .filter(|n| command(n).is_some_and(|c| !c.hidden))
            .collect();
        if !alternatives.is_empty() {
            let label = if alternatives.len() == 1 {
                "this"
            } else {
                "one of these"
            };
            message.push_str(&format!(
                "\n\nDid you mean {label}?\n    {}",
                alternatives.join("\n    ")
            ));
        }
        return Lookup::NotDefined {
            message,
            alternatives,
        };
    }
    if matches.len() > 1 {
        let usable = terminal_width() - 10;
        let max_len = matches.iter().map(|c| c.name.len()).max().unwrap_or(0);
        let visible: Vec<&&LpvCommand> = matches.iter().filter(|c| !c.hidden).collect();
        if visible.len() > 1 {
            let abbrevs: Vec<String> = visible
                .iter()
                .map(|c| {
                    let abbrev = format!("{:<max_len$} {}", c.name, c.description);
                    if i64::try_from(abbrev.chars().count()).unwrap_or(i64::MAX) > usable {
                        let keep = usize::try_from(usable - 3).unwrap_or(0);
                        format!("{}...", abbrev.chars().take(keep).collect::<String>())
                    } else {
                        abbrev
                    }
                })
                .collect();
            return Lookup::Error(format!(
                "Command \"{name}\" is ambiguous.\nDid you mean one of these?\n    {}.",
                abbrevs.join("\n    ")
            ));
        }
        if let Some(only) = visible.first() {
            return Lookup::Found(only.name);
        }
    }
    match matches.first() {
        Some(cmd) => Lookup::Found(cmd.name),
        None => Lookup::Error(format!("Command \"{name}\" is not defined.")),
    }
}

/// PHP's `levenshtein()` with unit costs (byte based).
fn levenshtein(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1; b.len() + 1];
        for (j, cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        prev = cur;
    }
    prev[b.len()]
}

/// `Application::findAlternatives()` over every registered command name.
fn find_alternatives(name: &str) -> Vec<&'static str> {
    const THRESHOLD: f64 = 1e3;
    let mut alternatives: Vec<(&'static str, f64)> = Vec::new();
    let close = |lev: usize, sub: &str, item: &str| {
        (lev as f64) <= sub.len() as f64 / 3.0 || (!sub.is_empty() && item.contains(sub))
    };
    for (i, sub) in name.split(':').enumerate() {
        for cmd in LPV_COMMANDS {
            let parts: Vec<&str> = cmd.name.split(':').collect();
            let existing = alternatives.iter().position(|(n, _)| *n == cmd.name);
            let Some(part) = parts.get(i) else {
                if let Some(idx) = existing {
                    alternatives[idx].1 += THRESHOLD;
                }
                continue;
            };
            let lev = levenshtein(sub, part);
            if close(lev, sub, part) {
                match existing {
                    Some(idx) => alternatives[idx].1 += lev as f64,
                    None => alternatives.push((cmd.name, lev as f64)),
                }
            } else if let Some(idx) = existing {
                alternatives[idx].1 += THRESHOLD;
            }
        }
    }
    for cmd in LPV_COMMANDS {
        let lev = levenshtein(name, cmd.name);
        if (lev as f64) <= name.len() as f64 / 3.0 || cmd.name.contains(name) {
            match alternatives.iter().position(|(n, _)| *n == cmd.name) {
                Some(idx) => alternatives[idx].1 -= lev as f64,
                None => alternatives.push((cmd.name, lev as f64)),
            }
        }
    }
    let mut names: Vec<&'static str> = alternatives
        .into_iter()
        .filter(|(_, lev)| *lev < 2.0 * THRESHOLD)
        .map(|(n, _)| n)
        .collect();
    // ksort(SORT_NATURAL | SORT_FLAG_CASE); command names carry no digits.
    names.sort_by_key(|n| n.to_ascii_lowercase());
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_and_abbreviations() {
        assert_eq!(find("validate"), Lookup::Found("validate"));
        assert_eq!(find("upd"), Lookup::Found("update"));
        assert_eq!(find("creat"), Lookup::Found("create"));
        assert_eq!(find("Validate-remote"), Lookup::Found("validate-remote"));
    }

    #[test]
    fn ambiguity_matches_reference() {
        // Captured from the 6.3.2 phar with an 80 column terminal.
        let Lookup::Error(message) = find("val") else {
            panic!("expected an ambiguity");
        };
        if terminal_width() == 80 {
            assert_eq!(
                message,
                "Command \"val\" is ambiguous.\nDid you mean one of these?\n    validate        Validate the .gitattributes file of a given project...\n    validate-remote Validate the .gitattributes file of a remote GitHub...."
            );
        }
        let Lookup::Error(message) = find("re") else {
            panic!("expected an ambiguity");
        };
        assert!(message.ends_with("    refresh  Refresh a present .lpv file\n    reformat Reformat a present .gitattributes file."));
    }

    #[test]
    fn alternatives() {
        assert_eq!(
            find("valdate"),
            Lookup::NotDefined {
                message: "Command \"valdate\" is not defined.\n\nDid you mean this?\n    validate"
                    .to_string(),
                alternatives: vec!["validate"],
            }
        );
        assert_eq!(
            find("frobnicate"),
            Lookup::NotDefined {
                message: "Command \"frobnicate\" is not defined.".to_string(),
                alternatives: vec![],
            }
        );
        assert_eq!(
            find("x:y"),
            Lookup::Error("There are no commands defined in the \"x\" namespace.".to_string())
        );
    }

    #[test]
    fn levenshtein_basics() {
        assert_eq!(levenshtein("valdate", "validate"), 1);
        assert_eq!(levenshtein("", "abc"), 3);
    }
}
