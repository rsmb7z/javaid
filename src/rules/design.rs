//! Class-design modules: `FinalClass`, `HideUtilityClassConstructor`, `InterfaceIsType` and
//! `VisibilityModifier`.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use crate::rules::util::has_modifier;
use tree_sitter::Node;

/// What `FinalClass` keeps while it walks a file.
#[derive(Default)]
pub struct State {
    package: String,
    /// The type declarations around the node being visited, innermost last.
    stack: Vec<Open>,
    /// Every class declaration, in order, and where the name was first met.
    classes: Vec<Class>,
    by_name: std::collections::HashMap<String, usize>,
    anonymous: Vec<(String, String)>,
}

struct Open {
    qualified: String,
    end: usize,
    class: Option<usize>,
    is_enum_or_record: bool,
}

struct Class {
    qualified: String,
    depth: usize,
    is_final: bool,
    is_abstract: bool,
    is_private: bool,
    /// Checkstyle's `hasDeclaredConstructor`, which is true when the class declares *no* constructor.
    no_constructor: bool,
    non_private_ctor: bool,
    nested_subclass: bool,
    anonymous_super: bool,
    position: (u32, u32),
    super_name: Option<String>,
}

impl State {
    fn pop_until(&mut self, at: usize) {
        while self.stack.last().is_some_and(|o| o.end <= at) {
            self.stack.pop();
        }
    }
}

/// The name of a type as written, without its type arguments or any blank.
fn type_name(cx: &Ctx, ty: Node) -> String {
    let base = if ty.kind_s() == "generic_type" { ty.child(0).unwrap_or(ty) } else { ty };
    String::from_utf8_lossy(cx.text(base)).chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn on_package(cx: &Ctx, st: &mut State, node: Node) {
    if !cx.cfg.final_class {
        return;
    }
    let mut cur = node.walk();
    let name = node.children(&mut cur).find(|c| matches!(c.kind_s(), "scoped_identifier" | "identifier"));
    st.package = name.map(|n| String::from_utf8_lossy(cx.text(n)).chars().filter(|c| !c.is_whitespace()).collect()).unwrap_or_default();
}

/// Called for every type declaration (class, interface, enum, record, annotation type).
pub fn on_type_decl(cx: &Ctx, st: &mut State, node: Node) {
    if !cx.cfg.final_class {
        return;
    }
    st.pop_until(node.start_byte());
    let Some(name) = node.child_by_field_id(cx.syn.f.name) else { return };
    let name = String::from_utf8_lossy(cx.text(name)).into_owned();
    let qualified = match st.stack.last() {
        Some(outer) => format!("{}.{name}", outer.qualified),
        None if st.package.is_empty() => name,
        None => format!("{}.{name}", st.package),
    };
    let is_class = node.kind_s() == "class_declaration";
    let class = is_class.then(|| {
        let body = node.child_by_field_id(cx.syn.f.body);
        let no_constructor = body.is_none_or(|b| !b.children(&mut b.walk()).any(|c| c.kind_s() == "constructor_declaration"));
        let super_name = node.child_by_field_name("superclass").and_then(|s| s.named_child(0)).map(|t| type_name(cx, t));
        let c = Class {
            qualified: qualified.clone(),
            depth: st.stack.len(),
            is_final: has_modifier(cx, node, b"final"),
            is_abstract: has_modifier(cx, node, b"abstract"),
            is_private: has_modifier(cx, node, b"private"),
            no_constructor,
            non_private_ctor: false,
            nested_subclass: false,
            anonymous_super: false,
            position: (node.start_position().row as u32 + 1, node.start_position().column as u32 + 1),
            super_name,
        };
        // A second class with the same qualified name replaces the first, as in a map.
        match st.by_name.get(&qualified) {
            Some(&i) => {
                st.classes[i] = c;
                i
            }
            None => {
                st.classes.push(c);
                st.by_name.insert(qualified.clone(), st.classes.len() - 1);
                st.classes.len() - 1
            }
        }
    });
    st.stack.push(Open { qualified, end: node.end_byte(), class, is_enum_or_record: matches!(node.kind_s(), "enum_declaration" | "record_declaration") });
}

pub fn on_constructor(cx: &Ctx, st: &mut State, node: Node) {
    if !cx.cfg.final_class || node.kind_s() != "constructor_declaration" {
        return;
    }
    st.pop_until(node.start_byte());
    let Some(top) = st.stack.last() else { return };
    if top.is_enum_or_record || has_modifier(cx, node, b"private") {
        return;
    }
    if let Some(i) = top.class {
        st.classes[i].non_private_ctor = true;
    }
}

/// `new X() { ... }`: remember which type declaration it sits in, and the class it extends.
pub fn on_new(cx: &Ctx, st: &mut State, node: Node) {
    if !cx.cfg.final_class || !node.children(&mut node.walk()).any(|c| c.kind_s() == "class_body") {
        return;
    }
    // Found by probing: when a member is selected from the new object (`new B() { }.go()`), Checkstyle's
    // tree no longer ends the `new` with the class body, and the class is not counted as extended.
    if cx.parent_of(node).is_some_and(|p| {
        (matches!(p.kind_s(), "method_invocation" | "field_access") && p.child_by_field_name("object").is_some_and(|o| o.id() == node.id()))
            || (p.kind_s() == "object_creation_expression" && p.child(0).is_some_and(|c| c.id() == node.id()))
    }) {
        return;
    }
    // Found by probing: neither is one that is, through operators, casts and `?:`, the value assigned by
    // an assignment expression (`o = new B() { }`; a declaration's initialiser and an argument are fine).
    let mut up = node;
    while let Some(p) = cx.parent_of(up) {
        match p.kind_s() {
            "assignment_expression" => return,
            "parenthesized_expression" | "cast_expression" | "ternary_expression" | "binary_expression" | "unary_expression" | "array_access" | "instanceof_expression" => up = p,
            _ => break,
        }
    }
    st.pop_until(node.start_byte());
    let Some(ty) = node.child_by_field_name("type") else { return };
    let name = type_name(cx, ty);
    let short = name.rsplit('.').next().unwrap_or(&name).to_string();
    let outer = st.stack.last().map_or_else(|| st.package.clone(), |o| o.qualified.clone());
    st.anonymous.push((short, outer));
}

/// How many leading characters of `a` and `b` agree up to the last `.` they share (Checkstyle's measure
/// of how close two qualified names are).
fn matching(pattern: &str, other: &str) -> usize {
    let (p, o) = (pattern.as_bytes(), other.as_bytes());
    let min = p.len().min(o.len());
    let tail_is_dot = o.len() > min && o[min] == b'.';
    let mut result = 0;
    let mut i = 0;
    while i < min && p[i] == o[i] {
        if (i == min - 1 && tail_is_dot) || p[i] == b'.' {
            result = i;
        }
        i += 1;
    }
    result
}

fn nearest(st: &State, name: &str, count: impl Fn(&Class) -> usize) -> Option<usize> {
    let suffix = format!(".{name}");
    let mut best: Option<usize> = None;
    for (i, c) in st.classes.iter().enumerate() {
        if !c.qualified.ends_with(&suffix) {
            continue;
        }
        best = match best {
            None => Some(i),
            Some(b) => {
                let (cb, ci) = (count(&st.classes[b]), count(c));
                if ci > cb || (ci == cb && c.depth < st.classes[b].depth) {
                    Some(i)
                } else {
                    Some(b)
                }
            }
        };
    }
    best.or_else(|| st.by_name.get(name).copied())
}

pub fn finish(cx: &mut Ctx, st: &mut State) {
    if !cx.cfg.final_class {
        return;
    }
    for (short, outer) in std::mem::take(&mut st.anonymous) {
        if let Some(i) = nearest(st, &short, |c| matching(&outer, &c.qualified)) {
            st.classes[i].anonymous_super = true;
        }
    }
    for k in 0..st.classes.len() {
        let (qualified, sup) = (st.classes[k].qualified.clone(), st.classes[k].super_name.clone());
        if let Some(sup) = sup {
            if let Some(i) = nearest(st, &sup, |c| matching(&qualified, &c.qualified)) {
                st.classes[i].nested_subclass = true;
            }
        }
    }
    for c in &st.classes {
        let skip = c.is_final || c.is_abstract || c.anonymous_super || c.nested_subclass;
        let should = !skip && if c.no_constructor { c.is_private } else { !c.non_private_ctor };
        if should {
            let name = c.qualified.rsplit('.').next().unwrap_or(&c.qualified);
            cx.diag_at("FinalClass", c.position.0, c.position.1, format!("Class {name} should be declared as final."));
        }
    }
}

/// `HideUtilityClassConstructor`: a class that holds only static members but offers a public or
/// default constructor. A `static` nested class, an abstract one and one with an `extends` clause are left alone.
pub fn on_class(cx: &mut Ctx, node: Node) {
    if !cx.cfg.hide_utility_class_constructor || node.kind_s() != "class_declaration" {
        return;
    }
    if has_modifier(cx, node, b"abstract") || has_modifier(cx, node, b"static") || node.child_by_field_name("superclass").is_some() {
        return;
    }
    let Some(body) = node.child_by_field_id(cx.syn.f.body) else { return };
    let (mut default_ctor, mut public_ctor, mut instance_member, mut static_member) = (true, false, false, false);
    let mut cur = body.walk();
    for member in body.children(&mut cur) {
        match member.kind_s() {
            "constructor_declaration" => {
                default_ctor = false;
                if !has_modifier(cx, member, b"private") && !has_modifier(cx, member, b"protected") {
                    public_ctor = true;
                }
            }
            "method_declaration" | "field_declaration" => {
                if has_modifier(cx, member, b"static") {
                    if !has_modifier(cx, member, b"private") {
                        static_member = true;
                    }
                } else {
                    instance_member = true;
                }
            }
            _ => {}
        }
    }
    if !instance_member && static_member && (default_ctor || public_ctor) {
        cx.diag("HideUtilityClassConstructor", node, "Utility classes should not have a public or default constructor.".into());
    }
}

/// `InterfaceIsType`: an interface with no methods is a bag of constants, or (when markers are not
/// allowed) a marker.
pub fn on_interface(cx: &mut Ctx, node: Node) {
    let Some(allow_markers) = cx.cfg.interface_is_type else { return };
    let Some(body) = node.child_by_field_id(cx.syn.f.body) else { return };
    let mut cur = body.walk();
    let (mut methods, mut fields) = (false, false);
    for m in body.children(&mut cur) {
        match m.kind_s() {
            "method_declaration" => methods = true,
            "constant_declaration" | "field_declaration" => fields = true,
            _ => {}
        }
    }
    if !methods && (!allow_markers || fields) {
        cx.diag("InterfaceIsType", node, "interfaces should describe a type and hence have methods.".into());
    }
}

/// `FinalParameters`: a parameter of a method or constructor that has a body, a catch parameter or
/// the variable of an enhanced `for` that is not `final`. With `ignorePrimitiveTypes`, parameters of
/// a primitive type are left alone (and so is `int... xs`, which Checkstyle does not see as an array).
pub fn on_final_parameter(cx: &mut Ctx, node: Node) {
    let Some(opts) = &cx.cfg.final_parameters else { return };
    let (tokens, ignore_primitive) = (opts.tokens, opts.ignore_primitive_types);
    use crate::rules::tokens::Token;
    let name = match node.kind_s() {
        "formal_parameter" | "spread_parameter" => {
            let Some(list) = cx.parent_of(node).filter(|p| p.kind_s() == "formal_parameters") else { return };
            let Some(owner) = cx.parent_of(list) else { return };
            let wanted = match owner.kind_s() {
                "method_declaration" => tokens.contains(Token::MethodDef) && owner.child_by_field_id(cx.syn.f.body).is_some(),
                "constructor_declaration" => tokens.contains(Token::CtorDef),
                _ => false,
            };
            if !wanted {
                return;
            }
            node.children(&mut node.walk()).filter(|c| c.kind_s() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)).or_else(|| node.child_by_field_id(cx.syn.f.name))
        }
        "catch_formal_parameter" => {
            if !tokens.contains(Token::LiteralCatch) {
                return;
            }
            node.child_by_field_id(cx.syn.f.name)
        }
        "enhanced_for_statement" => {
            if !tokens.contains(Token::ForEachClause) {
                return;
            }
            node.child_by_field_id(cx.syn.f.name)
        }
        _ => return,
    };
    let Some(name) = name else { return };
    if has_modifier(cx, node, b"final") {
        return;
    }
    if ignore_primitive {
        // The type is a direct child; upstream grammars do not name it in a spread parameter.
        let primitive = node.children(&mut node.walk()).any(|t| matches!(t.kind_s(), "integral_type" | "floating_point_type" | "boolean_type"));
        // `int x[]` keeps its brackets after the name: an array.
        let c_style_array = node.children(&mut node.walk()).any(|c| c.kind_s() == "dimensions") || cx.parent_of(name).is_some_and(|p| p.children(&mut p.walk()).any(|c| c.kind_s() == "dimensions"));
        if primitive && !c_style_array {
            return;
        }
    }
    let text = String::from_utf8_lossy(cx.text(name)).into_owned();
    // Reported where the parameter starts, which for the variable of an enhanced `for` is its first
    // modifier or type, not the `for`.
    let anchor = if node.kind_s() == "enhanced_for_statement" {
        node.children(&mut node.walk()).find(|c| c.kind_s() == "modifiers" || Some(c.id()) == node.child_by_field_id(cx.syn.f.type_).map(|t| t.id())).unwrap_or(name)
    } else {
        node
    };
    let at = anchor.start_position();
    cx.diag_at("FinalParameters", at.row as u32 + 1, at.column as u32 + 1, format!("Parameter {text} should be final."));
}

/// A field that is not private needs a reason: `static final`, a waiver in the configuration, or an
/// ignored annotation. Fields of an interface or annotation type are constants and never reported;
/// a class nested in one is checked like any other. Local and anonymous classes are checked too.
pub fn on_visibility(cx: &mut Ctx, field: Node) {
    let Some(opts) = &cx.cfg.visibility_modifier else { return };
    if cx.parent().is_some_and(|p| matches!(p.kind_s(), "interface_body" | "annotation_type_body")) {
        return;
    }
    let mut scope = "package";
    let mut is_static = false;
    let mut is_final = false;
    let mut ignored = false;
    if let Some(mods) = field.children(&mut field.walk()).find(|c| c.kind_s() == "modifiers") {
        for m in mods.children(&mut mods.walk()) {
            match m.kind_s() {
                "marker_annotation" | "annotation" => {
                    let name = m.child_by_field_id(cx.syn.f.name).map(|n| cx.text(n).iter().filter(|b| !b.is_ascii_whitespace()).map(|b| *b as char).collect::<String>());
                    ignored |= name.is_some_and(|n| opts.ignore_annotations.contains(&n));
                }
                _ => match cx.text(m) {
                    b"public" => scope = "public",
                    b"protected" => scope = "protected",
                    b"private" => scope = "private",
                    b"static" => is_static = true,
                    b"final" => is_final = true,
                    _ => {}
                },
            }
        }
    }
    if ignored || scope == "private" || (is_static && is_final) {
        return;
    }
    let (package_allowed, protected_allowed, final_public) = (opts.package_allowed, opts.protected_allowed, opts.allow_public_final_fields);
    let mut cur = field.walk();
    for d in field.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        let Some(name) = d.child_by_field_id(cx.syn.f.name) else { continue };
        let text = cx.text(name);
        let allowed = (package_allowed && scope == "package")
            || (protected_allowed && scope == "protected")
            || (scope == "public" && opts.public_member_pattern.matches(text))
            || (final_public && is_final); // whatever the scope: `final int x;` passes too
        if !allowed {
            let msg = format!("Variable '{}' must be private and have accessor methods.", String::from_utf8_lossy(text));
            cx.diag("VisibilityModifier", name, msg);
        }
    }
}
