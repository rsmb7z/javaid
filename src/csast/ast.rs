// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/DetailAstImpl.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! The arena tree, with the same linking operations as Checkstyle's `DetailAstImpl`.
//!
//! The operations (`add_child`, `set_next_sibling`, `add_previous_sibling`) deliberately reproduce
//! the Java class, including its odd corners (appending a chain re-parents the whole chain; setting
//! a next sibling drops whatever followed), because the AST shapes of Checkstyle depend on them.

use super::tok::Tok;

/// A node handle. `NIL` stands for Java's `null`.
pub type N = u32;
pub const NIL: N = u32::MAX;

const NOT_INITIALIZED: i32 = i32::MIN;

/// Marks a node that has no source token (the imaginary nodes).
pub const NO_SPAN: u32 = u32::MAX;

#[derive(Debug, Clone)]
pub struct Node {
    pub ty: Tok,
    pub text: std::borrow::Cow<'static, str>,
    line: i32,
    col: i32,
    parent: N,
    first: N,
    next: N,
    prev: N,
    /// Byte range of the source token the node was created from (`NO_SPAN` for imaginary nodes).
    start: u32,
    end: u32,
}

#[derive(Debug, Default)]
pub struct CsAst {
    nodes: Vec<Node>,
    /// The root, or `NIL` for an empty file (Checkstyle prints nothing for it).
    pub root: N,
}

impl CsAst {
    pub fn new() -> Self {
        CsAst { nodes: Vec::new(), root: NIL }
    }

    /// A token node: type, text and 0-based position.
    pub fn token(&mut self, ty: Tok, text: &str, line: u32, col: u32) -> N {
        self.token_text(ty, std::borrow::Cow::Owned(text.to_string()), line, col)
    }

    /// As [`CsAst::token`], taking the text over (no copy) or borrowing a static one.
    pub fn token_text(&mut self, ty: Tok, text: std::borrow::Cow<'static, str>, line: u32, col: u32) -> N {
        self.push(Node { ty, text, line: line as i32 + 1, col: col as i32, parent: NIL, first: NIL, next: NIL, prev: NIL, start: NO_SPAN, end: NO_SPAN })
    }

    /// Records the source bytes a token node covers.
    pub fn set_span(&mut self, n: N, start: usize, end: usize) {
        self.nodes[n as usize].start = start as u32;
        self.nodes[n as usize].end = end as u32;
    }

    /// The source bytes of the token this node was created from (none for imaginary nodes).
    pub fn span(&self, n: N) -> Option<(usize, usize)> {
        let node = &self.nodes[n as usize];
        if node.start == NO_SPAN { None } else { Some((node.start as usize, node.end as usize)) }
    }

    /// An imaginary node: its text is the token name and its position comes from its children.
    pub fn imaginary(&mut self, ty: Tok) -> N {
        self.push(Node {
            ty,
            text: std::borrow::Cow::Borrowed(ty.name()),
            line: NOT_INITIALIZED,
            col: NOT_INITIALIZED,
            parent: NIL,
            first: NIL,
            next: NIL,
            prev: NIL,
            start: NO_SPAN,
            end: NO_SPAN,
        })
    }

    fn push(&mut self, node: Node) -> N {
        self.nodes.push(node);
        (self.nodes.len() - 1) as N
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn ty(&self, n: N) -> Tok {
        self.nodes[n as usize].ty
    }

    pub fn set_type(&mut self, n: N, ty: Tok) {
        self.nodes[n as usize].ty = ty;
    }

    pub fn text(&self, n: N) -> &str {
        &self.nodes[n as usize].text
    }

    pub fn set_text(&mut self, n: N, text: &str) {
        self.nodes[n as usize].text = std::borrow::Cow::Owned(text.to_string());
    }

    pub fn parent(&self, n: N) -> Option<N> {
        opt(self.nodes[n as usize].parent)
    }

    pub fn first_child(&self, n: N) -> Option<N> {
        opt(self.nodes[n as usize].first)
    }

    pub fn next_sibling(&self, n: N) -> Option<N> {
        opt(self.nodes[n as usize].next)
    }

    pub fn prev_sibling(&self, n: N) -> Option<N> {
        opt(self.nodes[n as usize].prev)
    }

    pub fn last_child(&self, n: N) -> Option<N> {
        let mut c = self.first_child(n)?;
        while let Some(x) = self.next_sibling(c) {
            c = x;
        }
        Some(c)
    }

    /// The children, in order.
    pub fn children(&self, n: N) -> Children<'_> {
        Children { ast: self, cur: self.nodes[n as usize].first }
    }

    pub fn child_count(&self, n: N) -> usize {
        self.children(n).count()
    }

    /// `findFirstToken`: the first direct child of the given type.
    pub fn find_first_token(&self, n: N, ty: Tok) -> Option<N> {
        self.children(n).find(|&c| self.ty(c) == ty)
    }

    /// `branchContains`: whether this node or any descendant has the type.
    pub fn branch_contains(&self, n: N, ty: Tok) -> bool {
        if self.ty(n) == ty {
            return true;
        }
        self.children(n).any(|c| self.branch_contains(c, ty))
    }

    /// The 1-based line number as Checkstyle reports it (imaginary nodes borrow it).
    pub fn line_no(&self, n: N) -> i32 {
        let node = &self.nodes[n as usize];
        let mut result = -1;
        if node.line == NOT_INITIALIZED {
            result = self.find_pos(node.first, true);
            if result == -1 {
                result = self.find_pos(node.next, true);
            }
        }
        if result == -1 { node.line } else { result }
    }

    /// The 0-based column as Checkstyle reports it (the printer adds one).
    pub fn column_no(&self, n: N) -> i32 {
        let node = &self.nodes[n as usize];
        let mut result = -1;
        if node.col == NOT_INITIALIZED {
            result = self.find_pos(node.first, false);
            if result == -1 {
                result = self.find_pos(node.next, false);
            }
        }
        if result == -1 { node.col } else { result }
    }

    fn find_pos(&self, start: N, line: bool) -> i32 {
        // Comment nodes cannot start a construct; there are none in this tree unless the comment
        // mode adds them.
        let mut node = start;
        while node != NIL {
            if is_comment(self.nodes[node as usize].ty) {
                node = self.nodes[node as usize].next;
            } else {
                return if line { self.line_no(node) } else { self.column_no(node) };
            }
        }
        -1
    }

    // ---- linking, as in DetailAstImpl ----

    /// `addChild`: appends `child` (and the siblings chained after it) as last children.
    pub fn add_child(&mut self, parent: N, child: N) {
        if child == NIL {
            return;
        }
        self.set_parent_chain(child, parent);
        let first = self.nodes[parent as usize].first;
        if first == NIL {
            self.nodes[parent as usize].first = child;
        } else {
            let mut t = first;
            while self.nodes[t as usize].next != NIL {
                t = self.nodes[t as usize].next;
            }
            self.set_next_sibling(t, child);
        }
    }

    fn set_parent_chain(&mut self, n: N, parent: N) {
        let mut i = n;
        while i != NIL {
            self.nodes[i as usize].parent = parent;
            i = self.nodes[i as usize].next;
        }
    }

    /// `setNextSibling`: replaces whatever followed `n`.
    pub fn set_next_sibling(&mut self, n: N, sibling: N) {
        self.nodes[n as usize].next = sibling;
        if sibling != NIL {
            let parent = self.nodes[n as usize].parent;
            if parent != NIL {
                self.set_parent_chain(sibling, parent);
            }
            self.nodes[sibling as usize].prev = n;
        }
    }

    /// `setFirstChild`.
    pub fn set_first_child(&mut self, n: N, child: N) {
        self.nodes[n as usize].first = child;
        if child != NIL {
            self.set_parent_chain(child, n);
        }
    }

    /// `addNextSibling`: inserts `ast` right after `n`, pushing the other siblings back.
    pub fn add_next_sibling(&mut self, n: N, ast: N) {
        if ast == NIL {
            return;
        }
        let next = self.nodes[n as usize].next;
        self.set_next_sibling(ast, next);
        self.set_next_sibling(n, ast);
    }

    /// `addPreviousSibling`: inserts `ast` (a single node or chain) before `n`.
    pub fn add_previous_sibling(&mut self, n: N, ast: N) {
        if ast == NIL {
            return;
        }
        let prev = self.nodes[n as usize].prev;
        let parent = self.nodes[n as usize].parent;
        if prev != NIL {
            self.set_next_sibling(prev, ast);
        } else if parent != NIL {
            self.set_first_child(parent, ast);
        }
        self.set_next_sibling(ast, n);
    }

    /// The builder's `addLastSibling`: appends `sibling` after the last node of `n`'s chain.
    pub fn add_last_sibling(&mut self, n: N, sibling: N) {
        if n == NIL {
            return;
        }
        let mut t = n;
        while self.nodes[t as usize].next != NIL {
            t = self.nodes[t as usize].next;
        }
        self.set_next_sibling(t, sibling);
    }
}

fn opt(n: N) -> Option<N> {
    if n == NIL { None } else { Some(n) }
}

fn is_comment(ty: Tok) -> bool {
    matches!(ty, Tok::SINGLE_LINE_COMMENT | Tok::BLOCK_COMMENT_BEGIN | Tok::BLOCK_COMMENT_END | Tok::COMMENT_CONTENT)
}

pub struct Children<'a> {
    ast: &'a CsAst,
    cur: N,
}

impl Iterator for Children<'_> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        if self.cur == NIL {
            return None;
        }
        let c = self.cur;
        self.cur = self.ast.nodes[c as usize].next;
        Some(c)
    }
}
