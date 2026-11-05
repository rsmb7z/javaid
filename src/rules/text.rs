//! The modules that read a file's lines: `FileTabCharacter`, `LineLength`, `RegexpSingleline`,
//! `FileLength` and `NewlineAtEndOfFile`.
//!
//! They need no syntax tree, so they run on every file the configuration says they read (a
//! Checker-level `fileExtensions` can pull in `.properties` or `.xml`) and share one pass over the
//! lines. What a "line" is, how a tab is measured and what a regular expression sees were all
//! pinned by probing Checkstyle; see `bench/oracle/probes/make-text-probes.py`.

use crate::config::{Config, Extensions, LineSeparator, RegexpSingleline};
use crate::engine::Diagnostic;
use memchr::{memchr, memchr2};

/// A file's lines as Checkstyle splits them: at `\n`, `\r\n` or a lone `\r`, without the
/// terminator. A final terminator does not start another, empty, line. Form feed, vertical tab and
/// the Unicode separators end nothing.
pub fn lines(src: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = src;
    std::iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let end = memchr2(b'\n', b'\r', rest).unwrap_or(rest.len());
        let line = &rest[..end];
        let terminator = match (rest.get(end), rest.get(end + 1)) {
            (Some(b'\r'), Some(b'\n')) => 2,
            (Some(_), _) => 1,
            (None, _) => 0,
        };
        rest = &rest[end + terminator..];
        Some(line)
    })
}

/// 1-based column of byte `at`, counting UTF-16 code units (no tab expansion).
fn column(line: &[u8], at: usize) -> u32 {
    line[..at].iter().filter(|&&b| b & 0xC0 != 0x80).map(|&b| if b >= 0xF0 { 2 } else { 1 }).sum::<u32>() + 1
}

pub fn run(cfg: &Config, ext: &str, src: &[u8], out: &mut Vec<Diagnostic>) {
    let reads = |own: &Extensions| cfg.reads(own, ext);
    let tab = cfg.file_tab_character.as_ref().filter(|m| reads(&m.extensions));
    let length = cfg.line_length.as_ref().filter(|m| reads(&m.extensions));
    let patterns: Vec<&RegexpSingleline> = cfg.regexp_singleline.iter().filter(|m| reads(&m.extensions)).collect();
    if let Some(m) = cfg.newline_at_end_of_file.as_ref().filter(|m| reads(&m.extensions)) {
        newline_at_end_of_file(cfg, m.separator, src, out);
    }
    if let Some(m) = cfg.multi_file_regexp_header.as_ref().filter(|m| reads(&m.extensions)) {
        multi_file_regexp_header(cfg, m, src, out);
    }
    let size = cfg.file_length.as_ref().filter(|m| reads(&m.extensions));
    if let Some(m) = size {
        let count = lines(src).count();
        if count > m.max {
            out.push(Diagnostic {
                rule: cfg.display("FileLength"),
                line: 1,
                col: 0,
                message: format!("File length is {} lines (max allowed is {}).", crate::engine::grouped(count), crate::engine::grouped(m.max)),
            });
        }
    }
    if tab.is_none() && length.is_none() && patterns.is_empty() {
        return;
    }

    // Two whole-file shortcuts: with no tab anywhere, FileTabCharacter has nothing to find and a
    // line is only as wide as its characters, never wider than its bytes.
    let file_has_tab = memchr(b'\t', src).is_some();
    let mut looking_for_tabs = tab.is_some() && file_has_tab;
    let mut counts = vec![0usize; patterns.len()];

    for (i, line) in lines(src).enumerate() {
        let n = i as u32 + 1;

        if looking_for_tabs {
            if let (Some(at), Some(m)) = (memchr(b'\t', line), tab) {
                let message = if m.each_line { "Line contains a tab character." } else { "File contains tab characters (this is the first instance)." };
                out.push(Diagnostic { rule: cfg.display("FileTabCharacter"), line: n, col: column(line, at), message: message.into() });
                looking_for_tabs = m.each_line;
            }
        }

        if let Some(m) = length {
            if line.len() > m.max || (file_has_tab && memchr(b'\t', line).is_some()) {
                let found = crate::columns::line_length(line, cfg.tab_width);
                if found > m.max && !m.ignore_pattern.matches(line) {
                    let message = format!("Line is longer than {} characters (found {}).", crate::engine::grouped(m.max), crate::engine::grouped(found));
                    out.push(Diagnostic { rule: cfg.display("LineLength"), line: n, col: 0, message });
                }
            }
        }

        for (m, count) in patterns.iter().zip(counts.iter_mut()) {
            if m.format.matches(line) {
                *count += 1;
                if *count > m.maximum {
                    let message = m.message.clone().unwrap_or_else(|| format!("Line matches the illegal pattern '{}'.", m.format.source));
                    out.push(Diagnostic { rule: m.name, line: n, col: 0, message });
                }
            }
        }
    }

    for (m, &count) in patterns.iter().zip(&counts) {
        if count < m.minimum {
            let message =
                m.message.clone().unwrap_or_else(|| format!("File does not contain at least {} matches for pattern '{}'.", m.minimum, m.format.source));
            out.push(Diagnostic { rule: m.name, line: 1, col: 0, message });
        }
    }
}

/// `NewlineAtEndOfFile`. Found by probing Checkstyle: an empty file ends with every separator, so
/// `lf` calls it a CRLF file; a file shorter than the separator does not end with it; and at `lf`
/// a file that ends in `\r\n` is reported as having the wrong ending, not as lacking a newline.
fn newline_at_end_of_file(cfg: &Config, separator: LineSeparator, src: &[u8], out: &mut Vec<Diagnostic>) {
    let ends_with = |sep: &[u8]| src.is_empty() || src.ends_with(sep);
    let message = match separator {
        LineSeparator::Lf if ends_with(b"\r\n") => "Expected line ending for file is LF(\\n), but CRLF(\\r\\n) is detected.".to_string(),
        LineSeparator::Lf if !ends_with(b"\n") => "File does not end with a newline 'lf'.".to_string(),
        LineSeparator::Cr if !ends_with(b"\r") => "File does not end with a newline 'cr'.".to_string(),
        LineSeparator::Crlf if !ends_with(b"\r\n") => "File does not end with a newline 'crlf'.".to_string(),
        LineSeparator::LfCrCrlf if !(ends_with(b"\n") || ends_with(b"\r")) => "File does not end with a newline.".to_string(),
        _ => return,
    };
    out.push(Diagnostic { rule: cfg.display("NewlineAtEndOfFile"), line: 1, col: 0, message });
}

/// `MultiFileRegexpHeader`: the file's first lines must match, line by line, one of the header
/// files. When none does, only the first header file is held against the file: its length, or the
/// first line that does not match (both probed).
fn multi_file_regexp_header(cfg: &Config, m: &crate::config::MultiFileRegexpHeader, src: &[u8], out: &mut Vec<Diagnostic>) {
    let file: Vec<&[u8]> = lines(src).collect();
    let mismatch = |patterns: &[crate::config::Format]| -> Option<usize> {
        let n = patterns.len().min(file.len());
        (0..n).find(|&i| !patterns[i].matches(file[i])).or((file.len() < patterns.len()).then_some(n))
    };
    if m.headers.iter().any(|(_, p)| mismatch(p).is_none()) {
        return;
    }
    let (first_name, first) = &m.headers[0];
    let message = if first.len() > file.len() {
        format!("Header is missing or too short based on configured header templates '{first_name}'. File must match one of them.")
    } else {
        let at = mismatch(first).unwrap_or(0);
        let shown = if first[at].source.is_empty() { "^$" } else { first[at].source.as_str() };
        format!("Header mismatch, expected line content was '{shown}'. File must match one of the configured header templates: '{}'.", m.names)
    };
    let line = if first.len() > file.len() { 1 } else { mismatch(first).unwrap_or(0) as u32 + 1 };
    out.push(Diagnostic { rule: cfg.display("MultiFileRegexpHeader"), line, col: 0, message });
}
