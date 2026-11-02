//! `TodoComment`.

use crate::engine::{Ctx, Diagnostic};
use tree_sitter::Node;

/// Called for every comment. Findings are per comment and per instance: the pattern is tried on
/// each line of the comment (a pattern can never match across a line break), the first match
/// reports once, at the line where the comment starts and the column where its text starts, just
/// after the `//` or `/*`.
///
/// Found by probing Checkstyle: in a file with CRLF line ends the lines of a comment keep their
/// `\r`, which Java's `$` tolerates before the end of the input. Hence the second try without it.
pub fn on_comment(cx: &mut Ctx, node: Node) {
    let cfg = cx.cfg;
    if cfg.todo_comments.is_empty() {
        return;
    }
    let text = cx.text(node);
    let content = match text.strip_prefix(b"//") {
        Some(rest) => rest,
        None => text.strip_prefix(b"/*").and_then(|t| t.strip_suffix(b"*/")).unwrap_or(&[]),
    };
    let at = node.start_position();
    for todo in &cfg.todo_comments {
        let hit = content
            .split(|&b| b == b'\n')
            .any(|line| todo.format.matches(line) || line.strip_suffix(b"\r").is_some_and(|l| todo.format.matches(l)));
        if hit {
            let message = format!("Comment matches to-do format '{}'.", todo.format.source);
            let line = at.row as u32 + 1;
            let col = cx.column(line, at.column as u32 + 1) + 2;
            cx.out.push(Diagnostic { rule: todo.name, line, col, message });
        }
    }
}
