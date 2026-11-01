//! `EqualsHashCode`.

use crate::engine::Ctx;
use crate::syntax::Act;
use tree_sitter::Node;

pub fn on_type_body(cx: &mut Ctx, body: Node) {
    if !cx.cfg.equals_hash_code {
        return;
    }
    let mut equals = None;
    let mut hash = None;
    let mut cur = body.walk();
    for m in body.named_children(&mut cur) {
        if cx.syn.act(m) != Act::Method {
            continue;
        }
        let Some(name) = m.child_by_field_id(cx.syn.f.name) else { continue };
        // Anchor on the whole declaration (its first annotation), as Checkstyle does. Like
        // Checkstyle, an abstract declaration is not an override: it needs a body (or `native`).
        match cx.text(name) {
            b"equals" if equals.is_none() && is_concrete_public_instance(cx, m) && has_single_object_param(cx, m) => equals = Some(m),
            b"hashCode" if hash.is_none() && is_concrete_public_instance(cx, m) && has_no_params(cx, m) => hash = Some(m),
            _ => {}
        }
    }
    match (equals, hash) {
        (Some(at), None) => cx.diag("EqualsHashCode", at, "Definition of 'equals()' without corresponding definition of 'hashCode()'.".into()),
        (None, Some(at)) => cx.diag("EqualsHashCode", at, "Definition of 'hashCode()' without corresponding definition of 'equals()'.".into()),
        _ => {}
    }
}

fn is_concrete_public_instance(cx: &Ctx, method: Node) -> bool {
    let Some(mods) = method.child(0).filter(|c| c.kind() == "modifiers") else { return false };
    let mut cur = mods.walk();
    let (mut public, mut stat, mut native) = (false, false, false);
    for c in mods.children(&mut cur) {
        match cx.text(c) {
            b"public" => public = true,
            b"static" => stat = true,
            b"native" => native = true,
            _ => {}
        }
    }
    public && !stat && (native || method.child_by_field_id(cx.syn.f.body).is_some())
}

fn has_no_params(cx: &Ctx, method: Node) -> bool {
    method.child_by_field_id(cx.syn.f.parameters).is_some_and(|p| p.named_child_count() == 0)
}

fn has_single_object_param(cx: &Ctx, method: Node) -> bool {
    let Some(params) = method.child_by_field_id(cx.syn.f.parameters) else { return false };
    if params.named_child_count() != 1 {
        return false;
    }
    let Some(param) = params.named_child(0) else { return false };
    param.kind() == "formal_parameter"
        && param.child_by_field_id(cx.syn.f.type_).is_some_and(|t| matches!(cx.text(t), b"Object" | b"java.lang.Object"))
}

/// `MissingSwitchDefault`: a `switch` statement (never an expression) without a `default`.
/// A switch with a pattern or `null` label is left alone, as the compiler already requires it to
/// be exhaustive; an empty switch is reported. Found by probing Checkstyle.
pub fn on_switch(cx: &mut Ctx, switch: Node) {
    if !cx.cfg.missing_switch_default {
        return;
    }
    let is_statement = cx.parent().is_some_and(|p| {
        matches!(
            p.kind(),
            "block"
                | "constructor_body"
                | "switch_block_statement_group"
                | "if_statement"
                | "while_statement"
                | "do_statement"
                | "for_statement"
                | "enhanced_for_statement"
                | "labeled_statement"
        )
    });
    let Some(body) = switch.child_by_field_id(cx.syn.f.body).filter(|_| is_statement) else { return };
    let mut cur = body.walk();
    for group in body.named_children(&mut cur) {
        let mut inner = group.walk();
        for label in group.named_children(&mut inner).filter(|l| l.kind() == "switch_label") {
            let mut parts = label.walk();
            for part in label.children(&mut parts) {
                if cx.text(part) == b"default" || matches!(part.kind(), "pattern" | "type_pattern" | "record_pattern" | "null_literal") {
                    return;
                }
            }
        }
    }
    cx.diag("MissingSwitchDefault", switch, "switch without \"default\" clause.".into());
}

/// `MultipleVariableDeclarations`, for a local variable, field or constant declaration. Two
/// findings exist, and one declaration can have both:
///
/// * `int a, b;`: several variables in one statement (not in a `for` initialiser), reported at the
///   first token of the declaration, annotations and modifiers included;
/// * `int a; int b;`: the next declaration starts on the line where this one ends. A comment in
///   between does not matter and a statement does, so `int a; f(); int b;` is fine.
pub fn on_declaration(cx: &mut Ctx, decl: Node) {
    if !cx.cfg.multiple_variable_declarations {
        return;
    }
    let in_for_header = cx.parent().is_some_and(|p| p.kind() == "for_statement");
    if !in_for_header && decl.children_by_field_id(cx.syn.f.declarator, &mut decl.walk()).count() > 1 {
        cx.diag("MultipleVariableDeclarations", decl, "Each variable declaration must be in its own statement.".into());
    }
    // Looked at from the later declaration: the one before it ended on the line where this starts.
    // The finding is still reported at the earlier one.
    if let Some(before) = cx.previous_sibling() {
        if matches!(before.kind(), "local_variable_declaration" | "field_declaration" | "constant_declaration") && before.end_position().row == decl.start_position().row {
            cx.diag("MultipleVariableDeclarations", before, "Only one variable definition per line allowed.".into());
        }
    }
}

/// `InnerAssignment`: an assignment that is not the whole of a statement. Allowed are the statement
/// itself, a `for` initialiser or update, a lambda's expression body, a rule of a `switch` statement, and
/// the body of an `if`, `else` or loop written without braces. An assignment compared with something in
/// the condition of a `while`, `do` or `for` (`while ((line = in.readLine()) != null)`) is the loop
/// idiom and is allowed too, whatever `&&`, `||`, `!`, `&` and `|` sit between.
pub fn on_assignment(cx: &mut Ctx, assignment: Node) {
    if !cx.cfg.inner_assignment {
        return;
    }
    // Parentheses are not nodes of their own to Checkstyle.
    let mut top = assignment;
    while let Some(p) = top.parent().filter(|p| p.kind() == "parenthesized_expression") {
        top = p;
    }
    let Some(parent) = top.parent() else { return };
    let f = &cx.syn.f;
    let allowed = match parent.kind() {
        "expression_statement" => match parent.parent() {
            Some(g) => match g.kind() {
                "block" | "constructor_body" | "switch_block_statement_group" => true,
                // The body of an `if`, `else` or loop without braces.
                "if_statement" => g.child_by_field_id(f.consequence).is_some_and(|c| c.id() == parent.id()) || g.child_by_field_id(f.alternative).is_some_and(|c| c.id() == parent.id()),
                "while_statement" | "for_statement" | "enhanced_for_statement" | "do_statement" => g.child_by_field_id(f.body).is_some_and(|c| c.id() == parent.id()),
                "switch_rule" => g.parent().and_then(|b| b.parent()).and_then(|sw| sw.parent()).is_some_and(|s| matches!(s.kind(), "block" | "constructor_body" | "switch_block_statement_group")),
                _ => false,
            },
            None => false,
        },
        "for_statement" => parent.child_by_field_name("condition").is_none_or(|c| c.id() != top.id()),
        "lambda_expression" => parent.child_by_field_id(f.body).is_some_and(|b| b.id() == top.id()),
        _ => false,
    };
    if allowed || in_loop_idiom(cx, top) {
        return;
    }
    let at = assignment.child_by_field_name("operator").unwrap_or(assignment);
    cx.diag("InnerAssignment", at, "Inner assignments should be avoided.".into());
}

/// Is `top` an operand of a comparison that makes up the condition of a `while`, `do` or `for`?
fn in_loop_idiom(cx: &Ctx, top: Node) -> bool {
    let Some(comparison) = top.parent().filter(|p| {
        p.kind() == "binary_expression" && p.child_by_field_name("operator").is_some_and(|o| matches!(cx.text(o), b"==" | b"!=" | b"<" | b">" | b"<=" | b">="))
    }) else {
        return false;
    };
    let mut node = comparison;
    while let Some(parent) = node.parent() {
        let skip = match parent.kind() {
            "parenthesized_expression" => true,
            "binary_expression" => parent.child_by_field_name("operator").is_some_and(|o| matches!(cx.text(o), b"&&" | b"||" | b"&" | b"|")),
            "unary_expression" => parent.child(0).is_some_and(|o| o.kind() == "!"),
            _ => false,
        };
        if !skip {
            return match parent.kind() {
                "while_statement" | "do_statement" => true,
                "for_statement" => parent.child_by_field_name("condition").is_some_and(|c| c.id() == node.id()),
                _ => false,
            };
        }
        node = parent;
    }
    false
}

/// `SimplifyBooleanExpression`: a `true` or `false` that is an operand of `==`, `!=`, `&&`, `||` or `!`
/// (reported at the operator, once per literal), or that is, or sits next to, a literal in a
/// `?:`. Parentheses are not nodes of their own to Checkstyle, so they are looked through.
pub fn on_boolean_literal(cx: &mut Ctx, literal: Node) {
    if !cx.cfg.simplify_boolean_expression {
        return;
    }
    let mut top = literal;
    while let Some(p) = top.parent().filter(|p| p.kind() == "parenthesized_expression") {
        top = p;
    }
    let Some(parent) = top.parent() else { return };
    match parent.kind() {
        "binary_expression" => {
            let Some(op) = parent.child_by_field_name("operator") else { return };
            if matches!(cx.text(op), b"==" | b"!=" | b"&&" | b"||") {
                cx.diag_once("SimplifyBooleanExpression", op, "Expression can be simplified.".into());
            }
        }
        "unary_expression" => {
            if let Some(op) = parent.child(0).filter(|o| o.kind() == "!") {
                cx.diag_once("SimplifyBooleanExpression", op, "Expression can be simplified.".into());
            }
        }
        "ternary_expression" => {
            fn strip(mut n: Node<'_>) -> bool {
                while n.kind() == "parenthesized_expression" {
                    match n.named_child(0) {
                        Some(inner) => n = inner,
                        None => break,
                    }
                }
                matches!(n.kind(), "true" | "false")
            }
            let (Some(cond), Some(then), Some(other)) =
                (parent.child_by_field_name("condition"), parent.child_by_field_name("consequence"), parent.child_by_field_name("alternative"))
            else {
                return;
            };
            let flagged = strip(cond) || (then.id() == top.id() && strip(other));
            if flagged {
                if let Some(question) = parent.children(&mut parent.walk()).find(|c| c.kind() == "?") {
                    cx.diag_once("SimplifyBooleanExpression", question, "Expression can be simplified.".into());
                }
            }
        }
        _ => {}
    }
}

/// `SimplifyBooleanReturn`: an `if` with an `else` whose two branches each start with `return true;`
/// or `return false;` (or `yield`). Only the first statement of a block is looked at.
pub fn on_simplify_return(cx: &mut Ctx, node: Node) {
    if !cx.cfg.simplify_boolean_return {
        return;
    }
    let f = &cx.syn.f;
    let (Some(then), Some(other)) = (node.child_by_field_id(f.consequence), node.child_by_field_id(f.alternative)) else { return };
    if returns_only_a_boolean(then) && returns_only_a_boolean(other) {
        cx.diag("SimplifyBooleanReturn", node, "Conditional logic can be removed.".into());
    }
}

fn returns_only_a_boolean(statement: Node) -> bool {
    fn literal_return(n: Node) -> bool {
        matches!(n.kind(), "return_statement" | "yield_statement") && n.named_child(0).is_some_and(|v| matches!(v.kind(), "true" | "false"))
    }
    if literal_return(statement) {
        return true;
    }
    if statement.kind() != "block" {
        return false;
    }
    let mut cur = statement.walk();
    let first = statement.children(&mut cur).find(|c| !matches!(c.kind(), "{" | "line_comment" | "block_comment"));
    first.is_some_and(literal_return)
}
