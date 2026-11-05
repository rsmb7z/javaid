//! The single-pass tree walk.
//!
//! Every file is parsed once and walked once. Each node is dispatched through the kind-id lookup
//! table in `syntax`, so adding a rule never adds another traversal.

use crate::config::Config;
use crate::rules::{self, State};
use crate::syntax::KindExt;
use crate::syntax::Syntax;
use std::path::Path;
use tree_sitter::{Node, Tree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The Checkstyle module name, e.g. `NeedBraces`.
    pub rule: &'static str,
    pub line: u32,
    /// The column Checkstyle prints, 1-based, or 0 for a finding that has none.
    ///
    /// Checkstyle reports a finding on the syntax tree at a column that counts code points (a
    /// supplementary character is one column) and runs a tab to the next multiple of the `tabWidth` (default 8): a tab-indented line puts
    /// everything after the indentation further right than its character index. `Ctx::diag` and
    /// `Ctx::diag_at` convert from tree-sitter's byte column to that, so a rule never does. A
    /// module that builds a `Diagnostic` itself (the line-based text modules, the javadoc ones,
    /// the file-set ones) supplies the column Checkstyle prints, exactly.
    pub col: u32,
    pub message: String,
}

/// Everything a rule needs while the walk is in progress.
pub struct Ctx<'a> {
    pub syn: &'a Syntax,
    pub cfg: &'a Config,
    pub src: &'a [u8],
    pub out: &'a mut Vec<Diagnostic>,
    /// The file being linted, for the modules that read its name or directory.
    pub path: Option<&'a Path>,
    /// The ancestors of the node being visited, outermost first. Tree-sitter keeps no parent
    /// pointers: `Node::parent` searches from the root, and a sibling call searches the parent's
    /// children, which adds up when a rule asks for one of every statement. The walk knows these
    /// already, so it keeps them.
    parents: Vec<Node<'a>>,
    /// The previous sibling that is not a comment, for the node being visited (the last entry) and
    /// for each of its ancestors. Unnamed tokens such as `{` and `;` count.
    previous: Vec<Option<Node<'a>>>,
    /// Byte offsets at which each line starts, built the first time a finding needs a column.
    line_starts: std::cell::OnceCell<Vec<usize>>,
    /// The kind name of the node being visited (set by `rules::visit`).
    pub kind_name: &'static str,
    /// `Node::id` of the node being visited (`usize::MAX` outside the walk).
    pub node_id: usize,
}

impl<'a> Ctx<'a> {
    pub fn new(syn: &'a Syntax, cfg: &'a Config, src: &'a [u8], out: &'a mut Vec<Diagnostic>) -> Self {
        Ctx {
            syn,
            cfg,
            src,
            out,
            path: None,
            parents: Vec::new(),
            previous: vec![None],
            line_starts: std::cell::OnceCell::new(),
            kind_name: "",
            node_id: usize::MAX,
        }
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

    /// `node.parent()` without the search from the root that tree-sitter makes: the walk already
    /// holds the ancestors of the node being visited, and rules mostly climb from that node.
    pub fn parent_of(&self, node: Node<'a>) -> Option<Node<'a>> {
        let id = node.id();
        let found = if id == self.node_id {
            self.parents.last().copied()
        } else {
            match self.parents.iter().rposition(|p| p.id() == id) {
                Some(i) => i.checked_sub(1).map(|j| self.parents[j]),
                None => node.parent(),
            }
        };
        // In debug builds, which includes `cargo test` over every probe file, check it against tree-sitter.
        #[cfg(debug_assertions)]
        assert_eq!(found.map(|p| p.id()), node.parent().map(|p| p.id()), "parent_of disagrees with tree-sitter at {node:?}");
        found
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

    /// The column Checkstyle prints for the 1-based `byte_col` of `line` (0 stays 0, "no column").
    pub fn column(&self, line: u32, byte_col: u32) -> u32 {
        self.column_of(line, byte_col, self.cfg.walker_tab())
    }

    /// The 1-based position in code points, a tab counting as one: the "column N" some
    /// messages quote, which Checkstyle does not expand.
    pub fn char_column(&self, line: u32, byte_col: u32) -> u32 {
        self.column_of(line, byte_col, 1)
    }

    fn column_of(&self, line: u32, byte_col: u32, tab_width: usize) -> u32 {
        if byte_col == 0 {
            return 0;
        }
        let starts = self.line_starts.get_or_init(|| std::iter::once(0).chain(memchr::memchr_iter(b'\n', self.src).map(|i| i + 1)).collect());
        let Some(&start) = starts.get((line as usize).wrapping_sub(1)) else { return byte_col };
        let end = (start + byte_col as usize - 1).min(self.src.len());
        expanded_column(&self.src[start.min(end)..end], tab_width)
    }

    pub fn diag(&mut self, rule: &'static str, at: Node, message: String) {
        let p = anchor(at).start_position();
        self.diag_at(rule, p.row as u32 + 1, p.column as u32 + 1, message);
    }

    /// A finding at a position that is not a node's: a declaration's reported line, say. `col` is
    /// a 1-based byte column, converted like `diag`'s (0 means the finding has no column).
    pub fn diag_at(&mut self, rule: &'static str, line: u32, col: u32, message: String) {
        let col = self.column(line, col);
        self.out.push(Diagnostic { rule: self.cfg.display(rule), line, col, message });
    }

    /// Like `diag`, for a rule that Checkstyle reports at the node's very first token: a declaration
    /// is then not moved to its type's `.` (see `anchor`).
    pub fn diag_start(&mut self, rule: &'static str, at: Node, message: String) {
        let p = at.start_position();
        self.diag_at(rule, p.row as u32 + 1, p.column as u32 + 1, message);
    }

    /// Like `diag_at`, but `col` is already the column Checkstyle prints.
    pub fn diag_exact(&mut self, rule: &'static str, line: u32, col: u32, message: String) {
        self.out.push(Diagnostic { rule: self.cfg.display(rule), line, col, message });
    }

    /// Like `diag`, for a finding Checkstyle can reach twice at one spot (`true == true` reports the
    /// `==` for each operand): it reports an identical message at the same place only once.
    pub fn diag_once(&mut self, rule: &'static str, at: Node, message: String) {
        let p = anchor(at).start_position();
        let (line, name) = (p.row as u32 + 1, self.cfg.display(rule));
        let col = self.column(line, p.column as u32 + 1);
        if !self.out.iter().rev().take(8).any(|d| d.line == line && d.col == col && d.rule == name && d.message == message) {
            self.diag(rule, at, message);
        }
    }
}

/// Where Checkstyle puts a finding reported on `node`. Almost always the node's own start; but a
/// declaration with no modifiers, annotations or type parameters has an empty `MODIFIERS` node
/// that has no position, and Checkstyle takes the position of the first child that has one: the
/// type. A qualified type (`Map.Entry`, `java.util.List<T>`, `String[]` of those) is a `DOT` node,
/// and a `DOT` sits at its own `.`, the last one of the name.
fn anchor(node: Node) -> Node {
    if !matches!(
        node.kind_s(),
        "method_declaration"
            | "field_declaration"
            | "constant_declaration"
            | "local_variable_declaration"
            | "formal_parameter"
            | "catch_formal_parameter"
            | "annotation_type_element_declaration"
            | "resource"
    ) {
        return node;
    }
    if node.child(0).is_some_and(|c| matches!(c.kind_s(), "modifiers" | "type_parameters")) {
        return node;
    }
    let Some(mut ty) = node.child_by_field_name("type") else { return node };
    loop {
        match ty.kind_s() {
            "array_type" => match ty.child_by_field_name("element") {
                Some(e) => ty = e,
                None => return node,
            },
            "generic_type" => match ty.named_child(0) {
                Some(e) => ty = e,
                None => return node,
            },
            _ => break,
        }
    }
    if ty.kind_s() != "scoped_type_identifier" {
        return node;
    }
    let mut cur = ty.walk();
    let dot = ty.children(&mut cur).filter(|c| c.kind_s() == ".").last();
    dot.unwrap_or(node)
}

pub use crate::columns::expanded_column;

/// An integer as `MessageFormat` prints it in the default (US) locale, `{0,number,integer}` and a
/// bare `{0}` holding an `Integer` alike: groups of three digits separated by commas.
pub fn grouped(n: impl std::fmt::Display) -> String {
    let digits = n.to_string();
    let (sign, digits) = digits.strip_prefix('-').map_or(("", digits.as_str()), |d| ("-", d));
    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    out.push_str(sign);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
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
    let from = out.len();
    rules::text::run(cfg, ext, src, out);
    crate::messages::apply(cfg, out, from);
}

pub fn lint(syn: &Syntax, cfg: &Config, tree: &Tree, src: &[u8], out: &mut Vec<Diagnostic>) {
    lint_file(syn, cfg, tree, src, None, out);
}

/// `lint`, for a file with a path: `PackageDeclaration` and `OuterTypeFilename` read it.
pub fn lint_file(syn: &Syntax, cfg: &Config, tree: &Tree, src: &[u8], path: Option<&Path>, out: &mut Vec<Diagnostic>) {
    lint_file_cached(syn, cfg, &crate::csast::CsCache::new(tree, src), path, out);
}

/// `lint_file` with the file's emulated-AST cache supplied, so the caller can go on sharing it with the
/// suppression filters (`suppress::by_file_cached`) that run afterwards.
pub fn lint_file_cached(syn: &Syntax, cfg: &Config, cache: &crate::csast::CsCache, path: Option<&Path>, out: &mut Vec<Diagnostic>) {
    let (tree, src) = (cache.tree().expect("lint_file_cached needs the parse tree"), cache.src());
    let from = out.len();
    let mut cx = Ctx::new(syn, cfg, src, out);
    cx.path = path;
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
    cx.node_id = usize::MAX;
    rules::finish(&mut cx, &mut st);
    rules::indentation::check(&mut cx, cache);
    rules::csast_checks::run(cfg, cache, cx.out);
    rules::xpathfilter::match_xpath(&mut cx, cache);
    lint_text(cfg, "java", src, cx.out);
    rules::cs::run(cfg, cache, path, cx.out);
    crate::messages::apply(cfg, cx.out, from);
}
