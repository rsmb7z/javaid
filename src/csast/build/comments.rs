// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaParser.java (appendHiddenCommentNodes)
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Comment nodes (`checkstyle -T`): `JavaParser.appendHiddenCommentNodes`.
//!
//! In the ANTLR parse the comments are tokens on a hidden channel. Every AST node created from a
//! real token remembers the comments right before it and right after it; after the tree is built,
//! each node gets the comments before it as previous siblings, and the last node in document order
//! gets the comments after it as next siblings. Because the memory is per token, a token that the
//! visitor turns into several nodes (the type of `int a, b;`) gets its comments several times.

use tree_sitter::Node;

use super::{B, is_comment};
use crate::csast::ast::{N, NIL};
use crate::csast::tok::Tok;

#[derive(Debug, Clone, Copy)]
struct Comment {
    start: usize,
    /// For a line comment, includes the line terminator (the lexer's token does).
    end: usize,
    line: bool,
}

fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | 0x0c)
}

impl<'s> B<'s> {
    fn collect_comments(&self, root: Node) -> Vec<Comment> {
        let mut out = Vec::new();
        let mut cursor = root.walk();
        'outer: loop {
            let n = cursor.node();
            if is_comment(n) {
                let start = n.start_byte();
                if n.kind() == "line_comment" {
                    let mut end = start;
                    while end < self.src.len() && self.src[end] != b'\n' && self.src[end] != b'\r' {
                        end += 1;
                    }
                    if end < self.src.len() {
                        if self.src[end] == b'\r' && self.src.get(end + 1) == Some(&b'\n') {
                            end += 1;
                        }
                        end += 1;
                    }
                    out.push(Comment { start, end, line: true });
                } else {
                    out.push(Comment { start, end: n.end_byte(), line: false });
                }
            } else if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    break 'outer;
                }
            }
        }
        out.sort_by_key(|c| c.start);
        out
    }

    /// The comments directly before the token starting at `start`, in source order.
    fn hidden_before(&self, comments: &[Comment], start: usize) -> Vec<Comment> {
        let mut out = Vec::new();
        let mut idx = comments.partition_point(|c| c.end <= start);
        let mut limit = start;
        while idx > 0 {
            let c = comments[idx - 1];
            if self.src[c.end..limit].iter().all(|&b| is_ws(b)) {
                out.push(c);
                limit = c.start;
                idx -= 1;
            } else {
                break;
            }
        }
        out.reverse();
        out
    }

    /// The comments directly after the token ending at `end`, in source order.
    fn hidden_after(&self, comments: &[Comment], end: usize) -> Vec<Comment> {
        let mut out = Vec::new();
        let mut idx = comments.partition_point(|c| c.start < end);
        let mut from = end;
        while idx < comments.len() {
            let c = comments[idx];
            if c.start >= from && self.src[from..c.start].iter().all(|&b| is_ws(b)) {
                out.push(c);
                from = c.end;
                idx += 1;
            } else {
                break;
            }
        }
        out
    }

    fn comment_node(&mut self, c: Comment) -> N {
        let (line, col) = self.pos(c.start);
        if c.line {
            let sl = self.a.token(Tok::SINGLE_LINE_COMMENT, "//", line, col);
            let text = String::from_utf8_lossy(&self.src[c.start + 2..c.end]).into_owned();
            let content = self.a.token(Tok::COMMENT_CONTENT, &text, line, col + 2);
            self.a.add_child(sl, content);
            sl
        } else {
            let begin = self.a.token(Tok::BLOCK_COMMENT_BEGIN, "/*", line, col);
            let text = String::from_utf8_lossy(&self.src[c.start + 2..c.end - 2]).into_owned();
            let content = self.a.token(Tok::COMMENT_CONTENT, &text, line, col + 2);
            // ParserUtil.countLinesColumns: UTF-16 columns, `\r`, `\n` and `\r\n` end a line.
            let mut lines = line + 1;
            let mut columns = col + 1;
            let mut found_cr = false;
            for ch in text.chars() {
                if ch == '\n' {
                    found_cr = false;
                    lines += 1;
                    columns = 0;
                } else {
                    if found_cr {
                        found_cr = false;
                        lines += 1;
                        columns = 0;
                    }
                    if ch == '\r' {
                        found_cr = true;
                    }
                    columns += ch.len_utf16() as u32;
                }
            }
            if found_cr {
                lines += 1;
                columns = 0;
            }
            let end = self.a.token(Tok::BLOCK_COMMENT_END, "*/", lines - 1, columns);
            self.a.add_child(begin, content);
            self.a.add_child(begin, end);
            begin
        }
    }

    /// `appendHiddenCommentNodes`.
    pub(super) fn append_hidden_comment_nodes(&mut self, root: Node) {
        let comments = self.collect_comments(root);
        if comments.is_empty() || self.a.root == NIL {
            return;
        }
        let mut cur = self.a.root;
        let mut last = cur;
        while cur != NIL {
            last = cur;
            if let Some((s, _)) = self.a.span(cur) {
                let before = self.hidden_before(&comments, s);
                let mut sibling = cur;
                for c in before.into_iter().rev() {
                    let node = self.comment_node(c);
                    self.a.add_previous_sibling(sibling, node);
                    sibling = node;
                }
            }
            let mut to_visit = self.a.first_child(cur).unwrap_or(NIL);
            while cur != NIL && to_visit == NIL {
                to_visit = self.a.next_sibling(cur).unwrap_or(NIL);
                cur = self.a.parent(cur).unwrap_or(NIL);
            }
            cur = to_visit;
        }
        if let Some((_, e)) = self.a.span(last) {
            let after = self.hidden_after(&comments, e);
            let mut sibling = last;
            for c in after {
                let node = self.comment_node(c);
                self.a.add_next_sibling(sibling, node);
                sibling = node;
            }
        }
    }
}
