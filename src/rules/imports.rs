//! `UnusedImports`.
//!
//! An import is used if its simple name appears as a *reference* somewhere in the file. What
//! counts as a reference was settled against Checkstyle's output, case by case: see the
//! regression tests in `rules/tests.rs`.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use crate::javadoc;
use crate::syntax::Act;
use rustc_hash::FxHashMap;
use std::ops::Range;
use tree_sitter::Node;

struct ImportRef<'a> {
    full: &'a [u8],
    line: u32,
    col: u32,
    used: bool,
    /// Next import with the same simple name (overloaded static imports), or u32::MAX.
    next: u32,
}

/// An open type body that declares a member type named like an import. Inside such a body the
/// member shadows the import; outside it (e.g. in the `extends` clause) it does not.
struct Scope<'a> {
    /// Byte offset where the declaring body ends.
    end: usize,
    names: Vec<&'a [u8]>,
}

#[derive(Default)]
pub struct State<'a> {
    imports: Vec<ImportRef<'a>>,
    wanted: FxHashMap<&'a [u8], u32>,
    package_uses: Vec<&'a [u8]>,
    scopes: Vec<Scope<'a>>,
    /// Javadoc comments seen so far, keyed by the row they end on.
    javadocs: FxHashMap<usize, Range<usize>>,
    /// Every comment seen so far, in document order (they never overlap).
    comments: Vec<Range<usize>>,
    /// Byte offset of each line start; built the first time a declaration looks for its Javadoc.
    line_starts: Vec<usize>,
    /// Names referenced by the Javadoc before the `package` statement. Imports come after it, so
    /// they are resolved in `finish`.
    pending: Vec<Vec<u8>>,
    scratch: Vec<u8>,
}

impl<'a> State<'a> {
    fn mark_used(&mut self, name: &[u8]) {
        let mut idx = self.wanted.get(name).copied().unwrap_or(u32::MAX);
        while idx != u32::MAX {
            let imp = &mut self.imports[idx as usize];
            imp.used = true;
            idx = imp.next;
        }
    }

    /// The Javadoc that Checkstyle associates with a declaration starting on `decl_row`: the one
    /// ending on the nearest line above that is not blank and not comment-only. Found by probing,
    /// so odd cases are reproduced on purpose (a Javadoc after `int a;` on the same line documents
    /// the *next* member; of two stacked Javadocs only the lower counts).
    fn javadoc_before(&mut self, src: &[u8], decl_row: usize) -> Option<Range<usize>> {
        if self.javadocs.is_empty() {
            return None;
        }
        if self.line_starts.is_empty() {
            self.line_starts.push(0);
            self.line_starts.extend(src.iter().enumerate().filter(|(_, b)| **b == b'\n').map(|(i, _)| i + 1));
        }
        let mut row = decl_row;
        while row > 0 {
            row -= 1;
            if let Some(range) = self.javadocs.get(&row) {
                return Some(range.clone());
            }
            // Skip a line that is blank or lies entirely inside one comment, whether a `//`
            // comment, a one-line block comment or a line of a multi-line one.
            let start = *self.line_starts.get(row)?;
            let end = self.line_starts.get(row + 1).copied().unwrap_or(src.len());
            let line = &src[start..end];
            let trimmed = line.trim_ascii();
            if trimmed.is_empty() {
                continue;
            }
            let a = start + (line.len() - line.trim_ascii_start().len());
            let b = a + trimmed.len();
            let i = self.comments.partition_point(|c| c.start <= a);
            if i > 0 && self.comments[i - 1].end >= b {
                continue;
            }
            return None;
        }
        None
    }
}

/// Document-order walk: leave every scope whose body ended before this node starts.
pub fn leave_scopes(st: &mut State, node: Node) {
    while st.scopes.last().is_some_and(|s| node.start_byte() >= s.end) {
        st.scopes.pop();
    }
}

pub fn on_ident<'a>(cx: &Ctx<'a>, st: &mut State<'a>, node: Node) {
    if st.wanted.is_empty() {
        return;
    }
    let name = cx.text(node);
    let Some(&first) = st.wanted.get(name) else { return };
    if st.scopes.iter().any(|s| s.names.contains(&name)) {
        return; // a member type of an enclosing body shadows the import here
    }
    if is_member_name(cx, node) {
        return; // `x.NAME` selects a member; it does not refer to the imported type
    }
    let mut idx = first;
    while idx != u32::MAX {
        let imp = &mut st.imports[idx as usize];
        imp.used = true;
        idx = imp.next;
    }
}

/// `AvoidStarImport`. A static star import is reported unless `allowStaticMemberImports`, any other
/// star import unless `allowClassImports`, and either is fine if its name is in `excludes`.
pub fn on_star_import(cx: &mut Ctx, import: Node) {
    let Some(opts) = &cx.cfg.avoid_star_import else { return };
    let (mut star, mut name, mut is_static) = (None, None, false);
    let mut cur = import.walk();
    for c in import.children(&mut cur) {
        match c.kind_s() {
            "asterisk" => star = Some(c),
            "static" => is_static = true,
            "identifier" | "scoped_identifier" => name = Some(c),
            _ => {}
        }
    }
    let (Some(star), Some(name)) = (star, name) else { return };
    if if is_static { opts.allow_static_member_imports } else { opts.allow_class_imports } {
        return;
    }
    let mut qualified = String::new();
    qualified_name(cx, name, &mut qualified);
    if opts.excludes.contains(&qualified) {
        return;
    }
    // Checkstyle reports at the dot before the star.
    let at = star.prev_sibling().unwrap_or(star);
    cx.diag("AvoidStarImport", at, format!("Using the '.*' form of import should be avoided - {qualified}.*."));
}

/// `a.b.C` written however it was spelled: spaces and comments between the parts are dropped.
fn qualified_name(cx: &Ctx, n: Node, out: &mut String) {
    if n.kind_s() == "scoped_identifier" {
        if let Some(scope) = n.child_by_field_id(cx.syn.f.scope) {
            qualified_name(cx, scope, out);
            out.push('.');
        }
        if let Some(name) = n.child_by_field_id(cx.syn.f.name) {
            out.push_str(&String::from_utf8_lossy(cx.text(name)));
        }
    } else {
        out.push_str(&String::from_utf8_lossy(cx.text(n)));
    }
}

pub fn on_import<'a>(cx: &Ctx<'a>, st: &mut State<'a>, node: Node) {
    if cx.cfg.unused_imports.is_none() {
        return;
    }
    let mut cur = node.walk();
    let mut name_node = None;
    for c in node.named_children(&mut cur) {
        match c.kind_s() {
            "asterisk" => return, // star imports are never reported
            "identifier" | "scoped_identifier" => name_node = Some(c),
            _ => {}
        }
    }
    let Some(full) = name_node else { return };
    let simple = match full.kind_s() {
        "scoped_identifier" => match full.child_by_field_id(cx.syn.f.name) {
            Some(n) => cx.text(n),
            None => return,
        },
        _ => cx.text(full),
    };
    let p = full.start_position();
    let idx = st.imports.len() as u32;
    let next = st.wanted.insert(simple, idx).unwrap_or(u32::MAX);
    st.imports.push(ImportRef { full: cx.text(full), line: p.row as u32 + 1, col: p.column as u32 + 1, used: false, next });
}

/// Identifiers used by annotations on the package declaration (package-info.java). Imports follow
/// the package declaration, so these are resolved in `finish`.
pub fn on_package<'a>(cx: &Ctx<'a>, st: &mut State<'a>, package: Node) {
    if cx.cfg.unused_imports.is_none() {
        return;
    }
    let mut cur = package.walk();
    for child in package.named_children(&mut cur) {
        if !matches!(child.kind_s(), "marker_annotation" | "annotation") {
            continue; // the package name itself is not a use
        }
        let mut walk = child.walk();
        'outer: loop {
            let n = walk.node();
            if cx.syn.act(n) == Act::Ident {
                st.package_uses.push(cx.text(n));
            }
            if walk.goto_first_child() {
                continue;
            }
            loop {
                if walk.goto_next_sibling() {
                    break;
                }
                if !walk.goto_parent() {
                    break 'outer;
                }
            }
        }
    }
}

/// Open a shadowing scope if this body declares a member type named like an import.
pub fn on_type_body<'a>(cx: &Ctx<'a>, st: &mut State<'a>, body: Node) {
    // Imports precede declarations, so `wanted` is already complete here.
    if st.wanted.is_empty() {
        return;
    }
    let mut names = Vec::new();
    let mut cur = body.walk();
    for child in body.named_children(&mut cur) {
        if cx.syn.act(child) != Act::TypeDecl {
            continue;
        }
        if let Some(n) = child.child_by_field_id(cx.syn.f.name) {
            let name = cx.text(n);
            if st.wanted.contains_key(name) {
                names.push(name);
            }
        }
    }
    if !names.is_empty() {
        st.scopes.push(Scope { end: body.end_byte(), names });
    }
}

/// Remember a comment, so that the declaration that follows can find its Javadoc.
pub fn on_comment(cx: &Ctx, st: &mut State, node: Node) {
    let Some(opts) = &cx.cfg.unused_imports else { return };
    if !opts.process_javadoc {
        return;
    }
    let range = node.byte_range();
    st.comments.push(range.clone());
    if cx.text(node).starts_with(b"/**") && range.len() > 4 {
        st.javadocs.insert(node.end_position().row, range);
    }
}

/// A declaration that can own a Javadoc comment: count the names that comment refers to.
pub fn on_documented<'a>(cx: &Ctx<'a>, st: &mut State<'a>, node: Node) {
    let Some(opts) = &cx.cfg.unused_imports else { return };
    if !opts.process_javadoc {
        return;
    }
    let is_package = node.kind_s() == "package_declaration";
    // Imports precede every declaration except the package statement, so with no import seen
    // there is nothing to mark.
    if st.wanted.is_empty() && !is_package {
        return;
    }
    let Some(range) = st.javadoc_before(cx.src, node.start_position().row) else { return };
    let mut scratch = std::mem::take(&mut st.scratch);
    javadoc::for_each_reference(&cx.src[range], &mut scratch, &mut |name| {
        if is_package {
            st.pending.push(name.to_vec());
        } else {
            st.mark_used(name);
        }
    });
    st.scratch = scratch;
}

pub fn finish<'a>(cx: &mut Ctx<'a>, st: &mut State<'a>) {
    for i in 0..st.package_uses.len() {
        let name = st.package_uses[i];
        st.mark_used(name);
    }
    for name in std::mem::take(&mut st.pending) {
        st.mark_used(&name);
    }
    for imp in &st.imports {
        if !imp.used {
            cx.diag_at("UnusedImports", imp.line, imp.col, format!("Unused import - {}.", String::from_utf8_lossy(imp.full)));
        }
    }
}

/// True if `node` names a *member* (the right-hand side of a dot, a method's own name, the target
/// of `::`) and so cannot refer to an imported type. Only the leftmost name of a qualified chain,
/// and an unqualified call (needed for static imports), count as uses. Called only after a name
/// already matched an import, so the parent lookup stays off the hot path.
fn is_member_name(cx: &Ctx, node: Node) -> bool {
    let Some(parent) = cx.parent() else { return false };
    let is = |field: u16| parent.child_by_field_id(field).is_some_and(|c| c.id() == node.id());
    match parent.kind_s() {
        "field_access" => is(cx.syn.f.field),
        "method_invocation" => is(cx.syn.f.name) && parent.child_by_field_id(cx.syn.f.object).is_some(),
        "method_declaration" => is(cx.syn.f.name),
        "scoped_type_identifier" | "scoped_identifier" => node.prev_named_sibling().is_some(),
        "method_reference" => node.prev_sibling().is_some(),
        _ => false,
    }
}
