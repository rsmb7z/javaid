//! `SuppressionCommentFilter` and `SuppressWithNearbyCommentFilter`: comments that switch the
//! TreeWalker's findings off, or hold them back from nearby lines.
//!
//! Both read the comments the way `FileContents` reports them: a line comment as the rest of its
//! line, a block comment line by line (the first from the comment's column, the last up to `*/`).
//! They only see TreeWalker modules; the file-level text modules (line length, tabs, trailing
//! space...) report to the Checker and are out of their reach.

use crate::config::Format;
use crate::engine::Ctx;
use crate::suppress::{CommentFilter, fill, module_of, source_name};
use crate::syntax::KindExt;
use tree_sitter::Node;

/// Modules that report to the Checker, not to the TreeWalker.
pub const CHECKER_LEVEL: &[&str] = &[
    "FileTabCharacter",
    "RegexpSingleline",
    "RegexpMultiline",
    "Regexp",
    "NewlineAtEndOfFile",
    "LineLength",
    "JavadocPackage",
    "Translation",
    "UniqueProperties",
    "OrderedProperties",
    "Header",
    "RegexpHeader",
    "MultiFileRegexpHeader",
    "RegexpOnFilename",
    "FileLength",
];

struct Comment {
    /// `(line number, column of the text, text)` per line.
    lines: Vec<(u32, usize, String)>,
    block: bool,
}

#[derive(Default)]
pub struct State {
    comments: Vec<Comment>,
}

pub fn on_comment(cx: &Ctx, st: &mut State, node: Node) {
    if cx.cfg.comment_filters.is_empty() {
        return;
    }
    let start = node.start_position();
    let text = String::from_utf8_lossy(cx.text(node)).into_owned();
    let block = node.kind_s() == "block_comment";
    // The line of the source the comment starts on, from the comment's column to its end of line.
    let first_line = text.split('\n').next().unwrap_or("").trim_end_matches('\r').to_string();
    let mut lines = vec![(start.row as u32 + 1, start.column, first_line)];
    if block {
        let parts: Vec<&str> = text.split('\n').collect();
        for (i, part) in parts.iter().enumerate().skip(1) {
            lines.push((start.row as u32 + 1 + i as u32, 0, part.trim_end_matches('\r').to_string()));
        }
    }
    st.comments.push(Comment { lines, block });
}

struct Tag {
    line: u32,
    col: usize,
    on: bool,
    first: u32,
    last: u32,
    source: Option<Format>,
    message: Option<Format>,
    id: Option<Format>,
}

impl Tag {
    fn matches(&self, source: &str, id: Option<&str>, message: &str) -> bool {
        self.source.as_ref().is_none_or(|f| f.matches(source.as_bytes()))
            && self.id.as_ref().is_none_or(|f| id.is_some_and(|id| f.matches(id.as_bytes())))
            && self.message.as_ref().is_none_or(|f| f.matches(message.as_bytes()))
    }
}

fn formats(check: &str, message: &Option<String>, id: &Option<String>, text: &str, re: &Format) -> (Option<Format>, Option<Format>, Option<Format>) {
    (
        Format::new(&fill(check, text, re)).ok(),
        message.as_ref().and_then(|m| Format::new(&fill(m, text, re)).ok()),
        id.as_ref().and_then(|m| Format::new(&fill(m, text, re)).ok()),
    )
}

fn tags_of(filter: &CommentFilter, comments: &[Comment]) -> Vec<Tag> {
    let mut tags = Vec::new();
    for c in comments {
        match filter {
            CommentFilter::OnOff { off, on, check, message, id, check_c, check_cpp } => {
                if (c.block && !check_c) || (!c.block && !check_cpp) {
                    continue;
                }
                for (line, col, text) in &c.lines {
                    let (is_on, re, group) = if let Some(g) = off.captures(text) {
                        (false, off, g)
                    } else if let Some(g) = on.captures(text) {
                        (true, on, g)
                    } else {
                        continue;
                    };
                    let text = group[0].clone().unwrap_or_default();
                    let (source, message, id) = formats(check, message, id, &text, re);
                    tags.push(Tag { line: *line, col: *col, on: is_on, first: 0, last: 0, source, message, id });
                }
            }
            CommentFilter::Nearby { comment, check, message, id, influence, check_c, check_cpp } => {
                if (c.block && !check_c) || (!c.block && !check_cpp) {
                    continue;
                }
                for (line, _, text) in &c.lines {
                    let Some(g) = comment.captures(text) else { continue };
                    let matched = g[0].clone().unwrap_or_default();
                    let Ok(influence) = fill(influence, &matched, comment).trim().parse::<i64>() else { continue };
                    let (first, last) = if influence >= 1 { (*line as i64, *line as i64 + influence) } else { (*line as i64 + influence, *line as i64) };
                    let (source, message, id) = formats(check, message, id, &matched, comment);
                    tags.push(Tag { line: *line, col: 0, on: false, first: first.max(0) as u32, last: last.max(0) as u32, source, message, id });
                }
            }
        }
    }
    tags.sort_by_key(|t| (t.line, t.col));
    tags
}

pub fn finish(cx: &mut Ctx, st: &mut State) {
    let cfg = cx.cfg;
    if cfg.comment_filters.is_empty() || cx.out.is_empty() {
        return;
    }
    let per_filter: Vec<Vec<Tag>> = cfg.comment_filters.iter().map(|f| tags_of(f, &st.comments)).collect();
    let mut out = std::mem::take(cx.out);
    out.retain(|d| {
        let (module, id) = module_of(cfg, d.rule);
        if CHECKER_LEVEL.contains(&module.as_str()) {
            return true;
        }
        let column = d.col;
        let source = source_name(module);
        !cfg.comment_filters.iter().zip(&per_filter).any(|(filter, tags)| match filter {
            CommentFilter::OnOff { .. } => {
                let mut nearest: Option<&Tag> = None;
                for t in tags {
                    if t.line > d.line || (t.line == d.line && t.col as u32 > column) {
                        break;
                    }
                    if t.matches(&source, id, &d.message) {
                        nearest = Some(t);
                    }
                }
                nearest.is_some_and(|t| !t.on)
            }
            CommentFilter::Nearby { .. } => tags.iter().any(|t| t.first <= d.line && d.line <= t.last && t.matches(&source, id, &d.message)),
        })
    });
    *cx.out = out;
}
