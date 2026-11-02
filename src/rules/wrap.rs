//! Where a line breaks around a token: `OperatorWrap`, `SeparatorWrap` and `LeftCurly`.
//!
//! Checkstyle decides by comparing lines of tree nodes, not by reading text, so a comment after an
//! operator is no obstacle and `x = new` at the end of a line counts as an operator at the end of
//! its line. javaid walks tokens instead of Checkstyle's tree: the token before an operator is the
//! last of its left operand, and the token after it is the first of its right operand, except that
//! Checkstyle's tree puts a prefix operator, `new` and a cast's `(` above the operand they
//! introduce, so those are passed over.

use super::tokens::Token;
use crate::syntax::KindExt;
use super::pad;
use crate::config::{CurlyOption, RightCurlyOption};
use super::whitespace::{is_comment, is_empty_statement, operator_token, punctuation_token};
use crate::engine::Ctx;
use tree_sitter::Node;

/// An operator token, for the rules that look at which line it is on.
pub fn on_token(cx: &mut Ctx, node: Node) {
    if !cx.cfg.separator_wrap.is_empty() {
        separator_wrap(cx, node);
    }
    if !cx.cfg.left_curly.is_empty() && node.kind_s() == "{" {
        left_curly(cx, node);
    }
    if cx.cfg.empty_block.is_some() && node.kind_s() == "{" {
        empty_block(cx, node);
    }
    if cx.cfg.right_curly.is_some() && node.kind_s() == "{" {
        right_curly(cx, node);
    }
    if cx.cfg.operator_wrap.is_empty() {
        return;
    }
    let Some(token) = wrapped_token(cx, node) else { return };
    let cfg = cx.cfg;
    let mut instances = cfg.operator_wrap.iter().filter(|m| m.tokens.contains(token)).peekable();
    if instances.peek().is_none() {
        return;
    }
    let row = node.start_position().row;
    let left_on_line = previous_token(node).is_none_or(|t| t.end_position().row == row);
    let right_on_line = right_operand_start(node).is_none_or(|t| t.start_position().row == row);
    let text = String::from_utf8_lossy(cx.text(node)).into_owned();
    for instance in instances {
        if instance.end_of_line {
            if !left_on_line {
                cx.diag(instance.name, node, format!("'{text}' should be on the previous line."));
            }
        } else if left_on_line && !right_on_line {
            cx.diag(instance.name, node, format!("'{text}' should be on a new line."));
        }
    }
}

/// What a `{` opens, which decides what follows it: a block's `{` is followed by statements, a
/// type's by members, a `switch`'s by its cases.
#[derive(Clone, Copy, PartialEq)]
enum Opens {
    Block,
    Members,
    Cases,
}

/// A construct Checkstyle checks the brace of, and the two facts about it that the check reads.
struct Owner<'a> {
    token: Token,
    /// The node whose first token starts the construct, for `nlow`.
    decl: Node<'a>,
    opens: Opens,
    /// The declaration is an enum with no annotation, whose braces `ignoreEnums` excuses.
    plain_enum: bool,
    /// The body of an anonymous class.
    anonymous: bool,
}

/// Which constructs have this `{` as the brace Checkstyle looks at. A switch rule's block belongs
/// to both the rule and its `case` or `default`.
fn brace_owners<'a>(cx: &Ctx<'a>, brace: Node<'a>) -> Vec<Owner<'a>> {
    let (Some(parent), Some(grand)) = (cx.parent(), cx.ancestor(1)) else { return Vec::new() };
    let owner = |token, opens| Owner { token, decl: grand, opens, plain_enum: false, anonymous: false };
    let is = |field: &str| grand.child_by_field_name(field).is_some_and(|c| c.id() == parent.id());
    let label_token = |label: Option<Node>| match label {
        Some(l) if cx.text(l).starts_with(b"default") => Token::LiteralDefault,
        _ => Token::LiteralCase,
    };
    match parent.kind_s() {
        "block" => match grand.kind_s() {
            "method_declaration" => vec![owner(Token::MethodDef, Opens::Block)],
            "compact_constructor_declaration" => vec![owner(Token::CompactCtorDef, Opens::Block)],
            "static_initializer" => vec![owner(Token::StaticInit, Opens::Block)],
            "lambda_expression" => vec![owner(Token::Lambda, Opens::Block)],
            "if_statement" if is("consequence") => vec![owner(Token::LiteralIf, Opens::Block)],
            "if_statement" if is("alternative") => {
                let keyword = previous_non_comment(parent).unwrap_or(grand);
                vec![Owner { decl: keyword, ..owner(Token::LiteralElse, Opens::Block) }]
            }
            "while_statement" => vec![owner(Token::LiteralWhile, Opens::Block)],
            "do_statement" => vec![owner(Token::LiteralDo, Opens::Block)],
            "for_statement" | "enhanced_for_statement" => vec![owner(Token::LiteralFor, Opens::Block)],
            "synchronized_statement" => vec![owner(Token::LiteralSynchronized, Opens::Block)],
            "try_statement" | "try_with_resources_statement" if is("body") => vec![owner(Token::LiteralTry, Opens::Block)],
            "catch_clause" => vec![owner(Token::LiteralCatch, Opens::Block)],
            "finally_clause" => vec![owner(Token::LiteralFinally, Opens::Block)],
            "switch_rule" => {
                let label = preceding_label(parent);
                vec![
                    owner(Token::SwitchRule, Opens::Block),
                    Owner { decl: label.unwrap_or(grand), ..owner(label_token(label), Opens::Block) },
                ]
            }
            // Only the block that comes first after a label is the one the label owns.
            "switch_block_statement_group" => match preceding_label(parent) {
                Some(label) => vec![Owner { decl: label, ..owner(label_token(Some(label)), Opens::Block) }],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },
        "constructor_body" => vec![owner(Token::CtorDef, Opens::Block)],
        "class_body" => match grand.kind_s() {
            "class_declaration" => vec![owner(Token::ClassDef, Opens::Members)],
            "record_declaration" => vec![owner(Token::RecordDef, Opens::Members)],
            "enum_constant" => vec![owner(Token::EnumConstantDef, Opens::Members)],
            "object_creation_expression" => vec![Owner { anonymous: true, decl: brace, ..owner(Token::Objblock, Opens::Members) }],
            _ => Vec::new(),
        },
        "interface_body" => vec![owner(Token::InterfaceDef, Opens::Members)],
        "annotation_type_body" => vec![owner(Token::AnnotationDef, Opens::Members)],
        "enum_body" => {
            // `ignoreEnums` excuses an enum only while no annotation comes first.
            let annotated = grand.children(&mut grand.walk()).find(|c| c.kind_s() == "modifiers").is_some_and(|m| {
                m.children(&mut m.walk()).any(|a| matches!(a.kind_s(), "annotation" | "marker_annotation"))
            });
            vec![Owner { plain_enum: !annotated, ..owner(Token::EnumDef, Opens::Members) }]
        }
        "switch_block" => vec![owner(Token::LiteralSwitch, Opens::Cases)],
        _ => {
            let _ = brace;
            Vec::new()
        }
    }
}

/// The `case` or `default` label right before a block, over the `:` or `->` that follows it.
fn preceding_label(block: Node) -> Option<Node> {
    let mut n = block;
    loop {
        n = n.prev_sibling()?;
        match n.kind_s() {
            "switch_label" => return Some(n),
            ":" | "->" => {}
            _ if is_comment(n) => {}
            _ => return None,
        }
    }
}

fn previous_non_comment(mut n: Node) -> Option<Node> {
    loop {
        n = n.prev_sibling()?;
        if !is_comment(n) {
            return Some(n);
        }
    }
}

/// The row where Checkstyle starts a declaration for `nlow`: its first token, or the one after
/// the first run of annotations among its modifiers.
fn declaration_start_row(decl: Node) -> usize {
    let own = decl.start_position().row;
    let Some(modifiers) = decl.children(&mut decl.walk()).find(|c| c.kind_s() == "modifiers") else { return own };
    let is_annotation = |n: Node| matches!(n.kind_s(), "annotation" | "marker_annotation");
    let Some(first) = modifiers.children(&mut modifiers.walk()).find(|c| is_annotation(*c)) else { return own };
    let mut last = first;
    while let Some(next) = next_sibling_skipping_comments(last).filter(|n| is_annotation(*n)) {
        last = next;
    }
    next_sibling_skipping_comments(last)
        .or_else(|| next_sibling_skipping_comments(modifiers))
        .map_or(own, |n| n.start_position().row)
}

/// Where the construct that owns a brace starts, for `nlow`.
fn construct_start_row(owner: &Owner, brace: Node) -> usize {
    match owner.token {
        Token::MethodDef | Token::CtorDef | Token::CompactCtorDef | Token::ClassDef | Token::RecordDef | Token::InterfaceDef | Token::AnnotationDef | Token::EnumDef => {
            declaration_start_row(owner.decl)
        }
        Token::Lambda => owner.decl.children(&mut owner.decl.walk()).find(|c| c.kind_s() == "->").map_or(brace.start_position().row, |a| a.start_position().row),
        Token::Objblock => brace.start_position().row,
        _ => owner.decl.start_position().row,
    }
}

fn next_sibling_skipping_comments(mut n: Node) -> Option<Node> {
    loop {
        n = n.next_sibling()?;
        if !is_comment(n) {
            return Some(n);
        }
    }
}

/// The line of a node as Checkstyle's tree gives it: an expression statement is placed at its
/// root operator, a call at its `(`.
fn tree_row(node: Node) -> usize {
    let at = |n: Option<Node>| n.map(|n| n.start_position().row);
    let row = match node.kind_s() {
        "expression_statement" => node.named_child(0).map(tree_row),
        "assignment_expression" | "binary_expression" => at(node.child_by_field_name("operator")),
        "method_invocation" => at(node.child_by_field_name("arguments")),
        "update_expression" => at(node.children(&mut node.walk()).find(|c| matches!(c.kind_s(), "++" | "--"))),
        _ => None,
    };
    row.unwrap_or_else(|| node.start_position().row)
}

/// Does a line break follow the `{`, or does nothing but the `}` follow it?
fn line_break_after(brace: Node, owner: &Owner, ignore_enums: bool) -> bool {
    if owner.opens == Opens::Members && ignore_enums && owner.plain_enum {
        return true;
    }
    let Some(next) = next_sibling_skipping_comments(brace) else { return true };
    // The first thing in an anonymous class body may be an initialiser block, which counts as a break.
    if owner.anonymous && next.kind_s() == "block" {
        return true;
    }
    next.kind_s() == "}" || tree_row(next) != brace.start_position().row
}

/// `LeftCurly`.
fn left_curly(cx: &mut Ctx, brace: Node) {
    let owners = brace_owners(cx, brace);
    if owners.is_empty() {
        return;
    }
    let cfg = cx.cfg;
    let src = cx.src;
    let (start, end) = (brace.start_byte(), brace.end_byte());
    // `{}` is left alone.
    if src.get(end) == Some(&b'}') {
        return;
    }
    let line_start = src[..start].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let starts_line = src[line_start..start].iter().all(|&b| pad::is_blank(b));
    let column = cx.char_column(brace.start_position().row as u32 + 1, (start - line_start) as u32 + 1);
    let row = brace.start_position().row;
    let on_previous_line = previous_token(brace).is_none_or(|t| t.end_position().row == row);
    let single_line = cx.parent().is_some_and(|p| p.end_position().row == row);
    // Checkstyle keeps one of two identical findings, so a brace two of an instance's tokens reach
    // is reported once: collect what each token finds, then report each distinct finding.
    for instance in &cfg.left_curly {
        let mut mine = owners.iter().filter(|o| instance.tokens.contains(o.token)).peekable();
        if mine.peek().is_none() {
            continue;
        }
        let (mut new_line, mut previous, mut break_after) = (false, false, false);
        match instance.option {
            CurlyOption::Nl => new_line = !starts_line,
            CurlyOption::NlOrSingleline => new_line = !single_line && !starts_line,
            CurlyOption::Eol => {
                previous = !on_previous_line;
                break_after = mine.any(|o| !line_break_after(brace, o, instance.ignore_enums));
            }
            CurlyOption::Nlow => {
                for owner in mine {
                    let from = construct_start_row(owner, brace);
                    if from == row {
                        continue;
                    }
                    if from + 1 == row {
                        previous |= starts_line;
                        new_line |= !starts_line;
                    } else {
                        new_line |= !starts_line;
                    }
                }
            }
        }
        if new_line {
            cx.diag(instance.name, brace, format!("'{{' at column {column} should be on a new line."));
        }
        if previous {
            cx.diag(instance.name, brace, format!("'{{' at column {column} should be on the previous line."));
        }
        if break_after {
            cx.diag(instance.name, brace, format!("'{{' at column {column} should have line break after."));
        }
    }
}

/// A parenthesis, for `SeparatorWrap`.
pub fn on_paren(cx: &mut Ctx, node: Node) {
    if !cx.cfg.separator_wrap.is_empty() {
        separator_wrap(cx, node);
    }
}

/// `SeparatorWrap`: read from the text of the line, as Checkstyle does, so a comment after a
/// separator does keep it from ending its line.
fn separator_wrap(cx: &mut Ctx, node: Node) {
    let Some(token) = separator_token(cx, node) else { return };
    let cfg = cx.cfg;
    let mut instances = cfg.separator_wrap.iter().filter(|m| m.tokens.contains(token)).peekable();
    if instances.peek().is_none() {
        return;
    }
    let src = cx.src;
    let (start, end) = (node.start_byte(), node.end_byte());
    let line_start = src[..start].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let line_end = src[end..].iter().position(|&b| b == b'\n').map_or(src.len(), |i| end + i);
    let ends_line = src[end..line_end].iter().all(|&b| pad::is_blank(b));
    let starts_line = src[line_start..start].iter().all(|&b| pad::is_blank(b));
    let text = String::from_utf8_lossy(cx.text(node)).into_owned();
    for instance in instances {
        if instance.end_of_line {
            if starts_line {
                cx.diag(instance.name, node, format!("'{text}' should be on the previous line."));
            }
        } else if ends_line {
            cx.diag(instance.name, node, format!("'{text}' should be on a new line."));
        }
    }
}

/// The Checkstyle token a node is, among those `SeparatorWrap` takes. A `;` that is a statement
/// on its own is an empty statement, which is a token of its own.
fn separator_token(cx: &Ctx, node: Node) -> Option<Token> {
    let parent = cx.parent()?;
    match node.kind_s() {
        "[" if matches!(parent.kind_s(), "dimensions" | "dimensions_expr") => Some(Token::ArrayDeclarator),
        "]" => Some(Token::Rbrack),
        // A method call's `(` and a cast's `(` are tokens of their own, not LPAREN.
        "(" => {
            let call = parent.kind_s() == "argument_list" && cx.ancestor(1).is_some_and(|g| g.kind_s() == "method_invocation");
            (!call && parent.kind_s() != "cast_expression").then_some(Token::Lparen)
        }
        ")" => Some(Token::Rparen),
        "," | ";" | "..." | "." | "::" => match punctuation_token(cx, node)? {
            Token::Semi if is_empty_statement(cx, parent, node) => None,
            token => Some(token),
        },
        _ => None,
    }
}

/// The Checkstyle token a node is, among those `OperatorWrap` takes.
fn wrapped_token(cx: &Ctx, node: Node) -> Option<Token> {
    let parent = cx.parent()?.kind_s();
    match node.kind_s() {
        "instanceof" if parent == "instanceof_expression" => Some(Token::LiteralInstanceof),
        "::" => Some(Token::MethodRef),
        // A switch rule's arrow is a lambda arrow to Checkstyle.
        "->" if matches!(parent, "lambda_expression" | "switch_rule") => Some(Token::Lambda),
        kind => operator_token(kind, parent),
    }
}

/// The last child that is not a comment.
fn last_child(n: Node) -> Option<Node> {
    (0..n.child_count()).rev().filter_map(|i| n.child(i)).find(|c| !is_comment(*c))
}

/// The first child that is not a comment.
fn first_child(n: Node) -> Option<Node> {
    (0..n.child_count()).filter_map(|i| n.child(i)).find(|c| !is_comment(*c))
}

/// The token before `node`, comments aside.
pub fn previous_token(node: Node) -> Option<Node> {
    let mut n = node;
    loop {
        let mut sibling = n.prev_sibling();
        while let Some(s) = sibling {
            if !is_comment(s) {
                let mut leaf = s;
                while let Some(c) = last_child(leaf) {
                    leaf = c;
                }
                return Some(leaf);
            }
            sibling = s.prev_sibling();
        }
        n = n.parent()?;
    }
}

/// The token after `node`, comments aside.
fn next_token(node: Node) -> Option<Node> {
    let mut n = node;
    loop {
        let mut sibling = n.next_sibling();
        while let Some(s) = sibling {
            if !is_comment(s) {
                let mut leaf = s;
                while let Some(c) = first_child(leaf) {
                    leaf = c;
                }
                return Some(leaf);
            }
            sibling = s.next_sibling();
        }
        n = n.parent()?;
    }
}

/// Where the operand to the right of an operator starts, as Checkstyle's tree sees it.
fn right_operand_start(node: Node) -> Option<Node> {
    let mut token = next_token(node)?;
    loop {
        let parent = token.parent().map(|p| p.kind_s());
        let introduces_operand = match (token.kind_s(), parent) {
            ("-" | "+" | "!" | "~", Some("unary_expression")) => true,
            // A `++` with nothing before it is a prefix one.
            ("++" | "--", Some("update_expression")) => token.prev_sibling().is_none(),
            ("new", Some("object_creation_expression" | "array_creation_expression")) => true,
            ("(", Some("cast_expression")) => true,
            _ => false,
        };
        if !introduces_operand {
            return Some(token);
        }
        token = next_token(token)?;
    }
}

/// `EmptyBlock`: a block of one of the configured statements with nothing in it. With the default
/// option, `statement`, a comment is not a statement; with `text`, any text between the braces will do.
/// Found by probing Checkstyle: a `case` label owns the block that follows it (and a `switch` rule's
/// block), but a label with no block, and an array initialiser, are never reported.
fn empty_block(cx: &mut Ctx, brace: Node) {
    let Some(opts) = &cx.cfg.empty_block else { return };
    let Some(container) = cx.parent() else { return };
    let token = if container.kind_s() == "block" && cx.ancestor(1).is_some_and(|g| g.kind_s() == "class_body") {
        Some(Token::InstanceInit)
    } else {
        brace_owners(cx, brace).into_iter().map(|o| o.token).find(|t| opts.tokens.contains(*t))
    };
    let Some(token) = token.filter(|t| opts.tokens.contains(*t)) else { return };
    let mut cur = container.walk();
    let has_content = container.children(&mut cur).any(|c| !matches!(c.kind_s(), "{" | "}") && !is_comment(c));
    if has_content {
        return;
    }
    let message = if opts.text {
        let close = container.child(container.child_count().saturating_sub(1)).map_or(brace.end_byte(), |c| c.start_byte());
        if cx.src[brace.end_byte()..close.max(brace.end_byte())].iter().any(|b| !pad::is_blank(*b) && !matches!(b, b'\n' | b'\r')) {
            return;
        }
        let name = match token {
            Token::InstanceInit => "INSTANCE_INIT".to_string(),
            Token::StaticInit => "STATIC_INIT".to_string(),
            Token::LiteralDefault => "case".to_string(),
            t => format!("{t:?}").trim_start_matches("Literal").to_ascii_lowercase().replace("case", "case"),
        };
        format!("Empty {name} block.")
    } else {
        "Must have at least one statement.".to_string()
    };
    cx.diag("EmptyBlock", brace, message);
}

/// How `RightCurly` wants a closing brace placed, and what Checkstyle's tree makes of the brace.
fn right_curly(cx: &mut Ctx, lbrace: Node) {
    let Some(opts) = &cx.cfg.right_curly else { return };
    let (option, tokens) = (opts.option, opts.tokens);
    let Some(container) = cx.parent() else { return };
    let token = if container.kind_s() == "block" && cx.ancestor(1).is_some_and(|g| g.kind_s() == "class_body") {
        Some(Token::InstanceInit)
    } else {
        brace_owners(cx, lbrace).into_iter().map(|o| o.token).find(|t| tokens.contains(*t))
    };
    let Some(token) = token.filter(|t| tokens.contains(*t)) else { return };
    let Some(rcurly) = container.child(container.child_count().saturating_sub(1)).filter(|c| c.kind_s() == "}") else { return };
    if token == Token::LiteralSwitch && switch_in_expression(cx, container) {
        return;
    }
    // A rule or label of a switch expression is skipped as well.
    if matches!(token, Token::LiteralCase | Token::LiteralDefault) {
        let mut up = container.parent();
        while let Some(n) = up.filter(|n| n.kind_s() != "switch_block") {
            up = n.parent();
        }
        if up.is_some_and(|b| switch_in_expression(cx, b)) {
            return;
        }
    }
    let src = cx.src;
    let (lrow, rrow) = (lbrace.start_position().row, rcurly.start_position().row);
    let next = next_token(rcurly);
    let next_kind = next.map(|n| n.kind_s());
    // Whether the brace ends a chain (`try`/`catch`, `if`/`else`, `do`/`while`): such a brace is judged by
    // what follows it, the last one of the chain by itself.
    let check_last = match token {
        Token::LiteralTry | Token::LiteralCatch => !matches!(next_kind, Some("catch" | "finally")),
        Token::LiteralIf => next_kind != Some("else"),
        Token::LiteralDo => false,
        _ => true,
    };
    let prev_row = last_member_row(container);
    let has_break_before = rrow != prev_row.unwrap_or(lrow);
    let alone = {
        let line_start = src[..rcurly.start_byte()].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
        let blank_before = src[line_start..rcurly.start_byte()].iter().all(|&b| pad::is_blank(b));
        // `new X() {{ ... }};` closing over two lines of braces counts as alone (Checkstyle's "double brace" case).
        let double_brace = token == Token::InstanceInit
            && next.is_some_and(|n| n.kind_s() == "}")
            && next.and_then(next_token).is_some_and(|semi| semi.kind_s() == ";" && next_token(semi).is_some_and(|after| after.start_position().row != rrow));
        blank_before && (next.is_none_or(|n| n.start_position().row != rrow) || double_brace)
    };
    let single_line_block = {
        let mut after = next;
        if after.is_some_and(|n| n.kind_s() == "else") {
            after = after.and_then(|n| next_token(outermost_if(n)));
        }
        if matches!(after.map(|n| n.kind_s()), Some("catch" | "finally")) {
            // The keyword sits in a clause; the statement is the clause's parent.
            after = after.and_then(|n| n.parent()).and_then(|c| c.parent()).and_then(next_token);
        } else if token == Token::LiteralDo {
            after = after.and_then(|n| n.parent()).and_then(next_token);
        }
        lrow == rrow && (after.is_none_or(|n| n.start_position().row != rrow) || next.is_some_and(|n| n.kind_s() == ";"))
    };
    let message = if option == RightCurlyOption::Same && !has_break_before && lrow != rrow {
        "should have line break before"
    } else if option == RightCurlyOption::Same && !check_last && next.is_some_and(|n| n.start_position().row != rrow) {
        "should be on the same line as the next part of a multi-block statement (one that directly contains multiple blocks: if/else-if/else, do/while or try/catch/finally)"
    } else if (option == RightCurlyOption::Alone && !alone)
        || ((option == RightCurlyOption::AloneOrSingleline || check_last) && !alone && !single_line_block)
    {
        "should be alone on a line"
    } else {
        return;
    };
    cx.diag("RightCurly", rcurly, format!("'}}' at column {} {message}.", cx.char_column(rcurly.start_position().row as u32 + 1, rcurly.start_position().column as u32 + 1)));
}

/// The row Checkstyle compares the closing brace with: that of the last statement's own token, which for a
/// plain statement is its `;` and for anything bigger is where it starts.
fn last_member_row(container: Node) -> Option<usize> {
    let mut cur = container.walk();
    let mut last = container.children(&mut cur).filter(|c| !matches!(c.kind_s(), "{" | "}") && !is_comment(*c)).last()?;
    // An enum's members (and the `;` that opens them) are in a node of their own here, but are the
    // body's own children to Checkstyle.
    if last.kind_s() == "enum_body_declarations" {
        let mut inner = last.walk();
        last = last.children(&mut inner).filter(|c| !is_comment(*c)).last().unwrap_or(last);
    }
    // A field's `;` is part of the declaration to Checkstyle, a local variable's and an expression
    // statement's is a token of its own after it.
    Some(match last.kind_s() {
        "expression_statement" | "local_variable_declaration" => last.end_position().row,
        _ => last.start_position().row,
    })
}

fn outermost_if(else_keyword: Node) -> Node {
    let mut node = else_keyword.parent().unwrap_or(else_keyword);
    while let Some(p) = node.parent().filter(|p| p.kind_s() == "if_statement") {
        node = p;
    }
    node
}

/// Checkstyle treats a `switch` as an expression when any ancestor is an expression, even a lambda's
/// block far above it. (Found by probing: a lambda that is a variable's whole initialiser does not
/// count as one, an anonymous class does.)
fn switch_in_expression(cx: &Ctx, switch_block: Node) -> bool {
    let Some(switch) = switch_block.parent() else { return false };
    let is_statement = |n: Node| n.parent().is_some_and(|p| matches!(p.kind_s(), "block" | "constructor_body" | "switch_block_statement_group" | "if_statement" | "while_statement" | "do_statement" | "for_statement" | "enhanced_for_statement" | "labeled_statement"));
    if !is_statement(switch) {
        return true;
    }
    let mut level = 0;
    while let Some(a) = cx.ancestor(level) {
        let in_expression = match a.kind_s() {
            "switch_expression" => !is_statement(a),
            // A lambda that is a variable's whole initialiser has no expression node above it.
            "variable_declarator" => a.child_by_field_name("value").is_none_or(|v| v.kind_s() != "lambda_expression"),
            "lambda_expression" => !a.parent().is_some_and(|p| p.kind_s() == "variable_declarator"),
            "argument_list" | "assignment_expression" | "return_statement" | "yield_statement" | "parenthesized_expression"
            | "binary_expression" | "ternary_expression" | "unary_expression" | "cast_expression" | "method_invocation" | "object_creation_expression"
            | "expression_statement" | "array_access" | "field_access" | "element_value_pair" | "array_initializer" => true,
            _ => false,
        };
        if in_expression && a.id() != switch.id() {
            return true;
        }
        level += 1;
    }
    false
}
