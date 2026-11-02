// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/whitespace/EmptyLineSeparatorCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `EmptyLineSeparator`, ported from `EmptyLineSeparatorCheck.java` onto the emulated
//! `DetailAST` (with comment nodes, as the check asks for them).

use super::cs::{is_blank, is_comment_type, is_package_info, preorder, Log};
use crate::config::EmptyLineSeparator;
use crate::csast::{CsAst, Tok, N};

const RULE: &str = "EmptyLineSeparator";

fn separated(text: &str) -> String {
    format!("'{text}' should be separated from previous line.")
}

fn multiple_lines(text: &str) -> String {
    format!("'{text}' has more than 1 empty lines before.")
}

fn multiple_lines_after(text: &str) -> String {
    format!("'{text}' has more than 1 empty lines after.")
}

const INSIDE: &str = "There is more than 1 empty line after this line.";

struct Check<'a, 'b, 'c> {
    ast: &'a CsAst,
    opts: &'a EmptyLineSeparator,
    log: &'b mut Log<'c>,
}

pub fn check(ast: &CsAst, opts: &EmptyLineSeparator, log: &mut Log) {
    let mut c = Check { ast, opts, log };
    preorder(ast, |_, n| {
        if opts.tokens.contains(&ast.ty(n)) {
            c.visit_token(n);
        }
    });
}

impl Check<'_, '_, '_> {
    fn ty(&self, n: N) -> Tok {
        self.ast.ty(n)
    }

    fn line_no(&self, n: N) -> i32 {
        self.ast.line_no(n)
    }

    /// `getLine(index)`: the 0-based line, or none past the end of the file.
    fn get_line(&self, index: i32) -> &str {
        if index < 0 { "" } else { self.log.line(index as usize) }
    }

    fn log_at(&mut self, n: N, message: String) {
        self.log.at(RULE, self.ast, n, &message);
    }

    fn log_text(&mut self, n: N, message: impl FnOnce(&str) -> String) {
        let text = self.ast.text(n).to_string();
        self.log_at(n, message(&text));
    }

    fn visit_token(&mut self, n: N) {
        let ast = self.ast;
        self.check_comments(n);

        if self.has_multiple_lines_before(n) {
            self.log_text(n, multiple_lines);
        }
        if !self.opts.allow_multiple_empty_lines_inside_class_members {
            self.process_multiple_lines_inside(n);
        }
        if self.ty(n) == Tok::PACKAGE_DEF {
            self.check_comment_in_modifiers(n);
        }
        let mut next = ast.next_sibling(n);
        while let Some(t) = next.filter(|&t| is_comment_type(self.ty(t))) {
            next = ast.next_sibling(t);
        }
        if self.ty(n) == Tok::PACKAGE_DEF {
            self.process_package(n, next);
        } else if let Some(next) = next {
            self.check_token(n, next);
        }
    }

    fn check_token(&mut self, n: N, next: N) {
        let ast = self.ast;
        match self.ty(n) {
            Tok::VARIABLE_DEF => self.process_variable_def(n, next),
            Tok::IMPORT | Tok::STATIC_IMPORT | Tok::MODULE_IMPORT => self.process_import(n, next),
            Tok::ENUM_CONSTANT_DEF => self.process_enum_constant(n),
            _ => {
                if self.ty(next) == Tok::RCURLY {
                    if self.has_not_allowed_two_empty_lines_before(next) {
                        let result = self.get_last_element_before_empty_lines(n, self.line_no(next));
                        self.log_text(result, multiple_lines_after);
                    }
                } else if !self.has_empty_line_after(n) {
                    let _ = ast;
                    self.log_text(next, separated);
                }
            }
        }
    }

    fn check_comment_in_modifiers(&mut self, package_def: N) {
        let ast = self.ast;
        let comment = ast
            .next_sibling(package_def)
            .and_then(|s| ast.find_first_token(s, Tok::MODIFIERS))
            .and_then(|m| ast.first_child(m))
            .filter(|&t| is_comment_type(self.ty(t)))
            .filter(|&c| self.line_no(c) == self.line_no(package_def) + 1);
        if let Some(c) = comment {
            self.log_text(c, separated);
        }
    }

    fn process_multiple_lines_inside(&mut self, n: N) {
        if matches!(self.ty(n), Tok::STATIC_INIT | Tok::INSTANCE_INIT | Tok::METHOD_DEF | Tok::CTOR_DEF | Tok::COMPACT_CTOR_DEF) {
            let empty_lines = self.get_empty_lines(n);
            let mut to_log = Vec::new();
            let mut previous = -1;
            for &line in &empty_lines {
                if previous + 1 == line {
                    to_log.push(previous);
                }
                previous = line;
            }
            for line_no in to_log {
                let at = self.get_last_element_before_empty_lines(n, line_no);
                self.log_at(at, INSIDE.to_string());
            }
        }
    }

    fn get_last_element_before_empty_lines(&self, n: N, line: i32) -> N {
        let ast = self.ast;
        let mut result = n;
        if self.line_no(ast.first_child(n).expect("a child")) <= line {
            result = ast.first_child(n).expect("a child");
            while let Some(s) = ast.next_sibling(result).filter(|&s| self.line_no(s) <= line) {
                result = s;
            }
            if ast.first_child(result).is_some() {
                result = self.get_last_element_before_empty_lines(result, line);
            }
        }
        self.position_to_potential_post_fix_node(result, line)
    }

    fn position_to_potential_post_fix_node(&self, post_fix: N, line: i32) -> N {
        let ast = self.ast;
        let mut result = post_fix;
        if let Some(next) = ast.next_sibling(result) {
            if let Some(node) = self.get_post_fix_node(next) {
                result = self.get_last_element_before_empty_lines(node, line);
            }
        }
        if self.line_no(result) > line {
            result = post_fix;
        }
        result
    }

    fn get_post_fix_node(&self, n: N) -> Option<N> {
        let ast = self.ast;
        if self.ty(n) == Tok::EXPR {
            let first = ast.first_child(n)?;
            if self.ty(first) == Tok::METHOD_CALL {
                let node = ast.first_child(first)?;
                if self.ty(node) == Tok::DOT {
                    return Some(node);
                }
            }
        }
        None
    }

    fn get_empty_lines(&self, n: N) -> Vec<i32> {
        let ast = self.ast;
        let last_token = ast.last_child(n).and_then(|c| ast.last_child(c));
        let mut last_token_line = 0;
        if let Some(t) = last_token {
            last_token_line = self.line_no(t) - 2;
        }
        let mut empty = Vec::new();
        let mut line_no = self.line_no(n);
        while line_no <= last_token_line {
            if is_blank(self.get_line(line_no)) {
                empty.push(line_no);
            }
            line_no += 1;
        }
        empty
    }

    fn has_multiple_lines_before(&self, n: N) -> bool {
        (self.ty(n) != Tok::VARIABLE_DEF || self.is_type_field(n)) && self.has_not_allowed_two_empty_lines_before(n) && self.ty(n) != Tok::ENUM_CONSTANT_DEF
    }

    fn process_package(&mut self, n: N, next: Option<N>) {
        let ast = self.ast;
        if self.line_no(n) > 1 && !self.has_empty_line_before(n) {
            if is_package_info() {
                let first = ast.first_child(n).expect("annotations");
                if ast.first_child(first).is_none() && !self.is_preceded_by_javadoc(n) {
                    self.log_text(n, separated);
                }
            } else {
                self.log_text(n, separated);
            }
        }
        if self.is_line_empty_after_package(n) {
            let element = self.get_violation_ast_for_package(n);
            self.log_text(element, separated);
        } else if let Some(next) = next {
            if !self.has_empty_line_after(n) {
                self.log_text(next, separated);
            }
        }
    }

    fn is_line_empty_after_package(&self, n: N) -> bool {
        let ast = self.ast;
        let mut element = n;
        let last_child_line = self.line_no(ast.last_child(n).expect("a child"));
        while self.line_no(element) < last_child_line + 1 {
            match ast.next_sibling(element) {
                Some(s) => element = s,
                None => break,
            }
        }
        self.line_no(element) == last_child_line + 1
    }

    fn get_violation_ast_for_package(&self, n: N) -> N {
        let ast = self.ast;
        let mut element = n;
        let last_child_line = self.line_no(ast.last_child(n).expect("a child"));
        while self.line_no(element) < last_child_line + 1 {
            element = ast.next_sibling(element).expect("a following sibling");
        }
        element
    }

    fn process_import(&mut self, n: N, next: N) {
        if !matches!(self.ty(next), Tok::IMPORT | Tok::STATIC_IMPORT | Tok::MODULE_IMPORT) && !self.has_empty_line_after(n) {
            self.log_text(next, separated);
        }
    }

    fn process_variable_def(&mut self, n: N, next: N) {
        if self.is_type_field(n) && !self.has_empty_line_after(n) && self.is_violating_empty_line_between_fields_policy(next) {
            self.log_text(next, separated);
        }
    }

    fn process_enum_constant(&mut self, n: N) {
        if !self.is_enum_constant_on_shared_line(n) && self.has_not_allowed_two_empty_lines_before(n) {
            self.log_text(n, multiple_lines);
        }
        if !self.opts.allow_multiple_empty_lines {
            self.check_multiple_empty_lines_after_last_enum_constant(n);
        }
    }

    fn is_enum_constant_on_shared_line(&self, n: N) -> bool {
        let ast = self.ast;
        let object_block = ast.parent(n).expect("parent");
        if self.line_no(ast.parent(object_block).expect("parent")) == self.line_no(n) {
            return true;
        }
        let mut previous = ast.prev_sibling(n);
        while let Some(p) = previous {
            if self.ty(p) == Tok::ENUM_CONSTANT_DEF && self.line_no(p) == self.line_no(n) {
                return true;
            }
            previous = ast.prev_sibling(p);
        }
        false
    }

    fn check_multiple_empty_lines_after_last_enum_constant(&mut self, n: N) {
        let ast = self.ast;
        let mut next = ast.next_sibling(n).expect("a following sibling");
        while self.ty(next) == Tok::COMMA || is_comment_type(self.ty(next)) {
            next = ast.next_sibling(next).expect("a following sibling");
        }
        if self.ty(next) == Tok::RCURLY && self.has_consecutive_empty_lines(self.line_no(n), self.line_no(next)) {
            self.log_text(n, multiple_lines_after);
        }
    }

    fn has_consecutive_empty_lines(&self, start: i32, end: i32) -> bool {
        let mut empty = 0;
        let mut line = start;
        while line <= end - 2 {
            if is_blank(self.get_line(line)) {
                empty += 1;
                if empty > 1 {
                    return true;
                }
            } else {
                empty = 0;
            }
            line += 1;
        }
        false
    }

    fn is_violating_empty_line_between_fields_policy(&self, n: N) -> bool {
        self.ty(n) != Tok::RCURLY && self.ty(n) != Tok::COMMA && (!self.opts.allow_no_empty_line_between_fields || self.ty(n) != Tok::VARIABLE_DEF)
    }

    fn has_not_allowed_two_empty_lines_before(&self, token: N) -> bool {
        !self.opts.allow_multiple_empty_lines
            && (self.has_empty_line_before(token) || ast_has_type_child(self.ast, token))
            && self.is_pre_previous_line_empty(token)
    }

    fn check_comments(&mut self, token: N) {
        if self.opts.allow_multiple_empty_lines {
            return;
        }
        let ast = self.ast;
        if matches!(self.ty(token), Tok::PACKAGE_DEF | Tok::IMPORT | Tok::STATIC_IMPORT | Tok::MODULE_IMPORT | Tok::STATIC_INIT) {
            let mut previous = ast.prev_sibling(token);
            while self.is_comment_in_beginning_of_line(previous) {
                let p = previous.expect("a comment");
                if self.has_empty_line_before(p) && self.is_pre_previous_line_empty(p) {
                    self.log_text(p, multiple_lines);
                }
                previous = ast.prev_sibling(p);
            }
        } else {
            self.check_comments_inside_token(token);
        }
    }

    fn check_comments_inside_token(&mut self, token: N) {
        let ast = self.ast;
        let mut children = Vec::new();
        let mut child = ast.last_child(token);
        while let Some(c) = child {
            if self.ty(c) == Tok::MODIFIERS {
                let mut node = ast.first_child(token).and_then(|f| ast.last_child(f));
                while let Some(x) = node {
                    if self.is_comment_in_beginning_of_line(Some(x)) {
                        children.push(x);
                    }
                    node = ast.prev_sibling(x);
                }
            } else if self.is_comment_in_beginning_of_line(Some(c)) {
                children.push(c);
            }
            child = ast.prev_sibling(c);
        }
        for node in children {
            if self.has_empty_line_before(node) && self.is_pre_previous_line_empty(node) {
                self.log_text(node, multiple_lines);
            }
        }
    }

    fn get_start_line_no(&self, token: N) -> i32 {
        let ast = self.ast;
        let mut line_no = self.line_no(token);
        if let Some(modifiers) = ast.find_first_token(token, Tok::MODIFIERS) {
            if let Some(first) = ast.first_child(modifiers).filter(|&f| self.ty(f) == Tok::ANNOTATION) {
                let comment = ast.first_child(first);
                if self.is_comment_in_beginning_of_line(comment) {
                    let comment = comment.expect("a comment");
                    let mut last_comment = comment;
                    while is_comment_type(self.ty(ast.next_sibling(last_comment).expect("a following sibling"))) {
                        last_comment = ast.next_sibling(last_comment).expect("a following sibling");
                    }
                    let after = ast.next_sibling(last_comment).expect("a following sibling");
                    let last_comment_line = ast.last_child(last_comment).expect("comment content");
                    if self.line_no(after) - self.line_no(last_comment_line) <= 1 {
                        line_no = self.line_no(comment);
                    }
                }
            }
        }
        line_no
    }

    fn is_pre_previous_line_empty(&self, token: N) -> bool {
        let mut result = false;
        let line_no = self.get_start_line_no(token);
        if line_no >= 3 {
            let pre_previous = self.get_line(line_no - 3);
            result = is_blank(pre_previous);
            let previous_empty = is_blank(self.get_line(line_no - 2));
            if previous_empty && result {
                result = true;
            } else if ast_has_type_child(self.ast, token) {
                result = self.is_two_preceding_previous_lines_from_comment_empty(token);
            }
        }
        result
    }

    fn is_two_preceding_previous_lines_from_comment_empty(&self, token: N) -> bool {
        let ast = self.ast;
        let ty = ast.find_first_token(token, Tok::TYPE).expect("a type");
        let mut type_child = ast.last_child(ty);
        while let Some(c) = type_child {
            if self.line_no(c) > 2 && self.is_token_not_on_previous_sibling_lines(c, token) {
                let before = self.get_line(self.line_no(c) - 2);
                let before_before = self.get_line(self.line_no(c) - 3);
                if is_blank(before) && is_blank(before_before) {
                    return true;
                }
            }
            type_child = ast.prev_sibling(c);
        }
        false
    }

    fn is_token_not_on_previous_sibling_lines(&self, token: N, parent: N) -> bool {
        let ast = self.ast;
        let mut previous = ast.prev_sibling(parent);
        let mut node = previous;
        while let Some(x) = node {
            previous = Some(x);
            node = ast.last_child(x);
        }
        previous.is_none_or(|p| self.line_no(token) != self.line_no(p))
    }

    fn has_empty_line_after(&self, token: N) -> bool {
        let ast = self.ast;
        let last = ast.last_child(token).expect("a child");
        let last_token = ast.last_child(last).unwrap_or(last);
        let mut next = ast.next_sibling(token).expect("a following sibling");
        if is_comment_type(self.ty(next)) {
            next = ast.next_sibling(next).expect("a following sibling");
        }
        let next_begin = self.line_no(next);
        let current_end = self.line_no(last_token);
        self.has_empty_line(current_end + 1, next_begin - 1)
    }

    fn has_empty_line(&self, start: i32, end: i32) -> bool {
        (start..=end).any(|line| is_blank(self.get_line(line - 1)))
    }

    fn has_empty_line_before(&self, token: N) -> bool {
        let line_no = self.line_no(token);
        line_no != 1 && is_blank(self.get_line(line_no - 2))
    }

    fn is_comment_in_beginning_of_line(&self, comment: Option<N>) -> bool {
        let Some(c) = comment else { return false };
        let line = java_trim(self.get_line(self.line_no(c) - 1));
        line.starts_with("//") || line.starts_with("/*")
    }

    fn is_preceded_by_javadoc(&self, token: N) -> bool {
        let ast = self.ast;
        let Some(previous) = ast.prev_sibling(token) else { return false };
        if self.ty(previous) != Tok::BLOCK_COMMENT_BEGIN {
            return false;
        }
        ast.first_child(previous).is_some_and(|c| ast.text(c).starts_with('*'))
    }

    fn is_type_field(&self, variable_def: N) -> bool {
        let ast = self.ast;
        let parent = ast.parent(variable_def).expect("parent");
        self.ty(parent) == Tok::COMPACT_COMPILATION_UNIT
            || ast.parent(parent).is_some_and(|g| matches!(self.ty(g), Tok::CLASS_DEF | Tok::INTERFACE_DEF | Tok::ANNOTATION_DEF | Tok::ENUM_DEF | Tok::RECORD_DEF))
    }
}

fn ast_has_type_child(ast: &CsAst, n: N) -> bool {
    ast.find_first_token(n, Tok::TYPE).is_some()
}

/// `String.trim()`: strips every character up to and including the space.
fn java_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c <= ' ')
}
