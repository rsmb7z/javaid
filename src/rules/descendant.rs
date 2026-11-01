//! `DescendantToken`, for the one use the bundled configurations make of it: the commas of a
//! `for` header's init and update clauses.
//!
//! Checkstyle counts tokens in its own syntax tree, so the depth of a comma has to be reproduced:
//! the commas between declarators sit one level below `FOR_INIT`; those between expressions sit
//! two below, under an expression list. Commas inside an argument list or an array initializer
//! are deeper still, which is why only a `maximumDepth` of 2 or less is accepted.

use crate::engine::Ctx;
use tree_sitter::Node;

pub fn on_for(cx: &mut Ctx, stmt: Node) {
    let cfg = cx.cfg;
    for rule in &cfg.descendant_tokens {
        if rule.for_init {
            check(cx, rule, stmt, true);
        }
        if rule.for_iterator {
            check(cx, rule, stmt, false);
        }
    }
}

fn check(cx: &mut Ctx, rule: &crate::config::DescendantToken, stmt: Node, init: bool) {
    let mut cur = stmt.walk();
    let nodes: Vec<Node> = stmt.children_by_field_name(if init { "init" } else { "update" }, &mut cur).collect();
    let declaration = nodes.first().filter(|n| n.kind() == "local_variable_declaration").copied();
    // (commas, depth of each, where Checkstyle puts the clause)
    let (count, depth, at): (u32, u32, Option<(u32, u32)>) = match (declaration, nodes.first()) {
        (Some(d), _) => {
            let commas = d.children(&mut d.walk()).filter(|c| c.kind() == ",").count() as u32;
            let p = d.start_position();
            (commas, 1, Some((p.row as u32 + 1, p.column as u32 + 1)))
        }
        (None, Some(first)) => {
            let p = root_token(*first);
            (nodes.len() as u32 - 1, 2, Some(p))
        }
        (None, None) => (0, 0, None),
    };
    let counted = if depth >= rule.min_depth && depth <= rule.max_depth { count } else { 0 };
    let name = if init { "FOR_INIT" } else { "FOR_ITERATOR" };
    let message = if counted < rule.min_number {
        rule.minimum_message.clone().unwrap_or_else(|| format!("Count of {counted} for '{name}' descendant 'COMMA' is less than minimum count {}.", rule.min_number))
    } else if counted > rule.max_number {
        rule.maximum_message.clone().unwrap_or_else(|| format!("Count of {counted} for '{name}' descendant 'COMMA' exceeds maximum count {}.", rule.max_number))
    } else {
        return;
    };
    let (line, col) = at.unwrap_or_else(|| {
        // An empty clause is placed on the token that follows it: the first `;`, or the `)`.
        let kind = if init { ";" } else { ")" };
        let tok = stmt.children(&mut stmt.walk()).find(|c| c.kind() == kind).unwrap_or(stmt);
        let p = tok.start_position();
        (p.row as u32 + 1, p.column as u32 + 1)
    });
    cx.diag_at(rule.name, line, col, message);
}

/// Where Checkstyle places an expression: on the operator or paren that is the root of its tree.
fn root_token(expr: Node) -> (u32, u32) {
    let tok = match expr.kind() {
        "assignment_expression" => expr.child_by_field_name("operator").unwrap_or(expr),
        "update_expression" => {
            let first = expr.child(0).unwrap_or(expr);
            if matches!(first.kind(), "++" | "--") {
                first
            } else {
                expr.child(expr.child_count().saturating_sub(1) as u32).unwrap_or(expr)
            }
        }
        "method_invocation" => expr.child_by_field_name("arguments").unwrap_or(expr),
        _ => expr,
    };
    let p = tok.start_position();
    (p.row as u32 + 1, p.column as u32 + 1)
}
