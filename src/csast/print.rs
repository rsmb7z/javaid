// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/AstTreeStringPrinter.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! The printer of `AstTreeStringPrinter`: `|--`/`` `-- `` branches, one node per line.

use super::ast::{CsAst, N, NIL};

/// The text of `checkstyle -t` for the tree (empty for an empty file).
pub fn dump(ast: &CsAst) -> String {
    let mut out = String::with_capacity(ast.len() * 48);
    if ast.root == NIL {
        return out;
    }
    // Iterative pre-order walk, so a deeply nested tree cannot overflow the stack.
    let mut stack: Vec<N> = vec![ast.root];
    while let Some(n) = stack.pop() {
        indentation(ast, n, &mut out);
        node_info(ast, n, &mut out);
        out.push('\n');
        if let Some(s) = ast.next_sibling(n) {
            stack.push(s);
        }
        if let Some(c) = ast.first_child(n) {
            stack.push(c);
        }
    }
    out
}

fn node_info(ast: &CsAst, n: N, out: &mut String) {
    out.push_str(ast.ty(n).name());
    out.push_str(" -> ");
    for ch in ast.text(n).chars() {
        match ch {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push_str(" [");
    out.push_str(&ast.line_no(n).to_string());
    out.push(':');
    out.push_str(&(ast.column_no(n) as i64 + 1).to_string());
    out.push(']');
}

/// Ancestors contribute `|   ` (they have a following sibling) or four spaces, outermost first;
/// the node's own branch marker comes last. The root has no marker at all.
fn indentation(ast: &CsAst, n: N, out: &mut String) {
    let Some(mut node) = ast.parent(n) else { return };
    let mut pieces: Vec<&'static str> = Vec::new();
    while let Some(p) = ast.parent(node) {
        pieces.push(if ast.next_sibling(node).is_none() { "    " } else { "|   " });
        node = p;
    }
    for s in pieces.iter().rev() {
        out.push_str(s);
    }
    out.push_str(if ast.next_sibling(n).is_none() { "`--" } else { "|--" });
}
