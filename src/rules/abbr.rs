//! `AbbreviationAsWordInName`: a run of capital letters longer than `allowedAbbreviationLength`.
//!
//! The declarations it looks at are Checkstyle's CLASS_DEF .. RECORD_COMPONENT_DEF tokens, which
//! the tree-sitter grammar spreads over several node kinds; the hooks below mirror those of the
//! naming and hidden-field rules.

use crate::config::AbbreviationAsWordInName;
use crate::engine::Ctx;
use crate::rules::util::has_modifier;
use crate::syntax::KindExt;
use tree_sitter::Node;

/// The first run of capitals that is too long, as Checkstyle reports it.
fn disallowed(name: &str, opts: &AbbreviationAsWordInName) -> bool {
    let chars: Vec<char> = name.chars().collect();
    let allowed = opts.allowed_length as i64;
    let mut begin: i64 = 0;
    let mut started = false;
    let illegal = |b: i64, e: i64, allowed_len: i64| -> bool {
        if e - b > allowed_len {
            let abbr: String =
                if e as usize == chars.len() - 1 { chars[b as usize..].iter().collect() } else { chars[b as usize..e as usize].iter().collect() };
            !opts.allowed_abbreviations.contains(&abbr)
        } else {
            false
        }
    };
    for (i, c) in chars.iter().enumerate() {
        let i = i as i64;
        if c.is_uppercase() {
            if !started {
                started = true;
                begin = i;
            }
        } else if started {
            started = false;
            let (end, allowed_len) = if *c == '_' { (i, allowed + 1) } else { (i - 1, allowed) };
            if illegal(begin, end, allowed_len) {
                return true;
            }
            begin = -1;
        }
    }
    started && illegal(begin, chars.len() as i64 - 1, allowed)
}

fn wants<'a>(cx: &Ctx<'a>, token: &str) -> Option<&'a AbbreviationAsWordInName> {
    let opts = cx.cfg.abbreviation_as_word_in_name.as_ref()?;
    opts.tokens.iter().any(|t| t == token).then_some(opts)
}

fn report(cx: &mut Ctx, opts: &AbbreviationAsWordInName, name: Node) {
    let text = String::from_utf8_lossy(cx.text(name)).into_owned();
    if disallowed(&text, opts) {
        cx.diag(
            "AbbreviationAsWordInName",
            name,
            format!("Abbreviation in name '{text}' must contain no more than '{}' consecutive capital letters.", opts.allowed_length + 1),
        );
    }
}

pub fn on_type_decl(cx: &mut Ctx, node: Node) {
    let token = match node.kind_s() {
        "class_declaration" => "CLASS_DEF",
        "interface_declaration" => "INTERFACE_DEF",
        "enum_declaration" => "ENUM_DEF",
        "annotation_type_declaration" => "ANNOTATION_DEF",
        "record_declaration" => "RECORD_DEF",
        _ => return,
    };
    let cfg = cx.cfg;
    let Some(opts) = cfg.abbreviation_as_word_in_name.as_ref() else { return };
    if opts.tokens.iter().any(|t| t == token) {
        if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
            report(cx, opts, name);
        }
    }
    let Some(body) = node.child_by_field_id(cx.syn.f.body) else { return };
    let mut cur = body.walk();
    for member in body.named_children(&mut cur) {
        match member.kind_s() {
            "annotation_type_element_declaration" if opts.tokens.iter().any(|t| t == "ANNOTATION_FIELD_DEF") => {
                if let Some(name) = member.child_by_field_id(cx.syn.f.name) {
                    report(cx, opts, name);
                }
            }
            "enum_constant" if opts.tokens.iter().any(|t| t == "ENUM_CONSTANT_DEF") => {
                if let Some(name) = member.child_by_field_id(cx.syn.f.name) {
                    report(cx, opts, name);
                }
            }
            _ => {}
        }
    }
}

fn has_override(cx: &Ctx, method: Node) -> bool {
    let mut cur = method.walk();
    let Some(mods) = method.children(&mut cur).find(|c| c.kind_s() == "modifiers") else { return false };
    let mut inner = mods.walk();
    let found = mods
        .children(&mut inner)
        .any(|c| matches!(c.kind_s(), "marker_annotation" | "annotation") && c.child_by_field_id(cx.syn.f.name).is_some_and(|n| cx.text(n) == b"Override"));
    found
}

pub fn on_method(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "METHOD_DEF") else { return };
    if opts.ignore_overridden_methods && has_override(cx, node) {
        return;
    }
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, opts, name);
    }
}

fn ignored_by_modifiers(cx: &Ctx, decl: Node, opts: &AbbreviationAsWordInName) -> bool {
    let is_static = has_modifier(cx, decl, b"static");
    let is_final = has_modifier(cx, decl, b"final");
    if is_static && is_final { opts.ignore_static_final } else { opts.ignore_static && is_static || opts.ignore_final && is_final }
}

pub fn on_field(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "VARIABLE_DEF") else { return };
    let in_interface = cx.parent().is_some_and(|p| matches!(p.kind_s(), "interface_body" | "annotation_type_body"));
    let ignore = if in_interface { opts.ignore_static_final } else { ignored_by_modifiers(cx, node, opts) };
    if ignore {
        return;
    }
    let mut cur = node.walk();
    for d in node.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        if let Some(name) = d.child_by_field_id(cx.syn.f.name) {
            report(cx, opts, name);
        }
    }
}

pub fn on_local_var(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "VARIABLE_DEF") else { return };
    if ignored_by_modifiers(cx, node, opts) {
        return;
    }
    let mut cur = node.walk();
    for d in node.children_by_field_id(cx.syn.f.declarator, &mut cur) {
        if let Some(name) = d.child_by_field_id(cx.syn.f.name) {
            report(cx, opts, name);
        }
    }
}

pub fn on_enhanced_for(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "VARIABLE_DEF") else { return };
    if ignored_by_modifiers(cx, node, opts) {
        return;
    }
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, opts, name);
    }
}

pub fn on_param(cx: &mut Ctx, node: Node) {
    let record = node.parent().and_then(|l| l.parent()).is_some_and(|o| o.kind_s() == "record_declaration");
    let Some(opts) = wants(cx, if record { "RECORD_COMPONENT_DEF" } else { "PARAMETER_DEF" }) else { return };
    let name = match node.kind_s() {
        "formal_parameter" | "catch_formal_parameter" => node.child_by_field_id(cx.syn.f.name),
        "spread_parameter" => node.children(&mut node.walk()).filter(|c| c.kind_s() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)),
        _ => None,
    };
    if let Some(name) = name {
        report(cx, opts, name);
    }
}

pub fn on_lambda(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "PARAMETER_DEF") else { return };
    let Some(params) = node.child_by_field_id(cx.syn.f.parameters) else { return };
    match params.kind_s() {
        // A lone `x ->` parameter is a bare IDENT in Checkstyle's tree, not a PARAMETER_DEF.
        "inferred_parameters" => {
            let mut cur = params.walk();
            for p in params.named_children(&mut cur).filter(|p| p.kind_s() == "identifier") {
                report(cx, opts, p);
            }
        }
        _ => {}
    }
}

pub fn on_instanceof(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "PATTERN_VARIABLE_DEF") else { return };
    if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
        report(cx, opts, name);
    }
}

pub fn on_pattern(cx: &mut Ctx, node: Node) {
    let Some(opts) = wants(cx, "PATTERN_VARIABLE_DEF") else { return };
    let last = node.named_child_count().checked_sub(1).and_then(|i| node.named_child(i as u32));
    if let Some(name) = last.filter(|c| c.kind_s() == "identifier") {
        report(cx, opts, name);
    }
}
