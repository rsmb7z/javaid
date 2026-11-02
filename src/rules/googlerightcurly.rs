// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/blocks/GoogleRightCurlyCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `GoogleRightCurly`, ported from `GoogleRightCurlyCheck.java` onto the emulated `DetailAST`
//! (with comment nodes, as the check asks for them).

use super::cs::{preorder, same_line, ty_is, Log};
use crate::csast::{CsAst, Tok, N};

const RULE: &str = "GoogleRightCurly";

fn alone(text: &str, col: i32) -> String {
    format!("'{text}' at column {col} should be alone on a line.")
}

fn break_after(text: &str, col: i32) -> String {
    format!("'{text}' at column {col} should have line break after.")
}

fn break_before(text: &str, col: i32) -> String {
    format!("'{text}' at column {col} should have line break before.")
}

fn same_msg(text: &str, col: i32) -> String {
    format!("'{text}' at column {col} should be on the same line as the next part of a multi-block statement (one that directly contains multiple blocks: if/else-if/else, do/while or try/catch/finally).")
}

const CONCISE: &str = "Empty block should be concise {}.";

pub fn check(ast: &CsAst, log: &mut Log) {
    preorder(ast, |ast, n| {
        let mut right_curly = None;
        match ast.ty(n) {
            Tok::CLASS_DEF | Tok::INTERFACE_DEF | Tok::ANNOTATION_DEF | Tok::RECORD_DEF | Tok::ENUM_DEF => {
                right_curly = ast.find_first_token(n, Tok::OBJBLOCK).and_then(|c| ast.find_first_token(c, Tok::RCURLY));
            }
            Tok::LITERAL_SWITCH => right_curly = ast.last_child(n),
            Tok::LITERAL_CASE | Tok::LITERAL_DEFAULT => handle_case_and_default(ast, n, log),
            Tok::LITERAL_IF
            | Tok::LITERAL_ELSE
            | Tok::LITERAL_TRY
            | Tok::LITERAL_CATCH
            | Tok::LITERAL_FINALLY
            | Tok::LITERAL_DO
            | Tok::METHOD_DEF
            | Tok::CTOR_DEF
            | Tok::COMPACT_CTOR_DEF
            | Tok::LITERAL_FOR
            | Tok::LITERAL_WHILE
            | Tok::STATIC_INIT
            | Tok::INSTANCE_INIT
            | Tok::LITERAL_SYNCHRONIZED => {
                if let Some(child) = ast.find_first_token(n, Tok::SLIST) {
                    right_curly = ast.last_child(child);
                }
            }
            _ => {}
        }
        if let Some(brace) = right_curly {
            check_right_brace(ast, n, brace, log);
        }
    });
}

fn handle_case_and_default(ast: &CsAst, n: N, log: &mut Log) {
    let mut start = Some(n);
    if ast.parent(n).is_some_and(|p| ty_is(ast, p, Tok::CASE_GROUP)) {
        if let Some(next) = ast.next_sibling(n) {
            start = ast.find_first_token(next, Tok::SLIST);
        }
    }
    let mut current = start;
    while let Some(c) = current {
        if ty_is(ast, c, Tok::SLIST) {
            if let Some(brace) = ast.last_child(c) {
                check_right_brace(ast, n, brace, log);
            }
        }
        current = ast.next_sibling(c);
    }
}

fn violation(ast: &CsAst, brace: N, log: &mut Log, message: impl FnOnce(&str, i32) -> String) {
    let text = ast.text(brace).to_string();
    let col = ast.column_no(brace) + 1;
    log.at(RULE, ast, brace, &message(&text, col));
}

fn check_right_brace(ast: &CsAst, current: N, brace: N, log: &mut Log) {
    let next = next_token(ast, brace);
    let has_content_around = next.is_some_and(|t| has_content_on_right_side(ast, brace, t)) || has_content_on_left_side(ast, brace);
    if let Some(next) = next.filter(|&t| is_part_of_multi_block(ast, current, t)) {
        check_multi_block_statement(ast, current, brace, next, log);
    } else if ast.parent(current).is_some_and(|p| ty_is(ast, p, Tok::LITERAL_ELSE))
        || matches!(ast.ty(current), Tok::LITERAL_ELSE | Tok::LITERAL_CATCH | Tok::LITERAL_FINALLY)
    {
        if has_content_around {
            violation(ast, brace, log, alone);
        }
    } else if is_empty(ast, brace) {
        verify_empty_block(ast, brace, next, log);
    } else if has_content_around {
        violation(ast, brace, log, alone);
    }
}

fn check_multi_block_statement(ast: &CsAst, current: N, brace: N, next: N, log: &mut Log) {
    if same_line(ast, brace, next) {
        if has_content_on_left_side(ast, brace) && !(ty_is(ast, current, Tok::LITERAL_DO) && is_empty(ast, brace)) {
            violation(ast, brace, log, break_before);
        }
    } else {
        violation(ast, brace, log, same_msg);
    }
}

fn verify_empty_block(ast: &CsAst, brace: N, next: Option<N>, log: &mut Log) {
    if is_not_concise(ast, brace) {
        log.at(RULE, ast, brace, CONCISE);
    } else if next.is_some_and(|t| has_content_on_right_side(ast, brace, t)) {
        violation(ast, brace, log, break_after);
    }
}

fn is_part_of_multi_block(ast: &CsAst, current: N, next: N) -> bool {
    let next_type = ast.ty(next);
    match ast.ty(current) {
        Tok::LITERAL_IF => next_type == Tok::LITERAL_ELSE,
        Tok::LITERAL_TRY | Tok::LITERAL_CATCH => next_type == Tok::LITERAL_CATCH || next_type == Tok::LITERAL_FINALLY,
        Tok::LITERAL_DO => true,
        _ => false,
    }
}

fn has_content_on_left_side(ast: &CsAst, brace: N) -> bool {
    let sibling = ast.prev_sibling(brace);
    let mut previous = sibling.or_else(|| ast.parent(brace)).expect("a brace has a parent");
    if !ty_is(ast, previous, Tok::SLIST) {
        while let Some(last) = ast.last_child(previous) {
            previous = last;
        }
    }
    same_line(ast, brace, previous) && !sibling.is_some_and(|s| matches!(ast.ty(s), Tok::ENUM_CONSTANT_DEF | Tok::COMMA))
}

fn has_content_on_right_side(ast: &CsAst, brace: N, next: N) -> bool {
    let valid_previous = ast.prev_sibling(next).is_some_and(|p| matches!(ast.ty(p), Tok::EXPR | Tok::VARIABLE_DEF | Tok::ELIST | Tok::LAMBDA));
    let next_is_valid = valid_previous || matches!(ast.ty(next), Tok::SINGLE_LINE_COMMENT | Tok::BLOCK_COMMENT_BEGIN | Tok::BLOCK_COMMENT_END | Tok::COMMENT_CONTENT);
    !next_is_valid && same_line(ast, brace, next)
}

fn is_empty(ast: &CsAst, brace: N) -> bool {
    ast.prev_sibling(brace).is_none_or(|p| ty_is(ast, p, Tok::LCURLY))
}

fn is_not_concise(ast: &CsAst, brace: N) -> bool {
    let lcurly = ast.parent(brace).expect("a brace has a parent");
    ast.line_no(lcurly) == ast.line_no(brace) && ast.column_no(lcurly) + 1 != ast.column_no(brace)
}

fn next_token(ast: &CsAst, node: N) -> Option<N> {
    let mut current = Some(node);
    let mut next = None;
    while let Some(c) = current {
        if next.is_some() {
            break;
        }
        next = ast.next_sibling(c);
        current = ast.parent(c);
    }
    next
}
