//! The naming family.
//!
//! Which declaration belongs to which check was settled by running Checkstyle on
//! `bench/oracle/probes`, not by reading its documentation alone. Notable facts from the probes:
//! catch parameters belong to `CatchParameterName` (a `final` one is also a final local); try
//! resources are final locals even without `final`; constructor type parameters, enum constants
//! and annotation elements are checked by nobody; pattern variables, `final` or not, belong to
//! `PatternVariableName` alone.

use crate::config::Format;
use crate::syntax::KindExt;
use crate::engine::Ctx;
use crate::rules::util::{enclosing_type_name, has_modifier, has_override_annotation};
use tree_sitter::Node;

fn report(cx: &mut Ctx, rule: &'static str, name: Node, fmt: &Format) {
    let text = cx.text(name);
    if !fmt.matches(text) {
        let msg = format!("Name '{}' must match pattern '{}'.", String::from_utf8_lossy(text), fmt.source);
        cx.diag(rule, name, msg);
    }
}

pub fn on_package(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(fmt) = &cfg.naming.package_name else { return };
    let mut cur = node.walk();
    let Some(name) = node.children(&mut cur).find(|c| matches!(c.kind_s(), "scoped_identifier" | "identifier")) else { return };
    report(cx, "PackageName", name, fmt);
}

pub fn on_type_decl(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let t = &cfg.naming.type_name;
    let fmt = match node.kind_s() {
        "class_declaration" => &t.class,
        "interface_declaration" => &t.interface,
        "enum_declaration" => &t.enumeration,
        "annotation_type_declaration" => &t.annotation,
        "record_declaration" => &t.record,
        _ => return,
    };
    let Some(fmt) = fmt else { return };
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, "TypeName", name, fmt);
    }
}

pub fn on_method(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.naming.method_name else { return };
    let Some(name) = node.child_by_field_id(cx.syn.f.name) else { return };
    let text = cx.text(name);
    let bad_format = !opts.format.matches(text);
    let same_as_class = !opts.allow_class_name && enclosing_type_name(cx).is_some_and(|c| c == text);
    if !(bad_format || same_as_class) || has_override_annotation(cx, node) {
        return; // an overriding method cannot be renamed, so it is exempt from both
    }
    if same_as_class {
        cx.diag("MethodName", name, format!("Method Name '{}' must not equal the enclosing class name.", String::from_utf8_lossy(text)));
    }
    if bad_format {
        report(cx, "MethodName", name, &opts.format);
    }
}

pub fn on_field(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let n = &cfg.naming;
    if n.member_name.is_none() && n.constant_name.is_none() && n.static_variable_name.is_none() {
        return;
    }
    // Interface and annotation fields are constants whatever their modifiers say.
    let in_interface = cx.parent().is_some_and(|p| matches!(p.kind_s(), "interface_body" | "annotation_type_body"));
    let is_static = has_modifier(cx, node, b"static");
    let is_final = has_modifier(cx, node, b"final");
    let mut cur = node.walk();
    for d in node.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        let Some(name) = d.child_by_field_id(cx.syn.f.name) else { continue };
        if in_interface || (is_static && is_final) {
            if let Some(fmt) = &n.constant_name {
                if !matches!(cx.text(name), b"serialVersionUID" | b"serialPersistentFields") {
                    report(cx, "ConstantName", name, fmt);
                }
            }
        } else if is_static {
            if let Some(fmt) = &n.static_variable_name {
                report(cx, "StaticVariableName", name, fmt);
            }
        } else if let Some(fmt) = &n.member_name {
            report(cx, "MemberName", name, fmt);
        }
    }
}

fn one_char(name: &[u8]) -> bool {
    std::str::from_utf8(name).is_ok_and(|s| s.chars().count() == 1)
}

pub fn on_local_var(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let n = &cfg.naming;
    if n.local_variable_name.is_none() && n.local_final_variable_name.is_none() {
        return;
    }
    let is_final = has_modifier(cx, node, b"final");
    let in_for_init = cx.parent().is_some_and(|p| p.kind_s() == "for_statement");
    let mut cur = node.walk();
    for d in node.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        let Some(name) = d.child_by_field_id(cx.syn.f.name) else { continue };
        if is_final {
            if let Some(fmt) = &n.local_final_variable_name {
                report(cx, "LocalFinalVariableName", name, fmt);
            }
        } else if let Some(opts) = &n.local_variable_name {
            if opts.allow_one_char_var_in_for_loop && in_for_init && one_char(cx.text(name)) {
                continue;
            }
            report(cx, "LocalVariableName", name, &opts.format);
        }
    }
}

pub fn on_enhanced_for(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let n = &cfg.naming;
    let Some(name) = node.child_by_field_id(cx.syn.f.name) else { return };
    if has_modifier(cx, node, b"final") {
        if let Some(fmt) = &n.local_final_variable_name {
            report(cx, "LocalFinalVariableName", name, fmt);
        }
    } else if let Some(opts) = &n.local_variable_name {
        if opts.allow_one_char_var_in_for_loop && one_char(cx.text(name)) {
            return;
        }
        report(cx, "LocalVariableName", name, &opts.format);
    }
}

/// A try-with-resources variable is implicitly final.
pub fn on_resource(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(fmt) = &cfg.naming.local_final_variable_name else { return };
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, "LocalFinalVariableName", name, fmt);
    }
}

pub fn on_catch_param(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(name) = node.child_by_field_id(cx.syn.f.name) else { return };
    if let Some(fmt) = &cfg.naming.catch_parameter_name {
        report(cx, "CatchParameterName", name, fmt);
    }
    if let Some(fmt) = &cfg.naming.local_final_variable_name {
        if has_modifier(cx, node, b"final") {
            report(cx, "LocalFinalVariableName", name, fmt);
        }
    }
}

/// The name of a `formal_parameter`, or of a varargs `spread_parameter` (which nests it in a declarator).
fn param_name<'t>(cx: &Ctx, param: Node<'t>) -> Option<Node<'t>> {
    if param.kind_s() == "spread_parameter" {
        let mut cur = param.walk();
        let d = param.children(&mut cur).find(|c| c.kind_s() == "variable_declarator")?;
        d.child_by_field_id(cx.syn.f.name)
    } else {
        param.child_by_field_id(cx.syn.f.name)
    }
}

/// Method and constructor parameters, lambda parameters with explicit types, and record
/// components all look alike in the tree; the owner of the parameter list tells them apart.
pub fn on_param(cx: &mut Ctx, node: Node) {
    let Some(owner) = cx.ancestor(1) else { return };
    let cfg = cx.cfg;
    match owner.kind_s() {
        "method_declaration" | "constructor_declaration" => {
            let Some(opts) = &cfg.naming.parameter_name else { return };
            let Some(name) = param_name(cx, node) else { return };
            if opts.format.matches(cx.text(name)) {
                return;
            }
            if opts.ignore_overridden && owner.kind_s() == "method_declaration" && has_override_annotation(cx, owner) {
                return;
            }
            report(cx, "ParameterName", name, &opts.format);
        }
        "lambda_expression" => {
            if let (Some(fmt), Some(name)) = (&cfg.naming.lambda_parameter_name, param_name(cx, node)) {
                report(cx, "LambdaParameterName", name, fmt);
            }
        }
        "record_declaration" => {
            if let (Some(fmt), Some(name)) = (&cfg.naming.record_component_name, param_name(cx, node)) {
                report(cx, "RecordComponentName", name, fmt);
            }
        }
        _ => {}
    }
}

/// `x -> ...` and `(x, y) -> ...`: parameters without types are bare identifiers.
pub fn on_lambda(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(fmt) = &cfg.naming.lambda_parameter_name else { return };
    let Some(params) = node.child_by_field_id(cx.syn.f.parameters) else { return };
    match params.kind_s() {
        "identifier" => report(cx, "LambdaParameterName", params, fmt),
        "inferred_parameters" => {
            let mut cur = params.walk();
            for p in params.named_children(&mut cur).filter(|p| p.kind_s() == "identifier") {
                report(cx, "LambdaParameterName", p, fmt);
            }
        }
        _ => {}
    }
}

/// `o instanceof String s`
pub fn on_instanceof(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(fmt) = &cfg.naming.pattern_variable_name else { return };
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, "PatternVariableName", name, fmt);
    }
}

/// `case Integer i ->`
pub fn on_type_pattern(cx: &mut Ctx, node: Node) {
    pattern_variable(cx, node);
}

/// `Point(int x, var y)`: the components of a record pattern.
pub fn on_record_pattern_component(cx: &mut Ctx, node: Node) {
    pattern_variable(cx, node);
}

fn pattern_variable(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(fmt) = &cfg.naming.pattern_variable_name else { return };
    // The variable is the last child; a nested record pattern has none.
    let last = node.named_child_count().checked_sub(1).and_then(|i| node.named_child(i as u32));
    if let Some(last) = last.filter(|c| c.kind_s() == "identifier") {
        report(cx, "PatternVariableName", last, fmt);
    }
}

pub fn on_type_param(cx: &mut Ctx, node: Node) {
    let Some(owner) = cx.ancestor(1) else { return };
    let cfg = cx.cfg;
    let (rule, fmt) = match owner.kind_s() {
        "class_declaration" => ("ClassTypeParameterName", &cfg.naming.class_type_parameter_name),
        "interface_declaration" => ("InterfaceTypeParameterName", &cfg.naming.interface_type_parameter_name),
        "method_declaration" => ("MethodTypeParameterName", &cfg.naming.method_type_parameter_name),
        "record_declaration" => ("RecordTypeParameterName", &cfg.naming.record_type_parameter_name),
        _ => return, // e.g. generic constructors: Checkstyle checks none of these
    };
    let Some(fmt) = fmt else { return };
    let mut cur = node.walk();
    if let Some(name) = node.children(&mut cur).find(|c| c.kind_s() == "type_identifier") {
        report(cx, rule, name, fmt);
    }
}
