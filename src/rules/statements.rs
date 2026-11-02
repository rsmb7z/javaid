//! `OneStatementPerLine`.
//!
//! The walk meets statements in document order. Each `;` that ends one records its line, and a
//! statement that starts on that same line is reported. What counts as a statement, which `;`
//! count and a few quirks of members sharing a line were all found by probing Checkstyle (see the
//! `OneProbe*` fixtures); none of it is obvious from the module's description.

use crate::engine::Ctx;
use tree_sitter::Node;

#[derive(Default)]
pub struct State {
    /// The line of the last `;` that ended a statement.
    last_end: Option<usize>,
}

/// A `;` that ends a statement. Not the ones in a `for` header (`for (int i = 0; i < n; i++)`) or
/// between resources, not an empty statement (`;` on its own in a block or a `case` group), and not
/// the one after `do ... while (cond)`: a statement may follow that on the same line. That last one
/// was found by running both tools on a mutated copy of Spring, which joined a loop's closing
/// `while (...);` to the `return` after it; the hand-written probes had missed it.
pub fn on_semicolon(cx: &Ctx, st: &mut State, semi: Node) {
    if cx.cfg.one_statement_per_line.is_none() {
        return;
    }
    let Some(parent) = cx.parent() else { return };
    let ends_a_statement = match parent.kind() {
        "for_statement" | "resource_specification" | "block" | "switch_block_statement_group" | "do_statement" => false,
        "local_variable_declaration" => cx.ancestor(1).is_none_or(|g| g.kind() != "for_statement"),
        _ => true,
    };
    if ends_a_statement {
        st.last_end = Some(semi.start_position().row);
    }
}

/// Called as the walk enters a declaration or statement. Control statements (`if`, `for`,
/// `while`, `try`, ...), blocks and methods with a body are never reported themselves; what they
/// contain is.
pub fn on_statement(cx: &mut Ctx, st: &mut State, node: Node) {
    if cx.cfg.one_statement_per_line.is_none() {
        return;
    }
    let kind = node.kind();
    let row = node.start_position().row;
    let checked = match kind {
        "package_declaration" => false,
        "local_variable_declaration" => cx.parent().is_none_or(|p| p.kind() != "for_statement"),
        // A method ends with `;` only when it has no body (abstract, interface, native).
        "method_declaration" => node.child_by_field_id(cx.syn.f.body).is_none(),
        _ => true,
    };
    if checked && (st.last_end == Some(row) || shares_a_line_with_a_member(cx, node, row)) {
        cx.diag("OneStatementPerLine", reported_at(node), "Only one statement per line allowed.".into());
    }
    // The children of an import or package statement are not walked, so its `;` is recorded here.
    if matches!(kind, "import_declaration" | "package_declaration") {
        st.last_end = Some(node.end_position().row);
    }
}

/// Where Checkstyle puts the report for a statement. A declaration or a control statement is at its
/// first token. An expression statement, and the expression of a `throw`, is at the position of
/// its expression node, which is not always the expression's first token (probed): a prefix
/// operator, `new` and the parenthesis of a cast do not count, so `++i` is at `i`, `new T()` at
/// `T` and `(T) e` at `T`; a chain of calls, fields, indexes or operators is at its leftmost part.
fn reported_at(statement: Node) -> Node {
    let mut e = match statement.kind() {
        "expression_statement" | "throw_statement" => match statement.named_child(0) {
            Some(e) => e,
            None => return statement,
        },
        _ => return statement,
    };
    loop {
        let next = match e.kind() {
            "unary_expression" | "update_expression" => e.named_child(0),
            "cast_expression" => e.child_by_field_name("type"),
            "object_creation_expression" if e.child(0).is_some_and(|c| c.kind() != "new") => e.named_child(0),
            "object_creation_expression" | "array_creation_expression" => e.child_by_field_name("type"),
            "method_invocation" => e.child_by_field_name("object"),
            "field_access" => e.child_by_field_name("object"),
            "array_access" => e.child_by_field_name("array"),
            "assignment_expression" | "binary_expression" | "instanceof_expression" => e.child_by_field_name("left"),
            "ternary_expression" => e.child_by_field_name("condition"),
            _ => None,
        };
        match next {
            Some(n) => e = n,
            None => return e,
        }
    }
}

fn is_type_declaration(kind: &str) -> bool {
    matches!(kind, "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration")
}

/// Two quirks of members of a type body that no `;` explains: a member that starts on the line
/// where a method with a body or a nested type ended, and a nested type on the line where the body
/// it sits in opens (`class A { class B {} }`). A constructor or an initialiser ends nothing.
fn shares_a_line_with_a_member(cx: &Ctx, node: Node, row: usize) -> bool {
    let Some(parent) = cx.parent() else { return false };
    if !matches!(parent.kind(), "class_body" | "interface_body" | "enum_body" | "enum_body_declarations" | "annotation_type_body") {
        return false;
    }
    if is_type_declaration(node.kind()) && parent.start_position().row == row {
        return true;
    }
    cx.previous_sibling().is_some_and(|b| {
        b.end_position().row == row && (is_type_declaration(b.kind()) || (b.kind() == "method_declaration" && b.child_by_field_id(cx.syn.f.body).is_some()))
    })
}

/// With `treatTryResourcesAsStatement`, a resource is reported when the one before it in the same
/// `try (...)` ends on the line where it starts. The `;` between resources does not count as the
/// end of a statement, and neither does naming an existing variable (`try (r; r2)`).
pub fn on_resource(cx: &mut Ctx, resource: Node) {
    let Some(opts) = &cx.cfg.one_statement_per_line else { return };
    if !opts.treat_try_resources_as_statement || resource.child_by_field_id(cx.syn.f.name).is_none() {
        return;
    }
    let mut before = resource.prev_sibling();
    while let Some(b) = before.filter(|b| b.is_extra() || b.kind() == ";") {
        before = b.prev_sibling();
    }
    if before.is_some_and(|b| b.kind() == "resource" && b.end_position().row == resource.start_position().row) {
        cx.diag("OneStatementPerLine", resource, "Only one statement per line allowed.".into());
    }
}
