// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaAstVisitor.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Building the Checkstyle AST from the tree-sitter parse.
//!
//! Each function mirrors one production of Checkstyle's `JavaAstVisitor`: it receives the
//! tree-sitter node that stands for the ANTLR rule and creates the nodes the visitor would
//! create, in the same order and with the same linking (including the visitor's habit of
//! returning chains of siblings). Names such as `type_type` and `qualified_name` echo the
//! grammar rules they stand for.

mod comments;
mod decl;
mod expr;
mod stmt;
mod types;

use tree_sitter::{Node, Tree};
use crate::syntax::KindExt;

use super::ast::{CsAst, N, NIL};
use super::tok::Tok;

/// Builds the Checkstyle AST for a parsed file (`checkstyle -t`).
pub fn build(tree: &Tree, src: &[u8]) -> CsAst {
    let mut b = B::new(src);
    b.compilation_unit(tree.root_node());
    b.a
}

/// The tree-sitter nodes of the expressions the builder re-associates (see `expr.rs`): found by a
/// walk over the whole parse, which [`build_checked_with`] can be handed to skip when it builds
/// a second tree from the same parse.
pub fn misparsed_cast_roots(tree: &Tree) -> std::collections::HashSet<usize> {
    expr::misparsed_cast_roots(tree.root_node())
}

/// As [`build_checked`], with the result of [`misparsed_cast_roots`] for this tree.
pub fn build_checked_with(tree: &Tree, src: &[u8], comments: bool, roots: &std::collections::HashSet<usize>) -> Result<CsAst, String> {
    if let Some(e) = crate::syntax::first_syntax_error(tree) {
        let p = e.start_position();
        return Err(format!("syntax error at {}:{}", p.row + 1, p.column + 1));
    }
    let mut b = B::new(src);
    b.reassoc_roots = roots.clone();
    b.roots_given = true;
    b.compilation_unit(tree.root_node());
    if comments {
        b.append_hidden_comment_nodes(tree.root_node());
    }
    Ok(b.a)
}

/// As [`build`] / [`build_with_comments`], but declines a tree that has syntax errors instead of
/// building from it. The builder assumes a clean tree (it panics on shapes only error recovery
/// produces), so callers that may meet broken files should use this.
pub fn build_checked(tree: &Tree, src: &[u8], comments: bool) -> Result<CsAst, String> {
    if let Some(e) = crate::syntax::first_syntax_error(tree) {
        let p = e.start_position();
        return Err(format!("syntax error at {}:{}", p.row + 1, p.column + 1));
    }
    Ok(if comments { build_with_comments(tree, src) } else { build(tree, src) })
}

/// As [`build`], with the comment nodes of `checkstyle -T`.
pub fn build_with_comments(tree: &Tree, src: &[u8]) -> CsAst {
    let mut b = B::new(src);
    b.compilation_unit(tree.root_node());
    b.append_hidden_comment_nodes(tree.root_node());
    b.a
}

/// Parses `src` with the given grammar and builds the Checkstyle AST. Runs on a thread with a
/// large stack, because nesting (long `a + b + c ...` chains) makes the recursion deep.
pub fn build_source(src: &[u8], grammar: crate::syntax::Grammar) -> CsAst {
    try_build_source(src, grammar, false).unwrap_or_else(|e| panic!("{e}"))
}

/// As [`build_source`], reporting a construct the builder cannot handle as an error. (Only with
/// `panic = "unwind"`; the release profile aborts instead.)
pub fn try_build_source(src: &[u8], grammar: crate::syntax::Grammar, comments: bool) -> Result<CsAst, String> {
    super::shared::with_big_stack(|| parse_and_build(src, grammar, comments))
        .map_err(|p| p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_else(|| "panic".into()))?
}

/// Parses `src` with the given grammar and builds the AST, on the current thread (which needs the
/// stack for the builder's recursion; see `shared::with_big_stack`).
pub(crate) fn parse_and_build(src: &[u8], grammar: crate::syntax::Grammar, comments: bool) -> Result<CsAst, String> {
    let syn = crate::syntax::Syntax::new(grammar);
    let mut parser = syn.new_parser();
    // The ANTLR lexer ends a `//` comment at a lone `\r` as well; tree-sitter only at `\n`.
    // Same length, so every offset stays valid for the builder, which reads the original.
    let mut parse_src = src.to_vec();
    for i in 0..parse_src.len() {
        if parse_src[i] == b'\r' && parse_src.get(i + 1) != Some(&b'\n') {
            parse_src[i] = b'\n';
        }
    }
    let tree = parser.parse(&parse_src, None).expect("tree-sitter produced no tree");
    build_checked(&tree, src, comments)
}

pub(crate) struct B<'s> {
    pub src: &'s [u8],
    pub a: CsAst,
    line_starts: Vec<usize>,
    ascii: bool,
    /// Tree-sitter nodes of expressions that tree-sitter parsed as `(Name) +x` casts; see `expr.rs`.
    pub reassoc_roots: std::collections::HashSet<usize>,
    /// `reassoc_roots` was handed in rather than to be found.
    pub roots_given: bool,
}

/// Non-comment children of a tree-sitter node, in order.
pub(crate) fn kids<'t>(n: Node<'t>) -> Vec<Node<'t>> {
    let mut c = n.walk();
    let mut out = Vec::with_capacity(n.child_count() as usize);
    out.extend(n.children(&mut c).filter(|k| !is_comment(*k)));
    out
}

pub(crate) fn is_comment(n: Node) -> bool {
    matches!(n.kind_s(), "line_comment" | "block_comment")
}

impl<'s> B<'s> {
    fn new(src: &'s [u8]) -> Self {
        let mut line_starts = vec![0usize];
        let mut i = 0;
        while i < src.len() {
            match src[i] {
                b'\n' => line_starts.push(i + 1),
                b'\r' if src.get(i + 1) != Some(&b'\n') => line_starts.push(i + 1),
                _ => {}
            }
            i += 1;
        }
        B { src, a: CsAst::new(), line_starts, ascii: src.is_ascii(), reassoc_roots: Default::default(), roots_given: false }
    }

    /// 0-based line and column (counted in code points, as ANTLR does) of a byte offset.
    fn pos(&self, byte: usize) -> (u32, u32) {
        let line = self.line_starts.partition_point(|&s| s <= byte) - 1;
        let start = self.line_starts[line];
        let col = if self.ascii {
            byte - start
        } else {
            self.src[start..byte].iter().filter(|&&b| (b & 0xC0) != 0x80).count()
        };
        (line as u32, col as u32)
    }

    pub fn text(&self, n: Node) -> String {
        String::from_utf8_lossy(&self.src[n.start_byte()..n.end_byte()]).into_owned()
    }

    /// A token node of the given type with the node's text and start position.
    pub fn tok(&mut self, ty: Tok, n: Node) -> N {
        self.tok_range(ty, n.start_byte(), n.end_byte())
    }

    pub fn tok_range(&mut self, ty: Tok, start: usize, end: usize) -> N {
        let bytes = &self.src[start..end];
        let text = match std::str::from_utf8(bytes) {
            Ok(s) => s.to_owned(),
            Err(_) => String::from_utf8_lossy(bytes).into_owned(),
        };
        let (l, c) = self.pos(start);
        let n = self.a.token_text(ty, std::borrow::Cow::Owned(text), l, c);
        self.a.set_span(n, start, end);
        n
    }

    /// A token node whose text differs from the source (`STATIC_INIT`, `>>`).
    pub fn tok_text(&mut self, ty: Tok, n: Node, text: &str) -> N {
        let (l, c) = self.pos(n.start_byte());
        let t = self.a.token(ty, text, l, c);
        self.a.set_span(t, n.start_byte(), n.end_byte());
        t
    }

    pub fn imag(&mut self, ty: Tok) -> N {
        self.a.imaginary(ty)
    }

    pub fn add(&mut self, parent: N, child: N) {
        self.a.add_child(parent, child);
    }

    pub fn last_sibling(&mut self, n: N, sib: N) {
        self.a.add_last_sibling(n, sib);
    }

    /// `create(Token)` for an anonymous token (punctuation or keyword): the type follows the text.
    pub fn punct(&mut self, n: Node) -> N {
        let kind = n.kind_s();
        let ty = tok_for_kind(kind).unwrap_or_else(|| panic!("no token type for {:?} at byte {}", kind, n.start_byte()));
        // Punctuation and keywords are their own kind name: no copy of the text needed.
        if self.src[n.start_byte()..n.end_byte()] == *kind.as_bytes() {
            let (l, c) = self.pos(n.start_byte());
            let t = self.a.token_text(ty, std::borrow::Cow::Borrowed(kind), l, c);
            self.a.set_span(t, n.start_byte(), n.end_byte());
            return t;
        }
        self.tok(ty, n)
    }

    /// `visitId`: any identifier-like token becomes an `IDENT`.
    pub fn ident(&mut self, n: Node) -> N {
        self.tok(Tok::IDENT, n)
    }

    /// Adds `children` (single nodes or chains) to a fresh chain, as `processChildren` would add them
    /// to a parent: returns the head of the resulting chain through a temporary holder.
    pub fn chain(&mut self, parts: &[N]) -> N {
        let mut head = NIL;
        for &p in parts {
            if p == NIL {
                continue;
            }
            if head == NIL {
                head = p;
            } else {
                self.last_sibling(head, p);
            }
        }
        head
    }

    fn compilation_unit(&mut self, root: Node) {
        if !self.roots_given {
            self.reassoc_roots = expr::misparsed_cast_roots(root);
        }
        let kids = kids(root);
        if kids.is_empty() {
            return;
        }
        let compact = kids.iter().any(|k| matches!(k.kind_s(), "method_declaration" | "local_variable_declaration" | "field_declaration"));
        let cu = self.imag(if compact { Tok::COMPACT_COMPILATION_UNIT } else { Tok::COMPILATION_UNIT });
        let mut semis: Option<N> = None;
        for k in kids {
            let part = match k.kind_s() {
                ";" => {
                    // Adjacent `;` form one typeDeclaration; the nodes end up as siblings either way.
                    let s = self.punct(k);
                    semis = Some(s);
                    s
                }
                _ => self.top_level(k),
            };
            let _ = semis;
            self.add(cu, part);
        }
        self.a.root = cu;
    }

    fn top_level(&mut self, k: Node) -> N {
        match k.kind_s() {
            "package_declaration" => self.package_declaration(k),
            "import_declaration" => self.import_declaration(k),
            "module_declaration" => self.module_declaration(k),
            "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
                self.type_declaration(k)
            }
            "method_declaration" => self.method_declaration(k),
            "local_variable_declaration" | "field_declaration" => self.field_declaration(k),
            _ => self.block_statement(k),
        }
    }
}

/// Token type of an anonymous tree-sitter node, from its text.
pub(crate) fn tok_for_kind(kind: &str) -> Option<Tok> {
    Some(match kind {
        "abstract" => Tok::ABSTRACT,
        "assert" => Tok::LITERAL_ASSERT,
        "boolean" => Tok::LITERAL_BOOLEAN,
        "break" => Tok::LITERAL_BREAK,
        "byte" => Tok::LITERAL_BYTE,
        "case" => Tok::LITERAL_CASE,
        "catch" => Tok::LITERAL_CATCH,
        "char" => Tok::LITERAL_CHAR,
        "class" => Tok::LITERAL_CLASS,
        "continue" => Tok::LITERAL_CONTINUE,
        "default" => Tok::LITERAL_DEFAULT,
        "do" => Tok::LITERAL_DO,
        "double" => Tok::LITERAL_DOUBLE,
        "else" => Tok::LITERAL_ELSE,
        "enum" => Tok::ENUM,
        "extends" => Tok::EXTENDS_CLAUSE,
        "final" => Tok::FINAL,
        "finally" => Tok::LITERAL_FINALLY,
        "float" => Tok::LITERAL_FLOAT,
        "for" => Tok::LITERAL_FOR,
        "if" => Tok::LITERAL_IF,
        "implements" => Tok::IMPLEMENTS_CLAUSE,
        "import" => Tok::IMPORT,
        "instanceof" => Tok::LITERAL_INSTANCEOF,
        "int" => Tok::LITERAL_INT,
        "interface" => Tok::LITERAL_INTERFACE,
        "long" => Tok::LITERAL_LONG,
        "module" => Tok::LITERAL_MODULE,
        "native" => Tok::LITERAL_NATIVE,
        "new" => Tok::LITERAL_NEW,
        "private" => Tok::LITERAL_PRIVATE,
        "protected" => Tok::LITERAL_PROTECTED,
        "public" => Tok::LITERAL_PUBLIC,
        "return" => Tok::LITERAL_RETURN,
        "short" => Tok::LITERAL_SHORT,
        "static" => Tok::LITERAL_STATIC,
        "strictfp" => Tok::STRICTFP,
        "super" => Tok::LITERAL_SUPER,
        "switch" => Tok::LITERAL_SWITCH,
        "synchronized" => Tok::LITERAL_SYNCHRONIZED,
        "this" => Tok::LITERAL_THIS,
        "throw" => Tok::LITERAL_THROW,
        "throws" => Tok::LITERAL_THROWS,
        "transient" => Tok::LITERAL_TRANSIENT,
        "try" => Tok::LITERAL_TRY,
        "void" => Tok::LITERAL_VOID,
        "volatile" => Tok::LITERAL_VOLATILE,
        "while" => Tok::LITERAL_WHILE,
        "record" => Tok::LITERAL_RECORD,
        "yield" => Tok::LITERAL_YIELD,
        "non-sealed" => Tok::LITERAL_NON_SEALED,
        "sealed" => Tok::LITERAL_SEALED,
        "permits" => Tok::LITERAL_PERMITS,
        "when" => Tok::LITERAL_WHEN,
        "open" => Tok::LITERAL_OPEN,
        "transitive" => Tok::LITERAL_TRANSITIVE,
        "true" => Tok::LITERAL_TRUE,
        "false" => Tok::LITERAL_FALSE,
        "null" => Tok::LITERAL_NULL,
        "(" => Tok::LPAREN,
        ")" => Tok::RPAREN,
        "{" => Tok::LCURLY,
        "}" => Tok::RCURLY,
        "]" => Tok::RBRACK,
        ";" => Tok::SEMI,
        "," => Tok::COMMA,
        "." => Tok::DOT,
        "=" => Tok::ASSIGN,
        ">" => Tok::GT,
        "<" => Tok::LT,
        "!" => Tok::LNOT,
        "~" => Tok::BNOT,
        "?" => Tok::QUESTION,
        ":" => Tok::COLON,
        "==" => Tok::EQUAL,
        "<=" => Tok::LE,
        ">=" => Tok::GE,
        "!=" => Tok::NOT_EQUAL,
        "&&" => Tok::LAND,
        "||" => Tok::LOR,
        "++" => Tok::INC,
        "--" => Tok::DEC,
        "+" => Tok::PLUS,
        "-" => Tok::MINUS,
        "*" => Tok::STAR,
        "/" => Tok::DIV,
        "&" => Tok::BAND,
        "|" => Tok::BOR,
        "^" => Tok::BXOR,
        "%" => Tok::MOD,
        "<<" => Tok::SL,
        ">>" => Tok::SR,
        ">>>" => Tok::BSR,
        "+=" => Tok::PLUS_ASSIGN,
        "-=" => Tok::MINUS_ASSIGN,
        "*=" => Tok::STAR_ASSIGN,
        "/=" => Tok::DIV_ASSIGN,
        "&=" => Tok::BAND_ASSIGN,
        "|=" => Tok::BOR_ASSIGN,
        "^=" => Tok::BXOR_ASSIGN,
        "%=" => Tok::MOD_ASSIGN,
        "<<=" => Tok::SL_ASSIGN,
        ">>=" => Tok::SR_ASSIGN,
        ">>>=" => Tok::BSR_ASSIGN,
        "->" => Tok::LAMBDA,
        "::" => Tok::DOUBLE_COLON,
        "@" => Tok::AT,
        "..." => Tok::ELLIPSIS,
        _ => return None,
    })
}
