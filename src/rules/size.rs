//! `MethodLength` and `ParameterNumber`: how big a method or constructor is.

use crate::engine::Ctx;
use crate::rules::tokens::Token;
use crate::rules::util::has_override_annotation;
use tree_sitter::Node;

/// A method, constructor or compact constructor, as the two modules see it.
pub fn on_method_like(cx: &mut Ctx, node: Node) {
    let token = match node.kind() {
        "method_declaration" => Token::MethodDef,
        "constructor_declaration" => Token::CtorDef,
        _ => Token::CompactCtorDef,
    };
    method_length(cx, node, token);
    parameter_number(cx, node, token);
}


/// `MethodLength`: the lines from the line of the opening brace to the line of the closing one. With
/// `countEmpty=false`, only the lines on which some token of the body starts count (comments are not
/// tokens), and a text block counts every line it spans.
fn method_length(cx: &mut Ctx, node: Node, token: Token) {
    let Some(opts) = &cx.cfg.method_length else { return };
    if !opts.tokens.contains(token) {
        return;
    }
    let (max, count_empty) = (opts.max, opts.count_empty);
    let Some(body) = node.child_by_field_id(cx.syn.f.body) else { return };
    let length = if count_empty { body.end_position().row - body.start_position().row + 1 } else { used_lines(body) };
    if length > max {
        let name = node.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).into_owned()).unwrap_or_default();
        cx.diag("MethodLength", node, format!("Method {name} length is {} lines (max allowed is {}).", crate::engine::grouped(length), crate::engine::grouped(max)));
    }
}

/// `ParameterNumber`: more parameters than allowed. A method that carries `@Override` can be exempt,
/// and so can one with an annotation from the `ignoreAnnotatedBy` list.
fn parameter_number(cx: &mut Ctx, node: Node, token: Token) {
    let Some(opts) = &cx.cfg.parameter_number else { return };
    if !opts.tokens.contains(token) {
        return;
    }
    let max = opts.max;
    let Some(params) = node.child_by_field_id(cx.syn.f.parameters) else { return };
    let mut cur = params.walk();
    let count = params.children(&mut cur).filter(|c| matches!(c.kind(), "formal_parameter" | "spread_parameter")).count();
    if count <= max {
        return;
    }
    if opts.ignore_overridden && has_override_annotation(cx, node) {
        return;
    }
    if !opts.ignore_annotated_by.is_empty() && annotated_by(cx, node, &opts.ignore_annotated_by) {
        return;
    }
    // At the method's name.
    let at = node.child_by_field_id(cx.syn.f.name).unwrap_or(node);
    cx.diag("ParameterNumber", at, format!("More than {} parameters (found {}).", crate::engine::grouped(max), crate::engine::grouped(count)));
}

fn annotated_by(cx: &Ctx, decl: Node, names: &[String]) -> bool {
    let mut cur = decl.walk();
    let Some(mods) = decl.children(&mut cur).find(|c| c.kind() == "modifiers") else { return false };
    let mut inner = mods.walk();
    let found = mods.children(&mut inner).any(|c| {
        matches!(c.kind(), "marker_annotation" | "annotation")
            && c.child_by_field_id(cx.syn.f.name).is_some_and(|n| names.iter().any(|w| w.as_bytes() == cx.text(n)))
    });
    found
}

/// The number of distinct lines that hold a token of `body`, braces included.
fn used_lines(body: Node) -> usize {
    let mut rows = std::collections::BTreeSet::new();
    let mut cursor = body.walk();
    'walk: loop {
        let node = cursor.node();
        if matches!(node.kind(), "line_comment" | "block_comment") {
            // not a token to Checkstyle
        } else if node.kind() == "string_literal" && node.start_position().row != node.end_position().row {
            rows.extend(node.start_position().row..=node.end_position().row);
        } else if cursor.goto_first_child() {
            continue;
        } else {
            rows.insert(node.start_position().row);
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() || cursor.node().id() == body.id() {
                break 'walk;
            }
        }
    }
    rows.len()
}
