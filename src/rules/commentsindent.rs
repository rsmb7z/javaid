// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/indentation/CommentsIndentationCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `CommentsIndentation`, ported from Checkstyle 14.4.0 onto the emulated AST with comment nodes.
//!
//! The method names follow `CommentsIndentationCheck`. A Java `NullPointerException` or index
//! error (Checkstyle would fail the file) is answered with "nothing found" here.

use super::vdud::{grouped, report_col};
use crate::config::CommentsIndentation as Opts;
use crate::csast::{CsAst, N, Tok};
use crate::engine::Diagnostic;

/// `Character.isWhitespace`.
fn is_ws(c: char) -> bool {
    match c {
        '\u{00A0}' | '\u{2007}' | '\u{202F}' | '\u{0085}' => false,
        '\u{1C}'..='\u{1F}' => true,
        c => c.is_whitespace(),
    }
}

struct Ck<'a> {
    a: &'a CsAst,
    /// The file's lines, as characters.
    lines: Vec<Vec<char>>,
    raw: &'a [&'a [u8]],
    tab_width: usize,
    out: &'a mut Vec<Diagnostic>,
}

pub fn run(a: &CsAst, o: &Opts, raw: &[&[u8]], tab_width: usize, out: &mut Vec<Diagnostic>) {
    if a.root == crate::csast::NIL {
        return;
    }
    let lines = raw.iter().map(|l| String::from_utf8_lossy(l).chars().collect()).collect();
    let mut ck = Ck { a, lines, raw, tab_width, out };
    let mut stack = vec![a.root];
    while let Some(n) = stack.pop() {
        if let Some(s) = a.next_sibling(n) {
            stack.push(s);
        }
        if let Some(c) = a.first_child(n) {
            stack.push(c);
        }
        match a.ty(n) {
            Tok::SINGLE_LINE_COMMENT if o.single => ck.visit_comment(n),
            Tok::BLOCK_COMMENT_BEGIN if o.block => ck.visit_comment(n),
            _ => {}
        }
    }
}

fn is_comment(a: &CsAst, n: N) -> bool {
    matches!(a.ty(n), Tok::SINGLE_LINE_COMMENT | Tok::BLOCK_COMMENT_BEGIN | Tok::COMMENT_CONTENT | Tok::BLOCK_COMMENT_END)
}

impl Ck<'_> {
    fn ty(&self, n: N) -> Tok {
        self.a.ty(n)
    }
    fn line(&self, n: N) -> i32 {
        self.a.line_no(n)
    }
    fn col(&self, n: N) -> i32 {
        self.a.column_no(n)
    }

    fn log(&mut self, comment: N, line_arg: String, comment_col: String, expected: String) {
        let key = if self.ty(comment) == Tok::SINGLE_LINE_COMMENT { "Comment" } else { "Block comment" };
        let (line, col) = (self.line(comment), self.col(comment));
        let col = report_col(self.raw, line, col, self.tab_width);
        let message = format!("{key} has incorrect indentation level {comment_col}, expected is {expected}, indentation should be the same level as line {line_arg}.");
        self.out.push(Diagnostic { rule: "CommentsIndentation", line: line as u32, col, message });
    }

    fn log_one(&mut self, comment: N, line_no: i32, comment_col: i32, expected: i32) {
        self.log(comment, grouped(line_no as i64), grouped(comment_col as i64), grouped(expected as i64));
    }

    fn visit_comment(&mut self, comment: N) {
        if self.is_trailing_comment(comment) {
            return;
        }
        let prev = self.previous_statement(comment);
        let next = self.next_stmt(comment);
        if self.in_empty_case_block(prev, next) {
            let (prev, next) = (prev.unwrap(), next.unwrap());
            if self.col(comment) < self.col(prev) || self.col(comment) < self.col(next) {
                self.log_multiline(prev, comment, next);
            }
        } else if self.is_fall_through(prev, next) {
            let (prev, next) = (prev.unwrap(), next.unwrap());
            if !self.same_level_indented(comment, prev, next) {
                self.log_multiline(prev, comment, next);
            }
        } else if self.in_empty_code_block(prev, next) {
            let next = next.unwrap();
            if self.col(comment) < self.col(next) {
                self.log_one(comment, self.line(next), self.col(comment), self.col(next));
            }
        } else if next.is_some_and(|n| self.ty(n) == Tok::RCURLY) {
            self.handle_end_of_block(prev, comment, next.unwrap());
        } else if let Some(next) = next {
            if !self.same_level_indented(comment, next, next) && !self.in_same_method_call_with_same_indent(comment) {
                self.log_one(comment, self.line(next), self.col(comment), self.col(next));
            }
        }
    }

    fn next_stmt(&self, comment: N) -> Option<N> {
        let a = self.a;
        let mut next = a.next_sibling(comment);
        while let Some(n) = next {
            if is_comment(a, n) && self.col(comment) != self.col(n) {
                next = a.next_sibling(n);
            } else {
                break;
            }
        }
        next
    }

    fn previous_statement(&self, comment: N) -> Option<N> {
        if self.is_distributed_previous_statement(comment) { self.distributed_previous_statement(comment) } else { self.one_line_previous_statement(comment) }
    }

    fn is_distributed_previous_statement(&self, comment: N) -> bool {
        let prev = self.a.prev_sibling(comment);
        self.is_distributed_expression(comment) || self.is_distributed_return(prev) || self.is_distributed_throw(prev)
    }

    fn is_distributed_expression(&self, comment: N) -> bool {
        let a = self.a;
        let mut prev = a.prev_sibling(comment);
        while let Some(p) = prev {
            if !is_comment(a, p) {
                break;
            }
            prev = a.prev_sibling(p);
        }
        let Some(prev) = prev else { return false };
        if self.ty(prev) == Tok::SEMI && self.on_previous_line_ignoring_comments(comment, prev) {
            let Some(mut cur) = a.prev_sibling(prev) else { return false };
            while let Some(c) = a.first_child(cur) {
                cur = c;
            }
            self.line(prev) != self.line(cur)
        } else {
            self.is_statement_with_possible_curlies(prev)
        }
    }

    fn is_statement_with_possible_curlies(&self, prev: N) -> bool {
        matches!(self.ty(prev), Tok::LITERAL_IF | Tok::LITERAL_TRY | Tok::LITERAL_FOR | Tok::LITERAL_DO | Tok::LITERAL_WHILE | Tok::LITERAL_SWITCH)
            || matches!(self.ty(prev), Tok::CLASS_DEF | Tok::INTERFACE_DEF | Tok::ENUM_DEF | Tok::ANNOTATION_DEF | Tok::RECORD_DEF | Tok::METHOD_DEF)
    }

    fn is_distributed_return(&self, prev: Option<N>) -> bool {
        let Some(p) = prev.filter(|&p| self.ty(p) == Tok::LITERAL_RETURN) else { return false };
        self.a.first_child(p).and_then(|f| self.a.next_sibling(f)).is_some()
    }

    fn is_distributed_throw(&self, prev: Option<N>) -> bool {
        let Some(p) = prev.filter(|&p| self.ty(p) == Tok::LITERAL_THROW) else { return false };
        match self.a.first_child(p).and_then(|f| self.a.next_sibling(f)) {
            Some(n) => self.line(n) != self.line(p),
            None => false,
        }
    }

    fn distributed_previous_statement(&self, comment: N) -> Option<N> {
        let a = self.a;
        let mut cur = a.prev_sibling(comment)?;
        while is_comment(a, cur) {
            cur = a.prev_sibling(cur)?;
        }
        if self.ty(cur) == Tok::SEMI {
            cur = a.prev_sibling(cur)?;
            while let Some(first) = a.first_child(cur) {
                cur = if is_comment(a, cur) { a.next_sibling(cur)? } else { first };
            }
        }
        Some(cur)
    }

    fn in_empty_case_block(&self, prev: Option<N>, next: Option<N>) -> bool {
        match (prev, next) {
            (Some(p), Some(n)) => matches!(self.ty(p), Tok::LITERAL_CASE | Tok::CASE_GROUP) && matches!(self.ty(n), Tok::LITERAL_CASE | Tok::LITERAL_DEFAULT),
            _ => false,
        }
    }

    fn is_fall_through(&self, prev: Option<N>, next: Option<N>) -> bool {
        match (prev, next) {
            (Some(p), Some(n)) => self.ty(p) != Tok::LITERAL_CASE && matches!(self.ty(n), Tok::LITERAL_CASE | Tok::LITERAL_DEFAULT),
            _ => false,
        }
    }

    fn in_empty_code_block(&self, prev: Option<N>, next: Option<N>) -> bool {
        match (prev, next) {
            (Some(p), Some(n)) => matches!(self.ty(p), Tok::SLIST | Tok::LCURLY | Tok::ARRAY_INIT | Tok::OBJBLOCK) && self.ty(n) == Tok::RCURLY,
            _ => false,
        }
    }

    fn handle_end_of_block(&mut self, prev: Option<N>, comment: N, next: N) {
        let Some(prev) = prev else { return };
        if matches!(self.ty(prev), Tok::LITERAL_CASE | Tok::CASE_GROUP | Tok::LITERAL_DEFAULT) {
            if self.col(comment) < self.col(next) {
                self.log_one(comment, self.line(next), self.col(comment), self.col(next));
            }
        } else if self.is_comment_for_multiblock(next) {
            if !self.same_level_indented(comment, prev, next) {
                self.log_multiline(prev, comment, next);
            }
        } else if !self.same_level_indented(comment, prev, prev) {
            let prev_line = self.line(prev);
            let start = self.line_start(prev_line);
            self.log_one(comment, prev_line, self.col(comment), start);
        }
    }

    fn is_comment_for_multiblock(&self, end_block: N) -> bool {
        let a = self.a;
        let Some(parent) = a.parent(end_block) else { return false };
        let next_block = a.next_sibling(parent);
        let end_line = self.line(end_block);
        let catch_ast = a.parent(parent);
        let finally_ast = catch_ast.and_then(|c| a.next_sibling(c));
        next_block.is_some_and(|n| self.line(n) == end_line)
            || finally_ast.is_some_and(|f| catch_ast.is_some_and(|c| self.ty(c) == Tok::LITERAL_CATCH) && self.line(f) == end_line)
    }

    fn one_line_previous_statement(&self, comment: N) -> Option<N> {
        let a = self.a;
        let mut root = a.parent(comment);
        while let Some(r) = root {
            if matches!(self.ty(r), Tok::SLIST | Tok::OBJBLOCK | Tok::ARRAY_INIT | Tok::CASE_GROUP) {
                break;
            }
            root = a.parent(r);
        }
        let mut stack: Vec<N> = Vec::new();
        let mut previous = None;
        while root.is_some() || !stack.is_empty() {
            if let Some(top) = stack.pop() {
                root = Some(top);
            }
            while let Some(r) = root {
                previous = self.find_previous_statement(comment, r);
                if previous.is_some() {
                    root = None;
                    stack.clear();
                    break;
                }
                if let Some(s) = a.next_sibling(r) {
                    stack.push(s);
                }
                root = a.first_child(r);
            }
        }
        previous
    }

    fn find_previous_statement(&self, comment: N, root: N) -> Option<N> {
        let a = self.a;
        let mut previous = None;
        if self.line(root).max(self.line(comment)) == self.line(root) {
            previous = self.prev_statement_from_switch_block(comment);
        }
        let begins = match self.ty(root) {
            Tok::EXPR => self.start_token_of_method_call_chain(root),
            Tok::PLUS => a.first_child(root),
            _ => Some(root),
        };
        if let Some(t) = begins {
            if !is_comment(a, t) && self.on_previous_line_ignoring_comments(comment, t) {
                previous = Some(t);
            }
        }
        previous
    }

    fn start_token_of_method_call_chain(&self, root: N) -> Option<N> {
        let a = self.a;
        let mut start = root;
        while let Some(f) = a.first_child(start) {
            if self.line(f) != self.line(root) {
                break;
            }
            start = f;
        }
        match a.first_child(start) {
            Some(f) => a.next_sibling(f),
            None => Some(start),
        }
    }

    fn on_previous_line_ignoring_comments(&self, current: N, checked: N) -> bool {
        let a = self.a;
        let mut next_token = self.next_token(checked);
        let mut aim = 1;
        if next_token.is_some_and(|n| is_comment(a, n)) {
            aim += self.count_empty_lines(checked, current);
        }
        while let Some(n) = next_token {
            if n == current || !is_comment(a, n) {
                break;
            }
            if self.ty(n) == Tok::BLOCK_COMMENT_BEGIN {
                if let Some(last) = a.last_child(n) {
                    aim += self.line(last) - self.line(n);
                }
            }
            aim += 1;
            next_token = a.next_sibling(n);
        }
        self.line(current) - self.line(checked) == aim
    }

    fn next_token(&self, checked: N) -> Option<N> {
        let a = self.a;
        let mut next = if matches!(self.ty(checked), Tok::SLIST | Tok::ARRAY_INIT | Tok::CASE_GROUP) { a.first_child(checked) } else { a.next_sibling(checked) };
        if let Some(n) = next {
            if is_comment(a, n) && self.is_trailing_comment(n) {
                next = a.next_sibling(n);
            }
        }
        next
    }

    fn count_empty_lines(&self, start: N, end: N) -> i32 {
        let mut count = 0;
        let end_line = self.line(end);
        for line_no in self.line(start)..end_line {
            if self.lines.get(line_no as usize).is_some_and(|l| l.iter().all(|&c| is_ws(c))) {
                count += 1;
            }
        }
        count
    }

    fn log_multiline(&mut self, prev: N, comment: N, next: N) {
        let (pl, nl) = (self.line(prev), self.line(next));
        let (ps, ns) = (self.line_start(pl), self.line_start(nl));
        self.log(comment, format!("{pl}, {nl}"), self.col(comment).to_string(), format!("{ps}, {ns}"));
    }

    fn prev_statement_from_switch_block(&self, comment: N) -> Option<N> {
        let parent = self.a.parent(comment)?;
        if self.ty(parent) == Tok::CASE_GROUP { self.prev_statement_when_comment_under_case(parent) } else { self.prev_case_token(parent) }
    }

    fn prev_statement_when_comment_under_case(&self, parent: N) -> Option<N> {
        let a = self.a;
        let prev_block = a.prev_sibling(parent)?;
        let last = a.last_child(prev_block)?;
        let mut body = a.last_child(last)?;
        if self.ty(body) == Tok::SEMI {
            body = a.prev_sibling(body)?;
        }
        let mut prev = if self.ty(body) == Tok::EXPR {
            self.start_token_of_method_call_chain(body)?
        } else if self.ty(body) == Tok::SLIST {
            a.parent(a.parent(body)?)?
        } else {
            body
        };
        if is_comment(a, prev) {
            prev = a.next_sibling(prev)?;
        }
        Some(prev)
    }

    fn prev_case_token(&self, parent: N) -> Option<N> {
        let a = self.a;
        let block = a.parent(parent)?;
        let before = a.parent(block).and_then(|g| a.prev_sibling(g))?;
        if self.ty(before) == Tok::LITERAL_CASE { Some(before) } else { None }
    }

    fn same_level_indented(&self, comment: N, prev: N, next: N) -> bool {
        self.col(comment) == self.line_start(self.line(next)) || self.col(comment) == self.line_start(self.line(prev))
    }

    fn line_start(&self, line_no: i32) -> i32 {
        let Some(line) = self.lines.get((line_no - 1).max(0) as usize) else { return 0 };
        line.iter().take_while(|&&c| is_ws(c)).count() as i32
    }

    fn is_trailing_comment(&self, comment: N) -> bool {
        let a = self.a;
        let Some(line) = self.lines.get((self.line(comment) - 1).max(0) as usize) else { return false };
        let col = self.col(comment).max(0) as usize;
        let before = line.iter().take(col).all(|&c| is_ws(c)) && col <= line.len();
        if self.ty(comment) == Tok::SINGLE_LINE_COMMENT {
            !before
        } else {
            !before || a.next_sibling(comment).is_some_and(|n| self.line(n) == self.line(comment))
        }
    }

    fn in_same_method_call_with_same_indent(&self, comment: N) -> bool {
        let a = self.a;
        let Some(parent) = a.parent(comment).filter(|&p| self.ty(p) == Tok::METHOD_CALL) else { return false };
        a.find_first_token(parent, Tok::ELIST).is_some_and(|e| self.col(comment) == self.col(e))
    }
}
