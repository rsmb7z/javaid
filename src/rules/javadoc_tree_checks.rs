// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/javadoc/SummaryJavadocCheck.java
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/javadoc/JavadocParagraphCheck.java
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/javadoc/JavadocTagContinuationIndentationCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! The Checkstyle modules that walk the Javadoc tree (`AbstractJavadocCheck` subclasses):
//! `SummaryJavadoc`, `JavadocParagraph` and `JavadocTagContinuationIndentation`. Each is a port of
//! the Java check, node for node, on the tree `javadoc_tree` builds.

use crate::config::SummaryJavadoc;
use crate::rules::javadoc_tree::{is_blank, is_java_ws, Id, Tree, Ty};

/// One finding: the line it is reported on, its column (1-based, 0 for none), and the message.
pub struct Finding {
    pub line: u32,
    pub col: u32,
    pub message: String,
}

fn at_node(tree: &Tree, id: Id, message: String) -> Finding {
    let n = tree.node(id);
    Finding { line: n.line, col: n.col + 1, message }
}

// ---- SummaryJavadoc --------------------------------------------------------------------------

const MSG_SUMMARY_FIRST_SENTENCE: &str = "First sentence of Javadoc is missing an ending period.";
const MSG_SUMMARY_JAVADOC: &str = "Forbidden summary fragment.";
const MSG_SUMMARY_JAVADOC_MISSING: &str = "Summary javadoc is missing.";
const MSG_SUMMARY_MISSING_PERIOD: &str = "Summary of Javadoc is missing an ending period.";

pub fn summary_javadoc(tree: &Tree, opts: &SummaryJavadoc, out: &mut Vec<Finding>) {
    let period = opts.period.as_str();
    let mut validate_untagged = true;
    for id in tree.preorder() {
        match tree.ty(id) {
            Ty::SummaryInlineTag if tree.parent(id).is_some_and(|p| is_defined_first(tree, p)) => {
                validate_untagged = false;
                validate_summary_tag(tree, id, opts, out);
            }
            Ty::ReturnInlineTag => {
                validate_untagged = false;
                validate_inline_return_tag(tree, id, opts, out);
            }
            _ => {}
        }
    }
    // Leaving JAVADOC_CONTENT.
    if validate_untagged && !starts_with_inherit_doc(tree) {
        validate_untagged_summary(tree, opts, period, out);
    }
}

fn validate_untagged_summary(tree: &Tree, opts: &SummaryJavadoc, period: &str, out: &mut Vec<Finding>) {
    let summary_doc = summary_sentence(tree);
    if summary_doc.is_empty() {
        out.push(at_node(tree, Tree::ROOT, MSG_SUMMARY_JAVADOC_MISSING.to_string()));
    } else if !period.is_empty() {
        if summary_doc.contains(period) {
            match first_sentence(tree, period) {
                Some(first) => {
                    if contains_forbidden_fragment(opts, &first) {
                        out.push(at_node(tree, Tree::ROOT, MSG_SUMMARY_JAVADOC.to_string()));
                    }
                }
                None => out.push(at_node(tree, Tree::ROOT, MSG_SUMMARY_FIRST_SENTENCE.to_string())),
            }
        } else {
            out.push(at_node(tree, Tree::ROOT, MSG_SUMMARY_FIRST_SENTENCE.to_string()));
        }
    }
}

/// `String.trim()`: strips characters up to U+0020 from both ends.
fn java_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c <= ' ')
}

fn is_defined_first(tree: &Tree, inline_tag: Id) -> bool {
    let mut defined = true;
    let mut cur = tree.prev_sibling(inline_tag);
    while let Some(c) = cur {
        if !defined {
            break;
        }
        match tree.ty(c) {
            Ty::Text => defined = is_blank(tree.text(c)),
            Ty::HtmlElement => defined = is_html_tag_without_text(tree, c),
            Ty::LeadingAsterisk | Ty::LeadingAsterisks | Ty::Newline => {}
            _ => defined = false,
        }
        cur = tree.prev_sibling(c);
    }
    defined
}

fn is_html_tag_without_text(tree: &Tree, node: Id) -> bool {
    match tree.find_first(node, Ty::HtmlContent) {
        Some(content) => match tree.first_child(content) {
            Some(child) => tree.ty(child) == Ty::HtmlElement && is_html_tag_without_text(tree, child),
            None => true,
        },
        None => true,
    }
}

fn validate_summary_tag(tree: &Tree, tag: Id, opts: &SummaryJavadoc, out: &mut Vec<Finding>) {
    let period = opts.period.as_str();
    let description = tree.find_first(tag, Ty::Description);
    let inline_summary = content_of_inline_tag(tree, description);
    let visible = visible_content(&inline_summary);
    if visible.is_empty() {
        out.push(at_node(tree, tag, MSG_SUMMARY_JAVADOC_MISSING.to_string()));
    } else if !period.is_empty() {
        let chars: Vec<char> = visible.chars().collect();
        let period_chars: Vec<char> = period.chars().collect();
        let last = (0..chars.len().saturating_sub(period_chars.len() - 1)).rev().find(|i| chars[*i..].starts_with(&period_chars));
        let not_at_end = last.map(|i| i as i64).unwrap_or(-1) != chars.len() as i64 - 1;
        if not_at_end {
            out.push(at_node(tree, tag, MSG_SUMMARY_MISSING_PERIOD.to_string()));
        } else if contains_forbidden_fragment(opts, &inline_summary) {
            out.push(at_node(tree, tag, MSG_SUMMARY_JAVADOC.to_string()));
        }
    }
}

fn validate_inline_return_tag(tree: &Tree, tag: Id, opts: &SummaryJavadoc, out: &mut Vec<Finding>) {
    let description = tree.find_first(tag, Ty::Description);
    let inline_return = content_of_inline_tag(tree, description);
    let visible = visible_content(&inline_return);
    if visible.is_empty() {
        out.push(at_node(tree, tag, MSG_SUMMARY_JAVADOC_MISSING.to_string()));
    } else if contains_forbidden_fragment(opts, &format!("Returns {inline_return}")) {
        out.push(at_node(tree, tag, MSG_SUMMARY_JAVADOC.to_string()));
    }
}

/// The text of every leaf under the description, except leading asterisks.
fn content_of_inline_tag(tree: &Tree, description: Option<Id>) -> String {
    let mut out = String::new();
    if let Some(d) = description {
        let mut stack = vec![d];
        while let Some(n) = stack.pop() {
            let node = tree.node(n);
            if node.children.is_empty() {
                if !matches!(node.ty, Ty::LeadingAsterisk | Ty::LeadingAsterisks) {
                    out.push_str(&node.text);
                }
            } else {
                for c in node.children.iter().rev() {
                    stack.push(*c);
                }
            }
        }
    }
    out
}

/// What the reader sees: `<[^>]*>` removed, then trimmed.
fn visible_content(summary: &str) -> String {
    let mut out = String::with_capacity(summary.len());
    let chars: Vec<char> = summary.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '<' {
            if let Some(j) = chars[i + 1..].iter().position(|c| *c == '>') {
                i += j + 2;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    java_trim(&out).to_string()
}

fn contains_forbidden_fragment(opts: &SummaryJavadoc, sentence: &str) -> bool {
    let text = single_line(sentence);
    opts.forbidden_fragments.matches(trim_excess_whitespace(&text).as_bytes())
}

/// `\n[ \t]*(\*)|^[ \t]*(\*)` replaced by one space: a line break with its asterisk, or an asterisk at the
/// very start.
fn single_line(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        let mut matched = None;
        if chars[i] == '\n' {
            let mut j = i + 1;
            while j < chars.len() && (chars[j] == ' ' || chars[j] == '\t') {
                j += 1;
            }
            if j < chars.len() && chars[j] == '*' {
                matched = Some(j + 1);
            }
        }
        if matched.is_none() && i == 0 {
            let mut j = 0;
            while j < chars.len() && (chars[j] == ' ' || chars[j] == '\t') {
                j += 1;
            }
            if j < chars.len() && chars[j] == '*' {
                matched = Some(j + 1);
            }
        }
        match matched {
            Some(end) => {
                out.push(' ');
                i = end;
            }
            None => {
                out.push(chars[i]);
                i += 1;
            }
        }
    }
    out
}

fn trim_excess_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut previous_whitespace = true;
    for c in text.chars() {
        if is_java_ws(c) {
            if previous_whitespace {
                continue;
            }
            previous_whitespace = true;
            out.push(' ');
        } else {
            previous_whitespace = false;
            out.push(c);
        }
    }
    out
}

fn starts_with_inherit_doc(tree: &Tree) -> bool {
    let mut found = false;
    let mut node = tree.first_child(Tree::ROOT);
    while let Some(n) = node {
        if tree.ty(n) == Ty::JavadocInlineTag && tree.first_child(n).is_some_and(|c| tree.ty(c) == Ty::InheritDocInlineTag) {
            found = true;
        }
        if matches!(tree.ty(n), Ty::Text | Ty::HtmlElement) && !is_blank(tree.text(n)) {
            break;
        }
        node = tree.next_sibling(n);
    }
    found
}

fn summary_sentence(tree: &Tree) -> String {
    let mut result = String::new();
    let mut node = tree.first_child(Tree::ROOT);
    while let Some(n) = node {
        if tree.ty(n) == Ty::Text {
            result.push_str(tree.text(n));
        } else {
            let summary = result.clone();
            if is_blank(&summary) && tree.ty(n) == Ty::HtmlElement {
                let content = tree.find_first(n, Ty::HtmlContent);
                result.push_str(&string_inside_html_tag(tree, &summary, content));
            }
        }
        node = tree.next_sibling(n);
    }
    java_trim(&result).to_string()
}

fn string_inside_html_tag(tree: &Tree, result: &str, content: Option<Id>) -> String {
    let mut contents = result.to_string();
    if let Some(c) = content {
        let mut temp = tree.first_child(c);
        while let Some(t) = temp {
            if tree.ty(t) == Ty::Text {
                contents.push_str(tree.text(t));
            } else {
                let inner = tree.find_first(t, Ty::HtmlContent);
                contents.push_str(&string_inside_html_tag(tree, "", inner));
            }
            temp = tree.next_sibling(t);
        }
    }
    contents
}

/// The first sentence: all text up to the sentence-ending period, or `None` if there is none.
fn first_sentence(tree: &Tree, period: &str) -> Option<String> {
    let mut parts = String::new();
    let mut stack = vec![Tree::ROOT];
    while let Some(n) = stack.pop() {
        let node = tree.node(n);
        if node.children.is_empty() {
            if let Some(end) = sentence_ending(&node.text, period) {
                parts.push_str(&end);
                return Some(parts);
            }
            parts.push_str(&node.text);
        } else {
            for c in node.children.iter().rev() {
                stack.push(*c);
            }
        }
    }
    None
}

fn sentence_ending(text: &str, period: &str) -> Option<String> {
    let mut from = 0;
    while let Some(rel) = text[from..].find(period) {
        let at = from + rel;
        let after = at + period.len();
        if period != "." || after >= text.len() || text[after..].chars().next().is_some_and(is_java_ws) {
            return Some(text[..at].to_string());
        }
        from = after;
    }
    None
}

// ---- JavadocParagraph ------------------------------------------------------------------------

const MSG_TAG_AFTER: &str = "Empty line should be followed by <p> tag on the next line.";
const MSG_LINE_BEFORE: &str = "<p> tag should be preceded with an empty line.";
const MSG_REDUNDANT_PARAGRAPH: &str = "Redundant <p> tag.";
const MSG_MISPLACED_TAG: &str = "<p> tag should be placed immediately before the first word, with no space after.";

const BLOCK_TAGS: [&str; 16] = ["address", "blockquote", "div", "dl", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "ol", "p", "pre", "table", "ul"];

pub fn javadoc_paragraph(tree: &Tree, allow_newline_paragraph: bool, out: &mut Vec<Finding>) {
    for id in tree.preorder() {
        let ty = tree.ty(id);
        if ty == Ty::Newline && is_empty_line(tree, id) {
            check_empty_line(tree, id, out);
        } else if ty == Ty::HtmlElement && tree.is_tag(id, "p") {
            check_paragraph_tag(tree, id, allow_newline_paragraph, out);
        }
    }
}

fn check_empty_line(tree: &Tree, newline: Id, out: &mut Vec<Finding>) {
    if let Some(nearest) = nearest_node(tree, newline) {
        if tree.ty(nearest) == Ty::Text && !is_blank(tree.text(nearest)) {
            out.push(at_node(tree, newline, MSG_TAG_AFTER.to_string()));
        }
    }
}

fn check_paragraph_tag(tree: &Tree, tag: Id, allow_newline_paragraph: bool, out: &mut Vec<Finding>) {
    if is_nested_paragraph(tree, tag) || is_inside_block_tag(tree, tag) {
        return;
    }
    let new_line = nearest_empty_line(tree, tag);
    if is_first_paragraph(tree, tag) {
        out.push(at_node(tree, tag, MSG_REDUNDANT_PARAGRAPH.to_string()));
    } else if new_line.is_none_or(|n| tree.node(tag).line as i64 - tree.node(n).line as i64 != 1) {
        out.push(at_node(tree, tag, MSG_LINE_BEFORE.to_string()));
    }
    if let Some(name) = followed_block_tag_name(tree, tag) {
        out.push(at_node(tree, tag, format!("<p> tag should not precede HTML block-tag '<{name}>', <p> tag should be removed.")));
    }
    if !allow_newline_paragraph && immediately_followed_by_newline(tree, tag) {
        out.push(at_node(tree, tag, MSG_MISPLACED_TAG.to_string()));
    }
    if immediately_followed_by_text(tree, tag) {
        out.push(at_node(tree, tag, MSG_MISPLACED_TAG.to_string()));
    }
}

fn is_nested_paragraph(tree: &Tree, tag: Id) -> bool {
    let mut parent = tree.parent(tag);
    while let Some(p) = parent {
        if tree.ty(p) == Ty::HtmlElement {
            return true;
        }
        parent = tree.parent(p);
    }
    false
}

fn is_inside_block_tag(tree: &Tree, tag: Id) -> bool {
    let mut cur = Some(tag);
    while let Some(c) = cur {
        if tree.ty(c) == Ty::JavadocBlockTag {
            return true;
        }
        cur = tree.parent(c);
    }
    false
}

fn followed_block_tag_name(tree: &Tree, tag: Id) -> Option<String> {
    let element = first_html_element_after(tree, tag)?;
    let start = tree.first_child(element)?;
    let name = tree.find_first(start, Ty::TagName)?;
    let text = tree.text(name);
    BLOCK_TAGS.contains(&text).then(|| text.to_string())
}

fn first_html_element_after(tree: &Tree, tag: Id) -> Option<Id> {
    let mut html = paragraph_next_sibling(tree, tag);
    while let Some(h) = html {
        if tree.ty(h) == Ty::HtmlElement {
            break;
        }
        if tree.ty(h) == Ty::HtmlContent {
            html = tree.first_child(h);
        } else if tree.ty(h) == Ty::Text && !is_blank(tree.text(h)) {
            html = None;
            break;
        } else {
            html = tree.next_sibling(h);
        }
    }
    let h = html?;
    tree.find_first(h, Ty::HtmlTagEnd).map(|_| h)
}

fn nearest_node(tree: &Tree, node: Id) -> Option<Id> {
    let mut cur = Some(node);
    while let Some(c) = cur {
        if matches!(tree.ty(c), Ty::LeadingAsterisk | Ty::Newline) {
            cur = tree.next_sibling(c);
        } else {
            break;
        }
    }
    match cur {
        Some(c) if tree.ty(c) == Ty::HtmlContent => tree.first_child(c),
        other => other,
    }
}

fn is_empty_line(tree: &Tree, newline: Id) -> bool {
    let Some(mut prev) = tree.prev_sibling(newline) else { return false };
    let parent = tree.parent(prev).expect("a sibling has a parent");
    if tree.ty(parent) == Ty::JavadocContent || inside_non_tight_html(tree, prev) {
        if tree.ty(prev) == Ty::Text && is_blank(tree.text(prev)) {
            match tree.prev_sibling(prev) {
                Some(p) => prev = p,
                None => return false,
            }
        }
        return tree.ty(prev) == Ty::LeadingAsterisk;
    }
    false
}

fn inside_non_tight_html(tree: &Tree, node: Id) -> bool {
    let parent = tree.parent(node).expect("a sibling has a parent");
    let html = if tree.ty(parent) == Ty::HtmlContent { tree.parent(parent).unwrap_or(parent) } else { parent };
    tree.ty(html) == Ty::HtmlElement && tree.find_first(html, Ty::HtmlTagEnd).is_none()
}

fn is_first_paragraph(tree: &Tree, tag: Id) -> bool {
    let mut prev = tree.prev_sibling(tag);
    while let Some(p) = prev {
        let ty = tree.ty(p);
        if (ty == Ty::Text && !is_blank(tree.text(p))) || !matches!(ty, Ty::LeadingAsterisk | Ty::Newline | Ty::Text) {
            return false;
        }
        prev = tree.prev_sibling(p);
    }
    true
}

fn nearest_empty_line(tree: &Tree, node: Id) -> Option<Id> {
    let mut cur = Some(node);
    while let Some(c) = cur {
        if tree.ty(c) == Ty::Newline && is_empty_line(tree, c) {
            return Some(c);
        }
        cur = tree.prev_sibling(c);
    }
    None
}

fn immediately_followed_by_text(tree: &Tree, tag: Id) -> bool {
    paragraph_next_sibling(tree, tag).is_none_or(|n| tree.text(n).starts_with(' '))
}

fn immediately_followed_by_newline(tree: &Tree, tag: Id) -> bool {
    paragraph_next_sibling(tree, tag).is_some_and(|n| tree.ty(n) == Ty::Newline)
}

/// What follows the paragraph's start tag: the first node of its content, or what follows the element.
fn paragraph_next_sibling(tree: &Tree, tag: Id) -> Option<Id> {
    let start = tree.first_child(tag)?;
    let mut next = match tree.next_sibling(start) {
        None => tree.next_sibling(tag),
        Some(n) if tree.ty(n) == Ty::HtmlContent => tree.first_child(n),
        Some(n) => Some(n),
    };
    if let Some(n) = next {
        if tree.ty(n) == Ty::HtmlComment {
            next = tree.next_sibling(n);
        }
    }
    next
}

// ---- JavadocTagContinuationIndentation -------------------------------------------------------

const MSG_CONTINUATION_PREFIX: &str = "Line continuation have incorrect indentation level, expected level should be ";

pub fn tag_continuation_indentation(tree: &Tree, offset: usize, out: &mut Vec<Finding>) {
    for id in tree.preorder() {
        let ty = tree.ty(id);
        if !matches!(ty, Ty::HtmlElement | Ty::Description) {
            continue;
        }
        if is_block_description(tree, id) && !is_inline_description(tree, id) {
            let nodes = if ty == Ty::Description { targeted_in_description(tree, id) } else { targeted_in_html_element(tree, id) };
            for n in nodes {
                if is_violation(tree, n, offset) {
                    out.push(Finding { line: tree.node(n).line, col: 0, message: format!("{MSG_CONTINUATION_PREFIX}{offset}.") });
                }
            }
        }
    }
}

fn is_block_description(tree: &Tree, node: Id) -> bool {
    let mut cur = Some(node);
    while let Some(c) = cur {
        if tree.ty(c) == Ty::JavadocBlockTag {
            return true;
        }
        cur = tree.parent(c);
    }
    false
}

fn is_inline_description(tree: &Tree, node: Id) -> bool {
    let mut cur = Some(node);
    while let Some(c) = cur {
        if tree.ty(c) == Ty::JavadocInlineTag {
            return true;
        }
        cur = tree.parent(c);
    }
    false
}

fn is_leading_asterisk(tree: &Tree, node: Id) -> bool {
    matches!(tree.ty(node), Ty::LeadingAsterisk | Ty::LeadingAsterisks)
}

fn is_inside_pre_tag(tree: &Tree, node: Id) -> bool {
    tree.parent(node).and_then(|p| tree.parent(p)).is_some_and(|gp| tree.is_tag(gp, "pre"))
}

fn is_target_text_node(tree: &Tree, node: Id) -> bool {
    let Some(prev) = tree.prev_sibling(node) else { return false };
    matches!(tree.ty(node), Ty::Text | Ty::AttributeValue) && !is_before_pre_tag(tree, node) && is_leading_asterisk(tree, prev)
}

fn is_before_pre_tag(tree: &Tree, node: Id) -> bool {
    match tree.next_sibling(node) {
        Some(next) if tree.ty(next) == Ty::Description => tree.first_child(next).is_some_and(|c| tree.is_tag(c, "pre")),
        Some(next) => tree.is_tag(next, "pre"),
        None => false,
    }
}

fn subtree_contains_attribute_value(tree: &Tree, node: Id) -> bool {
    matches!(tree.ty(node), Ty::HtmlTagStart | Ty::HtmlAttributes | Ty::HtmlAttribute)
}

fn targeted_in_html_element(tree: &Tree, ast: Id) -> Vec<Id> {
    let mut out = Vec::new();
    if !tree.is_tag(ast, "pre") && !is_inside_pre_tag(tree, ast) {
        if let Some(prev) = tree.prev_sibling(ast) {
            if is_target_text_node(tree, prev) {
                out.push(prev);
            }
        }
        let mut node = tree.first_child(ast);
        while let Some(n) = node {
            if tree.ty(n) == Ty::HtmlContent {
                out.extend(targeted_in_description(tree, n));
            } else if subtree_contains_attribute_value(tree, n) {
                out.extend(targeted_in_html_element(tree, n));
            } else if is_target_text_node(tree, n) {
                out.push(n);
            }
            node = tree.next_sibling(n);
        }
    }
    out
}

fn targeted_in_description(tree: &Tree, description: Id) -> Vec<Id> {
    let mut out = Vec::new();
    let first = tree.first_child(description);
    let previous = tree.prev_sibling(description);
    if let Some(p) = previous {
        // A description always has a token before it (the tag name at least).
        if is_target_text_node(tree, p) {
            out.push(p);
        }
        // The first child's leading asterisk is the description's previous sibling.
        if is_leading_asterisk(tree, p) {
            if let Some(f) = first {
                out.push(f);
            }
        }
    }
    let mut node = first;
    while let Some(n) = node {
        if is_target_text_node(tree, n) {
            out.push(n);
        }
        node = tree.next_sibling(n);
    }
    out
}

fn is_violation(tree: &Tree, node: Id, offset: usize) -> bool {
    let text: Vec<u16> = tree.text(node).encode_utf16().collect();
    let blank = |units: &[u16]| units.iter().all(|u| char::from_u32(*u as u32).is_some_and(is_java_ws));
    if text.len() <= offset {
        if blank(&text) {
            // The text is blank: only a violation when the line has not ended.
            tree.next_sibling(node).is_none_or(|n| tree.ty(n) != Ty::Newline)
        } else {
            true
        }
    } else {
        !blank(&text[1..offset + 1])
    }
}
