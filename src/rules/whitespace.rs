//! Padding inside and around punctuation: `ParenPad`, `MethodParamPad`, `NoWhitespaceBefore`, `WhitespaceAfter`, `WhitespaceAround`.
//!
//! Which Checkstyle token a parenthesis belongs to decides whether a module's `tokens` cover it, and
//! Checkstyle's token tree is not the shape tree-sitter has. The mapping below is what the
//! `PadProbe*` fixtures show, one token at a time, and not what the grammar suggests.

use super::pad::{self, Gap};
use super::tokens::Token;
use crate::engine::Ctx;
use tree_sitter::Node;

/// The Checkstyle token that owns the parenthesis `paren`, whose parent is `parent`.
fn paren_owner(cx: &Ctx, parent: Node) -> Option<Token> {
    let grand = cx.ancestor(1);
    let grand_kind = grand.map(|g| g.kind());
    Some(match parent.kind() {
        "annotation_argument_list" => Token::Annotation,
        "annotation_type_element_declaration" => Token::AnnotationFieldDef,
        "formal_parameters" => match grand_kind? {
            "constructor_declaration" => Token::CtorDef,
            "method_declaration" => Token::MethodDef,
            "record_declaration" => Token::RecordDef,
            "lambda_expression" => Token::Lambda,
            _ => return None,
        },
        "inferred_parameters" => Token::Lambda,
        "argument_list" => match grand_kind? {
            "method_invocation" => Token::MethodCall,
            "object_creation_expression" => Token::LiteralNew,
            "enum_constant" => Token::EnumConstantDef,
            "explicit_constructor_invocation" => {
                if grand?.child(0)?.kind() == "this" {
                    Token::CtorCall
                } else {
                    Token::SuperCtorCall
                }
            }
            _ => return None,
        },
        "record_pattern_body" => Token::RecordPatternDef,
        "catch_clause" => Token::LiteralCatch,
        "for_statement" | "enhanced_for_statement" => Token::LiteralFor,
        "resource_specification" => Token::ResourceSpecification,
        "parenthesized_expression" => match grand_kind? {
            "if_statement" => Token::LiteralIf,
            "while_statement" => Token::LiteralWhile,
            "do_statement" => Token::LiteralDo,
            "switch_expression" => Token::LiteralSwitch,
            "synchronized_statement" => Token::LiteralSynchronized,
            // A parenthesised receiver belongs to the dot after it.
            "field_access" | "method_invocation" => Token::Dot,
            // Anything else belongs to the expression it sits in, unless that is (part of) the
            // condition or a branch of a `?:`, which takes the parentheses of everything below it
            // up to the next operator that is not arithmetic or logic: `(a) > 0 ? x : y` is the `?`'s.
            _ => {
                let (mut level, mut in_ternary) = (1, false);
                while let Some(a) = cx.ancestor(level) {
                    match a.kind() {
                        "ternary_expression" => {
                            in_ternary = true;
                            break;
                        }
                        "binary_expression" | "unary_expression" | "parenthesized_expression" | "instanceof_expression" | "cast_expression" => level += 1,
                        _ => break,
                    }
                }
                if in_ternary {
                    Token::Question
                } else {
                    Token::Expr
                }
            }
        },
        _ => return None,
    })
}

pub fn on_paren(cx: &mut Ctx, paren: Node) {
    if cx.cfg.paren_pad.is_none() && cx.cfg.method_param_pad.is_none() && cx.cfg.whitespace_after.is_none() {
        return;
    }
    let Some(parent) = cx.parent() else { return };
    // The `)` of a cast is the token WhitespaceAfter calls TYPECAST.
    if parent.kind() == "cast_expression" && paren.kind() == ")" {
        whitespace_after(cx, paren, Token::Typecast);
        return;
    }
    let Some(owner) = paren_owner(cx, parent) else { return };
    paren_pad(cx, paren, parent, owner);
    if paren.kind() == "(" {
        method_param_pad(cx, paren, owner);
    }
}

/// `MethodParamPad`: the padding between a method, constructor or record name and its `(`.
fn method_param_pad(cx: &mut Ctx, paren: Node, owner: Token) {
    let Some(opts) = &cx.cfg.method_param_pad else { return };
    if !opts.tokens.contains(owner) {
        return;
    }
    let src = cx.src;
    let start = paren.start_byte();
    // A `(` that starts its line is a line break before it, whatever else is true.
    let message = if pad::before(src, start) == Gap::Newline {
        if opts.allow_line_breaks {
            return;
        }
        "'(' should be on the previous line."
    } else {
        let padded = pad::is_blank(src[start - 1]);
        match (opts.space, padded) {
            (false, true) => "'(' is preceded with whitespace.",
            (true, false) => "'(' is not preceded with whitespace.",
            _ => return,
        }
    };
    cx.diag("MethodParamPad", paren, message.into());
}

fn paren_pad(cx: &mut Ctx, paren: Node, parent: Node, owner: Token) {
    let Some(opts) = &cx.cfg.paren_pad else { return };
    if !opts.tokens.contains(owner) {
        return;
    }
    let space = opts.space;
    let open = paren.kind() == "(";
    let src = cx.src;
    // `for ( ;; )`: nothing precedes the first `;`, and nothing follows the last, so there is
    // nothing to pad. The same holds after the last `;` of a resource list.
    let empty_clause = match (open, parent.kind()) {
        (true, "for_statement") => parent.child_by_field_name("init").is_none(),
        (false, "for_statement") => parent.child_by_field_name("update").is_none(),
        (false, "resource_specification") => semicolon_before(src, paren.start_byte()),
        _ => false,
    };
    if empty_clause {
        return;
    }
    let message = if open {
        // Only the character next to the parenthesis counts, so a blank before the line end still does.
        let next = src.get(paren.end_byte()).copied().filter(|b| !matches!(b, b'\n' | b'\r'));
        let Some(next) = next else { return };
        let padded = pad::is_blank(next);
        if space {
            if padded || next == b')' {
                return;
            }
            "'(' is not followed by whitespace."
        } else {
            if !padded {
                return;
            }
            "'(' is followed by whitespace."
        }
    } else {
        let start = paren.start_byte();
        if space {
            let Some(prev) = start.checked_sub(1).map(|i| src[i]) else { return };
            if pad::is_blank(prev) || prev == b'(' || matches!(prev, b'\n' | b'\r') {
                return;
            }
            "')' is not preceded with whitespace."
        } else {
            // A `)` that starts its line is indentation, not padding.
            if pad::before(src, start) != Gap::Space {
                return;
            }
            "')' is preceded with whitespace."
        }
    };
    cx.diag("ParenPad", paren, message.into());
}

/// The Checkstyle token a punctuation node stands for.
pub fn punctuation_token(cx: &Ctx, node: Node) -> Option<Token> {
    Some(match node.kind() {
        "," => Token::Comma,
        ";" => Token::Semi,
        "..." => Token::Ellipsis,
        "." => Token::Dot,
        "::" => Token::MethodRef,
        // A `++` after its operand is a postfix one: it is not the first child.
        "++" | "--" if cx.parent()?.kind() == "update_expression" && cx.previous_sibling().is_some() => {
            if node.kind() == "++" {
                Token::PostInc
            } else {
                Token::PostDec
            }
        }
        ":" if cx.parent()?.kind() == "labeled_statement" => Token::LabeledStat,
        "->" => Token::Lambda,
        "if" => Token::LiteralIf,
        "else" => Token::LiteralElse,
        "do" => Token::LiteralDo,
        "while" if cx.parent()?.kind() == "do_statement" => Token::DoWhile,
        "while" => Token::LiteralWhile,
        "for" => Token::LiteralFor,
        "finally" => Token::LiteralFinally,
        "catch" => Token::LiteralCatch,
        "try" => Token::LiteralTry,
        "return" => Token::LiteralReturn,
        "yield" => Token::LiteralYield,
        "assert" => Token::LiteralAssert,
        "switch" => Token::LiteralSwitch,
        "case" => Token::LiteralCase,
        "when" => Token::LiteralWhen,
        // As a modifier, `synchronized` is not the statement's keyword.
        "synchronized" if cx.parent()?.kind() == "synchronized_statement" => Token::LiteralSynchronized,
        _ => return None,
    })
}

/// A punctuation token, for the rules that look at the gap around it.
pub fn on_token(cx: &mut Ctx, node: Node) {
    if cx.cfg.no_whitespace_before.is_none() && cx.cfg.whitespace_after.is_none() && cx.cfg.whitespace_around.is_none() && !cx.cfg.generic_whitespace {
        return;
    }
    if cx.cfg.generic_whitespace {
        generic_whitespace(cx, node);
    }
    if let Some(token) = punctuation_token(cx, node) {
        no_whitespace_before(cx, node, token);
        whitespace_after(cx, node, token);
    }
    if cx.cfg.whitespace_around.is_some() {
        if let Some(token) = around_token(cx, node).or_else(|| punctuation_token(cx, node)) {
            whitespace_around(cx, node, token);
        }
    }
}

/// The Checkstyle token an operator stands for: an assignment, a binary operator, or the `?` of a
/// ternary and the `:` of a ternary, an `assert` or an enhanced `for`.
pub fn operator_token(kind: &str, parent: &str) -> Option<Token> {
    Some(match kind {
        "=" if matches!(parent, "assignment_expression" | "variable_declarator" | "element_value_pair" | "resource") => Token::Assign,
        "+=" => Token::PlusAssign,
        "-=" => Token::MinusAssign,
        "*=" => Token::StarAssign,
        "/=" => Token::DivAssign,
        "%=" => Token::ModAssign,
        "<<=" => Token::SlAssign,
        ">>=" => Token::SrAssign,
        ">>>=" => Token::BsrAssign,
        "&=" => Token::BandAssign,
        "|=" => Token::BorAssign,
        "^=" => Token::BxorAssign,
        "+" if parent == "binary_expression" => Token::Plus,
        "-" if parent == "binary_expression" => Token::Minus,
        "*" if parent == "binary_expression" => Token::Star,
        "/" if parent == "binary_expression" => Token::Div,
        "%" if parent == "binary_expression" => Token::Mod,
        "<<" if parent == "binary_expression" => Token::Sl,
        ">>" if parent == "binary_expression" => Token::Sr,
        ">>>" if parent == "binary_expression" => Token::Bsr,
        "^" if parent == "binary_expression" => Token::Bxor,
        "<=" if parent == "binary_expression" => Token::Le,
        ">=" if parent == "binary_expression" => Token::Ge,
        "==" if parent == "binary_expression" => Token::Equal,
        "!=" if parent == "binary_expression" => Token::NotEqual,
        "&&" if parent == "binary_expression" => Token::Land,
        "||" if parent == "binary_expression" => Token::Lor,
        "&" if parent == "binary_expression" => Token::Band,
        "&" => Token::TypeExtensionAnd,
        "|" if parent == "binary_expression" || parent == "catch_type" => Token::Bor,
        "<" if parent == "binary_expression" => Token::Lt,
        ">" if parent == "binary_expression" => Token::Gt,
        "?" if parent == "ternary_expression" => Token::Question,
        ":" if matches!(parent, "ternary_expression" | "assert_statement" | "enhanced_for_statement") => Token::Colon,
        _ => return None,
    })
}

/// The Checkstyle token an operator, a brace or a `?` or `:` stands for, as `WhitespaceAround` sees it.
fn around_token(cx: &Ctx, node: Node) -> Option<Token> {
    let parent = cx.parent()?.kind();
    let kind = node.kind();
    if kind == ":" && parent == "enhanced_for_statement" && cx.cfg.whitespace_around.as_ref()?.ignore_enhanced_for_colon {
        return None;
    }
    if let Some(token) = operator_token(kind, parent) {
        return Some(token);
    }
    Some(match kind {
        "<" if matches!(parent, "type_arguments" | "type_parameters") => Token::GenericStart,
        ">" if matches!(parent, "type_arguments" | "type_parameters") => Token::GenericEnd,
        "?" if parent == "wildcard" => Token::WildcardType,
        "{" | "}" => brace_token(cx, node, parent)?,
        _ => return None,
    })
}

/// Which token a brace is: a block's `{` is an `SLIST`, the body of a type or a `switch` has `LCURLY`,
/// an array initialiser's opening brace is `ARRAY_INIT`, and every closing brace is `RCURLY`.
fn brace_token(_cx: &Ctx, node: Node, parent: &str) -> Option<Token> {
    let open = node.kind() == "{";
    Some(match parent {
        "block" | "constructor_body" => {
            if open {
                Token::Slist
            } else {
                Token::Rcurly
            }
        }
        "class_body" | "interface_body" | "enum_body" | "annotation_type_body" | "switch_block" => {
            if open {
                Token::Lcurly
            } else {
                Token::Rcurly
            }
        }
        // An array initialiser's `{` is looked at only from the front, and its `}` not at all; an
        // annotation's array has no token of its own.
        "array_initializer" if open => Token::ArrayInit,
        "array_initializer" | "element_value_array_initializer" => return None,
        _ => return None,
    })
}

/// Is the node a comment, which Checkstyle's tree leaves out?
pub fn is_comment(n: Node) -> bool {
    matches!(n.kind(), "line_comment" | "block_comment")
}

fn next_sibling_skipping_comments(mut n: Node) -> Option<Node> {
    loop {
        n = n.next_sibling()?;
        if !is_comment(n) {
            return Some(n);
        }
    }
}

fn previous_sibling_skipping_comments(mut n: Node) -> Option<Node> {
    loop {
        n = n.prev_sibling()?;
        if !is_comment(n) {
            return Some(n);
        }
    }
}

/// Is `block` an instance initialiser: a block that is a member of a type body?
fn is_instance_initializer(block: Node) -> bool {
    block.kind() == "block" && block.parent().is_some_and(|p| p.kind() == "class_body")
}

/// Checkstyle looks past the doubled braces of `new T() {{ ... }}`: the first `{` need not be followed
/// by anything, the second not preceded, and likewise the pair of closing braces.
fn double_brace(node: Node, token: Token) -> (bool, bool) {
    let (mut skip_before, mut skip_after) = (false, false);
    match (token, node.parent()) {
        (Token::Lcurly, Some(body)) if body.kind() == "class_body" => {
            skip_after = next_sibling_skipping_comments(node).is_some_and(is_instance_initializer);
        }
        (Token::Slist, Some(block)) if is_instance_initializer(block) => {
            skip_before = previous_sibling_skipping_comments(block).is_some_and(|p| p.kind() == "{");
        }
        (Token::Rcurly, Some(block)) if is_instance_initializer(block) => {
            skip_after = next_sibling_skipping_comments(block).is_some_and(|n| n.kind() == "}");
        }
        (Token::Rcurly, Some(body)) if body.kind() == "class_body" => {
            skip_before = previous_sibling_skipping_comments(node).is_some_and(is_instance_initializer);
        }
        _ => {}
    }
    (skip_before, skip_after)
}

/// Checkstyle does not look at a `switch` that is a whole expression of its own (a variable's value,
/// an argument, a returned value, a condition), only at one that is an operand of something.
fn switch_is_whole_expression(cx: &Ctx) -> bool {
    // The `switch` keyword's parent is the switch itself, and the construct holding it is the next one up.
    let Some(up) = cx.ancestor(1) else { return false };
    match up.kind() {
        "variable_declarator" | "argument_list" | "array_initializer" | "lambda_expression" | "return_statement" | "yield_statement"
        | "assert_statement" | "expression_statement" | "throw_statement" | "element_value_pair" | "for_statement" | "enhanced_for_statement"
        | "array_access" | "dimensions_expr" => true,
        "parenthesized_expression" => {
            cx.ancestor(2).is_some_and(|g| matches!(g.kind(), "if_statement" | "while_statement" | "do_statement" | "synchronized_statement" | "switch_expression"))
        }
        _ => false,
    }
}

/// A block with nothing in it, comments aside. (A lone `;` is a statement.)
fn is_empty_block(block: Node) -> bool {
    let mut cursor = block.walk();
    let empty = block.children(&mut cursor).all(|c| matches!(c.kind(), "{" | "}") || is_comment(c));
    empty
}

/// Which sides of this brace an option of `WhitespaceAround` excuses because it belongs to an empty
/// body: `(before, after)`. An empty method or loop is excused whole; of an empty constructor only the
/// gap inside it, so its `{` must still follow the parameters' `)` with a blank.
fn empty_body_allowed(cx: &Ctx, node: Node, token: Token) -> (bool, bool) {
    let Some(opts) = &cx.cfg.whitespace_around else { return (false, false) };
    let Some(parent) = cx.parent() else { return (false, false) };
    match token {
        Token::Slist | Token::Rcurly if matches!(parent.kind(), "block" | "constructor_body") => {
            let Some(owner) = cx.ancestor(1) else { return (false, false) };
            let (allowed, whole) = match owner.kind() {
                "method_declaration" => (opts.allow_empty_methods, true),
                "constructor_declaration" | "compact_constructor_declaration" => (opts.allow_empty_constructors, false),
                "while_statement" | "for_statement" | "enhanced_for_statement" | "do_statement" => (opts.allow_empty_loops, true),
                "lambda_expression" => (opts.allow_empty_lambdas, true),
                "catch_clause" => (opts.allow_empty_catches, true),
                "switch_rule" | "switch_block_statement_group" => (opts.allow_empty_switch_block_statements, true),
                _ => (false, true),
            };
            if !(allowed && is_empty_block(parent)) {
                (false, false)
            } else if whole {
                (true, true)
            } else {
                (token == Token::Rcurly, token == Token::Slist)
            }
        }
        Token::Lcurly if opts.allow_empty_types => {
            let empty = next_sibling_skipping_comments(node).is_some_and(|n| n.kind() == "}");
            (empty, empty)
        }
        Token::Rcurly if opts.allow_empty_types => {
            let empty = previous_sibling_skipping_comments(node).is_some_and(|n| n.kind() == "{");
            (empty, empty)
        }
        _ => (false, false),
    }
}

/// `WhitespaceAround`: the character before and after the token must be a blank, unless the token
/// starts or ends its line.
fn whitespace_around(cx: &mut Ctx, node: Node, token: Token) {
    let Some(opts) = &cx.cfg.whitespace_around else { return };
    if !opts.tokens.contains(token) {
        return;
    }
    if token == Token::LiteralSwitch && cx.parent().is_some_and(|p| p.kind() == "switch_expression") && switch_is_whole_expression(cx) {
        return;
    }
    let (empty_before, empty_after) = empty_body_allowed(cx, node, token);
    let src = cx.src;
    let (start, end) = (node.start_byte(), node.end_byte());
    let (skip_before, mut skip_after) = double_brace(node, token);
    let skip_before = skip_before || empty_before;
    skip_after |= empty_after;
    // `return;` has nothing to be followed by.
    skip_after |= token == Token::LiteralReturn && src.get(end) == Some(&b';');
    skip_after |= token == Token::ArrayInit;
    let text = std::str::from_utf8(cx.text(node)).unwrap_or("?").to_string();
    let on_line = |b: u8| !matches!(b, b'\n' | b'\r');
    if !skip_before && start > 0 && on_line(src[start - 1]) && !pad::is_blank(src[start - 1]) {
        cx.diag("WhitespaceAround", node, format!("'{text}' is not preceded with whitespace."));
    }
    if let Some(&next) = src.get(end) {
        // The end of an anonymous class, lambda or switch expression may be followed by what ends the expression.
        let expression_end = token == Token::Rcurly && matches!(next, b')' | b';' | b',' | b'.');
        if !skip_after && on_line(next) && !pad::is_blank(next) && !expression_end {
            cx.diag("WhitespaceAround", node, format!("'{text}' is not followed by whitespace."));
        }
    }
}

/// `GenericWhitespace`: no blank inside the angle brackets or before the `<` of a type, a blank
/// before the `<` of a method's own type parameters, and after a `>` only what can follow a type.
/// It works on the physical line, as Checkstyle does, so a bracket at either edge of a line is
/// never padded wrongly.
fn generic_whitespace(cx: &mut Ctx, node: Node) {
    let open = match node.kind() {
        "<" => true,
        ">" => false,
        _ => return,
    };
    let Some(parent) = cx.parent() else { return };
    if !matches!(parent.kind(), "type_arguments" | "type_parameters") {
        return;
    }
    let src = cx.src;
    let start = node.start_byte();
    let line_start = src[..start].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let line_end = src[start..].iter().position(|&b| b == b'\n').map_or(src.len(), |i| start + i);
    let after = start + 1;
    // A bracket that starts its line, blanks aside, has nothing in front of it to pad.
    let padded_before = start > line_start && pad::is_blank(src[start - 1]) && !src[line_start..start - 1].iter().all(|&b| pad::is_blank(b));
    if open {
        // The type parameters of a method or constructor, and a constructor's own type arguments in
        // `new <T>Foo()`, are set off from what precedes them.
        let own_parameters = cx.ancestor(1).is_some_and(|n| match parent.kind() {
            "type_parameters" => matches!(n.kind(), "method_declaration" | "constructor_declaration"),
            _ => n.kind() == "object_creation_expression",
        });
        if own_parameters {
            if start > line_start && !pad::is_blank(src[start - 1]) {
                cx.diag("GenericWhitespace", node, "'<' is not preceded with whitespace.".to_string());
            }
        } else if padded_before {
            cx.diag("GenericWhitespace", node, "'<' is preceded with whitespace.".to_string());
        }
        if after < line_end && pad::is_blank(src[after]) {
            cx.diag("GenericWhitespace", node, "'<' is followed by whitespace.".to_string());
        }
        return;
    }
    if padded_before {
        cx.diag("GenericWhitespace", node, "'>' is preceded with whitespace.".to_string());
    }
    if after >= line_end {
        return;
    }
    // How many generic lists enclose this `>`, its own included: a nested one is followed by
    // `>`, `,` or `&`, the outermost by whatever may follow a type.
    let mut depth = 0;
    let mut level = 0;
    while let Some(n) = cx.ancestor(level) {
        depth += usize::from(matches!(n.kind(), "type_arguments" | "type_parameters"));
        level += 1;
    }
    if depth > 1 {
        let amp = src[after..line_end].iter().position(|&b| b == b'&').map(|i| after + i);
        match amp {
            Some(amp) if src[after..amp].iter().all(|&b| pad::is_blank(b)) => {
                if amp == after {
                    cx.diag("GenericWhitespace", node, "'&' is not preceded with whitespace.".to_string());
                } else if amp - after != 1 {
                    cx.diag("GenericWhitespace", node, "'>' is followed by whitespace.".to_string());
                }
            }
            _ if src[after] == b' ' => cx.diag("GenericWhitespace", node, "'>' is followed by whitespace.".to_string()),
            _ => {}
        }
    } else if generic_before_name(parent) {
        if pad::is_blank(src[after]) {
            cx.diag("GenericWhitespace", node, "'>' is followed by whitespace.".to_string());
        }
    } else if !(pad::is_blank(src[after]) || matches!(src[after], b'(' | b')' | b',' | b'[' | b'.' | b':' | b';')) {
        cx.diag("GenericWhitespace", node, "'>' is followed by an illegal character.".to_string());
    }
}

/// Are these type arguments or parameters written ahead of a name or a list that follows them
/// directly: those of a generic method call (`Foo.<T>bar()`), of a method reference, of the class
/// a `new` creates, or of a record header?
fn generic_before_name(arguments: Node) -> bool {
    let mut owner = arguments.parent();
    while let Some(n) = owner {
        if !matches!(n.kind(), "generic_type" | "scoped_type_identifier") {
            break;
        }
        owner = n.parent();
    }
    owner.is_some_and(|n| matches!(n.kind(), "method_invocation" | "method_reference" | "object_creation_expression" | "array_creation_expression" | "record_declaration"))
}

/// `WhitespaceAfter`: the character after the token must be a blank, or the line must end.
/// A `;` may be followed by the `)` or `;` that closes an empty `for` clause, and `return;` has
/// nothing to pad. An empty statement is not a `SEMI`.
fn whitespace_after(cx: &mut Ctx, node: Node, token: Token) {
    let Some(opts) = &cx.cfg.whitespace_after else { return };
    if !opts.tokens.contains(token) {
        return;
    }
    let Some(&next) = cx.src.get(node.end_byte()) else { return };
    if next == b'\n' || pad::is_blank(next) {
        return;
    }
    // Whatever the token, a `)` or a `;` may follow it: `for (;;)`, `return;`, `else;`, `do;`.
    let exempt = next == b')' || next == b';' || (token == Token::Semi && cx.parent().is_some_and(|p| is_empty_statement(cx, p, node)));
    if !exempt {
        let text = std::str::from_utf8(cx.text(node)).unwrap_or("?");
        cx.diag("WhitespaceAfter", node, format!("'{text}' is not followed by whitespace."));
    }
}

/// `WhitespaceAfter` on annotations: the first annotation of a group, outside a declaration's
/// modifiers, must be followed by a blank, a `)` or a `;`, or end its line.
pub fn on_annotation(cx: &mut Ctx, node: Node) {
    let wanted = cx.cfg.whitespace_after.as_ref().is_some_and(|w| w.tokens.contains(Token::Annotations));
    if !wanted {
        return;
    }
    let (Some(parent), Some(grand)) = (cx.parent(), cx.ancestor(1)) else { return };
    // An annotation that is a value of another is not in a list of annotations.
    if matches!(parent.kind(), "annotation_argument_list" | "element_value_pair" | "element_value_array_initializer") {
        return;
    }
    // A declaration's annotations are its modifiers to Checkstyle, except an enum constant's and
    // those on the plain type of an `instanceof`, which has no pattern variable to be modified.
    let on_plain_type = parent.kind() == "modifiers" && grand.kind() == "instanceof_expression" && grand.child_by_field_name("name").is_none();
    if matches!(parent.kind(), "modifiers" | "receiver_parameter") && grand.kind() != "enum_constant" && !on_plain_type {
        return;
    }
    if previous_sibling_skipping_comments(node).is_some_and(|p| matches!(p.kind(), "annotation" | "marker_annotation")) {
        return;
    }
    let Some(&next) = cx.src.get(node.end_byte()) else { return };
    if next == b'\n' || next == b';' || next == b')' || pad::is_blank(next) {
        return;
    }
    let mut last = node;
    while let Some(child) = last.child(last.child_count().saturating_sub(1)).filter(|_| last.child_count() > 0) {
        last = child;
    }
    let text = std::str::from_utf8(cx.text(last)).unwrap_or("?").to_string();
    cx.diag("WhitespaceAfter", last, format!("'{text}' is not followed by whitespace."));
}

/// Java's `Character.isWhitespace`.
fn java_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | '\u{1c}'..='\u{1f}' => true,
        '\u{85}' | '\u{a0}' | '\u{2007}' | '\u{202f}' => false,
        _ => c.is_whitespace(),
    }
}

/// `WhitespaceAfter` on single-line comments: after the slashes comes a blank, or nothing.
pub fn on_comment(cx: &mut Ctx, node: Node) {
    if node.kind() != "line_comment" || !cx.cfg.whitespace_after.as_ref().is_some_and(|w| w.tokens.contains(Token::SingleLineComment)) {
        return;
    }
    let text = cx.text(node);
    let rest = String::from_utf8_lossy(&text[2.min(text.len())..]);
    if rest.trim_start_matches('/').chars().next().is_some_and(|c| !java_whitespace(c)) {
        cx.diag("WhitespaceAfter", node, "'//' is not followed by whitespace.".to_string());
    }
}

/// `NoWhitespaceBefore`: a blank before the token is wrong, and so is none at all before one that
/// starts its line, unless `allowLineBreaks` says line starts are fine.
fn no_whitespace_before(cx: &mut Ctx, node: Node, token: Token) {
    let Some(opts) = &cx.cfg.no_whitespace_before else { return };
    if !opts.tokens.contains(token) {
        return;
    }
    let src = cx.src;
    let start = node.start_byte();
    // A `;` that is a statement on its own is an empty statement, which is a token of its own.
    if token == Token::Semi && cx.parent().is_some_and(|p| is_empty_statement(cx, p, node)) {
        return;
    }
    // `for ( ; ; )`: the `;` of an empty initialiser or condition has nothing in front of it to pad.
    if token == Token::Semi && cx.parent().is_some_and(|p| p.kind() == "for_statement") {
        let empty_clause = cx.previous_sibling().is_some_and(|p| matches!(p.kind(), "(" | ";" | "local_variable_declaration"));
        if empty_clause {
            return;
        }
    }
    let starts_line = pad::before(src, start) == Gap::Newline;
    let blank_before = start > 0 && pad::is_blank(src[start - 1]);
    let flagged = if starts_line { !opts.allow_line_breaks } else { blank_before };
    if flagged {
        let text = std::str::from_utf8(cx.text(node)).unwrap_or("?");
        cx.diag("NoWhitespaceBefore", node, format!("'{text}' is preceded with whitespace."));
    }
}

/// Is the `;` an empty statement, as in `{ ; }`, `if (c) ;` or `while (c) ;`?
pub fn is_empty_statement(cx: &Ctx, parent: Node, semi: Node) -> bool {
    let f = &cx.syn.f;
    let is = |field: u16| parent.child_by_field_id(field).is_some_and(|c| c.id() == semi.id());
    match parent.kind() {
        "block" | "constructor_body" | "switch_block_statement_group" | "labeled_statement" => true,
        "if_statement" => is(f.consequence) || is(f.alternative),
        "while_statement" | "for_statement" | "enhanced_for_statement" | "do_statement" => is(f.body),
        _ => false,
    }
}

/// Is the last thing before `pos`, blanks aside, a `;`?
fn semicolon_before(src: &[u8], pos: usize) -> bool {
    src[..pos].iter().rev().find(|b| !pad::is_blank(**b)) == Some(&b';')
}
