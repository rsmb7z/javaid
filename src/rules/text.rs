//! The modules that read a file's lines: `FileTabCharacter`, `LineLength` and `RegexpSingleline`.
//!
//! They need no syntax tree, so they run on every file the configuration says they read (a
//! Checker-level `fileExtensions` can pull in `.properties` or `.xml`) and share one pass over the
//! lines. What a "line" is, how a tab is measured and what a regular expression sees were all
//! pinned by probing Checkstyle; see `bench/oracle/probes/make-text-probes.py`.

use crate::config::{Config, Extensions, RegexpSingleline};
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

/// The number of columns a line occupies: one per code point (so a combining mark or an emoji
/// counts once each, as Checkstyle does), with a tab running to the next multiple of `tab_width`.
fn expanded_length(line: &[u8], tab_width: usize) -> usize {
    let mut len = 0;
    for &b in line {
        if b == b'\t' {
            len = (len / tab_width + 1) * tab_width;
        } else if b & 0xC0 != 0x80 {
            len += 1; // not a UTF-8 continuation byte, so the start of a code point
        }
    }
    len
}

/// 1-based column of byte `at`, counting code points.
fn column(line: &[u8], at: usize) -> u32 {
    line[..at].iter().filter(|&&b| b & 0xC0 != 0x80).count() as u32 + 1
}

pub fn run(cfg: &Config, ext: &str, src: &[u8], out: &mut Vec<Diagnostic>) {
    let reads = |own: &Extensions| cfg.reads(own, ext);
    let tab = cfg.file_tab_character.as_ref().filter(|m| reads(&m.extensions));
    let length = cfg.line_length.as_ref().filter(|m| reads(&m.extensions));
    let patterns: Vec<&RegexpSingleline> = cfg.regexp_singleline.iter().filter(|m| reads(&m.extensions)).collect();
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
                let found = expanded_length(line, cfg.tab_width);
                if found > m.max && !m.ignore_pattern.matches(line) {
                    let message = format!("Line is longer than {} characters (found {found}).", m.max);
                    out.push(Diagnostic { rule: cfg.display("LineLength"), line: n, col: 1, message });
                }
            }
        }

        for (m, count) in patterns.iter().zip(counts.iter_mut()) {
            if m.format.matches(line) {
                *count += 1;
                if *count > m.maximum {
                    let message = m.message.clone().unwrap_or_else(|| format!("Line matches the illegal pattern '{}'.", m.format.source));
                    out.push(Diagnostic { rule: m.name, line: n, col: 1, message });
                }
            }
        }
    }

    for (m, &count) in patterns.iter().zip(&counts) {
        if count < m.minimum {
            let message =
                m.message.clone().unwrap_or_else(|| format!("File does not contain at least {} matches for pattern '{}'.", m.minimum, m.format.source));
            out.push(Diagnostic { rule: m.name, line: 1, col: 1, message });
        }
    }
}
