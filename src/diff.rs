//! Port of `sebastian/diff` as bundled in the lean-package-validator 6.3.2 phar
//! (LCS-based Differ + StrictUnifiedDiffOutputBuilder
//! with `fromFile: Original`, `toFile: Expected`) as used by `validate --diff`.

const OLD: u8 = 0;
const ADDED: u8 = 1;
const REMOVED: u8 = 2;
const DIFF_LINE_END_WARNING: u8 = 3;
const NO_LINE_END_EOF_WARNING: u8 = 4;

type Entry = (Vec<u8>, u8);

/// `preg_split('/(.*\R)/', $input, -1, PREG_SPLIT_DELIM_CAPTURE | PREG_SPLIT_NO_EMPTY)`.
fn split_string_by_lines(input: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut p = 0;
    while p < input.len() {
        if let Some(q) = input[p..].iter().position(|&c| c == b'\n') {
            let end = p + q + 1;
            out.push(input[p..end].to_vec());
            p = end;
            continue;
        }
        if let Some(r) = input[p..]
            .iter()
            .rposition(|&c| matches!(c, b'\r' | 0x0b | 0x0c | 0x85))
        {
            let end = p + r + 1;
            out.push(input[p..end].to_vec());
            p = end;
            continue;
        }
        out.push(input[p..].to_vec());
        break;
    }
    out
}

fn linebreak(line: &[u8]) -> &'static str {
    match line.last() {
        Some(b'\r') => "\r",
        Some(b'\n') if line.ends_with(b"\r\n") => "\r\n",
        Some(b'\n') => "\n",
        _ => "",
    }
}

fn detect_unmatched_line_endings(diff: &[Entry]) -> bool {
    let mut new_breaks: Vec<&str> = vec![""];
    let mut old_breaks: Vec<&str> = vec![""];
    let add = |set: &mut Vec<&'static str>, lb: &'static str| {
        if !set.contains(&lb) {
            set.push(lb);
        }
    };
    for (line, kind) in diff {
        let lb = linebreak(line);
        match *kind {
            OLD => {
                add(&mut old_breaks, lb);
                add(&mut new_breaks, lb);
            }
            ADDED => add(&mut new_breaks, lb),
            REMOVED => add(&mut old_breaks, lb),
            _ => {}
        }
    }
    if new_breaks.len() == 1 || old_breaks.len() == 1 {
        return false;
    }
    new_breaks.iter().any(|b| !old_breaks.contains(b))
        || old_breaks.iter().any(|b| !new_breaks.contains(b))
}

fn diff_to_array(from: &[u8], to: &[u8]) -> Vec<Entry> {
    let mut from = split_string_by_lines(from);
    let mut to = split_string_by_lines(to);

    let mut prefix = 0;
    while prefix < from.len() && prefix < to.len() && from[prefix] == to[prefix] {
        prefix += 1;
    }
    let start: Vec<Vec<u8>> = from.drain(..prefix).collect();
    to.drain(..prefix);

    let mut end: Vec<Vec<u8>> = Vec::new();
    while let (Some(a), Some(b)) = (from.last(), to.last()) {
        if a != b {
            break;
        }
        to.pop();
        if let Some(line) = from.pop() {
            end.insert(0, line);
        }
    }

    // Differ::selectLcsImplementation(): 144 bytes per item on 64-bit PHP.
    let smaller = from.len().min(to.len()) as u128;
    let common = if 144 * smaller * smaller > 100 * 1024 * 1024 {
        lcs_memory_efficient(&from, &to)
    } else {
        lcs_time_efficient(&from, &to)
    };

    let mut diff: Vec<Entry> = start.into_iter().map(|l| (l, OLD)).collect();
    let mut from_q: std::collections::VecDeque<Vec<u8>> = from.into();
    let mut to_q: std::collections::VecDeque<Vec<u8>> = to.into();
    for token in common {
        while let Some(front) = from_q.front() {
            if *front == token {
                break;
            }
            if let Some(line) = from_q.pop_front() {
                diff.push((line, REMOVED));
            }
        }
        while let Some(front) = to_q.front() {
            if *front == token {
                break;
            }
            if let Some(line) = to_q.pop_front() {
                diff.push((line, ADDED));
            }
        }
        diff.push((token, OLD));
        from_q.pop_front();
        to_q.pop_front();
    }
    diff.extend(from_q.into_iter().map(|l| (l, REMOVED)));
    diff.extend(to_q.into_iter().map(|l| (l, ADDED)));
    diff.extend(end.into_iter().map(|l| (l, OLD)));

    if detect_unmatched_line_endings(&diff) {
        diff.insert(
            0,
            (
                b"#Warning: Strings contain different line endings!\n".to_vec(),
                DIFF_LINE_END_WARNING,
            ),
        );
    }
    diff
}

/// `TimeEfficientLongestCommonSubsequenceCalculator`.
fn lcs_time_efficient(from: &[Vec<u8>], to: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let from_len = from.len();
    let to_len = to.len();
    let width = from_len + 1;
    let mut matrix = vec![0usize; width * (to_len + 1)];
    for i in 1..=from_len {
        for j in 1..=to_len {
            let o = j * width + i;
            let first_or_last = if from[i - 1] == to[j - 1] {
                matrix[o - width - 1] + 1
            } else {
                0
            };
            matrix[o] = if matrix[o - 1] > matrix[o - width] {
                first_or_last.max(matrix[o - 1])
            } else {
                first_or_last.max(matrix[o - width])
            };
        }
    }
    let mut common = Vec::new();
    let (mut i, mut j) = (from_len, to_len);
    while i > 0 && j > 0 {
        if from[i - 1] == to[j - 1] {
            common.push(from[i - 1].clone());
            i -= 1;
            j -= 1;
        } else {
            let o = j * width + i;
            if matrix[o - width] > matrix[o - 1] {
                j -= 1;
            } else {
                i -= 1;
            }
        }
    }
    common.reverse();
    common
}

/// `MemoryEfficientLongestCommonSubsequenceCalculator` (Hirschberg).
fn lcs_memory_efficient(from: &[Vec<u8>], to: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let c_from = from.len();
    let c_to = to.len();
    if c_from == 0 {
        return Vec::new();
    }
    if c_from == 1 {
        return if to.contains(&from[0]) {
            vec![from[0].clone()]
        } else {
            Vec::new()
        };
    }
    let i = c_from / 2;
    let (from_start, from_end) = from.split_at(i);
    let ll_b = lcs_length(from_start, to);
    let from_end_rev: Vec<Vec<u8>> = from_end.iter().rev().cloned().collect();
    let to_rev: Vec<Vec<u8>> = to.iter().rev().cloned().collect();
    let ll_e = lcs_length(&from_end_rev, &to_rev);
    let mut j_max = 0;
    let mut max = 0;
    for j in 0..=c_to {
        let m = ll_b[j] + ll_e[c_to - j];
        if m >= max {
            max = m;
            j_max = j;
        }
    }
    let (to_start, to_end) = to.split_at(j_max);
    let mut out = lcs_memory_efficient(from_start, to_start);
    out.extend(lcs_memory_efficient(from_end, to_end));
    out
}

fn lcs_length(from: &[Vec<u8>], to: &[Vec<u8>]) -> Vec<usize> {
    let mut current = vec![0usize; to.len() + 1];
    for item in from {
        let prev = current.clone();
        for (j, other) in to.iter().enumerate() {
            current[j + 1] = if item == other {
                prev[j] + 1
            } else if current[j] > prev[j + 1] {
                current[j]
            } else {
                prev[j + 1]
            };
        }
    }
    current
}

struct HunkWriter {
    changed: bool,
    buffer: Vec<u8>,
}

impl HunkWriter {
    #[allow(clippy::too_many_arguments)]
    fn write_hunk(
        &mut self,
        diff: &[Entry],
        start: i64,
        end: i64,
        from_start: i64,
        from_range: i64,
        to_start: i64,
        to_range: i64,
    ) {
        self.buffer
            .extend_from_slice(format!("@@ -{from_start}").as_bytes());
        if from_range != 1 {
            self.buffer
                .extend_from_slice(format!(",{from_range}").as_bytes());
        }
        self.buffer
            .extend_from_slice(format!(" +{to_start}").as_bytes());
        if to_range != 1 {
            self.buffer
                .extend_from_slice(format!(",{to_range}").as_bytes());
        }
        self.buffer.extend_from_slice(b" @@\n");
        let mut i = start.max(0);
        while i < end {
            let Some((line, kind)) = diff.get(i as usize) else {
                break;
            };
            match *kind {
                ADDED => {
                    self.changed = true;
                    self.buffer.push(b'+');
                    self.buffer.extend_from_slice(line);
                }
                REMOVED => {
                    self.changed = true;
                    self.buffer.push(b'-');
                    self.buffer.extend_from_slice(line);
                }
                OLD => {
                    self.buffer.push(b' ');
                    self.buffer.extend_from_slice(line);
                }
                NO_LINE_END_EOF_WARNING => self.buffer.extend_from_slice(line),
                _ => {}
            }
            i += 1;
        }
    }

    fn write_diff_hunks(&mut self, mut diff: Vec<Entry>) {
        const CONTEXT: i64 = 3;
        const CUT_OFF: i64 = 6;
        let marker = || {
            (
                b"\n\\ No newline at end of file\n".to_vec(),
                NO_LINE_END_EOF_WARNING,
            )
        };

        let upper = diff.len();
        if diff[upper - 1].1 == OLD {
            if diff[upper - 1].0.last() != Some(&b'\n') {
                diff.push(marker());
            }
        } else {
            let mut find_added = true;
            let mut find_removed = true;
            let mut i = upper;
            while i > 0 {
                i -= 1;
                let kind = diff[i].1;
                let wanted = (kind == ADDED && find_added) || (kind == REMOVED && find_removed);
                if wanted {
                    if kind == ADDED {
                        find_added = false;
                    } else {
                        find_removed = false;
                    }
                    if diff[i].0.last() != Some(&b'\n') {
                        diff.insert(i + 1, marker());
                    }
                    if !find_added && !find_removed {
                        break;
                    }
                }
            }
        }

        let mut hunk_capture: Option<i64> = None;
        let (mut same_count, mut to_range, mut from_range) = (0i64, 0i64, 0i64);
        let (mut to_start, mut from_start) = (1i64, 1i64);
        let mut last_i = 0i64;

        for (idx, entry) in diff.iter().enumerate() {
            let i = idx as i64;
            last_i = i;
            if entry.1 == OLD {
                let Some(hc) = hunk_capture else {
                    from_start += 1;
                    to_start += 1;
                    continue;
                };
                same_count += 1;
                to_range += 1;
                from_range += 1;
                if same_count == CUT_OFF {
                    let cso = if hc - CONTEXT < 0 { hc } else { CONTEXT };
                    self.write_hunk(
                        &diff,
                        hc - cso,
                        i - CUT_OFF + CONTEXT + 1,
                        from_start - cso,
                        from_range - CUT_OFF + cso + CONTEXT,
                        to_start - cso,
                        to_range - CUT_OFF + cso + CONTEXT,
                    );
                    from_start += from_range;
                    to_start += to_range;
                    hunk_capture = None;
                    same_count = 0;
                    to_range = 0;
                    from_range = 0;
                }
                continue;
            }
            same_count = 0;
            if entry.1 == NO_LINE_END_EOF_WARNING {
                continue;
            }
            self.changed = true;
            if hunk_capture.is_none() {
                hunk_capture = Some(i);
            }
            if entry.1 == ADDED {
                to_range += 1;
            }
            if entry.1 == REMOVED {
                from_range += 1;
            }
        }

        let Some(hc) = hunk_capture else {
            return;
        };
        let cso = if hc - CONTEXT < 0 { hc } else { CONTEXT };
        let ceo = same_count.min(CONTEXT);
        from_range -= same_count;
        to_range -= same_count;
        self.write_hunk(
            &diff,
            hc - cso,
            last_i - same_count + ceo + 1,
            from_start - cso,
            from_range + cso + ceo,
            to_start - cso,
            to_range + cso + ceo,
        );
    }
}

/// `(new Differ(new StrictUnifiedDiffOutputBuilder(['fromFile' => 'Original',
/// 'toFile' => 'Expected'])))->diff($from, $to)`.
pub fn unified_diff(from: &str, to: &str) -> String {
    let diff = diff_to_array(from.as_bytes(), to.as_bytes());
    if diff.is_empty() {
        return String::new();
    }
    let mut writer = HunkWriter {
        changed: false,
        buffer: b"--- Original\n+++ Expected\n".to_vec(),
    };
    writer.write_diff_hunks(diff);
    if !writer.changed {
        return String::new();
    }
    let mut out = writer.buffer;
    if !matches!(out.last(), Some(b'\n') | Some(b'\r')) {
        out.push(b'\n');
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let from = "* text=auto eol=lf\n\nREADME.md export-ignore\n";
        let to = "* text=auto eol=lf\n\n.gitattributes export-ignore\nREADME.md export-ignore\ntests/ export-ignore\n";
        assert_eq!(
            unified_diff(from, to),
            "--- Original\n+++ Expected\n@@ -1,3 +1,5 @@\n * text=auto eol=lf\n \n+.gitattributes export-ignore\n README.md export-ignore\n+tests/ export-ignore\n"
        );
    }

    #[test]
    fn missing_newline_marker() {
        let out = unified_diff("a\nb", "a\nc\n");
        assert_eq!(
            out,
            "--- Original\n+++ Expected\n@@ -1,2 +1,2 @@\n a\n-b\n\\ No newline at end of file\n+c\n"
        );
    }

    #[test]
    fn swapped_lines_follow_lcs_choice_of_reference() {
        // Captured from the 6.3.2 phar (validate --diff on tests/fixtures/gitignore).
        let from = "a\n.gitignore x\n.github/ x\nb\n";
        let to = "a\n.github/ x\n.gitignore x\nb\n";
        assert_eq!(
            unified_diff(from, to),
            "--- Original\n+++ Expected\n@@ -1,4 +1,4 @@\n a\n+.github/ x\n .gitignore x\n-.github/ x\n b\n"
        );
    }

    #[test]
    fn identical_is_empty() {
        assert_eq!(unified_diff("a\n", "a\n"), "");
    }
}
