//! `AvoidEscapedUnicodeCharacters`: `é` in a string, character, text block or comment.
//!
//! Checkstyle looks at the raw text of the literal, so an escape is whatever the source spells;
//! with `allowByTailComment` a finding waits until the end of the file, and a trailing comment on
//! its line cancels it.

use crate::config::AvoidEscapedUnicode;
use crate::engine::Ctx;
use regex::Regex;
use std::sync::LazyLock;
use tree_sitter::Node;

static UNICODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\u+[a-fA-F0-9]{4}").unwrap());
static CONTROL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\\u+(00[0-1][0-9A-Fa-f]|00[8-9][0-9A-Fa-f]|00[aA][dD]|034[fF]|070[fF]|180[eE]|200[b-fB-F]|202[a-eA-E]|206[0-4a-fA-F]|[fF]{3}[9a-bA-B]|[fF][eE][fF]{2})",
    )
    .unwrap()
});
static ALL_ESCAPED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(\\u+[a-fA-F0-9]{4}|"|'|\\|\\b|\\f|\\n|\r\n|[\n\x0B\x0C\r\x{85}\x{2028}\x{2029}]|\\r|\\s|\\t)+$"#).unwrap()
});
static NON_PRINTABLE: LazyLock<Regex> = LazyLock::new(|| {
    let alternatives = [
        r"0000", r"0009", r"000[bB]", r"000[cC]", r"0020", r"007[fF]", r"0085", r"009[fF]", r"00[aA]0", r"00[aA][dD]", r"04[fF]9", r"05[bB][eE]", r"05[dD]0", r"05[eE][aA]",
        r"05[fF]3", r"05[fF]4", r"0600", r"0604", r"061[cC]", r"06[dD]{2}", r"06[fF]{2}", r"070[fF]", r"0750", r"077[fF]", r"0[eE]00", r"0[eE]7[fF]", r"1680", r"180[eE]",
        r"1[eE]00", r"2000", r"2001", r"2002", r"2003", r"2004", r"2005", r"2006", r"2007", r"2008", r"2009", r"200[aA]", r"200[fF]", r"2025", r"2028", r"2029", r"202[fF]",
        r"205[fF]", r"2064", r"2066", r"2067", r"2068", r"2069", r"206[aA]", r"206[fF]", r"20[aA][fF]", r"2100", r"213[aA]", r"3000", r"[dD]800", r"[fF]8[fF]{2}",
        r"[fF][bB]50", r"[fF][dD][fF]{2}", r"[fF][eE]70", r"[fF][eE][fF]{2}", r"[fF]{2}0[eE]", r"[fF]{2}61", r"[fF]{2}[dD][cC]", r"[fF]{3}9", r"[fF]{3}[aA]", r"[fF]{3}[bB]",
        r"[fF]{4}",
    ];
    Regex::new(&alternatives.iter().map(|a| format!(r"\\u{a}")).collect::<Vec<_>>().join("|")).unwrap()
});

/// Findings that wait for a trailing comment to excuse them: `(line to key on, position)`.
#[derive(Default)]
pub struct State {
    pending: Vec<(u32, u32, u32)>,
}

fn is_ws(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

/// Checkstyle's `stripIndentAndInitialNewLineFromTextBlock`, which it applies to every literal.
fn strip_indent(content: &str) -> String {
    let mut text = content.to_string();
    // The first line break goes; the rest split the content into lines (empty trailing ones dropped).
    let breaks = |s: &str| -> Vec<(usize, usize)> {
        let b = s.as_bytes();
        let mut out = Vec::new();
        let mut i = 0;
        while i < b.len() {
            match b[i] {
                b'\r' if b.get(i + 1) == Some(&b'\n') => {
                    out.push((i, 2));
                    i += 2;
                }
                b'\n' | b'\r' | 0x0B | 0x0C => {
                    out.push((i, 1));
                    i += 1;
                }
                _ => {
                    let c = s[i..].chars().next().unwrap();
                    if matches!(c, '\u{85}' | '\u{2028}' | '\u{2029}') {
                        out.push((i, c.len_utf8()));
                    }
                    i += c.len_utf8();
                }
            }
        }
        out
    };
    if let Some(&(at, len)) = breaks(&text).first() {
        text.replace_range(at..at + len, "");
    }
    let mut lines: Vec<&str> = Vec::new();
    let mut last = 0;
    for (at, len) in breaks(&text) {
        lines.push(&text[last..at]);
        last = at + len;
    }
    lines.push(&text[last..]);
    while lines.len() > 1 && lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    if lines.len() == 1 && lines[0].is_empty() && text.is_empty() {
        // Java's split of "" gives [""], which stays.
    }
    let indent = lines.iter().map(|l| l.chars().take_while(|c| is_ws(*c)).map(char::len_utf8).sum::<usize>()).min().unwrap_or(0);
    lines
        .iter()
        .map(|l| {
            let end = l.trim_end_matches(is_ws).len();
            if end > 0 {
                l.get(indent..end).unwrap_or("").to_string()
            } else {
                String::new()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn has_unicode(literal: &str) -> bool {
    UNICODE.is_match(&literal.replace("\\\\", ""))
}

fn only_valid(literal: &str, pattern: &Regex) -> bool {
    UNICODE.find_iter(literal).count() == pattern.find_iter(literal).count()
}

fn excused(opts: &AvoidEscapedUnicode, literal: &str) -> bool {
    (opts.allow_if_all_characters_escaped && ALL_ESCAPED.is_match(literal))
        || (opts.allow_escapes_for_control_characters && only_valid(literal, &CONTROL))
        || (opts.allow_non_printable_escapes && only_valid(literal, &NON_PRINTABLE))
}

/// A string or character literal (a text block is a string literal that opens with three quotes).
pub fn on_literal(cx: &mut Ctx, st: &mut State, node: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.avoid_escaped_unicode else { return };
    let raw = String::from_utf8_lossy(cx.text(node)).into_owned();
    let text_block = node.kind() == "string_literal" && raw.starts_with("\"\"\"");
    let literal = if text_block { strip_indent(raw.get(3..raw.len().saturating_sub(3)).unwrap_or("")) } else { strip_indent(&raw) };
    if !has_unicode(&literal) || excused(opts, &literal) {
        return;
    }
    let start = node.start_position();
    let (line, col) = if text_block { (start.row as u32 + 1, start.column as u32 + 4) } else { (start.row as u32 + 1, start.column as u32 + 1) };
    if opts.allow_by_tail_comment {
        let key = if text_block { node.end_position().row as u32 + 1 } else { line };
        st.pending.push((key, line, col));
    } else {
        cx.diag_at("AvoidEscapedUnicodeCharacters", line, col, "Unicode escape(s) usage should be avoided.".to_string());
    }
}

/// A comment: its escapes count like a literal's, and a trailing one excuses the line's literals.
pub fn on_comment(cx: &mut Ctx, st: &mut State, node: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.avoid_escaped_unicode else { return };
    let _ = opts;
    // `isTrailingComment`: nothing follows it on its line. What follows is the next sibling in
    // Checkstyle's tree, which for an infix operator (`.`, `+`, `?`, `[`...) sits on the comment's
    // own line whatever the source says: the comment is never trailing before one.
    let mut next = node.next_sibling();
    let mut up = node.parent();
    while next.is_none() {
        let Some(p) = up else { break };
        next = p.next_sibling();
        up = p.parent();
    }
    let line = node.start_position().row as u32 + 1;
    let trailing = match next {
        None => true,
        Some(n) if n.kind() == "line_comment" || n.kind() == "block_comment" => n.start_position().row as u32 + 1 != line,
        Some(n) => !starts_with_infix(n) && n.start_position().row as u32 + 1 != line,
    };
    if trailing {
        st.pending.retain(|(key, _, _)| *key != line);
    }
}

/// Whether the node's first token is an infix operator with a left operand before it.
fn starts_with_infix(node: Node) -> bool {
    let mut leaf = node;
    while let Some(c) = leaf.child(0) {
        leaf = c;
    }
    let Some(parent) = leaf.parent() else { return false };
    let mut before = leaf.prev_sibling();
    while before.is_some_and(|p| matches!(p.kind(), "line_comment" | "block_comment")) {
        before = before.and_then(|p| p.prev_sibling());
    }
    let has_left = before.is_some();
    match leaf.kind() {
        "." => has_left && matches!(parent.kind(), "method_invocation" | "field_access" | "scoped_identifier" | "scoped_type_identifier"),
        "?" => parent.kind() == "ternary_expression",
        "instanceof" => parent.kind() == "instanceof_expression",
        "[" => has_left && parent.kind() == "array_access",
        "++" | "--" => has_left && parent.kind() == "update_expression",
        "::" => parent.kind() == "method_reference",
        "->" => has_left && parent.kind() == "lambda_expression",
        "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>=" => parent.kind() == "assignment_expression",
        "+" | "-" | "*" | "/" | "%" | "<<" | ">>" | ">>>" | "&" | "|" | "^" | "&&" | "||" | "==" | "!=" | "<" | ">" | "<=" | ">=" => parent.kind() == "binary_expression",
        _ => false,
    }
}

pub fn finish(cx: &mut Ctx, st: &mut State) {
    for (_, line, col) in std::mem::take(&mut st.pending) {
        cx.diag_at("AvoidEscapedUnicodeCharacters", line, col, "Unicode escape(s) usage should be avoided.".to_string());
    }
}
