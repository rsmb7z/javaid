//! `ArrayTypeStyle`, `UpperEll` and `EmptyStatement`: small checks on a single token.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use tree_sitter::Node;

/// `ArrayTypeStyle` at its default, Java style: report C-style brackets, the ones after a name
/// (`int a[]`, `int f()[]`) rather than in the type (`int[] a`).
///
/// Every `[` is reported on its own (`int a[][]` gives two findings). A bracket counts as C-style
/// only if it is on the same line as the token before it (the name, the `)` of a method, an
/// annotation or the previous `]`) and to the right of the declared name's column. Both halves
/// were found by probing Checkstyle: a bracket that starts a new line is left alone, and so is a
/// `)[]` that begins a line to the left of the method name.
pub fn on_dimensions(cx: &mut Ctx, dims: Node) {
    if !cx.cfg.array_type_style {
        return;
    }
    // Brackets in a type live in an `array_type`, and `new int[]` has its own parent.
    let Some(parent) = cx.parent() else { return };
    if !matches!(
        parent.kind_s(),
        "variable_declarator" | "formal_parameter" | "method_declaration" | "annotation_type_element_declaration" | "enhanced_for_statement" | "resource"
    ) {
        return;
    }
    let Some(name) = parent.child_by_field_id(cx.syn.f.name) else { return };
    let name_col = name.start_position().column;

    // The token before the brackets: the name, or for a method the parameter list.
    let mut row = cx.previous_sibling().unwrap_or(name).end_position().row;

    let mut cur = dims.walk();
    for part in dims.children(&mut cur) {
        if part.is_extra() {
            continue; // a comment
        }
        if part.kind_s() == "[" {
            let at = part.start_position();
            if at.row == row && at.column > name_col {
                cx.diag("ArrayTypeStyle", part, "Array brackets at illegal position.".into());
            }
        }
        row = part.end_position().row;
    }
}

/// `UpperEll`: an integer literal ends in a lowercase `l`, which looks like a one. Only integer
/// literals can end in `l`; `1f`, `1d` and strings cannot.
pub fn on_integer_literal(cx: &mut Ctx, literal: Node) {
    if cx.cfg.upper_ell && cx.text(literal).last() == Some(&b'l') {
        cx.diag("UpperEll", literal, "Should use uppercase 'L'.".into());
    }
}

/// `EmptyStatement`: a `;` that ends nothing. Checkstyle counts the stray `;` wherever the grammar
/// allows one: between members, after a type or an import, as a loop body, in a block. The `;`
/// that ends a declaration or statement is a child of that node, so only these parents matter.
/// An enum's separator (the first `;` of its member list) is not empty; a second one is.
pub fn on_empty_statement(cx: &mut Ctx, semi: Node) {
    if !cx.cfg.empty_statement {
        return;
    }
    let Some(parent) = cx.parent() else { return };
    let empty = match parent.kind_s() {
        "program" | "class_body" | "interface_body" | "annotation_type_body" | "constructor_body" => true,
        "enum_body_declarations" => parent.child(0).is_some_and(|first| first.id() != semi.id()),
        "package_declaration" | "import_declaration" => semi.prev_sibling().is_some_and(|p| p.kind_s() == ";"),
        _ => crate::rules::whitespace::is_empty_statement(cx, parent, semi),
    };
    if empty {
        cx.diag("EmptyStatement", semi, "Empty statement.".into());
    }
}
