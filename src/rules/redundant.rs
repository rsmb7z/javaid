//! `RedundantModifier`: a modifier the language already implies.
//!
//! Where each finding goes (the modifier itself) and which declarations count was settled against
//! Checkstyle on `bench/oracle/probes/RedundantProbe*.java`.

use crate::engine::Ctx;
use tree_sitter::Node;

pub fn on_modifiers(cx: &mut Ctx, modifiers: Node) {
    let Some(opts) = &cx.cfg.redundant_modifier else { return };
    let jdk = opts.jdk_version;
    let Some(decl) = modifiers.parent() else { return };
    let words: Vec<Node> = modifiers.children(&mut modifiers.walk()).filter(|m| !m.is_extra() && !matches!(m.kind(), "marker_annotation" | "annotation")).collect();
    let first = |cx: &Ctx, word: &[u8]| words.iter().copied().find(|m| cx.text(*m) == word);
    let in_interface = decl.parent().is_some_and(|p| matches!(p.kind(), "interface_body" | "annotation_type_body"));
    let is_method = decl.kind() == "method_declaration";
    let is_class = decl.kind() == "class_declaration";

    match decl.kind() {
        "interface_declaration" | "annotation_type_declaration" => {
            for word in [&b"static"[..], b"abstract"] {
                if let Some(m) = first(cx, word) {
                    redundant(cx, m);
                }
            }
        }
        "enum_declaration" => {
            if let Some(m) = first(cx, b"static") {
                redundant(cx, m);
            }
        }
        "record_declaration" => {
            let both: Vec<Node> = words.iter().copied().filter(|m| matches!(cx.text(*m), b"final" | b"static")).collect();
            for m in both {
                redundant(cx, m);
            }
        }
        "constructor_declaration" => constructor(cx, decl, &words),
        "method_declaration" => method(cx, decl, &words),
        "resource" => {
            if let Some(m) = first(cx, b"final") {
                redundant(cx, m);
            }
        }
        "formal_parameter" | "spread_parameter" => {
            // Only the parameters of a method without a body: abstract, interface and native.
            let owner = decl.parent().and_then(|l| l.parent());
            if owner.is_some_and(|o| o.kind() == "method_declaration" && o.child_by_field_id(cx.syn.f.body).is_none()) {
                if let Some(m) = first(cx, b"final") {
                    redundant(cx, m);
                }
            }
        }
        _ => {}
    }
    if jdk >= 22 && declares_underscore(cx, decl) {
        if let Some(m) = first(cx, b"final") {
            redundant(cx, m);
        }
    }
    if in_interface {
        for m in &words {
            let word = cx.text(*m);
            let flagged = match word {
                b"public" => true,
                b"static" => !is_method,
                b"abstract" | b"final" => !is_class,
                _ => false,
            };
            if flagged {
                redundant(cx, *m);
            }
        }
    }
    if jdk >= 17
        && matches!(
            decl.kind(),
            "method_declaration" | "constructor_declaration" | "class_declaration" | "interface_declaration" | "enum_declaration" | "annotation_type_declaration" | "record_declaration"
        )
    {
        if let Some(m) = first(cx, b"strictfp") {
            redundant(cx, m);
        }
    }
}

fn redundant(cx: &mut Ctx, modifier: Node) {
    let msg = format!("Redundant '{}' modifier.", String::from_utf8_lossy(cx.text(modifier)));
    // Reached twice for `static interface` inside an interface: reported once.
    cx.diag_once("RedundantModifier", modifier, msg);
}

/// `final` on a declaration of the unnamed variable `_`.
fn declares_underscore(cx: &Ctx, decl: Node) -> bool {
    match decl.kind() {
        "local_variable_declaration" => {
            let mut cur = decl.walk();
            let found = decl.children_by_field_id(cx.syn.f.declarator, &mut cur).any(|d| d.child_by_field_id(cx.syn.f.name).is_some_and(|n| cx.text(n) == b"_"));
            found
        }
        "formal_parameter" | "catch_formal_parameter" | "enhanced_for_statement" | "instanceof_expression" => decl.child_by_field_id(cx.syn.f.name).is_some_and(|n| cx.text(n) == b"_"),
        "type_pattern" | "record_pattern_component" => {
            let last = decl.named_child_count().checked_sub(1).and_then(|i| decl.named_child(i as u32));
            last.is_some_and(|n| n.kind() == "identifier" && cx.text(n) == b"_")
        }
        _ => false,
    }
}

fn constructor(cx: &mut Ctx, ctor: Node, words: &[Node]) {
    let Some(body) = ctor.parent() else { return };
    let owner = body_owner(body);
    if owner.is_some_and(|o| o.kind() == "enum_declaration") {
        // Every constructor of an enum is private, so whatever is written is redundant.
        if let Some(m) = words.first().copied() {
            redundant(cx, m);
        }
        return;
    }
    let Some(class) = owner else { return };
    if !is_public_class(cx, class) && !has_word(cx, class, b"protected") {
        if let Some(m) = words.iter().copied().find(|m| cx.text(*m) == b"public") {
            redundant(cx, m);
        }
    }
}

fn method(cx: &mut Ctx, method: Node, words: &[Node]) {
    let private = words.iter().any(|m| cx.text(*m) == b"private");
    let is_static = words.iter().any(|m| cx.text(*m) == b"static");
    let mut implied = private;
    if !implied {
        // The nearest class, anonymous class, enum constant or enum decides. Interfaces and records
        // in between are looked through.
        let mut p = method.parent();
        while let Some(n) = p {
            match n.kind() {
                "class_declaration" => {
                    implied = has_word(cx, n, b"final");
                    break;
                }
                "object_creation_expression" | "enum_constant" => {
                    implied = true;
                    break;
                }
                "enum_declaration" => {
                    implied = is_static;
                    break;
                }
                _ => p = n.parent(),
            }
        }
    }
    if implied && !has_safe_varargs(cx, method) {
        if let Some(m) = words.iter().copied().find(|m| cx.text(*m) == b"final") {
            redundant(cx, m);
        }
    }
}

/// `@SafeVarargs` needs the method to be final, static or private to compile.
fn has_safe_varargs(cx: &Ctx, method: Node) -> bool {
    let Some(mods) = method.children(&mut method.walk()).find(|c| c.kind() == "modifiers") else { return false };
    mods.children(&mut mods.walk()).filter(|m| matches!(m.kind(), "marker_annotation" | "annotation")).any(|a| {
        let Some(name) = a.child_by_field_id(cx.syn.f.name) else { return false };
        // Checkstyle reads the last token of the annotation name: a qualified name does not match.
        name.kind() == "identifier" && cx.text(name) == b"SafeVarargs"
    })
}

fn has_word(cx: &Ctx, decl: Node, word: &[u8]) -> bool {
    let Some(mods) = decl.children(&mut decl.walk()).find(|c| c.kind() == "modifiers") else { return false };
    mods.children(&mut mods.walk()).any(|m| !matches!(m.kind(), "marker_annotation" | "annotation") && cx.text(m) == word)
}

/// The declaration a member body belongs to: a class, enum, record or interface, or the node of an
/// anonymous class or enum constant.
fn body_owner(body: Node) -> Option<Node> {
    let mut b = body;
    if b.kind() == "enum_body_declarations" {
        b = b.parent()?;
    }
    b.parent()
}

/// Whether a class can be reached from outside the package, by Checkstyle's reckoning: a top-level
/// class is if it is `public`; a nested one if it is `public` (or sits in an interface) and so is
/// what encloses it.
fn is_public_class(cx: &Ctx, class: Node) -> bool {
    let Some(parent) = class.parent() else { return false };
    let public = has_word(cx, class, b"public");
    if parent.kind() == "program" {
        return public;
    }
    let owner = if matches!(parent.kind(), "class_body" | "interface_body" | "enum_body_declarations" | "annotation_type_body") { body_owner(parent) } else { None };
    let Some(owner) = owner else { return false };
    if (owner.kind() == "interface_declaration" || public) && matches!(owner.kind(), "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration") {
        return is_public_class(cx, owner);
    }
    false
}
