//! `DesignForExtension`: a method of a class that others can subclass should say how to override it.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use crate::rules::util::has_modifier;
use tree_sitter::Node;

pub fn on_method(cx: &mut Ctx, method: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.design_for_extension else { return };
    if !can_be_overridden(cx, method) {
        return;
    }
    // A method without a body that is not native is not Java; an empty body is the way to say
    // "nothing to extend here".
    let native = has_modifier(cx, method, b"native");
    let Some(class) = class_of(cx, method) else { return };
    if !native {
        match method.child_by_field_id(cx.syn.f.body) {
            Some(body) if body.children(&mut body.walk()).all(|c| matches!(c.kind_s(), "{" | "}" | "line_comment" | "block_comment")) => return,
            Some(_) => {}
            None => return,
        }
    }
    if has_ignored_annotation(cx, &opts.ignored_annotations, method) {
        return;
    }
    if javadoc_satisfies(cx, &opts.required_javadoc_phrase, method) {
        return;
    }
    if !can_be_subclassed(cx, class) {
        return;
    }
    let class_name = class.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).into_owned()).unwrap_or_default();
    let method_name = method.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).into_owned()).unwrap_or_default();
    let msg = format!(
        "Class '{class_name}' looks like designed for extension (can be subclassed), but the method '{method_name}' does not have javadoc that explains how to do that safely. If class is not designed for extension consider making the class '{class_name}' final or making the method '{method_name}' static/final/abstract/empty, or adding allowed annotation for the method."
    );
    cx.diag("DesignForExtension", method, msg);
}

fn is_type_decl(kind: &str) -> bool {
    matches!(kind, "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration")
}

/// Visibility rank of a type: public 1, protected 2, package 3, private 4.
fn rank(cx: &Ctx, decl: Node) -> u8 {
    // A member of an interface is public whatever it says.
    if cx.parent_of(decl).is_some_and(|p| matches!(p.kind_s(), "interface_body" | "annotation_type_body")) {
        return 1;
    }
    if has_modifier(cx, decl, b"public") {
        1
    } else if has_modifier(cx, decl, b"protected") {
        2
    } else if has_modifier(cx, decl, b"private") {
        4
    } else {
        3
    }
}

/// The least visible of the enclosing types must still be public or protected, and an anonymous
/// class ends the search with a verdict of no. Interface and annotation members are not
/// overridable in this sense; a method of a record is never checked.
fn can_be_overridden(cx: &Ctx, method: Node) -> bool {
    if ["private", "abstract", "final", "static"].iter().any(|w| has_modifier(cx, method, w.as_bytes())) {
        return false;
    }
    let mut scope: Option<u8> = None;
    let mut first_type = true;
    let mut p = cx.parent_of(method);
    while let Some(n) = p {
        if n.kind_s() == "object_creation_expression" {
            return false;
        }
        if is_type_decl(n.kind_s()) {
            if first_type && matches!(n.kind_s(), "interface_declaration" | "annotation_type_declaration" | "record_declaration") {
                return false;
            }
            first_type = false;
            let r = rank(cx, n);
            if scope.is_none_or(|s| s <= r) {
                scope = Some(r);
            }
        }
        p = cx.parent_of(n);
    }
    scope.is_some_and(|s| s <= 2)
}

/// The nearest class or enum around the method; records and interfaces are walked through.
fn class_of<'a>(cx: &Ctx<'a>, method: Node<'a>) -> Option<Node<'a>> {
    let mut p = cx.parent_of(method);
    while let Some(n) = p {
        if matches!(n.kind_s(), "class_declaration" | "enum_declaration") {
            return Some(n);
        }
        p = cx.parent_of(n);
    }
    None
}

fn can_be_subclassed(cx: &Ctx, class: Node) -> bool {
    if class.kind_s() == "enum_declaration" || has_modifier(cx, class, b"final") {
        return false;
    }
    let Some(body) = class.child_by_field_id(cx.syn.f.body) else { return true };
    let ctors: Vec<Node> = body.children(&mut body.walk()).filter(|c| c.kind_s() == "constructor_declaration").collect();
    ctors.is_empty() || ctors.iter().any(|c| !has_modifier(cx, *c, b"private"))
}

fn has_ignored_annotation(cx: &Ctx, ignored: &[String], method: Node) -> bool {
    let Some(mods) = method.children(&mut method.walk()).find(|c| c.kind_s() == "modifiers") else { return false };
    mods.children(&mut mods.walk()).filter(|m| matches!(m.kind_s(), "marker_annotation" | "annotation")).any(|a| {
        let Some(name) = a.child_by_field_id(cx.syn.f.name) else { return false };
        let text: String = String::from_utf8_lossy(cx.text(name)).split_whitespace().collect();
        let simple = text.rsplit('.').next().unwrap_or(&text);
        ignored.iter().any(|i| i == simple)
    })
}

/// The first Javadoc comment between the previous member and the first token of the method must match
/// the required phrase. One inside the declaration (`public /** x */ void m()`) does not count.
fn javadoc_satisfies(cx: &Ctx, phrase: &crate::config::Format, method: Node) -> bool {
    // Checkstyle hangs the comment on the method's first token, and looks only under its modifiers
    // and return type. With no modifiers that token is the `<` of type parameters, which is neither.
    let has_modifiers = method.children(&mut method.walk()).any(|c| c.kind_s() == "modifiers");
    if !has_modifiers && method.children(&mut method.walk()).any(|c| c.kind_s() == "type_parameters") {
        return false;
    }
    let mut comments: Vec<Node> = Vec::new();
    let mut prev = method.prev_sibling();
    while let Some(p) = prev.filter(|p| matches!(p.kind_s(), "line_comment" | "block_comment")) {
        comments.push(p);
        prev = p.prev_sibling();
    }
    comments.sort_by_key(|c| c.start_byte());
    let Some(doc) = comments.into_iter().find(|c| c.kind_s() == "block_comment" && cx.text(*c).starts_with(b"/**") && cx.text(*c).len() > 4) else { return false };
    let text = cx.text(doc);
    phrase.matches(&text[2..text.len() - 2])
}
