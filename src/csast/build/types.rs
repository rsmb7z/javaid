// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaAstVisitor.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Types, names, annotations and modifiers.

use tree_sitter::Node;

use super::{B, kids};
use crate::csast::ast::{N, NIL};
use crate::csast::tok::Tok;

/// The visitor's `DetailAstPair`: a root plus the node new children are appended after.
pub(crate) struct Pair {
    pub root: N,
    pub child: N,
}

impl Pair {
    pub fn new() -> Self {
        Pair { root: NIL, child: NIL }
    }
}

/// One `id typeArguments?` step of a (possibly dotted) class type.
struct Seg<'t> {
    dot: Option<Node<'t>>,
    annos: Vec<Node<'t>>,
    id: Node<'t>,
    targs: Option<Node<'t>>,
}

pub(crate) fn is_annotation(n: Node) -> bool {
    matches!(n.kind(), "marker_annotation" | "annotation")
}

impl<'s> B<'s> {
    pub fn pair_add(&mut self, p: &mut Pair, ast: N) {
        if ast == NIL {
            return;
        }
        if p.root == NIL {
            p.root = ast;
        } else {
            self.a.set_next_sibling(p.child, ast);
        }
        p.child = ast;
    }

    pub fn pair_make_root(&mut self, p: &mut Pair, ast: N) {
        self.a.add_child(ast, p.root);
        p.child = p.root;
        while let Some(nx) = self.a.next_sibling(p.child) {
            p.child = nx;
        }
        p.root = ast;
    }

    // ---- qualified names ----

    /// `qualifiedName`: `a.b.c` as left-nested `DOT`s. Accepts identifiers and `scoped_identifier`s.
    pub fn qualified_name(&mut self, n: Node) -> N {
        if n.kind() == "scoped_identifier" {
            let k = kids(n);
            let scope = self.qualified_name(k[0]);
            let dot = self.punct(k[1]);
            let name = self.ident(*k.last().unwrap());
            self.add(dot, scope);
            self.add(dot, name);
            dot
        } else {
            self.ident(n)
        }
    }

    // ---- annotations and modifiers ----

    /// `annotations[force]`: an `ANNOTATIONS` node, or nothing when empty and not forced.
    pub fn annotations(&mut self, annos: &[Node], force: bool) -> N {
        if annos.is_empty() && !force {
            return NIL;
        }
        let a = self.imag(Tok::ANNOTATIONS);
        for &x in annos {
            let an = self.annotation(x);
            self.add(a, an);
        }
        a
    }

    pub fn annotation(&mut self, n: Node) -> N {
        let node = self.imag(Tok::ANNOTATION);
        let k = kids(n);
        let at = self.punct(k[0]);
        self.add(node, at);
        let name = self.qualified_name(k[1]);
        self.add(node, name);
        if let Some(args) = k.get(2) {
            // annotation_argument_list: ( value | pairs )
            for x in kids(*args) {
                let part = match x.kind() {
                    "(" | ")" | "," => self.punct(x),
                    _ => self.element_value(x),
                };
                self.add(node, part);
            }
        }
        node
    }

    /// `elementValue` or `elementValuePair`.
    pub fn element_value(&mut self, x: Node) -> N {
        match x.kind() {
            "element_value_pair" => {
                let pair = self.imag(Tok::ANNOTATION_MEMBER_VALUE_PAIR);
                for (i, y) in kids(x).into_iter().enumerate() {
                    let part = match (i, y.kind()) {
                        (_, "=") => self.punct(y),
                        (0, _) => self.ident(y),
                        _ => self.element_value(y),
                    };
                    self.add(pair, part);
                }
                pair
            }
            "element_value_array_initializer" => {
                let k = kids(x);
                let init = self.tok_text(Tok::ANNOTATION_ARRAY_INIT, k[0], "{");
                for y in &k[1..] {
                    let part = match y.kind() {
                        "," | "}" => self.punct(*y),
                        _ => self.element_value(*y),
                    };
                    self.add(init, part);
                }
                init
            }
            "marker_annotation" | "annotation" => self.annotation(x),
            _ => self.expression(x),
        }
    }

    /// `createModifiers`: the `MODIFIERS` node for a tree-sitter `modifiers` node (or none).
    pub fn modifiers(&mut self, m: Option<Node>) -> N {
        let mods = self.imag(Tok::MODIFIERS);
        if let Some(m) = m {
            for x in kids(m) {
                let part = if is_annotation(x) {
                    self.annotation(x)
                } else {
                    // `visibility` and `modifier` wrap one keyword.
                    let text = self.text(x);
                    let ty = super::tok_for_kind(&text).unwrap_or_else(|| panic!("modifier {text:?}"));
                    self.tok(ty, x)
                };
                self.add(mods, part);
            }
        }
        mods
    }

    // ---- types ----

    fn collect_segs<'t>(&self, n: Node<'t>, out: &mut Vec<Seg<'t>>) {
        match n.kind() {
            "scoped_type_identifier" => {
                let k = kids(n);
                self.collect_segs(k[0], out);
                let dot = k.iter().position(|x| x.kind() == ".").map(|i| k[i]);
                let after_dot = k.iter().position(|x| x.kind() == ".").unwrap() + 1;
                let annos: Vec<Node> = k[after_dot..k.len() - 1].iter().copied().filter(|x| is_annotation(*x)).collect();
                out.push(Seg { dot, annos, id: *k.last().unwrap(), targs: None });
            }
            "generic_type" => {
                let k = kids(n);
                self.collect_segs(k[0], out);
                out.last_mut().unwrap().targs = Some(k[1]);
            }
            _ => out.push(Seg { dot: None, annos: vec![], id: n, targs: None }),
        }
    }

    /// `classOrInterfaceType[false]` (also `createdName` and dotted names with annotations):
    /// the identifier chain, with type arguments as siblings.
    pub fn class_type(&mut self, n: Node) -> N {
        let mut segs = Vec::new();
        self.collect_segs(n, &mut segs);
        let mut pair = Pair::new();
        for (i, s) in segs.iter().enumerate() {
            if i > 0 {
                let dot = self.punct(s.dot.expect("dot"));
                self.pair_make_root(&mut pair, dot);
                let an = self.annotations(&s.annos, false);
                self.pair_add(&mut pair, an);
            }
            // `A.super::m` is read by tree-sitter as a type named `A.super`.
            let id = if i > 0 && self.text(s.id) == "super" { self.tok(Tok::LITERAL_SUPER, s.id) } else { self.ident(s.id) };
            self.pair_add(&mut pair, id);
            if let Some(t) = s.targs {
                let ta = self.type_arguments(t);
                self.pair_add(&mut pair, ta);
            }
        }
        pair.root
    }

    /// The children of the `TYPE` node of `typeType`: annotations, the base type, array declarators.
    pub fn type_content(&mut self, t: Node) -> N {
        match t.kind() {
            "annotated_type" => {
                let k = kids(t);
                let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
                let inner = *k.iter().rev().find(|x| !is_annotation(**x)).unwrap();
                let an = self.annotations(&annos, false);
                let rest = self.type_content(inner);
                self.chain(&[an, rest])
            }
            "array_type" => {
                let k = kids(t);
                let base = self.type_content(k[0]);
                let dims = self.dimensions(k[1]);
                let mut parts = vec![base];
                parts.extend(dims);
                self.chain(&parts)
            }
            "integral_type" | "floating_point_type" | "boolean_type" | "void_type" => {
                let text = self.text(t);
                let ty = super::tok_for_kind(&text).unwrap_or_else(|| panic!("primitive {text:?}"));
                self.tok(ty, t)
            }
            _ => self.class_type(t),
        }
    }

    /// `typeType[true]`: a `TYPE` node.
    pub fn type_type(&mut self, t: Node) -> N {
        let ty = self.imag(Tok::TYPE);
        let c = self.type_content(t);
        self.add(ty, c);
        ty
    }

    /// Array declarators of a `dimensions` node, each possibly preceded by its annotations.
    pub fn dimensions(&mut self, d: Node) -> Vec<N> {
        let mut out = Vec::new();
        let mut annos: Vec<Node> = Vec::new();
        let mut lbrack: Option<Node> = None;
        for x in kids(d) {
            match x.kind() {
                "[" => lbrack = Some(x),
                "]" => {
                    let l = lbrack.take().expect("[");
                    let decl = self.tok(Tok::ARRAY_DECLARATOR, l);
                    let r = self.punct(x);
                    self.add(decl, r);
                    let an = self.annotations(&annos, false);
                    annos.clear();
                    if an == NIL {
                        out.push(decl);
                    } else {
                        self.last_sibling(an, decl);
                        out.push(an);
                    }
                }
                _ => annos.push(x),
            }
        }
        out
    }

    /// `typeArguments` (or a diamond): `TYPE_ARGUMENTS`.
    pub fn type_arguments(&mut self, t: Node) -> N {
        let ta = self.imag(Tok::TYPE_ARGUMENTS);
        for x in kids(t) {
            let part = match x.kind() {
                "<" => self.tok(Tok::GENERIC_START, x),
                ">" => self.tok(Tok::GENERIC_END, x),
                "," => self.punct(x),
                "wildcard" => self.wildcard(x),
                _ => {
                    let arg = self.imag(Tok::TYPE_ARGUMENT);
                    let c = self.type_content(x);
                    self.add(arg, c);
                    arg
                }
            };
            self.add(ta, part);
        }
        ta
    }

    fn wildcard(&mut self, w: Node) -> N {
        let arg = self.imag(Tok::TYPE_ARGUMENT);
        let k = kids(w);
        let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        let an = self.annotations(&annos, false);
        self.add(arg, an);
        let q = k.iter().position(|x| x.kind() == "?").unwrap();
        let wt = self.tok(Tok::WILDCARD_TYPE, k[q]);
        self.add(arg, wt);
        if let Some(kw) = k.get(q + 1) {
            let ty = if kw.kind() == "extends" { Tok::TYPE_UPPER_BOUNDS } else { Tok::TYPE_LOWER_BOUNDS };
            let bound = self.tok(ty, *kw);
            let c = self.type_content(k[q + 2]);
            self.add(bound, c);
            self.add(arg, bound);
        }
        arg
    }

    /// `typeParameters`.
    pub fn type_parameters(&mut self, t: Node) -> N {
        let tp = self.imag(Tok::TYPE_PARAMETERS);
        for x in kids(t) {
            let part = match x.kind() {
                "<" => self.tok(Tok::GENERIC_START, x),
                ">" => self.tok(Tok::GENERIC_END, x),
                "," => self.punct(x),
                _ => self.type_parameter(x),
            };
            self.add(tp, part);
        }
        tp
    }

    fn type_parameter(&mut self, t: Node) -> N {
        let tp = self.imag(Tok::TYPE_PARAMETER);
        let k = kids(t);
        let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        let an = self.annotations(&annos, false);
        self.add(tp, an);
        for x in k.iter().filter(|x| !is_annotation(**x)) {
            let part = if x.kind() == "type_bound" { self.type_bound(*x) } else { self.ident(*x) };
            self.add(tp, part);
        }
        tp
    }

    fn type_bound(&mut self, t: Node) -> N {
        let k = kids(t);
        let ub = self.tok(Tok::TYPE_UPPER_BOUNDS, k[0]);
        for x in &k[1..] {
            let part = if x.kind() == "&" {
                self.tok(Tok::TYPE_EXTENSION_AND, *x)
            } else {
                self.type_content(*x)
            };
            self.add(ub, part);
        }
        ub
    }

    /// `typeList`-like sequences (`implements A, B`): the contents of each type with `COMMA`s.
    pub fn type_list_into(&mut self, parent: N, list: Node) {
        for x in kids(list) {
            let part = if x.kind() == "," { self.punct(x) } else { self.type_content(x) };
            self.add(parent, part);
        }
    }
}
