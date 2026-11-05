//! `GoogleNonConstantFieldName` and `GoogleMethodName`: the Google style guide's camel case, with
//! its rules about underscores and numbering suffixes. Checkstyle spells them as regular
//! expressions with look-ahead and possessive quantifiers; the same strings are matched here
//! without them (the character classes at each seam are disjoint, so no backtracking differs).

use crate::engine::Ctx;
use crate::syntax::KindExt;
use regex::Regex;
use std::sync::LazyLock;
use tree_sitter::Node;

static UNDERSCORE_REGULAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^_|_$|__|[a-zA-Z]_[a-zA-Z]|[a-zA-Z]_[0-9]|[0-9]_[a-zA-Z]").unwrap());
static UNDERSCORE_TEST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^_|_$|__|[a-zA-Z]_[0-9]|[0-9]_[a-zA-Z]").unwrap());
static REGULAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9]+(?:[A-Z][a-z0-9]+)*[A-Z]?$").unwrap());
static TEST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-z][a-z0-9]+(?:[A-Z][a-z0-9]+)*[A-Z]?$|[a-z][a-z0-9]+(?:[A-Z][a-z0-9]+)*[A-Z]?(?:_[a-z][a-z0-9]+(?:[A-Z][a-z0-9]+)*[A-Z]?)+$)").unwrap()
});
static FIELD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap());

const TEST_ANNOTATIONS: &[&str] = &[
    "Test",
    "org.junit.jupiter.api.Test",
    "org.junit.Test",
    "ParameterizedTest",
    "org.junit.jupiter.params.ParameterizedTest",
    "RepeatedTest",
    "org.junit.jupiter.api.RepeatedTest",
    "TestFactory",
    "org.junit.jupiter.api.TestFactory",
];

/// `(?:_[0-9]++)+$` replaced by nothing: the trailing `_1_2` groups.
fn strip_numbering(name: &str) -> &str {
    let mut s = name;
    loop {
        let digits = s.bytes().rev().take_while(u8::is_ascii_digit).count();
        if digits > 0 && s.len() > digits && s.as_bytes()[s.len() - digits - 1] == b'_' {
            s = &s[..s.len() - digits - 1];
        } else {
            return s;
        }
    }
}

fn annotation_names(cx: &Ctx, decl: Node) -> Vec<String> {
    let mut cur = decl.walk();
    let Some(mods) = decl.children(&mut cur).find(|c| c.kind_s() == "modifiers") else { return Vec::new() };
    let mut inner = mods.walk();
    mods.children(&mut inner)
        .filter(|c| matches!(c.kind_s(), "marker_annotation" | "annotation"))
        .filter_map(|c| c.child_by_field_id(cx.syn.f.name))
        .map(|n| String::from_utf8_lossy(cx.text(n)).into_owned())
        .collect()
}

pub fn on_method(cx: &mut Ctx, node: Node) {
    if !cx.cfg.google_method_name {
        return;
    }
    let names = annotation_names(cx, node);
    if names.iter().any(|a| a == "Override" || a == "java.lang.Override") {
        return;
    }
    let Some(name) = node.child_by_field_id(cx.syn.f.name) else { return };
    let text = String::from_utf8_lossy(cx.text(name)).into_owned();
    let is_test = names.iter().any(|a| TEST_ANNOTATIONS.contains(&a.as_str()));
    let stripped = strip_numbering(&text);
    let (underscore, format) = if is_test { (&*UNDERSCORE_TEST, &*TEST) } else { (&*UNDERSCORE_REGULAR, &*REGULAR) };
    let tail = "must be more than a character, start lowercase, and not have a single lowercase followed by uppercase, or consecutive uppercase.";
    let msg = if underscore.is_match(&text) {
        if is_test {
            format!("Test method name '{text}' has invalid underscore usage, underscore only allowed between letters or between digits.")
        } else {
            format!("Method name '{text}' has invalid underscore usage, underscores only allowed between adjacent digits.")
        }
    } else if !format.is_match(stripped) {
        if is_test { format!("Test method name '{text}' segment {tail}") } else { format!("Method name '{text}' {tail}") }
    } else {
        return;
    };
    cx.diag("GoogleMethodName", name, msg);
}

pub fn on_field(cx: &mut Ctx, node: Node) {
    if !cx.cfg.google_non_constant_field_name {
        return;
    }
    if crate::rules::util::has_modifier(cx, node, b"static") || cx.parent().is_some_and(|p| matches!(p.kind_s(), "interface_body" | "annotation_type_body")) {
        return;
    }
    let mut cur = node.walk();
    for d in node.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        let Some(name) = d.child_by_field_id(cx.syn.f.name) else { continue };
        let text = String::from_utf8_lossy(cx.text(name)).into_owned();
        let stripped = strip_numbering(&text);
        let lowers_pair = {
            let mut it = stripped.chars();
            matches!((it.next(), it.next()), (Some(a), Some(b)) if a.is_ascii_lowercase() && b.is_ascii_uppercase())
        };
        let single_lower = stripped.len() == 1 && stripped.as_bytes()[0].is_ascii_lowercase();
        if UNDERSCORE_REGULAR.is_match(&text) || single_lower || lowers_pair || !FIELD.is_match(stripped) {
            cx.diag("GoogleNonConstantFieldName", name, format!("Non-constant field name '{text}' must start lowercase, be at least 2 chars, avoid single lowercase letter followed by uppercase, contain only letters, digits or underscores, with underscores allowed only between adjacent digits."));
        }
    }
}
