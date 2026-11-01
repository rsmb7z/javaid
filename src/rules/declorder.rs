//! `DeclarationOrder`: static variables, instance variables, constructors, methods, in that order,
//! and within the variables public before protected before package before private.
//!
//! One state per type body, kept while the walk is inside it. The file-wide set of field names
//! behind the forward-reference exemption grows in document order, as Checkstyle's does, so a
//! name only counts once its field has been seen (a nested class's fields included).

use crate::engine::Ctx;
use std::collections::HashSet;
use tree_sitter::Node;

const STATIC_VARIABLE: u8 = 1;
const INSTANCE_VARIABLE: u8 = 2;
const CONSTRUCTOR: u8 = 3;
const METHOD: u8 = 4;

struct Body {
    end: usize,
    state: u8,
    /// Index into public, protected, package, private.
    access: u8,
    interface: bool,
}

#[derive(Default)]
pub struct State {
    bodies: Vec<Body>,
    fields: HashSet<Vec<u8>>,
}

pub fn on_node(cx: &mut Ctx, st: &mut State, node: Node) {
    let Some(opts) = &cx.cfg.declaration_order else { return };
    let (ignore_modifiers, ignore_constructors) = (opts.ignore_modifiers, opts.ignore_constructors);
    while st.bodies.last().is_some_and(|b| node.start_byte() >= b.end) {
        st.bodies.pop();
    }
    let in_body = node.parent().is_some_and(|p| matches!(p.kind(), "class_body" | "interface_body" | "annotation_type_body" | "enum_body_declarations"));
    match node.kind() {
        "class_body" | "enum_body" => st.bodies.push(Body { end: node.end_byte(), state: STATIC_VARIABLE, access: 0, interface: false }),
        "interface_body" | "annotation_type_body" => st.bodies.push(Body { end: node.end_byte(), state: STATIC_VARIABLE, access: 0, interface: true }),
        "method_declaration" if in_body => {
            if let Some(b) = st.bodies.last_mut() {
                b.state = METHOD;
            }
        }
        "constructor_declaration" | "compact_constructor_declaration" if in_body => {
            let Some(b) = st.bodies.last_mut() else { return };
            if b.state > CONSTRUCTOR {
                if !ignore_constructors {
                    cx.diag_once("DeclarationOrder", node, "Constructor definition in wrong order.".to_string());
                }
            } else {
                b.state = CONSTRUCTOR;
            }
        }
        "field_declaration" | "constant_declaration" if in_body => field(cx, st, node, ignore_modifiers),
        _ => {}
    }
}

fn field(cx: &mut Ctx, st: &mut State, decl: Node, ignore_modifiers: bool) {
    let mut scope = 2u8;
    let mut is_static = false;
    if let Some(mods) = decl.children(&mut decl.walk()).find(|c| c.kind() == "modifiers") {
        for m in mods.children(&mut mods.walk()) {
            match cx.text(m) {
                b"public" => scope = 0,
                b"protected" => scope = 1,
                b"private" => scope = 3,
                b"static" => is_static = true,
                _ => {}
            }
        }
    }
    let mut cur = decl.walk();
    let declarators: Vec<Node> = decl.children_by_field_id(cx.syn.f.declarator, &mut cur).collect();
    for d in declarators {
        let Some(name) = d.child_by_field_id(cx.syn.f.name) else { continue };
        let Some(body) = st.bodies.last() else { return };
        if !body.interface {
            st.fields.insert(cx.text(name).to_vec());
        }
        let b = st.bodies.last_mut().unwrap();
        let mut valid = true;
        if !is_static {
            if b.state > INSTANCE_VARIABLE {
                cx.diag_once("DeclarationOrder", decl, "Instance variable definition in wrong order.".to_string());
                valid = false;
            } else if b.state == STATIC_VARIABLE {
                b.access = 0;
                b.state = INSTANCE_VARIABLE;
            }
        } else if b.state > INSTANCE_VARIABLE || (b.state == INSTANCE_VARIABLE && !ignore_modifiers) {
            cx.diag_once("DeclarationOrder", decl, "Static variable definition in wrong order.".to_string());
            valid = false;
        }
        // The access level moves on whether or not the kind of variable was in order.
        let b = st.bodies.last_mut().unwrap();
        if b.access > scope {
            if valid && !ignore_modifiers && !forward_reference(cx, st, d) {
                cx.diag_once("DeclarationOrder", decl, "Variable access definition in wrong order.".to_string());
            }
        } else {
            b.access = scope;
        }
    }
}

/// Whether the initializer (anything after the name) mentions a field already seen.
fn forward_reference(cx: &Ctx, st: &State, declarator: Node) -> bool {
    let name = declarator.child_by_field_id(cx.syn.f.name);
    let mut stack: Vec<Node> = Vec::new();
    let mut cur = declarator.walk();
    for c in declarator.children(&mut cur) {
        if Some(c) != name {
            stack.push(c);
        }
    }
    while let Some(n) = stack.pop() {
        if matches!(n.kind(), "identifier" | "type_identifier") && st.fields.contains(cx.text(n)) {
            return true;
        }
        let mut c = n.walk();
        stack.extend(n.children(&mut c));
    }
    false
}
