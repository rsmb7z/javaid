//! The single-pass tree walk.
//!
//! Every file is parsed once and walked once. Each node is dispatched through the kind-id lookup
//! table in `syntax`, so adding a rule never adds another traversal.

use crate::config::Config;
use crate::rules::{self, State};
use crate::syntax::Syntax;
use tree_sitter::{Node, Tree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The Checkstyle module name, e.g. `NeedBraces`.
    pub rule: &'static str,
    pub line: u32,
    pub col: u32,
    pub message: String,
}

/// Everything a rule needs while the walk is in progress.
pub struct Ctx<'a> {
    pub syn: &'a Syntax,
    pub cfg: &'a Config,
    pub src: &'a [u8],
    pub out: &'a mut Vec<Diagnostic>,
    /// The ancestors of the node being visited, outermost first. Tree-sitter keeps no parent
    /// pointers: `Node::parent` searches from the root, and a sibling call searches the parent's
    /// children, which adds up when a rule asks for one of every statement. The walk knows these
    /// already, so it keeps them.
    parents: Vec<Node<'a>>,
    /// The previous sibling that is not a comment, for the node being visited (the last entry) and
    /// for each of its ancestors. Unnamed tokens such as `{` and `;` count.
    previous: Vec<Option<Node<'a>>>,
}

impl<'a> Ctx<'a> {
    pub fn new(syn: &'a Syntax, cfg: &'a Config, src: &'a [u8], out: &'a mut Vec<Diagnostic>) -> Self {
        Ctx { syn, cfg, src, out, parents: Vec::new(), previous: vec![None] }
    }

    /// The parent of the node being visited. Only meaningful for that node, not for others.
    pub fn parent(&self) -> Option<Node<'a>> {
        self.parents.last().copied()
    }

    /// The `levels`-th ancestor above the parent: `ancestor(0)` is the parent, `ancestor(1)` the
    /// grandparent.
    pub fn ancestor(&self, levels: usize) -> Option<Node<'a>> {
        self.parents.len().checked_sub(levels + 1).map(|i| self.parents[i])
    }

    /// The previous sibling of the node being visited, skipping comments.
    pub fn previous_sibling(&self) -> Option<Node<'a>> {
        self.previous.last().copied().flatten()
    }

    /// In debug builds, which includes `cargo test` over every probe file, confirm the stack agrees
    /// with tree-sitter's own answers at every node. Release builds skip it: the point of the
    /// stack is not to ask tree-sitter.
    #[cfg(debug_assertions)]
    fn check_against_tree_sitter(&self, node: Node<'a>) {
        assert_eq!(self.parent().map(|p| p.id()), node.parent().map(|p| p.id()), "the ancestor stack disagrees with tree-sitter at {node:?}");
        let mut before = node.prev_sibling();
        while let Some(b) = before.filter(|b| b.is_extra()) {
            before = b.prev_sibling();
        }
        assert_eq!(self.previous_sibling().map(|p| p.id()), before.map(|p| p.id()), "the previous-sibling stack disagrees with tree-sitter at {node:?}");
    }

    fn visited(&mut self, node: Node<'a>) {
        if !node.is_extra() {
            if let Some(slot) = self.previous.last_mut() {
                *slot = Some(node);
            }
        }
    }

    fn enter(&mut self, parent: Node<'a>) {
        self.parents.push(parent);
        self.previous.push(None);
    }

    fn leave(&mut self) {
        self.parents.pop();
        self.previous.pop();
    }

    pub fn text(&self, n: Node) -> &'a [u8] {
        let src: &'a [u8] = self.src;
        &src[n.byte_range()]
    }

    pub fn diag(&mut self, rule: &'static str, at: Node, message: String) {
        let p = at.start_position();
        self.out.push(Diagnostic { rule: self.cfg.display(rule), line: p.row as u32 + 1, col: p.column as u32 + 1, message });
    }

    /// A finding at a position that is not a node's: a declaration's reported line, say.
    pub fn diag_at(&mut self, rule: &'static str, line: u32, col: u32, message: String) {
        self.out.push(Diagnostic { rule: self.cfg.display(rule), line, col, message });
    }

    /// Like `diag`, for a finding Checkstyle can reach twice at one spot (`true == true` reports the
    /// `==` for each operand): it reports an identical message at the same place only once.
    pub fn diag_once(&mut self, rule: &'static str, at: Node, message: String) {
        let p = at.start_position();
        let (line, col, name) = (p.row as u32 + 1, p.column as u32 + 1, self.cfg.display(rule));
        if !self.out.iter().rev().take(8).any(|d| d.line == line && d.col == col && d.rule == name && d.message == message) {
            self.diag(rule, at, message);
        }
    }
}

/// Make a Java buffer's line breaks the ones Checkstyle counts. Checkstyle ends a line at `\n`,
/// `\r\n` or a lone `\r`; tree-sitter only at `\n`, so in a file with old-Mac line endings every
/// finding would land on line 1. A lone `\r` becomes `\n` in place: the length is unchanged, so no
/// offset moves, and the Java language agrees that CR ends a line (a `//` comment stops there).
/// Form feed, vertical tab and the Unicode separators end nothing, in Checkstyle or here.
///
/// A `\r` that is the very last byte stays: nothing follows it to be put on another line, and
/// `NewlineAtEndOfFile` needs to see how the file ends.
pub fn normalize_line_breaks(src: &mut [u8]) {
    // Jump from carriage return to carriage return: none in most files, one per line in CRLF ones.
    let last = src.len().wrapping_sub(1);
    let lone: Vec<usize> = memchr::memchr_iter(b'\r', src).filter(|&i| i != last && src.get(i + 1) != Some(&b'\n')).collect();
    for i in lone {
        src[i] = b'\n';
    }
}

/// Run the text-level modules over a file's lines: what `ext` (the file's extension) is read by.
/// This is all that happens to a file that is not Java; a Java file gets it after the tree walk.
pub fn lint_text(cfg: &Config, ext: &str, src: &[u8], out: &mut Vec<Diagnostic>) {
    rules::text::run(cfg, ext, src, out);
}

pub fn lint(syn: &Syntax, cfg: &Config, tree: &Tree, src: &[u8], out: &mut Vec<Diagnostic>) {
    let mut cx = Ctx::new(syn, cfg, src, out);
    let mut st = State::default();
    let mut cursor = tree.walk();
    'outer: loop {
        let node = cursor.node();
        #[cfg(debug_assertions)]
        cx.check_against_tree_sitter(node);
        let descend = rules::visit(&mut cx, &mut st, node);
        cx.visited(node);
        if descend && cursor.goto_first_child() {
            cx.enter(node);
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break 'outer;
            }
            cx.leave();
        }
    }
    rules::finish(&mut cx, &mut st);
    lint_text(cfg, "java", src, cx.out);
    crate::suppress::apply(cfg, src, cx.out);
}
