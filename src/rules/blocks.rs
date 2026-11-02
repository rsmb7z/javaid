//! `EmptyCatchBlock`, `NeedBraces` and `AvoidNestedBlocks`.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use tree_sitter::Node;

pub fn on_catch(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.empty_catch_block else { return };
    let Some(body) = node.child_by_field_id(cx.syn.f.body) else { return };
    // Anything between the braces other than comments is content, including a lone `;`, which is
    // an unnamed token in the tree (Checkstyle does not call `{ ; }` empty).
    let mut cur = body.walk();
    let mut first_comment = None;
    for c in body.children(&mut cur) {
        match c.kind_s() {
            "{" | "}" => {}
            "line_comment" | "block_comment" => {
                first_comment.get_or_insert(c);
            }
            _ => return,
        }
    }
    // Allowed by the catch variable's name (`ignored`, `expected`, ...)?
    if let Some(fmt) = &opts.exception_variable_name {
        let mut cur = node.walk();
        let param = node.children(&mut cur).find(|c| c.kind_s() == "catch_formal_parameter");
        if let Some(name) = param.and_then(|p| p.child_by_field_id(cx.syn.f.name)) {
            if fmt.matches(cx.text(name)) {
                return;
            }
        }
    }
    // Allowed by a comment? By default any comment will do; with a commentFormat the *first* one
    // must match. Found by probing Checkstyle: only the first comment is tested, and its text is
    // exactly what lies between the delimiters, with nothing trimmed.
    let allowed = match first_comment {
        None => false,
        Some(c) => opts.comment_format.as_ref().is_none_or(|fmt| fmt.matches(comment_text(cx.text(c)))),
    };
    if !allowed {
        cx.diag("EmptyCatchBlock", body, "Empty catch block.".into());
    }
}

/// The text of a comment between its delimiters: after `//` to the end of the line, or between
/// `/*` and `*/`.
fn comment_text(raw: &[u8]) -> &[u8] {
    if let Some(rest) = raw.strip_prefix(b"//") {
        rest.strip_suffix(b"\r").unwrap_or(rest)
    } else {
        let rest = raw.strip_prefix(b"/*").unwrap_or(raw);
        rest.strip_suffix(b"*/").unwrap_or(rest)
    }
}

pub fn on_if(cx: &mut Ctx, node: Node) {
    if !cx.cfg.need_braces {
        return;
    }
    if let Some(c) = node.child_by_field_id(cx.syn.f.consequence) {
        if !cx.syn.kind(c).is_block {
            cx.diag("NeedBraces", node, "'if' construct must use '{}'s.".into());
        }
    }
    if let Some(a) = node.child_by_field_id(cx.syn.f.alternative) {
        let k = cx.syn.kind(a);
        if !k.is_block && !k.is_if {
            let at = a.prev_sibling().unwrap_or(a); // the `else` keyword
            cx.diag("NeedBraces", at, "'else' construct must use '{}'s.".into());
        }
    }
}

pub fn on_loop(cx: &mut Ctx, node: Node, keyword: &str) {
    if !cx.cfg.need_braces {
        return;
    }
    if let Some(b) = node.child_by_field_id(cx.syn.f.body) {
        if !cx.syn.kind(b).is_block {
            cx.diag("NeedBraces", node, format!("'{keyword}' construct must use '{{}}'s."));
        }
    }
}

/// `AvoidNestedBlocks`: a `{ ... }` that is a statement of another block. Checkstyle's `case` group
/// is a statement list without braces, so a block that is the only statement of one is the one
/// thing `allowInSwitchCase` excuses; every other block has its closing brace as a sibling.
pub fn on_block(cx: &mut Ctx, block: Node) {
    let Some(opts) = &cx.cfg.avoid_nested_blocks else { return };
    let Some(parent) = cx.parent() else { return };
    let nested = match parent.kind_s() {
        "block" | "constructor_body" => true,
        "switch_block_statement_group" => {
            let mut cur = parent.walk();
            let alone = parent.children(&mut cur).all(|c| c.id() == block.id() || matches!(c.kind_s(), "switch_label" | ":" | "line_comment" | "block_comment"));
            !(opts.allow_in_switch_case && alone)
        }
        _ => false,
    };
    if nested {
        cx.diag("AvoidNestedBlocks", block, "Avoid nested blocks.".into());
    }
}
