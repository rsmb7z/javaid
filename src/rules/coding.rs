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
