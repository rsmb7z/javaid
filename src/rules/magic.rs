//! `MagicNumber`: a numeric literal that is not a named constant.

use crate::engine::Ctx;
use tree_sitter::Node;

/// The value Checkstyle compares with `ignoreNumbers`: underscores and type suffixes dropped, hex, octal
/// and binary read as such.
fn value(text: &str) -> f64 {
    let t: String = text.chars().filter(|c| *c != '_').collect();
    let lower = t.to_ascii_lowercase();
    // Hex, octal and binary literals are bit patterns: 0xffffffff is the int -1, 0xffffffffffffffffL the long -1.
    let long = lower.ends_with('l');
    let int = |digits: &str, radix: u32| match u128::from_str_radix(digits.trim_end_matches('l'), radix) {
        Ok(v) if radix == 10 => v as f64,
        Ok(v) if long => v as u64 as i64 as f64,
        Ok(v) => v as u32 as i32 as f64,
        Err(_) => f64::NAN,
    };
    if let Some(h) = lower.strip_prefix("0x") {
        if let Some((mantissa, exponent)) = h.split_once('p') {
            // A hexadecimal floating-point literal: 0x1.8p1 is 1.5 * 2.
            let exponent: i32 = exponent.trim_end_matches(['f', 'd']).parse().unwrap_or(0);
            let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
            let digits = format!("{whole}{fraction}");
            let Ok(m) = u128::from_str_radix(if digits.is_empty() { "0" } else { &digits }, 16) else { return f64::NAN };
            return m as f64 * 2f64.powi(exponent - 4 * fraction.len() as i32);
        }
        return int(h, 16);
    }
    if let Some(b) = lower.strip_prefix("0b") {
        return int(b, 2);
    }
    let is_float = lower.contains('.') || lower.contains('e') || lower.ends_with('f') || lower.ends_with('d');
    if is_float {
        return lower.trim_end_matches(['f', 'd']).parse().unwrap_or(f64::NAN);
    }
    let lower = lower.trim_end_matches('l');
    if lower.len() > 1 && lower.starts_with('0') {
        return int(&lower[1..], 8);
    }
    int(lower, 10)
}

pub fn on_number(cx: &mut Ctx, literal: Node) {
    let Some(opts) = &cx.cfg.magic_number else { return };
    let Some(parent) = literal.parent() else { return };
    // Checkstyle has no node for parentheses, so `-(5)` is a negative five.
    let mut top = literal;
    while let Some(p) = top.parent().filter(|p| p.kind() == "parenthesized_expression") {
        top = p;
    }
    let sign = top.parent().filter(|p| p.kind() == "unary_expression").and_then(|p| p.child(0)).map(|o| o.kind().to_string());
    let negated = sign.as_deref() == Some("-");
    let text = String::from_utf8_lossy(cx.text(literal)).into_owned();
    let mut v = value(&text);
    if negated {
        v = -v;
    }
    if opts.ignore_numbers.iter().any(|n| n.total_cmp(&v).is_eq()) {
        return;
    }
    let ancestors: Vec<Node> = std::iter::successors(Some(parent), |n| n.parent()).collect();
    if opts.ignore_annotation && ancestors.iter().any(|a| matches!(a.kind(), "annotation" | "marker_annotation")) {
        return;
    }
    if opts.ignore_annotation_element_defaults && is_annotation_default(&ancestors, literal) {
        return;
    }
    if opts.ignore_hash_code_method && in_hash_code(cx, &ancestors) {
        return;
    }
    if opts.ignore_field_declaration && is_field_declaration(&ancestors) {
        return;
    }
    if is_magic(cx, literal, &ancestors) {
        let (at, shown) = match sign.as_deref() {
            Some("-") => (top.parent().unwrap_or(literal), format!("-{text}")),
            Some("+") => (top.parent().unwrap_or(literal), format!("+{text}")),
            _ => (literal, text),
        };
        cx.diag("MagicNumber", at, format!("'{shown}' is a magic number."));
    }
}

fn is_annotation_default(ancestors: &[Node], literal: Node) -> bool {
    ancestors.iter().any(|a| {
        a.kind() == "annotation_type_element_declaration" && a.children(&mut a.walk()).find(|c| c.kind() == "default").is_some_and(|d| d.end_byte() <= literal.start_byte())
    })
}

fn in_hash_code(cx: &Ctx, ancestors: &[Node]) -> bool {
    let Some(method) = ancestors.iter().find(|a| a.kind() == "method_declaration") else { return false };
    let named = method.child_by_field_id(cx.syn.f.name).is_some_and(|n| cx.text(n) == b"hashCode");
    let no_params = method.child_by_field_id(cx.syn.f.parameters).is_some_and(|p| p.named_child_count() == 0);
    named && no_params
}

/// The nearest variable declaration before the nearest type body, and whether that body is a class's,
/// a record's or an anonymous class's.
fn is_field_declaration(ancestors: &[Node]) -> bool {
    for (i, a) in ancestors.iter().enumerate() {
        match a.kind() {
            "local_variable_declaration" | "field_declaration" | "constant_declaration" => {
                let Some(body) = ancestors.get(i + 1) else { return false };
                let Some(owner) = ancestors.get(i + 2) else { return false };
                return body.kind() == "class_body" && matches!(owner.kind(), "class_declaration" | "record_declaration" | "object_creation_expression");
            }
            "class_body" | "interface_body" | "enum_body" | "annotation_type_body" => return false,
            _ => {}
        }
    }
    false
}

fn is_magic(cx: &Ctx, literal: Node, ancestors: &[Node]) -> bool {
    let Some(def_at) = ancestors.iter().position(|a| matches!(a.kind(), "local_variable_declaration" | "field_declaration" | "constant_declaration" | "enum_constant")) else {
        return true;
    };
    let def = ancestors[def_at];
    let constant = match def.kind() {
        "enum_constant" => true,
        _ => {
            // Implicit: the nearest type body is an interface's or an annotation type's. Explicit: `final`.
            let body = ancestors[def_at + 1..].iter().find(|a| matches!(a.kind(), "class_body" | "interface_body" | "enum_body" | "annotation_type_body"));
            body.is_some_and(|b| matches!(b.kind(), "interface_body" | "annotation_type_body")) || crate::rules::util::has_modifier(cx, def, b"final")
        }
    };
    if !constant {
        return true;
    }
    let mut child = literal;
    for a in &ancestors[..def_at] {
        if !waives(cx, *a, child) {
            return true;
        }
        child = *a;
    }
    false
}

/// Is the construct a Checkstyle token that `constantWaiverParentToken` lists?
fn waives(cx: &Ctx, node: Node, child: Node) -> bool {
    let op = |n: Node| n.child_by_field_name("operator").map(|o| cx.text(o).to_vec()).unwrap_or_default();
    match node.kind() {
        // A call whose result is the receiver of another call sits under a DOT, which is not waived.
        "method_invocation" => node.child_by_field_name("object").is_none_or(|o| o.id() != child.id()),
        "variable_declarator" | "parenthesized_expression" | "cast_expression" | "ternary_expression" | "argument_list" | "object_creation_expression"
        | "array_initializer" | "expression_statement" => true,
        "unary_expression" => !matches!(node.child(0).map(|c| c.kind()), Some("!")),
        "binary_expression" => matches!(op(node).as_slice(), b"*" | b"/" | b"+" | b"-" | b"%" | b">>" | b">>>" | b"<<" | b">=" | b">" | b"<=" | b"<" | b"==" | b"!=" | b"^" | b"|" | b"&"),
        "assignment_expression" => op(node) == b"=",
        "array_creation_expression" => child.kind() != "dimensions_expr",
        _ => false,
    }
}
