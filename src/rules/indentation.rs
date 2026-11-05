// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/indentation/IndentationCheck.java and the handler classes in checks/indentation/
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `Indentation`: a port of Checkstyle's `IndentationCheck` and its handlers, running on the
//! Checkstyle AST emulation in `csast`.
//!
//! Checkstyle visits every handled token with a stack of handlers (one per construct); each handler
//! knows the indentation its construct expects and checks the lines of its children. The Java
//! classes form a hierarchy (`BlockParentHandler` and its subclasses); here a handler is a `kind`
//! and every virtual method is a `match` over the kinds, with the `super` calls written out.

use crate::config::Indentation;
use crate::csast::Tok;
use crate::csast::Tok::*;
use crate::csast::{CsAst, N, NIL};
use crate::engine::Ctx;
use std::collections::{BTreeMap, HashMap, HashSet};

const MSG_ERROR: u8 = 0;
const MSG_ERROR_MULTI: u8 = 1;
const MSG_CHILD_ERROR: u8 = 2;
const MSG_CHILD_ERROR_MULTI: u8 = 3;

// ---------------------------------------------------------------------------------------------
// IndentLevel
// ---------------------------------------------------------------------------------------------

/// `IndentLevel`: a set of acceptable columns. Negative columns cannot be stored (a `BitSet`
/// refuses them) and are dropped.
#[derive(Clone, Debug)]
struct IL(Vec<i32>);

impl IL {
    fn one(i: i32) -> IL {
        IL(if i >= 0 { vec![i] } else { vec![] })
    }

    /// `new IndentLevel(base, offsets...)`.
    fn off(base: &IL, offs: &[i32]) -> IL {
        let mut v = Vec::new();
        for &b in &base.0 {
            for &o in offs {
                if b + o >= 0 {
                    v.push(b + o);
                }
            }
        }
        v.sort_unstable();
        v.dedup();
        IL(v)
    }

    /// `IndentLevel.addAcceptable(base, additions...)`.
    fn add(base: &IL, adds: &[i32]) -> IL {
        let mut v = base.0.clone();
        v.extend(adds.iter().copied().filter(|&a| a >= 0));
        v.sort_unstable();
        v.dedup();
        IL(v)
    }

    fn add_il(base: &IL, other: &IL) -> IL {
        let mut v = base.0.clone();
        v.extend(other.0.iter().copied());
        v.sort_unstable();
        v.dedup();
        IL(v)
    }

    fn is_multi(&self) -> bool {
        self.0.len() > 1
    }

    fn acceptable(&self, i: i32) -> bool {
        self.0.binary_search(&i).is_ok()
    }

    fn greater_than(&self, i: i32) -> bool {
        self.first() > i
    }

    fn first(&self) -> i32 {
        self.0.first().copied().unwrap_or(-1)
    }

    fn last(&self) -> i32 {
        self.0.last().copied().unwrap_or(-1)
    }

    fn show(&self) -> String {
        self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
    }
}

/// An `int` argument as `MessageFormat` prints it (digits grouped by thousands).
fn fmt_int(n: i32) -> String {
    let s = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 { format!("-{out}") } else { out }
}

/// `Character.isWhitespace` for a UTF-16 unit.
fn java_ws(c: u32) -> bool {
    matches!(c, 9..=13 | 0x1C..=0x1F | 0x20 | 0x1680 | 0x2000..=0x2006 | 0x2008..=0x200A | 0x2028 | 0x2029 | 0x205F | 0x3000)
}

// ---------------------------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum K {
    Primordial,
    Case,
    Switch,
    Slist,
    PackageDef,
    Else,
    If,
    Try,
    Catch,
    Finally,
    DoWhile,
    While,
    For,
    MethodDef,
    ClassDef,
    ObjBlock,
    Import,
    ArrayInit,
    AnnotationArrayInit,
    MethodCall,
    Label,
    StaticInit,
    MemberDef,
    New,
    Index,
    Synchronized,
    Lambda,
    SwitchRule,
    Yield,
}

/// Whether the Java class extends `BlockParentHandler`.
fn is_block_parent(k: K) -> bool {
    matches!(
        k,
        K::Slist
            | K::ObjBlock
            | K::ClassDef
            | K::MethodDef
            | K::ArrayInit
            | K::AnnotationArrayInit
            | K::Switch
            | K::If
            | K::Else
            | K::Try
            | K::Catch
            | K::Finally
            | K::DoWhile
            | K::While
            | K::For
            | K::Synchronized
            | K::StaticInit
    )
}

fn kind_for(t: Tok) -> Option<K> {
    Some(match t {
        CASE_GROUP => K::Case,
        LITERAL_SWITCH => K::Switch,
        SLIST | INSTANCE_INIT => K::Slist,
        PACKAGE_DEF => K::PackageDef,
        LITERAL_ELSE => K::Else,
        LITERAL_IF => K::If,
        LITERAL_TRY => K::Try,
        LITERAL_CATCH => K::Catch,
        LITERAL_FINALLY => K::Finally,
        LITERAL_DO => K::DoWhile,
        LITERAL_WHILE => K::While,
        LITERAL_FOR => K::For,
        METHOD_DEF | CTOR_DEF | ANNOTATION_FIELD_DEF | COMPACT_CTOR_DEF => K::MethodDef,
        CLASS_DEF | ENUM_DEF | INTERFACE_DEF | ANNOTATION_DEF | RECORD_DEF => K::ClassDef,
        OBJBLOCK => K::ObjBlock,
        IMPORT | MODULE_IMPORT => K::Import,
        ARRAY_INIT => K::ArrayInit,
        ANNOTATION_ARRAY_INIT => K::AnnotationArrayInit,
        METHOD_CALL | CTOR_CALL | SUPER_CTOR_CALL => K::MethodCall,
        LABELED_STAT => K::Label,
        STATIC_INIT => K::StaticInit,
        VARIABLE_DEF => K::MemberDef,
        LITERAL_NEW => K::New,
        INDEX_OP => K::Index,
        LITERAL_SYNCHRONIZED => K::Synchronized,
        LAMBDA => K::Lambda,
        SWITCH_RULE => K::SwitchRule,
        LITERAL_YIELD => K::Yield,
        _ => return None,
    })
}

fn handler_name(k: K, t: Tok) -> &'static str {
    match k {
        K::Primordial => "",
        K::Case | K::SwitchRule => "case",
        K::Switch => "switch",
        K::Slist => "block",
        K::PackageDef => "package def",
        K::Else => "else",
        K::If => "if",
        K::Try => "try",
        K::Catch => "catch",
        K::Finally => "finally",
        K::DoWhile => "do..while",
        K::While => "while",
        K::For => "for",
        K::MethodDef => match t {
            CTOR_DEF => "ctor def",
            ANNOTATION_FIELD_DEF => "annotation field def",
            COMPACT_CTOR_DEF => "compact ctor def",
            _ => "method def",
        },
        K::ClassDef => match t {
            CLASS_DEF => "class def",
            ENUM_DEF => "enum def",
            ANNOTATION_DEF => "annotation def",
            RECORD_DEF => "record def",
            _ => "interface def",
        },
        K::ObjBlock => "object def",
        K::Import => "import",
        K::ArrayInit => "array initialization",
        K::AnnotationArrayInit => "annotation array initialization",
        K::MethodCall => "method call",
        K::Label => "label",
        K::StaticInit => "static initialization",
        K::MemberDef => "member def",
        K::New => "new",
        K::Index => "index op",
        K::Synchronized => "synchronized",
        K::Lambda => "lambda",
        K::Yield => "yield",
    }
}

struct H {
    kind: K,
    ast: N,
    parent: Option<usize>,
    name: &'static str,
    indent: Option<IL>,
    /// `LambdaHandler.isLambdaCorrectlyIndented`.
    lambda_ok: bool,
    /// `SynchronizedHandler.methodModifier`.
    method_modifier: bool,
}

/// `DetailAstSet`: the first AST node found on each line.
#[derive(Default)]
struct AstSet {
    lines: BTreeMap<i32, N>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Wrap {
    IgnoreFirstLine,
    None,
}

const CHECKED_CHILDREN: &[Tok] =
    &[VARIABLE_DEF, EXPR, ANNOTATION, OBJBLOCK, LITERAL_BREAK, LITERAL_RETURN, LITERAL_THROW, LITERAL_CONTINUE, CTOR_CALL, SUPER_CTOR_CALL];
const CLASS_CHECKED_CHILDREN: &[Tok] = &[EXPR, OBJBLOCK, LITERAL_BREAK, LITERAL_RETURN, LITERAL_THROW, LITERAL_CONTINUE];
const CASE_CHILDREN: &[Tok] = &[LITERAL_CASE, LITERAL_DEFAULT];
const LINE_WRAP_NEW_PARENT_TYPES: &[Tok] = &[ASSIGN, LITERAL_RETURN, LITERAL_THROW];
const LINE_WRAPPING_INDENT_TRIGGERS: &[Tok] = &[ASSIGN, SWITCH_RULE, LAMBDA];
const IGNORED_LIST: &[Tok] = &[LCURLY, RCURLY, LITERAL_NEW, LITERAL_YIELD, ARRAY_INIT, LITERAL_DEFAULT, LITERAL_CASE];
const SLIST_PARENT_TYPES: &[Tok] = &[
    CTOR_DEF,
    METHOD_DEF,
    STATIC_INIT,
    LITERAL_SYNCHRONIZED,
    LITERAL_IF,
    LITERAL_WHILE,
    LITERAL_DO,
    LITERAL_FOR,
    LITERAL_ELSE,
    LITERAL_TRY,
    LITERAL_CATCH,
    LITERAL_FINALLY,
    COMPACT_CTOR_DEF,
];

struct Ind<'a> {
    a: &'a CsAst,
    lines: Vec<Vec<u16>>,
    tab: i32,
    o: &'a Indentation,
    hs: Vec<H>,
    created: HashMap<N, usize>,
    stack: Vec<usize>,
    logged: HashSet<i32>,
    out: Vec<(u32, u32, String)>,
    /// A Java `NullPointerException` would have been thrown.
    npe: std::cell::Cell<bool>,
}

pub fn check(cx: &mut Ctx, cache: &crate::csast::CsCache) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.indentation else { return };
    let Some(ast) = cache.get(false) else { return };
    if ast.root == NIL {
        return;
    }
    let lines = split_lines(cx.src);
    let mut ind = Ind {
        a: ast,
        lines,
        tab: cx.cfg.tab_width as i32,
        o: opts,
        hs: Vec::new(),
        created: HashMap::new(),
        stack: Vec::new(),
        logged: HashSet::new(),
        out: Vec::new(),
        npe: std::cell::Cell::new(false),
    };
    ind.run();
    if ind.npe.get() {
        return;
    }
    let found = std::mem::take(&mut ind.out);
    for (line, col, msg) in found {
        cx.diag_exact("Indentation", line, col, msg);
    }
}

/// The file's lines as Java strings (UTF-16 units), split the way `FileText` does.
fn split_lines(src: &[u8]) -> Vec<Vec<u16>> {
    let text = String::from_utf8_lossy(src);
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '\n' => lines.push(std::mem::take(&mut cur)),
            '\r' => {
                if it.peek() == Some(&'\n') {
                    it.next();
                }
                lines.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    lines.push(cur);
    lines.into_iter().map(|l| l.encode_utf16().collect()).collect()
}

impl<'a> Ind<'a> {
    // ---- AST access tolerant of `null` ----

    fn ty(&self, n: N) -> Tok {
        if n == NIL { COMPILATION_UNIT } else { self.a.ty(n) }
    }
    fn is(&self, n: N, t: Tok) -> bool {
        n != NIL && self.a.ty(n) == t
    }
    fn is_one_of(&self, n: N, ts: &[Tok]) -> bool {
        n != NIL && ts.contains(&self.a.ty(n))
    }
    fn fc(&self, n: N) -> N {
        if n == NIL { NIL } else { self.a.first_child(n).unwrap_or(NIL) }
    }
    fn lc(&self, n: N) -> N {
        if n == NIL { NIL } else { self.a.last_child(n).unwrap_or(NIL) }
    }
    fn ns(&self, n: N) -> N {
        if n == NIL { NIL } else { self.a.next_sibling(n).unwrap_or(NIL) }
    }
    fn ps(&self, n: N) -> N {
        if n == NIL { NIL } else { self.a.prev_sibling(n).unwrap_or(NIL) }
    }
    fn par(&self, n: N) -> N {
        if n == NIL { NIL } else { self.a.parent(n).unwrap_or(NIL) }
    }
    fn fft(&self, n: N, t: Tok) -> N {
        if n == NIL { NIL } else { self.a.find_first_token(n, t).unwrap_or(NIL) }
    }
    fn line_no(&self, n: N) -> i32 {
        if n == NIL { 0 } else { self.a.line_no(n) }
    }
    fn col_no(&self, n: N) -> i32 {
        if n == NIL { 0 } else { self.a.column_no(n) }
    }
    /// `TokenUtil.areOnSameLine`: Java throws a `NullPointerException` for a missing node (a
    /// constructor reference `X::new` at the start of a line reaches it), which aborts Checkstyle's
    /// run on the file; javaid records it and reports nothing for the file.
    fn same_line(&self, a: N, b: N) -> bool {
        if a == NIL || b == NIL {
            self.npe.set(true);
        }
        self.line_no(a) == self.line_no(b)
    }
    fn has_children(&self, n: N) -> bool {
        self.fc(n) != NIL
    }

    // ---- lines ----

    fn get_line(&self, idx: i32) -> &[u16] {
        if idx >= 0 && (idx as usize) < self.lines.len() { &self.lines[idx as usize] } else { &[] }
    }

    fn len_expanded(&self, line: &[u16], to: i32) -> i32 {
        let mut len = 0;
        for i in 0..to.max(0) as usize {
            if line.get(i) == Some(&9) {
                len = crate::columns::tab_stop(len as usize, self.tab as usize) as i32;
            } else {
                len += 1;
            }
        }
        len
    }

    /// `expandedTabsColumnNo`.
    fn ecol(&self, n: N) -> i32 {
        let line = self.get_line(self.line_no(n) - 1);
        self.len_expanded(line, self.col_no(n))
    }

    /// `getLineStart(String)`.
    fn line_start_str(&self, line: &[u16]) -> i32 {
        let mut idx = 0;
        while idx < line.len() && java_ws(line[idx] as u32) {
            idx += 1;
        }
        self.len_expanded(line, idx as i32)
    }

    /// `getLineStart(int lineNo)`.
    fn line_start_no(&self, line_no: i32) -> i32 {
        self.line_start_str(self.get_line(line_no - 1))
    }

    /// `getLineStart(DetailAST)`.
    fn line_start(&self, n: N) -> i32 {
        self.line_start_no(self.line_no(n))
    }

    fn on_start(&self, n: N) -> bool {
        self.line_start(n) == self.ecol(n)
    }

    // ---- logging ----

    fn log_ast(&mut self, n: N, msg: String) {
        let l = self.line_no(n);
        if self.logged.insert(l) {
            let line = self.get_line(l - 1);
            let col = 1 + self.len_expanded(line, self.col_no(n));
            self.out.push((l as u32, col as u32, msg));
        }
    }

    fn msg(key: u8, name: &str, actual: i32, expected: &str) -> String {
        match key {
            MSG_ERROR => format!("'{name}' has incorrect indentation level {}, expected level should be {expected}.", fmt_int(actual)),
            MSG_ERROR_MULTI => {
                format!("'{name}' has incorrect indentation level {}, expected level should be one of the following: {expected}.", fmt_int(actual))
            }
            MSG_CHILD_ERROR => format!("'{name}' child has incorrect indentation level {}, expected level should be {expected}.", fmt_int(actual)),
            _ => format!("'{name}' child has incorrect indentation level {}, expected level should be one of the following: {expected}.", fmt_int(actual)),
        }
    }

    fn log_error_with(&mut self, h: usize, n: N, subtype: &str, actual: i32, expected: &IL) {
        let name = self.hs[h].name;
        let type_str = if subtype.is_empty() { name.to_string() } else { format!("{name} {subtype}") };
        let key = if expected.is_multi() { MSG_ERROR_MULTI } else { MSG_ERROR };
        let m = Self::msg(key, &type_str, actual, &expected.show());
        self.log_ast(n, m);
    }

    fn log_error(&mut self, h: usize, n: N, subtype: &str, actual: i32) {
        let ind = self.indent(h);
        self.log_error_with(h, n, subtype, actual, &ind);
    }

    fn log_child_error(&mut self, h: usize, n: N, actual: i32, expected: &IL) {
        let key = if expected.is_multi() { MSG_CHILD_ERROR_MULTI } else { MSG_CHILD_ERROR };
        let m = Self::msg(key, self.hs[h].name, actual, &expected.show());
        self.log_ast(n, m);
    }

    // ---- driver ----

    fn run(&mut self) {
        self.hs.push(H { kind: K::Primordial, ast: NIL, parent: None, name: "", indent: None, lambda_ok: true, method_modifier: false });
        self.stack.push(0);
        // Iterative pre-order walk; only handled token types are visited.
        let mut cur = self.a.root;
        let mut entered: Vec<bool> = Vec::new();
        let a = self.a;
        'walk: loop {
            // enter `cur`
            let handled = kind_for(a.ty(cur)).is_some();
            if handled {
                let top = *self.stack.last().unwrap();
                let h = self.get_handler(cur, top);
                self.stack.push(h);
                self.check_indentation(h);
            }
            entered.push(handled);
            if let Some(c) = a.first_child(cur) {
                cur = c;
                continue;
            }
            // leave upwards
            loop {
                if entered.pop() == Some(true) {
                    self.stack.pop();
                }
                if let Some(s) = a.next_sibling(cur) {
                    cur = s;
                    continue 'walk;
                }
                match a.parent(cur) {
                    Some(p) => cur = p,
                    None => break 'walk,
                }
            }
        }
    }

    // ---- HandlerFactory ----

    fn new_handler(&mut self, ast: N, parent: usize) -> usize {
        let t = self.ty(ast);
        let kind = kind_for(t).unwrap_or(K::Index);
        let method_modifier = kind == K::Synchronized && self.ty(self.par(ast)) == MODIFIERS;
        self.hs.push(H { kind, ast, parent: Some(parent), name: handler_name(kind, t), indent: None, lambda_ok: true, method_modifier });
        self.hs.len() - 1
    }

    fn get_handler(&mut self, ast: N, parent: usize) -> usize {
        if let Some(&h) = self.created.get(&ast) {
            return h;
        }
        if self.ty(ast) == METHOD_CALL {
            let mut node = self.fc(ast);
            while self.is(node, DOT) {
                node = self.fc(node);
            }
            let mut the_parent = parent;
            if node != NIL && kind_for(self.ty(node)).is_some() {
                the_parent = self.get_handler(node, the_parent);
                self.created.insert(node, the_parent);
            }
            let h = self.new_handler(ast, the_parent);
            return h;
        }
        self.new_handler(ast, parent)
    }

    // ---- AbstractExpressionHandler ----

    fn main(&self, h: usize) -> N {
        self.hs[h].ast
    }

    fn parent_h(&self, h: usize) -> Option<usize> {
        self.hs[h].parent
    }

    fn kind(&self, h: usize) -> K {
        self.hs[h].kind
    }

    fn parent_kind(&self, h: usize) -> Option<K> {
        self.hs[h].parent.map(|p| self.hs[p].kind)
    }

    fn indent(&mut self, h: usize) -> IL {
        if let Some(i) = &self.hs[h].indent {
            return i.clone();
        }
        let i = self.indent_impl(h);
        self.hs[h].indent = Some(i.clone());
        i
    }

    /// `AbstractExpressionHandler.getIndentImpl`: ask the parent.
    fn indent_super(&mut self, h: usize) -> IL {
        match self.parent_h(h) {
            Some(p) => self.suggested_child_indent(p, h),
            None => IL::one(0),
        }
    }

    fn basic(&self) -> i32 {
        self.o.basic_offset
    }

    fn lw(&self) -> i32 {
        self.o.line_wrapping_indentation
    }

    fn indent_impl(&mut self, h: usize) -> IL {
        let main = self.main(h);
        match self.kind(h) {
            K::Primordial => IL::one(0),
            K::ObjBlock => {
                let parent_ast = self.par(main);
                let p = self.parent_h(h).unwrap();
                let mut indent = self.indent(p);
                if self.is(parent_ast, LITERAL_NEW) {
                    let s = self.indent_super(h);
                    indent = IL::add_il(&indent, &s);
                } else if self.is(parent_ast, ENUM_CONSTANT_DEF) {
                    indent = self.indent_super(h);
                }
                indent
            }
            K::ArrayInit => {
                let parent_ast = self.par(main);
                let t = self.ty(parent_ast);
                if t == LITERAL_NEW || t == ASSIGN {
                    IL::one(self.line_start(parent_ast))
                } else {
                    match self.parent_h(h) {
                        Some(p) if is_block_parent(self.kind(p)) => self.children_expected_indent(p),
                        Some(p) => self.indent(p),
                        None => IL::one(0),
                    }
                }
            }
            K::AnnotationArrayInit => IL::one(self.line_start(self.par(main))),
            K::Switch => {
                let mut level = self.indent_super(h);
                if self.on_start(main) {
                    let parent = self.par(main);
                    let grand = self.par(parent);
                    if self.should_indent_due_to_wrapping(h, parent, grand) {
                        level = IL::off(&level, &[self.lw()]);
                    }
                }
                level
            }
            K::If => {
                if self.is_if_after_else(h) {
                    let p = self.parent_h(h).unwrap();
                    self.indent(p)
                } else {
                    self.indent_super(h)
                }
            }
            K::MethodCall => self.method_call_indent_impl(h),
            K::Lambda => self.lambda_indent_impl(h),
            K::New => self.new_indent_impl(h),
            K::Case | K::SwitchRule => {
                let p = self.parent_h(h).unwrap();
                let pi = self.indent(p);
                IL::off(&pi, &[self.o.case_indent])
            }
            K::Label => {
                let s = self.indent_super(h);
                let level = IL::off(&s, &[-self.basic()]);
                let s2 = self.indent_super(h);
                IL::add_il(&level, &s2)
            }
            _ => self.indent_super(h),
        }
    }

    fn suggested_child_indent(&mut self, h: usize, child: usize) -> IL {
        match self.kind(h) {
            K::Primordial => self.indent_impl(h),
            K::Slist => {
                let pk = self.parent_kind(h);
                let child_slist = self.kind(child) == K::Slist;
                if (pk.is_some_and(is_block_parent) && pk != Some(K::Slist)) || (child_slist && pk == Some(K::Case)) {
                    let p = self.parent_h(h).unwrap();
                    self.suggested_child_indent(p, child)
                } else {
                    self.children_expected_indent(h)
                }
            }
            K::MemberDef | K::Case => self.indent(h),
            K::MethodCall => self.method_call_suggested(h, child),
            K::Lambda => self.lambda_suggested(h, child),
            K::New => {
                let offset = if self.ty(self.main(child)) == OBJBLOCK { self.basic() } else { self.lw() };
                let i = self.indent(h);
                IL::off(&i, &[offset])
            }
            K::SwitchRule => {
                let i = self.indent(h);
                if self.kind(child) == K::Slist || self.sr_same_line_as_switch(h, self.main(child)) { i } else { IL::off(&i, &[self.lw()]) }
            }
            K::Yield => {
                if self.same_line(self.main(h), self.main(child)) {
                    self.indent(h)
                } else {
                    let i = self.indent(h);
                    IL::off(&i, &[self.basic()])
                }
            }
            K::If => {
                if self.kind(child) == K::Else {
                    self.indent(h)
                } else {
                    self.children_expected_indent(h)
                }
            }
            K::Try => {
                let ck = self.kind(child);
                if ck == K::Catch || ck == K::Finally || (ck == K::New && self.is_try_resource_spec(child)) {
                    self.indent(h)
                } else {
                    self.children_expected_indent(h)
                }
            }
            k if is_block_parent(k) => self.children_expected_indent(h),
            _ => {
                let i = self.indent(h);
                IL::off(&i, &[self.basic()])
            }
        }
    }

    fn should_increase_indent(&self, h: usize) -> bool {
        match self.kind(h) {
            K::MethodCall | K::New => false,
            _ => {
                let main = self.main(h);
                if self.is(main, LITERAL_CATCH) {
                    let param = self.fft(main, PARAMETER_DEF);
                    !self.contains_annotation(param)
                } else {
                    true
                }
            }
        }
    }

    fn contains_annotation(&self, ast: N) -> bool {
        let holder = if self.is_one_of(ast, &[ENUM_CONSTANT_DEF, PACKAGE_DEF, MODULE_DEF]) { self.fft(ast, ANNOTATIONS) } else { self.fft(ast, MODIFIERS) };
        holder != NIL && self.fft(holder, ANNOTATION) != NIL
    }

    fn check_lines_indent(&mut self, h: usize, set: &AstSet, level: &IL, first_line_matches: bool, first_line: i32, allow_nesting: bool) {
        if set.lines.is_empty() {
            return;
        }
        let start_ast = set.first_line();
        let mut start_col = self.ecol(start_ast);
        let real_start = self.line_start_no(self.line_no(start_ast));
        if first_line_matches && !allow_nesting {
            start_col = real_start;
        }
        if real_start == start_col {
            self.check_line_indent(h, start_ast, level, first_line_matches);
        }
        self.check_remaining_lines(h, first_line_matches, level, first_line, set);
    }

    fn check_remaining_lines(&mut self, h: usize, first_line_matches: bool, indent_level: &IL, first_line: i32, set: &AstSet) {
        let start_ast = set.first_line();
        let end_line = set.last_line();
        let mut level = indent_level.clone();
        if self.should_increase_indent(h) && self.ty(start_ast) != ANNOTATION && (first_line_matches || first_line > self.line_no(self.main(h))) {
            level = IL::off(indent_level, &[self.lw()]);
        }
        let from = self.line_no(start_ast) + 1;
        if from > end_line {
            return;
        }
        let asts: Vec<N> = set.lines.range(from..=end_line).map(|(_, &n)| n).collect();
        for ast in asts {
            let aligned = self.ty(ast) == TEXT_BLOCK_LITERAL_END && self.on_start(self.par(ast)) && indent_level.acceptable(self.ecol(ast));
            if !aligned {
                self.check_line_indent(h, ast, &level, false);
            }
        }
    }

    fn check_line_indent(&mut self, h: usize, ast: N, level: &IL, must_match: bool) {
        let start = self.line_start_no(self.line_no(ast));
        let col = self.ecol(ast);
        if (must_match && !level.acceptable(start)) || (!must_match && col == start && level.greater_than(start)) {
            self.log_child_error(h, ast, start, level);
        }
    }

    fn check_wrapping(&mut self, first: N, last: N) {
        let lw = self.lw();
        self.check_wrapping_full(first, last, lw, -1, Wrap::IgnoreFirstLine);
    }

    fn check_children(&mut self, h: usize, parent_node: N, types: &[Tok], start_indent: &IL, first_line_matches: bool, allow_nesting: bool) {
        let mut child = self.fc(parent_node);
        while child != NIL {
            if types.contains(&self.ty(child)) && self.should_check_indentation_for_child(h, child) {
                self.check_expression_subtree(h, child, start_indent, first_line_matches, allow_nesting);
            }
            child = self.ns(child);
        }
    }

    fn should_check_indentation_for_child(&self, h: usize, child: N) -> bool {
        match self.kind(h) {
            K::Try | K::Catch | K::Finally => {
                let left_curly = self.left_curly(h);
                self.line_no(self.first_ast_node(child)) != self.line_no(left_curly)
            }
            _ => true,
        }
    }

    fn check_expression_subtree(&mut self, h: usize, tree: N, level: &IL, first_line_matches: bool, allow_nesting: bool) {
        let mut set = AstSet::default();
        let first_line = self.line_no(self.first_ast_node(tree));
        if first_line_matches && !allow_nesting {
            let first = self.first_ast_node(tree);
            set.add(self, first);
        }
        self.find_subtree_ast(&mut set, tree);
        self.check_lines_indent(h, &set, level, first_line_matches, first_line, allow_nesting);
    }

    /// `getFirstAstNode`: the leftmost token of the subtree on its first line.
    fn first_ast_node(&self, ast: N) -> N {
        let mut real = ast;
        for n in self.preorder(ast) {
            let (nl, rl) = (self.line_no(n), self.line_no(real));
            if nl < rl || (nl == rl && self.col_no(n).min(self.col_no(real)) == self.col_no(n)) {
                real = n;
            }
        }
        real
    }

    /// The nodes of the subtree, parents first.
    fn preorder(&self, root: N) -> Vec<N> {
        let mut v = Vec::new();
        if root == NIL {
            return v;
        }
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            v.push(n);
            let kids: Vec<N> = self.a.children(n).collect();
            for k in kids.into_iter().rev() {
                stack.push(k);
            }
        }
        v
    }

    /// `AbstractExpressionHandler.getFirstToken`.
    fn first_token(&self, ast: N) -> N {
        let mut first = ast;
        for n in self.preorder(ast) {
            // the Java recursion tests each subtree's own minimum against `first`; the minimum of
            // each subtree is the earliest strictly smaller node in pre-order, so a flat pass agrees
            if self.col_no(n) < self.col_no(first) {
                first = n;
            }
        }
        first
    }

    fn find_subtree_ast(&self, set: &mut AstSet, tree: N) {
        let mut stack = vec![tree];
        while let Some(t) = stack.pop() {
            if kind_for(self.ty(t)).is_some() {
                continue;
            }
            let t_ty = self.ty(t);
            let is_record_delim = (t_ty == LPAREN || t_ty == RPAREN) && self.ty(self.par(t)) == RECORD_PATTERN_DEF;
            if !is_record_delim {
                let line = self.line_no(t);
                let col_num = set.start_column(self, line);
                let this_col = self.ecol(t);
                if col_num.is_none() || this_col < col_num.unwrap() {
                    set.add(self, t);
                }
            }
            let kids: Vec<N> = self.a.children(t).collect();
            for k in kids.into_iter().rev() {
                stack.push(k);
            }
        }
    }

    fn check_right_paren(&mut self, h: usize, lparen: N, rparen: N) {
        if rparen != NIL {
            let rl = self.ecol(rparen);
            let ll = self.ecol(lparen);
            let ind = self.indent(h);
            if rl != ll + 1 && !ind.acceptable(rl) && self.on_start(rparen) {
                self.log_error(h, rparen, "rparen", rl);
            }
        }
    }

    fn check_left_paren(&mut self, h: usize, lparen: N) {
        if lparen != NIL {
            let ind = self.indent(h);
            let c = self.ecol(lparen);
            if !ind.acceptable(c) && self.on_start(lparen) {
                self.log_error(h, lparen, "lparen", c);
            }
        }
    }

    // ---- BlockParentHandler ----

    fn top_level_ast(&self, h: usize) -> N {
        match self.kind(h) {
            K::Slist | K::ObjBlock | K::ClassDef | K::MethodDef | K::ArrayInit | K::AnnotationArrayInit => NIL,
            _ => self.main(h),
        }
    }

    fn left_curly(&self, h: usize) -> N {
        let main = self.main(h);
        match self.kind(h) {
            K::Slist | K::ArrayInit | K::AnnotationArrayInit => main,
            K::ObjBlock | K::Switch => self.fft(main, LCURLY),
            K::ClassDef => self.fft(self.fft(main, OBJBLOCK), LCURLY),
            _ => self.fft(main, SLIST),
        }
    }

    fn right_curly(&self, h: usize) -> N {
        let main = self.main(h);
        match self.kind(h) {
            K::Slist | K::ObjBlock | K::ArrayInit | K::AnnotationArrayInit | K::Switch => self.fft(main, RCURLY),
            K::ClassDef => self.fft(self.fft(main, OBJBLOCK), RCURLY),
            _ => self.fft(self.fft(main, SLIST), RCURLY),
        }
    }

    fn list_child(&self, h: usize) -> N {
        let main = self.main(h);
        match self.kind(h) {
            K::Slist | K::ObjBlock | K::ArrayInit | K::AnnotationArrayInit => main,
            K::ClassDef => self.fft(main, OBJBLOCK),
            K::Switch => NIL,
            _ => self.fft(main, SLIST),
        }
    }

    fn non_list_child(&self, h: usize) -> N {
        let main = self.main(h);
        match self.kind(h) {
            K::Switch => NIL,
            K::Else | K::DoWhile => self.fc(main),
            _ => self.ns(self.fft(main, RPAREN)),
        }
    }

    fn has_curlies(&self, h: usize) -> bool {
        self.left_curly(h) != NIL && self.right_curly(h) != NIL
    }

    fn checked_children(&self, h: usize) -> &'static [Tok] {
        if self.kind(h) == K::ClassDef { CLASS_CHECKED_CHILDREN } else { CHECKED_CHILDREN }
    }

    fn can_children_be_nested(&self, h: usize) -> bool {
        matches!(self.kind(h), K::ArrayInit | K::AnnotationArrayInit)
    }

    fn curly_indent(&mut self, h: usize) -> IL {
        match self.kind(h) {
            K::ObjBlock => {
                let indent = self.bp_curly_indent(h);
                let f = indent.first();
                IL::add(&indent, &[f + self.lw()])
            }
            K::ArrayInit => {
                let mut offset = 0;
                let lcurly = self.left_curly(h);
                if self.on_start(lcurly) && self.ty(self.par(lcurly)) != ARRAY_INIT {
                    offset = self.o.brace_adjustment;
                }
                let ind = self.indent(h);
                let level = IL::off(&ind, &[offset]);
                let l = level.last();
                IL::add(&level, &[l + self.lw()])
            }
            K::AnnotationArrayInit => {
                let mut offset = 0;
                let lcurly = self.left_curly(h);
                if self.on_start(lcurly) {
                    offset = self.o.brace_adjustment;
                }
                let ind = self.indent(h);
                let level = IL::off(&ind, &[offset]);
                let l = level.last();
                IL::add(&level, &[l + self.lw()])
            }
            _ => self.bp_curly_indent(h),
        }
    }

    fn bp_curly_indent(&mut self, h: usize) -> IL {
        let lcurly = self.left_curly(h);
        let ind = self.indent(h);
        let mut exp = IL::off(&ind, &[self.o.brace_adjustment]);
        if !self.on_start(lcurly) || self.check_if_code_block(h) {
            exp = IL::off(&ind, &[0]);
        }
        exp
    }

    fn check_if_code_block(&self, h: usize) -> bool {
        let Some(p) = self.parent_h(h) else { return false };
        self.is(self.main(h), SLIST) && is_block_parent(self.kind(p)) && self.parent_kind(p).is_some_and(is_block_parent)
    }

    fn children_expected_indent(&mut self, h: usize) -> IL {
        match self.kind(h) {
            K::ArrayInit => {
                let mut expected = {
                    let ind = self.indent(h);
                    IL::off(&ind, &[self.o.array_init_indent, self.lw()])
                };
                let first_line = self.line_no(self.first_ast_node(self.list_child(h)));
                let lcurly_pos = self.ecol(self.left_curly(h));
                let first_child_pos = self.next_first_non_blank_codepoints(first_line, lcurly_pos);
                if first_child_pos != -1 {
                    expected = IL::add(&expected, &[first_child_pos, lcurly_pos + self.lw()]);
                }
                expected
            }
            K::AnnotationArrayInit => {
                let mut expected = {
                    let ind = self.indent(h);
                    IL::off(&ind, &[self.o.array_init_indent, self.lw()])
                };
                let left_curly = self.left_curly(h);
                let first_line = self.line_no(self.first_ast_node(self.list_child(h)));
                let lcurly_pos = self.ecol(left_curly);
                let first_child_pos = self.next_first_non_blank_chars(first_line, lcurly_pos);
                if first_child_pos == -1 && lcurly_pos == self.line_start(left_curly) {
                    expected = IL::add(&expected, &[lcurly_pos + self.lw()]);
                }
                if first_child_pos != -1 {
                    expected = IL::add(&expected, &[first_child_pos, lcurly_pos + self.lw()]);
                }
                expected
            }
            _ => {
                let ind = self.indent(h);
                let mut level = IL::off(&ind, &[self.basic()]);
                let has = self.has_curlies(h);
                if ind.is_multi() && has {
                    let left = self.left_curly(h);
                    if self.on_start(left) {
                        level = IL::one(self.ecol(left) + self.basic());
                    } else if self.on_start(self.right_curly(h)) {
                        let ci = self.curly_indent(h);
                        let lv = IL::off(&ci, &[self.basic()]);
                        level = IL::add(&lv, &[lv.first() + self.lw()]);
                    }
                }
                if has && self.on_start(self.left_curly(h)) {
                    let ci = self.curly_indent(h);
                    level = IL::add(&level, &[ci.first() + self.basic()]);
                }
                level
            }
        }
    }

    /// `ArrayInitHandler.getNextFirstNonBlankOnLineAfter`: indexes the line by code point.
    fn next_first_non_blank_codepoints(&self, line_no: i32, col: i32) -> i32 {
        let line = self.get_line(line_no - 1);
        let cps: Vec<u32> = char::decode_utf16(line.iter().copied()).map(|r| r.map(|c| c as u32).unwrap_or(0xFFFD)).collect();
        let len = cps.len() as i32;
        let mut real = col + 1;
        while real < len && java_ws(cps[real as usize]) {
            real += 1;
        }
        if real == len { -1 } else { real }
    }

    /// `AnnotationArrayInitHandler.getNextFirstNonBlankOnLineAfter`: indexes by char.
    fn next_first_non_blank_chars(&self, line_no: i32, col: i32) -> i32 {
        let line = self.get_line(line_no - 1);
        let len = line.len() as i32;
        let mut real = col + 1;
        while real < len && java_ws(line[real as usize] as u32) {
            real += 1;
        }
        if real == len { -1 } else { real }
    }

    fn bp_check_top_level_token(&mut self, h: usize) {
        let top = self.top_level_ast(h);
        if top != NIL {
            let ind = self.indent(h);
            let c = self.ecol(top);
            if !ind.acceptable(c) && self.on_start(top) {
                self.log_error(h, top, "", c);
            }
        }
    }

    fn check_top_level_token(&mut self, h: usize) {
        match self.kind(h) {
            K::If => {
                if !self.is_if_after_else(h) {
                    self.bp_check_top_level_token(h);
                }
            }
            K::Else => {
                let if_ast = self.par(self.main(h));
                let slist = self.fft(if_ast, SLIST);
                if slist == NIL {
                    self.bp_check_top_level_token(h);
                } else {
                    let lcurly = self.lc(slist);
                    if !self.same_line(lcurly, self.main(h)) {
                        self.bp_check_top_level_token(h);
                    }
                }
            }
            _ => self.bp_check_top_level_token(h),
        }
    }

    fn check_left_curly(&mut self, h: usize) {
        let lcurly = self.left_curly(h);
        let pos = self.ecol(lcurly);
        let ci = self.curly_indent(h);
        if !ci.acceptable(pos) && self.on_start(lcurly) {
            self.log_error_with(h, lcurly, "lcurly", pos, &ci);
        }
    }

    fn check_right_curly(&mut self, h: usize) {
        let rcurly = self.right_curly(h);
        let pos = self.ecol(rcurly);
        let ci = self.curly_indent(h);
        if !ci.acceptable(pos) && self.on_start(rcurly) {
            self.log_error_with(h, rcurly, "rcurly", pos, &ci);
        }
    }

    fn check_non_list_child(&mut self, h: usize) {
        let non_list = self.non_list_child(h);
        if non_list != NIL {
            let ind = self.indent(h);
            let expected = IL::off(&ind, &[self.basic()]);
            self.check_expression_subtree(h, non_list, &expected, false, false);
            let start = self.first_ast_node(non_list);
            if non_list != start {
                self.check_expression_subtree(h, start, &expected, false, false);
            }
        }
    }

    fn should_check_left_paren(&self, h: usize, lparen: N) -> bool {
        if self.kind(h) == K::MethodDef {
            let rp = self.fft(self.main(h), RPAREN);
            return !self.method_paren_own_line(lparen, rp);
        }
        true
    }

    fn should_check_right_paren(&self, h: usize, lparen: N, rparen: N) -> bool {
        if self.kind(h) == K::MethodDef {
            return !self.method_paren_own_line(lparen, rparen);
        }
        true
    }

    fn method_paren_own_line(&self, lparen: N, rparen: N) -> bool {
        rparen != NIL && !self.same_line(lparen, rparen) && self.on_start(lparen) && self.on_start(rparen) && self.ecol(lparen) == self.ecol(rparen)
    }

    /// `BlockParentHandler.checkIndentation`.
    fn bp_check(&mut self, h: usize) {
        self.check_top_level_token(h);
        let main = self.main(h);
        let lparen = self.fft(main, LPAREN);
        if self.should_check_left_paren(h, lparen) {
            self.check_left_paren(h, lparen);
        }
        let rparen = self.fft(main, RPAREN);
        if self.should_check_right_paren(h, lparen, rparen) {
            self.check_right_paren(h, lparen, rparen);
        }
        if self.has_curlies(h) {
            self.check_left_curly(h);
            self.check_right_curly(h);
        }
        let list = self.list_child(h);
        if list == NIL {
            self.check_non_list_child(h);
        } else if !self.has_curlies(h) || !self.same_line(self.left_curly(h), self.right_curly(h)) {
            let types = self.checked_children(h);
            let exp = self.children_expected_indent(h);
            let nested = self.can_children_be_nested(h);
            self.check_children(h, list, types, &exp, true, nested);
        }
    }

    // ---- dispatch of checkIndentation ----

    fn check_indentation(&mut self, h: usize) {
        let main = self.main(h);
        match self.kind(h) {
            K::Primordial | K::Index => {}
            K::Slist => {
                let ptype = self.ty(self.par(main));
                let has_block_parent = SLIST_PARENT_TYPES.contains(&ptype);
                let parent_node = self.par(main);
                let same_line_case_group = self.is(parent_node, CASE_GROUP) && self.same_line(main, parent_node);
                if !has_block_parent && !same_line_case_group {
                    self.bp_check(h);
                }
            }
            K::ObjBlock => {
                if self.is(self.par(main), LITERAL_NEW) {
                    self.bp_check(h);
                }
            }
            K::ClassDef => self.class_def_check(h),
            K::MethodDef => self.method_def_check(h),
            K::If => {
                self.bp_check(h);
                let cond = self.ns(self.fft(main, LPAREN));
                let ind = self.indent(h);
                let expected = IL::off(&ind, &[self.basic()]);
                self.check_expression_subtree(h, cond, &expected, false, false);
                let rp = self.fft(main, RPAREN);
                self.check_wrapping(main, rp);
            }
            K::Else | K::Finally | K::StaticInit | K::ArrayInit | K::AnnotationArrayInit => self.bp_check(h),
            K::Switch => {
                let expr = self.ns(self.fft(main, LPAREN));
                let ind = self.indent(h);
                self.check_expression_subtree(h, expr, &ind, false, false);
                self.bp_check(h);
            }
            K::Try => {
                self.bp_check(h);
                if self.ty(self.fc(main)) == RESOURCE_SPECIFICATION {
                    let lp = self.fc(self.fc(main));
                    self.check_try_res_paren(h, lp, "lparen");
                    let rp = self.lc(self.fc(main));
                    self.check_try_res_paren(h, rp, "rparen");
                    self.check_try_resources(h, self.fc(main));
                }
            }
            K::Catch => {
                self.bp_check(h);
                let cond = self.ns(self.fft(main, LPAREN));
                let mut level = self.indent(h);
                if self.on_start(cond) {
                    level = IL::off(&level, &[self.lw()]);
                }
                self.check_expression_subtree(h, cond, &level, true, true);
            }
            K::DoWhile => {
                self.bp_check(h);
                let while_ast = self.fft(main, DO_WHILE);
                let ind = self.indent(h);
                if self.on_start(while_ast) && !ind.acceptable(self.ecol(while_ast)) {
                    let c = self.ecol(while_ast);
                    self.log_error(h, while_ast, "while", c);
                }
                let cond = self.ns(self.fft(main, LPAREN));
                self.check_expression_subtree(h, cond, &ind, false, false);
            }
            K::While => {
                let cond = self.fft(main, EXPR);
                let ind = self.indent(h);
                let expected = IL::off(&ind, &[self.basic()]);
                self.check_expression_subtree(h, cond, &expected, false, false);
                self.bp_check(h);
            }
            K::For => {
                let ind = self.indent(h);
                let expected = IL::off(&ind, &[self.basic()]);
                let init = self.fft(main, FOR_INIT);
                if init == NIL {
                    let each = self.fft(main, FOR_EACH_CLAUSE);
                    self.check_expression_subtree(h, each, &expected, false, false);
                } else {
                    self.check_expression_subtree(h, init, &expected, false, false);
                    let cond = self.fft(main, FOR_CONDITION);
                    self.check_expression_subtree(h, cond, &expected, false, false);
                    let it = self.fft(main, FOR_ITERATOR);
                    self.check_expression_subtree(h, it, &expected, false, false);
                }
                self.bp_check(h);
                let rp = self.fft(main, RPAREN);
                self.check_wrapping(main, rp);
            }
            K::Synchronized => {
                if !self.hs[h].method_modifier {
                    self.bp_check(h);
                    let sync = self.ns(self.fft(main, LPAREN));
                    let ind = self.indent(h);
                    let expected = IL::off(&ind, &[self.basic()]);
                    self.check_expression_subtree(h, sync, &expected, false, false);
                    let rp = self.fft(main, RPAREN);
                    self.check_wrapping(main, rp);
                }
            }
            K::Case => {
                let parent_node = self.par(main);
                if !self.same_line(main, parent_node) {
                    let ind = self.indent(h);
                    self.check_children(h, main, CASE_CHILDREN, &ind, true, false);
                }
            }
            K::SwitchRule => {
                if !self.sr_same_line_as_switch(h, self.par(main)) {
                    let ind = self.indent(h);
                    self.check_children(h, main, CASE_CHILDREN, &ind, true, false);
                }
            }
            K::Yield => self.yield_check(h),
            K::Label => {
                let ind = self.indent(h);
                self.check_children(h, main, &[IDENT], &ind, true, false);
                let parent = self.ns(self.fc(main));
                let expected = IL::off(&ind, &[self.basic()]);
                self.check_expression_subtree(h, parent, &expected, true, false);
            }
            K::PackageDef | K::Import => {
                let c = self.ecol(main);
                let ind = self.indent(h);
                if !ind.acceptable(c) && self.on_start(main) {
                    self.log_error(h, main, "", c);
                }
                let semi = self.fft(main, SEMI);
                self.check_wrapping(main, semi);
            }
            K::MemberDef => self.member_def_check(h),
            K::MethodCall => self.method_call_check(h),
            K::New => self.new_check(h),
            K::Lambda => self.lambda_check(h),
        }
    }

    // ---- ClassDefHandler ----

    fn class_def_check(&mut self, h: usize) {
        let main = self.main(h);
        let modifiers = self.fft(main, MODIFIERS);
        if self.has_children(modifiers) {
            if self.is(main, ANNOTATION_DEF) {
                let mut m = self.fc(modifiers);
                while m != NIL {
                    self.check_modifier(h, m);
                    m = self.ns(m);
                }
            } else {
                let first_modifier = self.fc(modifiers);
                let mut first_non_ann = first_modifier;
                let mut m = first_modifier;
                while m != NIL {
                    if self.ty(m) != ANNOTATION {
                        first_non_ann = m;
                        break;
                    }
                    m = self.ns(m);
                }
                let first_line = self.line_no(first_modifier);
                let first_non_line = self.line_no(first_non_ann);
                let mut m = first_modifier;
                while m != NIL && self.line_no(m) == first_line {
                    self.check_modifier(h, m);
                    m = self.ns(m);
                }
                if first_line != first_non_line {
                    let mut m = first_non_ann;
                    while m != NIL {
                        self.check_modifier(h, m);
                        m = self.ns(m);
                    }
                }
            }
        } else if !self.is(main, ANNOTATION_DEF) {
            let ident = self.fft(main, IDENT);
            let mut token = main;
            if self.line_no(ident) == self.line_no(main) {
                token = ident;
            }
            let ls = self.line_start(token);
            if !self.indent(h).acceptable(ls) {
                self.log_error(h, token, "ident", ls);
            }
        }
        let list = self.list_child(h);
        if self.is(main, ANNOTATION_DEF) {
            let at = self.fft(main, AT);
            if self.on_start(at) {
                let start = self.indent(h).first();
                self.check_wrapping_full(main, list, 0, start, Wrap::None);
            }
        } else {
            self.check_wrapping(main, list);
        }
        self.bp_check(h);
    }

    fn check_modifier(&mut self, h: usize, m: N) {
        if self.on_start(m) && !self.indent(h).acceptable(self.ecol(m)) {
            let c = self.ecol(m);
            self.log_error(h, m, "modifier", c);
        }
    }

    // ---- MethodDefHandler ----

    fn method_def_check(&mut self, h: usize) {
        let main = self.main(h);
        // checkModifiers
        let modifier = self.fft(main, MODIFIERS);
        if self.on_start(modifier) && !self.indent(h).acceptable(self.ecol(modifier)) {
            let c = self.ecol(modifier);
            self.log_error(h, modifier, "modifier", c);
        }
        // checkThrows
        let throws_ast = self.fft(main, LITERAL_THROWS);
        if throws_ast != NIL {
            let ignore = if self.on_start(throws_ast) { Wrap::None } else { Wrap::IgnoreFirstLine };
            let start = self.line_start_no(self.method_def_line_start(main));
            let next = self.ns(throws_ast);
            self.check_wrapping_full(throws_ast, next, self.o.throws_indent, start, ignore);
        }
        let rp = self.fft(main, RPAREN);
        if rp != NIL {
            self.check_wrapping(main, rp);
        }
        if self.left_curly(h) != NIL {
            self.bp_check(h);
        }
    }

    fn method_def_line_start(&self, main: N) -> i32 {
        let mut line_start = self.line_no(self.fft(main, IDENT));
        let type_node = self.fft(main, TYPE);
        if type_node != NIL {
            line_start = self.line_no(self.first_ast_node(type_node));
        }
        let mut node = self.fc(self.fft(main, MODIFIERS));
        while node != NIL {
            if self.ty(node) != ANNOTATION {
                line_start = line_start.min(self.line_no(node));
            }
            node = self.ns(node);
        }
        line_start
    }

    // ---- MemberDefHandler ----

    fn member_def_check(&mut self, h: usize) {
        let main = self.main(h);
        let modifiers = self.fft(main, MODIFIERS);
        if self.has_children(modifiers) {
            if self.on_start(modifiers) && !self.indent(h).acceptable(self.ecol(modifiers)) {
                let c = self.ecol(modifiers);
                self.log_error(h, modifiers, "modifier", c);
            }
        } else {
            let ty = self.fft(main, TYPE);
            let ident = self.first_token(ty);
            let c = self.ecol(ident);
            if self.on_start(ident) && !self.indent(h).acceptable(c) {
                self.log_error(h, ident, "type", c);
            }
        }
        // array init node, or the statement's semicolon
        let assign = self.fft(main, ASSIGN);
        let mut array_init = NIL;
        if assign != NIL {
            let expr = self.fft(assign, EXPR);
            let mut node = assign;
            if expr != NIL {
                let new = self.fft(expr, LITERAL_NEW);
                if new != NIL {
                    node = new;
                }
            }
            array_init = self.fft(node, ARRAY_INIT);
        }
        let last = if array_init != NIL {
            array_init
        } else {
            let mut last = self.lc(main);
            if self.ty(last) != SEMI {
                last = self.ns(main);
            }
            last
        };
        if last != NIL {
            self.check_wrapping(main, last);
        }
    }

    // ---- Switch ----

    fn should_indent_due_to_wrapping(&self, h: usize, direct_parent: N, grand: N) -> bool {
        let enclosing = self.parent_h(h).map(|p| self.main(p)).unwrap_or(NIL);
        let is_direct = direct_parent == enclosing;
        let for_direct = !is_direct && self.is_one_of(direct_parent, LINE_WRAPPING_INDENT_TRIGGERS);
        let for_grand = !is_direct && grand != enclosing && self.is_one_of(grand, LINE_WRAPPING_INDENT_TRIGGERS);
        for_direct || for_grand
    }

    fn sr_same_line_as_switch(&self, h: usize, node: N) -> bool {
        self.is(node, LITERAL_SWITCH) && self.same_line(self.main(h), node)
    }

    // ---- If ----

    fn is_if_after_else(&self, h: usize) -> bool {
        let main = self.main(h);
        let parent = self.par(main);
        self.is(parent, LITERAL_ELSE) && self.same_line(parent, main)
    }

    // ---- Try ----

    fn is_try_resource_spec(&self, expr: usize) -> bool {
        let mut ast = self.main(expr);
        while ast != NIL && self.ty(ast) != LITERAL_TRY {
            if self.ty(ast) == RESOURCE_SPECIFICATION {
                return true;
            }
            ast = self.par(ast);
        }
        false
    }

    fn check_try_res_paren(&mut self, h: usize, paren: N, sub: &str) {
        if self.on_start(paren) {
            let ind = self.indent(h);
            let expected = IL::off(&ind, &[0, self.lw()]);
            self.check_child_indentation(h, paren, sub, &expected);
        }
    }

    fn check_child_indentation(&mut self, h: usize, ast: N, sub: &str, expected: &IL) {
        let c = self.ecol(ast);
        if self.o.force_strict_condition {
            if !expected.acceptable(c) {
                self.log_error_with(h, ast, sub, c, expected);
            }
        } else if c < expected.first() {
            self.log_error_with(h, ast, sub, c, expected);
        }
    }

    fn check_try_resources(&mut self, h: usize, spec: N) {
        let resources = self.fft(spec, RESOURCES);
        let indentation = self.indent(h).first() + self.lw();
        let expected = IL::one(indentation);
        let mut res = self.fc(resources);
        while res != NIL {
            if self.ty(res) == RESOURCE {
                let next = if self.ns(res) == NIL { self.lc(spec) } else { self.ns(res) };
                if self.on_start(res) {
                    self.check_child_indentation(h, res, "resource", &expected);
                    self.check_wrapping_full(res, next, self.lw(), expected.first(), Wrap::IgnoreFirstLine);
                } else {
                    self.check_wrapping(res, next);
                }
            }
            res = self.ns(res);
        }
    }

    // ---- Yield ----

    fn yield_check(&mut self, h: usize) {
        let main = self.main(h);
        let c = self.ecol(main);
        if self.on_start(main) && !self.indent(h).acceptable(c) {
            self.log_error(h, main, "", c);
        }
        let expr = self.fc(main);
        if !self.same_line(main, expr) {
            let ind = self.indent(h);
            self.check_expression_subtree(h, expr, &ind, false, false);
        }
    }

    // ---- MethodCallHandler ----

    fn method_call_indent_impl(&mut self, h: usize) -> IL {
        let main = self.main(h);
        if self.parent_kind(h) == Some(K::MethodCall) {
            let container = self.parent_h(h).unwrap();
            let cmain = self.main(container);
            if self.same_line(cmain, main) || self.is_chained_method_call_wrapped(main) || self.methods_chained(cmain, main) {
                self.indent(container)
            } else {
                let ci = self.indent(container);
                IL::off(&ci, &[self.lw()])
            }
        } else if self.ty(self.fc(main)) == LITERAL_NEW {
            self.indent_super(h)
        } else {
            let mut set = AstSet::default();
            self.find_subtree_ast(&mut set, self.fc(main));
            let first_col = self.ecol(set.first_line());
            let line_start = self.line_start(self.mc_first_ast(main));
            if line_start == first_col { self.indent_super(h) } else { IL::one(line_start) }
        }
    }

    fn methods_chained(&self, ast1: N, ast2: N) -> bool {
        let rparen = self.fft(ast1, RPAREN);
        self.same_line(rparen, ast2)
    }

    fn is_chained_method_call_wrapped(&self, main: N) -> bool {
        let dot = self.fc(main);
        let target = self.fc(dot);
        let dot1 = self.fc(target);
        let mut target1 = dot1;
        while self.fc(target1) != NIL && self.ty(target1) != METHOD_CALL {
            target1 = self.fc(target1);
        }
        self.ty(dot1) == DOT && self.ty(target1) == METHOD_CALL
    }

    fn mc_first_ast(&self, ast: N) -> N {
        let mut node = self.fc(ast);
        while self.is(node, DOT) {
            node = self.fc(node);
        }
        node
    }

    fn method_ident_ast(&self, h: usize) -> N {
        let mut ast = self.main(h);
        if self.ty(ast) != SUPER_CTOR_CALL {
            ast = self.fc(ast);
            if self.is(ast, DOT) {
                ast = self.lc(ast);
            }
        }
        ast
    }

    fn method_call_suggested(&mut self, h: usize, child: usize) -> IL {
        let ident = self.method_ident_ast(h);
        let rparen = self.fft(self.main(h), RPAREN);
        let mut suggested = IL::one(self.line_start(ident));
        if !self.same_line(self.fc(self.main(child)), ident) && !self.is_invocation_target(h, child) {
            suggested = IL::off(&suggested, &[self.basic(), self.lw()]);
        }
        if self.line_start(rparen) == self.col_no(rparen) {
            let p = self.parent_h(h).unwrap();
            let ps = self.suggested_child_indent(p, h);
            let extra = IL::off(&ps, &[self.lw()]);
            suggested = IL::add_il(&suggested, &extra);
        }
        suggested
    }

    fn is_invocation_target(&self, h: usize, child: usize) -> bool {
        let mut crossed = false;
        let mut node = self.par(self.main(child));
        while node != NIL && node != self.main(h) {
            if self.ty(node) == ELIST {
                crossed = true;
                break;
            }
            node = self.par(node);
        }
        !crossed
    }

    fn method_call_check(&mut self, h: usize) {
        let main = self.main(h);
        let mut lparen = NIL;
        if self.ty(main) == METHOD_CALL {
            let expr = self.par(main);
            if self.ty(self.par(expr)) == SLIST {
                let ind = self.indent(h);
                self.check_expression_subtree(h, self.fc(main), &ind, false, false);
                lparen = main;
            }
        } else {
            lparen = self.fc(main);
        }
        if lparen != NIL {
            let rparen = self.fft(main, RPAREN);
            self.check_left_paren(h, lparen);
            if !self.same_line(rparen, lparen) {
                let elist = self.fft(main, ELIST);
                let ind = self.indent(h);
                let exp = IL::off(&ind, &[self.basic()]);
                self.check_expression_subtree(h, elist, &exp, false, true);
                self.mc_check_rparen(h, lparen, rparen);
                let last = self.lc(main);
                self.check_wrapping(main, last);
            }
        } else if self.is_simple_return_method_call(main) {
            self.check_return_statement_call_arguments(h);
        }
    }

    fn is_simple_return_method_call(&self, main: N) -> bool {
        let expr = self.par(main);
        self.ty(self.fc(main)) != DOT && self.ty(self.par(expr)) == LITERAL_RETURN
    }

    fn check_return_statement_call_arguments(&mut self, h: usize) {
        let main = self.main(h);
        let rparen = self.fft(main, RPAREN);
        if !self.same_line(rparen, main) {
            let elist = self.fft(main, ELIST);
            let ind = self.indent(h);
            let arg_indent = IL::off(&ind, &[self.basic()]);
            let mut arg = self.fc(elist);
            while arg != NIL {
                if self.ty(arg) == EXPR {
                    let first = self.first_ast_node(arg);
                    if self.ty(first) != LITERAL_NEW && self.on_start(first) {
                        let actual = self.ecol(first);
                        if arg_indent.greater_than(actual) {
                            self.log_child_error(h, first, actual, &arg_indent);
                        }
                    }
                }
                arg = self.ns(arg);
            }
        }
    }

    fn mc_check_rparen(&mut self, h: usize, lparen: N, rparen: N) {
        let rl = self.ecol(rparen);
        let ll = self.ecol(lparen);
        let standard = self.indent(h);
        let mut enhanced = standard.clone();
        if self.parent_kind(h) == Some(K::MethodCall) {
            let line_start = self.line_start(self.mc_first_ast(self.main(h)));
            if line_start != standard.first() {
                enhanced = IL::add_il(&standard, &IL::one(line_start));
            }
        }
        if rl != ll + 1 && !enhanced.acceptable(rl) && self.on_start(rparen) {
            self.log_error(h, rparen, "rparen", rl);
        }
    }

    // ---- NewHandler ----

    fn new_indent_impl(&mut self, h: usize) -> IL {
        let main = self.main(h);
        if self.on_start(main) {
            let mut result = self.indent_super(h);
            let is_line_wrapped = self.is_one_of(self.par(self.par(main)), LINE_WRAP_NEW_PARENT_TYPES);
            let ternary = self.ternary_nesting_level(main);
            if is_line_wrapped || self.does_new_need_line_wrapping_indent(main) {
                result = IL::off(&result, &[self.lw()]);
            }
            if ternary >= 2 {
                for _ in 1..ternary {
                    result = IL::off(&result, &[self.lw()]);
                }
            }
            result
        } else {
            IL::one(self.line_start(main))
        }
    }

    fn does_new_need_line_wrapping_indent(&self, main: N) -> bool {
        let mut ast = self.par(main);
        while self.is_one_of(ast, &[DOT, METHOD_CALL, EXPR]) {
            ast = self.par(ast);
        }
        self.is_one_of(ast, LINE_WRAP_NEW_PARENT_TYPES) || (self.ty(ast) == QUESTION && self.is_parent_assign_return_or_throw(ast))
    }

    fn is_parent_assign_return_or_throw(&self, ast: N) -> bool {
        let mut parent = self.par(ast);
        while self.is_one_of(parent, &[EXPR, QUESTION]) {
            parent = self.par(parent);
        }
        self.is_one_of(parent, LINE_WRAP_NEW_PARENT_TYPES)
    }

    fn ternary_nesting_level(&self, main: N) -> i32 {
        let mut ast = self.par(main);
        while self.is_one_of(ast, &[DOT, METHOD_CALL, EXPR]) {
            ast = self.par(ast);
        }
        let mut level = 0;
        while self.ty(ast) == QUESTION {
            level += 1;
            loop {
                ast = self.par(ast);
                if self.ty(ast) != EXPR {
                    break;
                }
            }
        }
        if self.is_one_of(ast, LINE_WRAP_NEW_PARENT_TYPES) { level } else { 0 }
    }

    fn new_check(&mut self, h: usize) {
        let main = self.main(h);
        if self.on_start(main) {
            let c = self.ecol(main);
            let level = self.new_indent_impl(h);
            let strict = self.o.force_strict_condition;
            if (strict && !level.acceptable(c)) || (!strict && level.greater_than(c)) {
                self.log_error_with(h, main, "", c, &level);
            }
        }
        let first_child = self.fc(main);
        if first_child != NIL {
            let ind = self.indent(h);
            self.check_expression_subtree(h, first_child, &ind, false, false);
        }
        let expression = self.fft(main, ELIST);
        if self.check_nested_new(expression) && self.on_start(expression) {
            let ind = self.indent(h);
            let level = IL::off(&ind, &[self.lw()]);
            self.check_expression_subtree(h, expression, &level, false, false);
        }
        let lparen = self.fft(main, LPAREN);
        self.check_left_paren(h, lparen);
    }

    fn check_nested_new(&self, expression: N) -> bool {
        expression != NIL && self.fc(expression) != NIL && self.fft(self.fc(expression), LITERAL_NEW) == NIL
    }

    // ---- LambdaHandler ----

    fn lambda_suggested(&mut self, h: usize, child: usize) -> IL {
        let mut child_indent = self.indent(h);
        if self.hs[h].lambda_ok {
            let main = self.main(h);
            child_indent = IL::add(&child_indent, &[self.line_start(main)]);
            let cmain = self.main(child);
            if (self.is(cmain, LITERAL_SWITCH) && self.same_line(main, cmain)) || self.kind(child) == K::Slist {
                child_indent = IL::add(&child_indent, &[self.line_start(self.fc(main))]);
            } else {
                child_indent = IL::off(&child_indent, &[self.lw()]);
            }
        }
        child_indent
    }

    fn lambda_indent_impl(&mut self, h: usize) -> IL {
        let main = self.main(h);
        if self.parent_kind(h) == Some(K::MethodCall) {
            let p = self.parent_h(h).unwrap();
            return self.suggested_child_indent(p, h);
        }
        let mut enum_const = NIL;
        let mut parent = self.par(main);
        while parent != NIL {
            if self.ty(parent) == ENUM_CONSTANT_DEF {
                enum_const = parent;
                break;
            }
            parent = self.par(parent);
        }
        if enum_const != NIL {
            let base = IL::one(self.line_start(enum_const));
            IL::off(&base, &[self.lw()])
        } else {
            let mut parent = self.par(main);
            if self.parent_kind(h) == Some(K::New) {
                parent = self.par(parent);
            }
            let mut level = IL::one(self.line_start(parent));
            let first_child = self.fc(main);
            if self.line_start(first_child) == self.ecol(first_child) {
                level = IL::off(&level, &[self.lw()]);
            }
            level
        }
    }

    fn lambda_non_acceptable(&self, col: i32, level: &IL) -> bool {
        col < level.first() || (self.o.force_strict_condition && !level.acceptable(col))
    }

    fn lambda_check(&mut self, h: usize) {
        let main = self.main(h);
        let first_child = self.fc(main);
        let is_switch_rule = first_child == NIL;
        if !is_switch_rule && self.line_start(first_child) == self.ecol(first_child) {
            let c = self.ecol(first_child);
            let level = self.indent(h);
            if self.lambda_non_acceptable(c, &level) {
                self.hs[h].lambda_ok = false;
                self.log_error_with(h, first_child, "arguments", c, &level);
            }
        }
        let main_col = self.ecol(main);
        let is_wrapped = main_col == self.line_start(main);
        if is_wrapped {
            // checkLineWrappedLambda
            let level = if is_switch_rule {
                let prev = self.ps(main);
                let pls = self.line_start(prev);
                IL::off(&IL::one(pls), &[self.lw()])
            } else {
                let ind = self.indent(h);
                IL::off(&ind, &[self.lw()])
            };
            if self.lambda_non_acceptable(main_col, &level) {
                self.hs[h].lambda_ok = false;
                self.log_error_with(h, main, "", main_col, &level);
            }
        }
        let next = self.ns(main);
        if is_switch_rule && self.ty(next) == EXPR && !self.same_line(main, next) {
            // checkSingleStatementSwitchRuleIndentation
            let p = self.parent_h(h).unwrap();
            let mut level = self.suggested_child_indent(p, h);
            if is_wrapped {
                level = IL::off(&level, &[self.lw()]);
            }
            let first_line_matches = self.line_no(self.first_ast_node(next)) != self.line_no(main);
            self.check_expression_subtree(h, next, &level, first_line_matches, false);
        } else if !is_switch_rule {
            // checkSingleExpressionBodyIndentation
            let body = self.lc(main);
            let in_return = self.ty(self.par(main)) == LITERAL_RETURN;
            let body_next_line = self.ty(body) != SLIST && !self.same_line(main, body);
            if in_return && body_next_line {
                let first = self.first_ast_node(body);
                let own = kind_for(self.ty(first)).is_some();
                if !own {
                    let c = self.ecol(first);
                    let ind = self.indent(h);
                    let body_indent = IL::off(&ind, &[self.lw()]);
                    if self.lambda_non_acceptable(c, &body_indent) {
                        self.log_error_with(h, first, "", c, &body_indent);
                    }
                }
            }
        }
    }

    // ---- LineWrappingHandler ----

    fn check_wrapping_full(&mut self, first: N, last: N, indent_level: i32, start_indent: i32, ignore_first: Wrap) {
        let mut nodes = self.collect_first_nodes(first, last);
        let Some((&first_key, &first_line_node)) = nodes.iter().next() else { return };
        if self.ty(first_line_node) == AT {
            self.check_for_annotation_indentation(&mut nodes, indent_level);
        }
        if ignore_first == Wrap::IgnoreFirstLine {
            if let Some((&k, _)) = nodes.iter().next() {
                nodes.remove(&k);
            }
        }
        let _ = first_key;
        let first_node_indent = if start_indent == -1 { self.line_start(first_line_node) } else { start_indent };
        let current_indent = first_node_indent + indent_level;
        let vals: Vec<N> = nodes.values().copied().collect();
        for node in vals {
            self.check_node_indentation(node, first_node_indent, current_indent);
        }
    }

    fn check_node_indentation(&mut self, node: N, first_node_indent: i32, current_indent: i32) {
        let null_param = self.fc(node) == NIL && self.ty(node) == PARAMETERS;
        let method_lparen = self.ty(self.par(node)) == METHOD_DEF && self.ty(node) == LPAREN;
        let text_ok = self.ty(node) != TEXT_BLOCK_LITERAL_END || self.ecol(node) == self.line_start(node);
        if !null_param && !method_lparen && text_ok {
            let t = self.ty(node);
            if t == RPAREN {
                if self.ty(self.par(node)) == RECORD_PATTERN_DEF {
                    self.log_warning(node, current_indent);
                } else {
                    self.log_warning(node, first_node_indent);
                }
            } else if !IGNORED_LIST.contains(&t) {
                self.log_warning(node, current_indent);
            }
        }
    }

    fn check_for_annotation_indentation(&mut self, nodes: &mut BTreeMap<i32, N>, indent_level: i32) {
        let first_line_node = *nodes.values().next().unwrap();
        let mut node = self.par(first_line_node);
        while node != NIL {
            if self.ty(node) == ANNOTATION {
                let at = self.fc(node);
                let from = self.line_no(node);
                let to = {
                    let last_line = self.line_no(self.lc(node));
                    match nodes.range((last_line + 1)..).next() {
                        Some((&k, _)) => k,
                        None => *nodes.keys().next_back().unwrap_or(&last_line),
                    }
                };
                self.check_annotation_indentation(at, nodes, from, to, indent_level);
            }
            node = self.ns(node);
        }
    }

    fn check_annotation_indentation(&mut self, at: N, nodes: &mut BTreeMap<i32, N>, from: i32, to: i32, indent_level: i32) {
        if from > to {
            return;
        }
        let first_node_indent = self.line_start(at);
        let current_indent = first_node_indent + indent_level;
        let last_annotation = self.lc(self.par(at));
        let last_annotation_line = self.line_no(last_annotation);
        let keys: Vec<i32> = nodes.range(from..=to).map(|(&k, _)| k).collect();
        let mut remaining = keys.len();
        let mut idx = 0;
        while remaining > 1 {
            let key = keys[idx];
            let node = nodes[&key];
            let parent_node = self.par(node);
            let array_init_in_ancestors = self.parent_contains(node, ANNOTATION_ARRAY_INIT);
            let close_alone = self.line_no(node) == last_annotation_line && self.is_end_of_scope(last_annotation, node);
            if !array_init_in_ancestors
                && (close_alone
                    || (self.ty(node) == AT && (self.ty(self.par(parent_node)) == MODIFIERS || self.ty(self.par(parent_node)) == ANNOTATIONS))
                    || self.same_line(node, at))
            {
                self.log_warning(node, first_node_indent);
            } else if !array_init_in_ancestors {
                self.log_warning(node, current_indent);
            }
            nodes.remove(&key);
            idx += 1;
            remaining -= 1;
        }
    }

    fn is_end_of_scope(&self, last_annotation: N, node: N) -> bool {
        let mut check = node;
        let mut end = true;
        while end && check != last_annotation {
            match self.ty(check) {
                RCURLY | RBRACK => {
                    while self.ns(check) == NIL {
                        if self.par(check) == NIL {
                            return false;
                        }
                        check = self.par(check);
                    }
                    check = self.ns(check);
                }
                _ => end = false,
            }
        }
        end
    }

    fn parent_contains(&self, node: N, t: Tok) -> bool {
        let mut a = self.par(node);
        while a != NIL {
            if self.ty(a) == t {
                return true;
            }
            a = self.par(a);
        }
        false
    }

    fn collect_first_nodes(&self, first: N, last: N) -> BTreeMap<i32, N> {
        let mut result = BTreeMap::new();
        result.insert(self.line_no(first), first);
        let mut cur = self.fc(first);
        while cur != last && cur != NIL {
            if self.ty(cur) == OBJBLOCK || self.ty(cur) == SLIST {
                cur = self.lc(cur);
            }
            let line = self.line_no(cur);
            let take = match result.get(&line) {
                None => true,
                Some(&f) => self.ecol(f) >= self.ecol(cur),
            };
            if take {
                result.insert(line, cur);
            }
            cur = self.next_cur_node(cur);
        }
        result
    }

    fn next_cur_node(&self, cur: N) -> N {
        let mut to_visit = self.fc(cur);
        let mut current = cur;
        while to_visit == NIL {
            if current == NIL {
                return NIL;
            }
            to_visit = self.ns(current);
            if to_visit == NIL {
                current = self.par(current);
            }
        }
        to_visit
    }

    fn log_warning(&mut self, node: N, current_indent: i32) {
        let c = self.ecol(node);
        let bad = if self.o.force_strict_condition { c != current_indent } else { c < current_indent };
        if bad {
            let text = self.a.text(node).to_string();
            let m = Self::msg(MSG_ERROR, &text, c, &fmt_int(current_indent));
            self.log_ast(node, m);
        }
    }
}

impl AstSet {
    fn add(&mut self, _ind: &Ind, n: N) {
        self.lines.insert(_ind.line_no(n), n);
    }
    fn start_column(&self, ind: &Ind, line: i32) -> Option<i32> {
        self.lines.get(&line).map(|&n| ind.ecol(n))
    }
    fn first_line(&self) -> N {
        *self.lines.values().next().unwrap()
    }
    fn last_line(&self) -> i32 {
        *self.lines.keys().next_back().unwrap()
    }
}
