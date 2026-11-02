//! `HiddenField`: a local variable, parameter or pattern variable that has the name of a field.
//!
//! Which fields a name can hide follows Checkstyle's frames, one per class, enum, record and enum
//! constant body (never an interface or an anonymous class). A static field is hidden from
//! anywhere inside; an instance field only from a non-static context, and only through the
//! enclosing types up to the first one marked `static` (a record always counts as static).

use crate::config::HiddenField;
use crate::syntax::KindExt;
use crate::engine::Ctx;
use crate::rules::util::has_modifier;
use tree_sitter::Node;

/// What kind of declaration is being checked; the exemptions differ.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Local,
    Param,
    Pattern,
}

pub fn on_local(cx: &mut Ctx, decl: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    let mut cur = decl.walk();
    for d in decl.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        if let Some(name) = d.child_by_field_id(cx.syn.f.name) {
            check(cx, decl, name, Kind::Local);
        }
    }
}

/// A method, constructor or catch parameter (and a typed lambda parameter).
pub fn on_param(cx: &mut Ctx, param: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    // The components of a record are fields, not parameters.
    if cx.parent_of(param).and_then(|l| cx.parent_of(l)).is_some_and(|o| o.kind_s() == "record_declaration") {
        return;
    }
    let name = match param.kind_s() {
        "formal_parameter" | "catch_formal_parameter" => param.child_by_field_id(cx.syn.f.name),
        // The name sits in a declarator, as in the final-parameter rule.
        "spread_parameter" => param.children(&mut param.walk()).filter(|c| c.kind_s() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)),
        _ => None,
    };
    if let Some(name) = name {
        check(cx, param, name, Kind::Param);
    }
}

pub fn on_enhanced_for(cx: &mut Ctx, node: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        check(cx, node, name, Kind::Local);
    }
}

/// `o instanceof String s`
pub fn on_instanceof(cx: &mut Ctx, node: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        check(cx, node, name, Kind::Pattern);
    }
}

/// `case Integer i ->`, and the components of a record pattern.
pub fn on_pattern(cx: &mut Ctx, node: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    let last = node.named_child_count().checked_sub(1).and_then(|i| node.named_child(i as u32));
    if let Some(name) = last.filter(|c| c.kind_s() == "identifier") {
        check(cx, node, name, Kind::Pattern);
    }
}

/// `x -> ...` and `(x, y) -> ...`. A lone identifier is Checkstyle's separate lambda case: no
/// interface exemption and no `ignoreFormat`.
pub fn on_lambda(cx: &mut Ctx, node: Node) {
    if cx.cfg.hidden_field.is_none() {
        return;
    }
    let Some(params) = node.child_by_field_id(cx.syn.f.parameters) else { return };
    match params.kind_s() {
        "identifier" => {
            if hides(cx, node, cx.text(params)) {
                report(cx, params);
            }
        }
        "inferred_parameters" => {
            let mut cur = params.walk();
            for p in params.named_children(&mut cur).filter(|p| p.kind_s() == "identifier") {
                check(cx, params, p, Kind::Param);
            }
        }
        _ => {}
    }
}

fn report(cx: &mut Ctx, name: Node) {
    let msg = format!("'{}' hides a field.", String::from_utf8_lossy(cx.text(name)));
    cx.diag("HiddenField", name, msg);
}

fn check(cx: &mut Ctx, decl: Node, name: Node, kind: Kind) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.hidden_field else { return };
    if in_interface(cx, decl) {
        return;
    }
    let text = cx.text(name);
    if !hides(cx, decl, text) {
        return;
    }
    if opts.ignore_format.as_ref().is_some_and(|f| f.matches(text)) {
        return;
    }
    if kind == Kind::Param && ignored_param(cx, opts, decl, text) {
        return;
    }
    report(cx, name);
}

/// The nearest enclosing type body is an interface or annotation (an anonymous class is not).
fn in_interface<'a>(cx: &Ctx<'a>, decl: Node<'a>) -> bool {
    let mut p = cx.parent_of(decl);
    while let Some(n) = p {
        match n.kind_s() {
            "class_declaration" | "enum_declaration" | "enum_constant" | "record_declaration" | "object_creation_expression" => return false,
            "interface_declaration" | "annotation_type_declaration" => return true,
            _ => {}
        }
        p = cx.parent_of(n);
    }
    false
}

fn is_frame(kind: &str) -> bool {
    matches!(kind, "class_declaration" | "enum_declaration" | "record_declaration" | "enum_constant")
}

/// The names of the instance and static fields declared directly in a frame.
fn fields_of(cx: &Ctx, frame: Node, name: &[u8]) -> (bool, bool) {
    let (mut instance, mut stat) = (false, false);
    let mut scan = |members: Node| {
        let mut cur = members.walk();
        for field in members.children(&mut cur).filter(|c| c.kind_s() == "field_declaration") {
            let mut dc = field.walk();
            let declares = field.children_by_field_id(cx.syn.f.declarator, &mut dc).any(|d| d.child_by_field_id(cx.syn.f.name).is_some_and(|n| cx.text(n) == name));
            if declares {
                if has_modifier(cx, field, b"static") {
                    stat = true;
                } else {
                    instance = true;
                }
            }
        }
    };
    if let Some(body) = frame.child_by_field_id(cx.syn.f.body) {
        match body.kind_s() {
            "enum_body" => {
                let mut cur = body.walk();
                for part in body.children(&mut cur).filter(|c| c.kind_s() == "enum_body_declarations") {
                    scan(part);
                }
            }
            _ => scan(body),
        }
    }
    if frame.kind_s() == "enum_constant" {
        if let Some(body) = frame.children(&mut frame.walk()).find(|c| c.kind_s() == "class_body") {
            scan(body);
        }
    }
    // The components of a record are its instance fields.
    if frame.kind_s() == "record_declaration" {
        if let Some(list) = frame.child_by_field_id(cx.syn.f.parameters) {
            let mut cur = list.walk();
            for p in list.children(&mut cur).filter(|c| matches!(c.kind_s(), "formal_parameter" | "spread_parameter")) {
                let declared = p.child_by_field_id(cx.syn.f.name).or_else(|| p.children(&mut p.walk()).filter(|c| c.kind_s() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)));
                if declared.is_some_and(|n| cx.text(n) == name) {
                    instance = true;
                }
            }
        }
    }
    (instance, stat)
}

fn is_static_frame(cx: &Ctx, frame: Node) -> bool {
    frame.kind_s() == "record_declaration" || (frame.kind_s() != "enum_constant" && has_modifier(cx, frame, b"static"))
}

/// Whether `name`, declared at `at`, has the name of a field it can hide.
fn hides(cx: &Ctx, at: Node, name: &[u8]) -> bool {
    let mut frames = Vec::new();
    let mut p = cx.parent_of(at);
    while let Some(n) = p {
        if is_frame(n.kind_s()) {
            frames.push(n);
        }
        p = cx.parent_of(n);
    }
    let mut instance_visible = true;
    let mut static_seen = false;
    let mut instance_hit = false;
    for frame in &frames {
        let (instance, stat) = fields_of(cx, *frame, name);
        static_seen |= stat;
        if instance_visible && instance {
            instance_hit = true;
        }
        if is_static_frame(cx, *frame) {
            instance_visible = false;
        }
    }
    static_seen || (instance_hit && !in_static(cx, at))
}

/// Checkstyle's walk upwards: a static initializer is static; a method (other than one of an
/// anonymous class) or a variable declaration decides by its own `static`; anything else, a
/// constructor included, is passed through.
fn in_static(cx: &Ctx, at: Node) -> bool {
    let mut p = cx.parent_of(at);
    while let Some(n) = p {
        match n.kind_s() {
            "static_initializer" => return true,
            "method_declaration" if !in_anonymous_class(cx, n) => return has_modifier(cx, n, b"static"),
            "field_declaration" | "local_variable_declaration" => return has_modifier(cx, n, b"static"),
            _ => {}
        }
        p = cx.parent_of(n);
    }
    false
}

fn in_anonymous_class<'a>(cx: &Ctx<'a>, method: Node<'a>) -> bool {
    let mut p = cx.parent_of(method);
    while let Some(n) = p {
        match n.kind_s() {
            "object_creation_expression" => return true,
            "class_declaration" | "enum_declaration" | "record_declaration" | "interface_declaration" | "enum_constant" | "annotation_type_declaration" => return false,
            _ => {}
        }
        p = cx.parent_of(n);
    }
    false
}

fn ignored_param(cx: &Ctx, opts: &HiddenField, param: Node, name: &[u8]) -> bool {
    let list = cx.parent_of(param);
    let owner = list.and_then(|l| cx.parent_of(l));
    let Some(owner) = owner else { return false };
    if opts.ignore_constructor_parameter && owner.kind_s() == "constructor_declaration" && param.kind_s() != "catch_formal_parameter" {
        return true;
    }
    if owner.kind_s() != "method_declaration" {
        return false;
    }
    if opts.ignore_abstract_methods && has_modifier(cx, owner, b"abstract") {
        return true;
    }
    if opts.ignore_setter {
        let count = list.map_or(0, |l| l.named_children(&mut l.walk()).filter(|c| matches!(c.kind_s(), "formal_parameter" | "spread_parameter" | "receiver_parameter")).count());
        return count == 1 && is_setter(cx, opts, owner, name);
    }
    false
}

fn is_setter(cx: &Ctx, opts: &HiddenField, method: Node, name: &[u8]) -> bool {
    let Some(method_name) = method.child_by_field_id(cx.syn.f.name) else { return false };
    let mut expected = b"set".to_vec();
    let mut chars = name.to_vec();
    // JavaBeans: `xValue` keeps its lower-case first letter after `set`.
    if !(name.len() > 1 && name[1].is_ascii_uppercase()) {
        chars[0] = chars[0].to_ascii_uppercase();
    }
    expected.extend(chars);
    if cx.text(method_name) != expected.as_slice() {
        return false;
    }
    let Some(ty) = method.child_by_field_id(cx.syn.f.type_) else { return false };
    if ty.kind_s() == "void_type" {
        return true;
    }
    if !opts.setter_can_return_its_class {
        return false;
    }
    // Checkstyle compares the first token of the return type with the names of the enclosing
    // classes and enums.
    let first = leading_identifier(ty);
    let mut p = cx.parent_of(method);
    while let Some(n) = p {
        if matches!(n.kind_s(), "class_declaration" | "enum_declaration") && n.child_by_field_id(cx.syn.f.name).is_some_and(|nm| Some(cx.text(nm)) == first.map(|f| cx.text(f))) {
            return true;
        }
        p = cx.parent_of(n);
    }
    false
}

/// The simple type name a return type starts with: not for `a.B` or `B[]`, whose first token is a
/// dot or a bracket; `Foo<X>` starts with `Foo`.
fn leading_identifier(ty: Node) -> Option<Node> {
    match ty.kind_s() {
        "type_identifier" => Some(ty),
        "generic_type" => ty.named_child(0).and_then(leading_identifier),
        _ => None,
    }
}
