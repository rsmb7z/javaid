// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/coding/TextBlockGoogleStyleFormattingCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `TextBlockGoogleStyleFormatting`, ported from `TextBlockGoogleStyleFormattingCheck.java` onto
//! the emulated `DetailAST`.

use super::cs::{Log, is_java_whitespace, preorder, same_line, ty_is};
use crate::csast::{CsAst, N, Tok};

const RULE: &str = "TextBlockGoogleStyleFormatting";
const OPEN: &str = "Opening quotes (\"\"\") of text-block must be on the new line.";
const CLOSE: &str = "Closing quotes (\"\"\") of text-block should not be preceded by non-whitespace characters.";
const UNALIGNED: &str = "Text-block quotes are not vertically aligned.";
const CONTENT: &str = "Each line of text in the text block must be indented at least as much as the opening and closing quotes.";

pub fn check(ast: &CsAst, log: &mut Log) {
    preorder(ast, |ast, n| {
        if ast.ty(n) != Tok::TEXT_BLOCK_LITERAL_BEGIN {
            return;
        }
        if !opening_quotes_are_alone_on_the_line(ast, n) {
            log.at(RULE, ast, n, OPEN);
        }
        let content = ast.first_child(n).expect("text block content");
        let closing = ast.next_sibling(content).expect("closing quotes");
        if !closing_quotes_are_alone_on_the_line(ast, closing) {
            log.at(RULE, ast, closing, CLOSE);
        }
        if ast.column_no(n) != ast.column_no(closing) {
            log.at(RULE, ast, closing, UNALIGNED);
        }
        if !is_content_indented_properly(ast, n) {
            log.at(RULE, ast, content, CONTENT);
        }
    });
}

fn opening_quotes_are_alone_on_the_line(ast: &CsAst, opening: N) -> bool {
    let previous = ast.prev_sibling(opening);
    let mut not_preceded = previous.is_none_or(|p| !same_line(ast, opening, p));
    let mut parent = ast.parent(opening);
    while let Some(p) = parent {
        parent = ast.parent(p);
        if !not_preceded || ty_is(ast, p, Tok::ELIST) || ty_is(ast, p, Tok::EXPR) {
            continue;
        }
        if ty_is(ast, p, Tok::METHOD_DEF) {
            not_preceded = !quotes_are_preceded_with_comma(ast, opening);
        } else {
            not_preceded = !same_line(ast, opening, p);
        }
    }
    not_preceded
}

fn quotes_are_preceded_with_comma(ast: &CsAst, opening: N) -> bool {
    let expression = ast.parent(opening).expect("parent");
    ast.prev_sibling(expression).is_some_and(|p| same_line(ast, opening, p))
}

fn closing_quotes_are_alone_on_the_line(ast: &CsAst, closing: N) -> bool {
    let content = ast.prev_sibling(closing).expect("content");
    let text = ast.text(content);
    let trimmed = text.trim_end_matches(' ');
    trimmed.chars().next_back().is_some_and(is_java_whitespace)
}

fn is_content_indented_properly(ast: &CsAst, opening: N) -> bool {
    let quote_indent = ast.column_no(opening) as usize;
    let text = ast.text(ast.first_child(opening).expect("content"));
    let mut result = true;
    for line in text.split('\n') {
        let indentation = line.chars().take_while(|c| is_java_whitespace(*c)).count();
        if indentation < quote_indent && indentation < line.chars().count() {
            result = false;
        }
    }
    result
}
