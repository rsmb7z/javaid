//! The Javadoc style checks that read a comment's tags or its lines: `NonEmptyAtclauseDescription`,
//! `AtclauseOrder`, `SingleLineJavadoc` and `JavadocMissingLeadingAsterisk`.
//!
//! They run on every Javadoc that is in a position Checkstyle reads. A comment its grammar cannot
//! read gets one parse-error finding per such module, and nothing else (see `javadoc_tags`).

use crate::engine::Ctx;
use crate::rules::javadoc_tags::{self, Javadoc};
use crate::rules::{javadoc_tree, javadoc_tree_checks};
use crate::syntax::KindExt;
use tree_sitter::Node;

/// The Checkstyle token a declaration is, for `AtclauseOrder.target`.
fn token_of(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "class_declaration" => "CLASS_DEF",
        "interface_declaration" => "INTERFACE_DEF",
        "enum_declaration" => "ENUM_DEF",
        "method_declaration" => "METHOD_DEF",
        "constructor_declaration" => "CTOR_DEF",
        "field_declaration" | "constant_declaration" => "VARIABLE_DEF",
        "record_declaration" => "RECORD_DEF",
        "compact_constructor_declaration" => "COMPACT_CTOR_DEF",
        "annotation_type_declaration" => "ANNOTATION_DEF",
        "annotation_type_element_declaration" => "ANNOTATION_FIELD_DEF",
        "enum_constant" => "ENUM_CONSTANT_DEF",
        "package_declaration" => "PACKAGE_DEF",
        _ => return None,
    })
}

pub fn wanted(cx: &Ctx) -> bool {
    let c = cx.cfg;
    c.non_empty_atclause_description
        || c.atclause_order.is_some()
        || c.single_line_javadoc.is_some()
        || c.javadoc_missing_leading_asterisk
        || c.javadoc_asterisk_align.is_some()
        || c.require_empty_line_before_tag_group
        || c.summary_javadoc.is_some()
        || c.javadoc_paragraph.is_some()
        || c.javadoc_tag_continuation_indent.is_some()
}

/// `comment` is a Javadoc in a position Checkstyle reads; `decl` is the declaration it precedes.
pub fn on_javadoc(cx: &mut Ctx, comment: Node, decl: Node) {
    let cfg = cx.cfg;
    let text = String::from_utf8_lossy(cx.text(comment)).into_owned();
    let first_line = comment.start_position().row as u32 + 1;
    let doc = javadoc_tags::parse(&text, first_line);
    if let Some(e) = &doc.error {
        let msg = format!("Javadoc comment at column {} has parse error. Details: {}", e.col, e.details);
        if cfg.non_empty_atclause_description {
            cx.diag_at("NonEmptyAtclauseDescription", e.line, 0, msg.clone());
        }
        if cfg.atclause_order.is_some() {
            cx.diag_at("AtclauseOrder", e.line, 0, msg.clone());
        }
        if cfg.single_line_javadoc.is_some() {
            cx.diag_at("SingleLineJavadoc", e.line, 0, msg.clone());
        }
        if cfg.javadoc_missing_leading_asterisk {
            cx.diag_at("JavadocMissingLeadingAsterisk", e.line, 0, msg.clone());
        }
        if cfg.javadoc_asterisk_align.is_some() {
            cx.diag_at("JavadocLeadingAsteriskAlign", e.line, 0, msg.clone());
        }
        if cfg.require_empty_line_before_tag_group {
            cx.diag_at("RequireEmptyLineBeforeBlockTagGroup", e.line, 0, msg.clone());
        }
        if cfg.summary_javadoc.is_some() {
            cx.diag_at("SummaryJavadoc", e.line, 0, msg.clone());
        }
        if cfg.javadoc_paragraph.is_some() {
            cx.diag_at("JavadocParagraph", e.line, 0, msg.clone());
        }
        if cfg.javadoc_tag_continuation_indent.is_some() {
            cx.diag_at("JavadocTagContinuationIndentation", e.line, 0, msg);
        }
        return;
    }
    if cfg.non_empty_atclause_description {
        for tag in doc.tags.iter().filter(|t| matches!(t.name.as_str(), "param" | "return" | "throws" | "exception" | "deprecated" | "since")) {
            if !tag.described {
                cx.diag_at("NonEmptyAtclauseDescription", tag.line, 0, "At-clause should have a non-empty description.".to_string());
            }
        }
    }
    if let Some(opts) = &cfg.atclause_order {
        // Without modifiers, a generic method's Javadoc hangs on its TYPE_PARAMETERS, not on the METHOD_DEF.
        let on_type_params =
            matches!(decl.kind_s(), "method_declaration" | "constructor_declaration") && decl.named_child(0).is_some_and(|c| c.kind_s() == "type_parameters");
        if !on_type_params && token_of(decl.kind_s()).is_some_and(|t| opts.target.iter().any(|x| x == t)) {
            let mut max_seen = 0usize;
            for tag in &doc.tags {
                if let Some(i) = opts.tag_order.iter().position(|o| *o == format!("@{}", tag.name)) {
                    if i < max_seen {
                        let shown = format!("[{}]", opts.tag_order.join(", "));
                        cx.diag_at("AtclauseOrder", tag.line, 0, format!("Block tags have to appear in the order '{shown}'."));
                    } else {
                        max_seen = i;
                    }
                }
            }
        }
    }
    if let Some(opts) = &cfg.single_line_javadoc {
        single_line(cx, comment, &text, &doc, &opts.ignored_tags, opts.ignore_inline_tags, first_line);
    }
    if cfg.javadoc_missing_leading_asterisk {
        missing_asterisk(cx, &text, first_line);
    }
    if let Some(indentation) = cfg.javadoc_asterisk_align {
        asterisk_align(cx, comment, &text, first_line, indentation);
    }
    if cfg.require_empty_line_before_tag_group {
        empty_line_before_tags(cx, &text, &doc, first_line);
    }
    if (cfg.summary_javadoc.is_some() || cfg.javadoc_paragraph.is_some() || cfg.javadoc_tag_continuation_indent.is_some()) && text.len() >= 5 {
        tree_checks(cx, comment, &text, first_line);
    }
}

/// The modules that read the Javadoc tree: `SummaryJavadoc`, `JavadocParagraph` and
/// `JavadocTagContinuationIndentation`.
fn tree_checks(cx: &mut Ctx, comment: Node, text: &str, first_line: u32) {
    let cfg = cx.cfg;
    // The content is what lies between `/**` and `*/`, starting three columns in.
    let tree = javadoc_tree::parse(&text[3..text.len() - 2], first_line, comment.start_position().column as u32 + 3);
    let mut found: Vec<(&'static str, Vec<javadoc_tree_checks::Finding>)> = Vec::new();
    if let Some(opts) = &cfg.summary_javadoc {
        let mut out = Vec::new();
        javadoc_tree_checks::summary_javadoc(&tree, opts, &mut out);
        found.push(("SummaryJavadoc", out));
    }
    if let Some(allow_newline_paragraph) = cfg.javadoc_paragraph {
        let mut out = Vec::new();
        javadoc_tree_checks::javadoc_paragraph(&tree, allow_newline_paragraph, &mut out);
        found.push(("JavadocParagraph", out));
    }
    if let Some(offset) = cfg.javadoc_tag_continuation_indent {
        let mut out = Vec::new();
        javadoc_tree_checks::tag_continuation_indentation(&tree, offset, &mut out);
        found.push(("JavadocTagContinuationIndentation", out));
    }
    for (rule, findings) in found {
        // Checkstyle keeps a violation once however often a check reports it.
        let mut seen: Vec<(u32, u32, String)> = Vec::new();
        for f in findings {
            let key = (f.line, f.col, f.message.clone());
            if !seen.contains(&key) {
                seen.push(key);
                cx.diag_at(rule, f.line, f.col, f.message);
            }
        }
    }
}

/// `JavadocContentLocation`: every multi-line Javadoc, attached to a declaration or not.
pub fn content_location(cx: &mut Ctx, comment: Node, first_line: bool) {
    if comment.start_position().row == comment.end_position().row {
        return;
    }
    let text = String::from_utf8_lossy(cx.text(comment)).into_owned();
    let mut line_index = 0;
    let mut content_line = None;
    for c in text[..text.len() - 2].chars().skip(2) {
        if c == '\n' {
            line_index += 1;
        } else if c != '*' && !c.is_whitespace() {
            content_line = Some(line_index);
            break;
        }
    }
    let Some(at) = content_line else { return };
    // The closing `*/` is not content.
    if first_line && at != 0 {
        cx.diag("JavadocContentLocation", comment, "Javadoc content should start from the same line as /**.".to_string());
    } else if !first_line && at != 1 {
        cx.diag("JavadocContentLocation", comment, "Javadoc content should start from the next line after /**.".to_string());
    }
}

/// Width of `s` with tabs expanded to the next multiple of `tab_width`.
fn expanded_len(s: &str, tab_width: usize) -> usize {
    crate::columns::expanded_chars(s.as_bytes(), usize::MAX, tab_width).0
}

/// `JavadocLeadingAsteriskAlign`.
fn asterisk_align(cx: &mut Ctx, comment: Node, text: &str, first_line: u32, indentation: usize) {
    let tab = cx.cfg.walker_tab();
    let lines: Vec<&str> = text.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    // What stands before the comment on its first line.
    let line_start = cx.src[..comment.start_byte()].iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
    let before = String::from_utf8_lossy(&cx.src[line_start..comment.start_byte()]).into_owned();
    let start_col = 1 + expanded_len(&before, tab);
    // The opening line's first token: a leading asterisk after white space, or else the text after `/**`.
    let after: Vec<char> = lines[0].chars().skip(3).collect();
    let ws = after.iter().take_while(|c| **c == ' ' || **c == '\t').count();
    let opening_star = ws > 0 && after.get(ws) == Some(&'*') && after.get(ws + 1) != Some(&'/');
    let before_content = expanded_len(&format!("{before}{}", lines[0].chars().take(if opening_star { 2 + ws } else { 2 }).collect::<String>()), tab);
    let mut expected = if before_content != start_col + 1 { before_content } else { start_col + indentation };
    if opening_star {
        expected = 1 + expanded_len(&format!("{before}{}", lines[0].chars().take(3 + ws).collect::<String>()), tab);
    }
    let level = |col: usize| col as i64 - start_col as i64;
    for (i, line) in lines.iter().enumerate().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let lead = chars.iter().take_while(|c| **c == ' ' || **c == '\t').count();
        if chars.get(lead) != Some(&'*') {
            continue;
        }
        let col = 1 + expanded_len(&chars[..lead].iter().collect::<String>(), tab);
        if col != expected {
            cx.diag_at(
                "JavadocLeadingAsteriskAlign",
                first_line + i as u32,
                (lead + 1) as u32,
                format!("Leading asterisk has incorrect indentation level {}, expected is {}.", level(col), level(expected)),
            );
        }
    }
}

/// `RequireEmptyLineBeforeBlockTagGroup`: the first block tag needs a blank line above it, unless
/// nothing but the opening `/**` precedes it.
fn empty_line_before_tags(cx: &mut Ctx, text: &str, doc: &Javadoc, first_line: u32) {
    let Some(tag) = doc.tags.first() else { return };
    let lines: Vec<&str> = text.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    let idx = (tag.line - first_line) as usize;
    // Blank/asterisk-only lines directly above the tag's line.
    let mut newlines = 0;
    let mut j = idx;
    let mut reached_start = false;
    while j > 0 {
        newlines += 1;
        let above = lines[j - 1];
        let body = if j - 1 == 0 { above.get(3..).unwrap_or("") } else { above };
        if body.trim_matches(|c: char| c.is_whitespace() || c == '*').is_empty() {
            j -= 1;
            if j == 0 {
                reached_start = true;
            }
        } else {
            break;
        }
    }
    // Tag text on the opening line itself.
    if idx == 0 {
        newlines = 0;
    }
    let only_tag = reached_start && newlines == 1;
    if newlines <= 1 && !only_tag {
        let name = format!("@{}", tag.name);
        cx.diag_at("RequireEmptyLineBeforeBlockTagGroup", tag.line, 0, format!("Javadoc tag '{name}' should be preceded with an empty line."));
    }
}

fn single_line(cx: &mut Ctx, comment: Node, text: &str, doc: &Javadoc, ignored: &[String], ignore_inline: bool, first_line: u32) {
    if comment.start_position().row != comment.end_position().row {
        return;
    }
    let block = doc.tags.iter().any(|t| !ignored.iter().any(|i| *i == format!("@{}", t.name)));
    let inline = !ignore_inline && inline_tag_names(text).iter().any(|n| !ignored.iter().any(|i| *i == format!("@{n}")));
    // Only the first block tag decides: Checkstyle looks at that one.
    let first_block_ignored = doc.tags.first().is_some_and(|t| ignored.iter().any(|i| *i == format!("@{}", t.name)));
    if (block && !first_block_ignored) || inline {
        cx.diag_at("SingleLineJavadoc", first_line, 0, "Single-line Javadoc comment should be multi-line.".to_string());
    }
}

fn inline_tag_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b'{' && b[i + 1] == b'@' {
            let name: String = text[i + 2..].chars().take_while(char::is_ascii_alphabetic).collect();
            out.push(name);
        }
        i += 1;
    }
    out
}

/// A line after the first that does not start with `*` (outside an HTML comment), unless it is
/// only the white space before the closing `*/`.
fn missing_asterisk(cx: &mut Ctx, text: &str, first_line: u32) {
    let lines: Vec<&str> = text.split('\n').collect();
    let last = lines.len() - 1;
    let mut in_html_comment = false;
    for (i, raw) in lines.iter().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        let starts_in_comment = in_html_comment;
        if let Some(open) = line.rfind("<!--") {
            if !line[open..].contains("-->") {
                in_html_comment = true;
            }
        }
        if line.contains("-->") && in_html_comment && line.rfind("-->").is_some_and(|c| line.rfind("<!--").is_none_or(|o| c > o)) {
            in_html_comment = false;
        }
        if i == 0 || starts_in_comment {
            continue;
        }
        let trimmed = line.trim_start_matches([' ', '\t']);
        if trimmed.starts_with('*') {
            continue;
        }
        let body = if i == last { trimmed.strip_suffix("*/").unwrap_or(trimmed) } else { trimmed };
        let blank = body.trim().is_empty();
        if blank && i == last {
            continue;
        }
        cx.diag_at("JavadocMissingLeadingAsterisk", first_line + i as u32, 0, "Javadoc line should start with leading asterisk.".to_string());
    }
}

/// `MultilineCommentLeadingAsteriskPresence`: every block comment that is not a well-placed Javadoc.
pub fn multiline_asterisk(cx: &mut Ctx, comment: Node) {
    let text = String::from_utf8_lossy(cx.text(comment)).into_owned();
    let Some(content) = text.get(2..text.len().saturating_sub(2)) else { return };
    let start = comment.start_position().row as u32 + 1;
    let end = comment.end_position().row as u32 + 1;
    // Java's String.lines(): split on \n, \r and \r\n; a final empty line is dropped.
    let mut lines: Vec<&str> = Vec::new();
    let mut rest = content;
    while !rest.is_empty() {
        match rest.find(['\n', '\r']) {
            Some(i) => {
                lines.push(&rest[..i]);
                let skip = if rest[i..].starts_with("\r\n") { 2 } else { 1 };
                rest = &rest[i + skip..];
            }
            None => {
                lines.push(rest);
                break;
            }
        }
    }
    let mut missing = Vec::new();
    for (cur, line) in lines.iter().enumerate().skip(1) {
        let number = start + cur as u32;
        let line = line.trim_matches(|c: char| c <= ' ');
        if number == end && line.is_empty() {
            continue;
        }
        if !line.starts_with('*') {
            missing.push(number.to_string());
        }
    }
    if !missing.is_empty() {
        cx.diag(
            "MultilineCommentLeadingAsteriskPresence",
            comment,
            format!("Multiline comment line(s) {} should start with leading asterisk.", missing.join(", ")),
        );
    }
}
