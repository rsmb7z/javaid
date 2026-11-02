// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaAstVisitor.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Expressions.

use tree_sitter::Node;
use crate::syntax::KindExt;

use super::types::is_annotation;
use super::{B, kids};
use crate::csast::ast::{N, NIL};
use crate::csast::tok::Tok;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

/// An operand or operator of a flattened binary expression.
enum Item<'t> {
    Operand(Node<'t>),
    /// The `(Name)` of a cast that is really a parenthesized name (see `is_sign_cast`).
    Paren(Node<'t>),
    /// A cast, as a prefix operator on the next operand.
    Cast(Node<'t>),
    Op(Node<'t>),
}

/// A cast whose operand tree-sitter extended over a binary operator.
fn is_wide_cast(n: Node) -> bool {
    n.kind_s() == "cast_expression" && field(n, "value").is_some_and(|v| v.kind_s() == "binary_expression")
}

/// The tree-sitter nodes at the top of the binary/cast chains that contain a misparsed cast.
pub(crate) fn misparsed_cast_roots(root: Node) -> std::collections::HashSet<usize> {
    let mut roots = std::collections::HashSet::new();
    let mut cursor = root.walk();
    'outer: loop {
        let n = cursor.node();
        if n.kind_s() == "cast_expression" && is_misparsed_cast(n) {
            let mut top = n;
            while let Some(p) = top.parent() {
                if matches!(p.kind_s(), "binary_expression" | "cast_expression") {
                    top = p;
                } else {
                    break;
                }
            }
            roots.insert(top.id());
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break 'outer;
            }
        }
    }
    roots
}

fn is_misparsed_cast(n: Node) -> bool {
    is_sign_cast(n) || is_wide_cast(n)
}

/// `(Name) +x` or `(Name) -x` read as a cast.
fn is_sign_cast(n: Node) -> bool {
    if n.kind_s() != "cast_expression" {
        return false;
    }
    let k = kids(n);
    if k.len() != 4 || !matches!(k[1].kind_s(), "type_identifier" | "scoped_type_identifier") {
        return false;
    }
    match field(n, "value") {
        Some(v) if v.kind_s() == "unary_expression" => {
            matches!(field(v, "operator").map(|o| o.kind_s()), Some("+" | "-"))
        }
        _ => false,
    }
}

/// tree-sitter-java's precedence of the binary operators (all left-associative).
fn binary_precedence(op: &str) -> u8 {
    match op {
        "||" => 4,
        "&&" => 5,
        "|" => 6,
        "^" => 7,
        "&" => 8,
        "==" | "!=" => 9,
        "<" | ">" | "<=" | ">=" => 10,
        "<<" | ">>" | ">>>" => 11,
        "+" | "-" => 12,
        _ => 13,
    }
}

fn is_type_kind(k: &str) -> bool {
    matches!(
        k,
        "generic_type" | "array_type" | "integral_type" | "floating_point_type" | "boolean_type" | "void_type" | "annotated_type" | "scoped_type_identifier" | "type_identifier"
    )
}

impl<'s> B<'s> {
    /// `expression`: the `EXPR` root (not for lambdas and explicit constructor calls).
    pub fn expression(&mut self, n: Node) -> N {
        let e = self.expr(n);
        self.build_expression_node(e)
    }

    /// `buildExpressionNode` for an already visited expression.
    pub fn build_expression_node(&mut self, e: N) -> N {
        if matches!(self.a.ty(e), Tok::CTOR_CALL | Tok::SUPER_CTOR_CALL | Tok::LAMBDA) {
            e
        } else {
            let root = self.imag(Tok::EXPR);
            self.add(root, e);
            root
        }
    }

    /// `ELIST` of an argument list's expressions (and commas); empty when there are none.
    fn elist(&mut self, args: Node) -> N {
        let el = self.imag(Tok::ELIST);
        for x in kids(args) {
            match x.kind_s() {
                "(" | ")" => {}
                "," => {
                    let c = self.punct(x);
                    self.add(el, c);
                }
                _ => {
                    let e = self.expression(x);
                    self.add(el, e);
                }
            }
        }
        el
    }

    /// `arguments`: `(` `ELIST` `)` as a chain.
    pub fn arguments(&mut self, args: Node) -> N {
        let k = kids(args);
        let l = self.punct(k[0]);
        let el = self.elist(args);
        let r = self.punct(*k.last().unwrap());
        self.chain(&[l, el, r])
    }

    pub fn expr(&mut self, n: Node) -> N {
        if !self.reassoc_roots.is_empty() && self.reassoc_roots.contains(&n.id()) {
            return self.reassociate(n);
        }
        match n.kind_s() {
            "identifier" => self.ident(n),
            "this" | "super" => self.punct(n),
            "true" | "false" | "null_literal" => {
                let t = if n.kind_s() == "null_literal" { Tok::LITERAL_NULL } else if n.kind_s() == "true" { Tok::LITERAL_TRUE } else { Tok::LITERAL_FALSE };
                self.tok(t, n)
            }
            "decimal_integer_literal" | "hex_integer_literal" | "octal_integer_literal" | "binary_integer_literal" => {
                let long = matches!(self.src[n.end_byte() - 1], b'l' | b'L');
                self.tok(if long { Tok::NUM_LONG } else { Tok::NUM_INT }, n)
            }
            "decimal_floating_point_literal" | "hex_floating_point_literal" => {
                let double = matches!(self.src[n.end_byte() - 1], b'd' | b'D');
                self.tok(if double { Tok::NUM_DOUBLE } else { Tok::NUM_FLOAT }, n)
            }
            "character_literal" => self.tok(Tok::CHAR_LITERAL, n),
            "string_literal" => self.string_literal(n),
            "parenthesized_expression" => {
                let k = kids(n);
                let l = self.punct(k[0]);
                let e = self.expr(k[1]);
                let r = self.punct(k[2]);
                self.chain(&[l, e, r])
            }
            "field_access" => self.field_access(n),
            "method_invocation" => self.method_invocation(n),
            "array_access" => {
                let k = kids(n);
                let op = self.tok(Tok::INDEX_OP, k[1]);
                let arr = self.expr(k[0]);
                self.add(op, arr);
                let idx = self.expr(k[2]);
                let wrap = self.imag(Tok::EXPR);
                self.add(wrap, idx);
                self.add(op, wrap);
                let r = self.punct(k[3]);
                self.add(op, r);
                op
            }
            "unary_expression" => {
                let op = field(n, "operator").unwrap();
                let ty = match op.kind_s() {
                    "+" => Tok::UNARY_PLUS,
                    "-" => Tok::UNARY_MINUS,
                    "!" => Tok::LNOT,
                    _ => Tok::BNOT,
                };
                let root = self.tok(ty, op);
                let e = self.expr(field(n, "operand").unwrap());
                self.add(root, e);
                root
            }
            "update_expression" => {
                let k = kids(n);
                if k[0].kind_s() == "++" || k[0].kind_s() == "--" {
                    let root = self.punct(k[0]);
                    let e = self.expr(k[1]);
                    self.add(root, e);
                    root
                } else {
                    let ty = if k[1].kind_s() == "++" { Tok::POST_INC } else { Tok::POST_DEC };
                    let root = self.tok(ty, k[1]);
                    let e = self.expr(k[0]);
                    self.add(root, e);
                    root
                }
            }
            "cast_expression" => {
                let v = self.expr(field(n, "value").unwrap());
                self.cast_node(n, v)
            }
            "binary_expression" | "assignment_expression" => {
                let op = field(n, "operator").unwrap();
                let bop = self.punct(op);
                let l = self.expr(field(n, "left").unwrap());
                self.add(bop, l);
                let r = self.expr(field(n, "right").unwrap());
                self.add(bop, r);
                bop
            }
            "ternary_expression" => {
                let k = kids(n);
                let q = self.punct(k[1]);
                let c = self.expr(k[0]);
                self.add(q, c);
                let a = self.expr(k[2]);
                self.add(q, a);
                let colon = self.punct(k[3]);
                self.add(q, colon);
                let b = self.expr(k[4]);
                self.add(q, b);
                q
            }
            "instanceof_expression" => self.instanceof(n),
            "lambda_expression" => self.lambda(n),
            "switch_expression" => self.switch(n),
            "object_creation_expression" => self.object_creation(n),
            "array_creation_expression" => self.array_creation(n),
            "class_literal" => self.class_literal(n),
            "method_reference" => self.method_reference(n),
            other => panic!("unsupported expression {other} at byte {}", n.start_byte()),
        }
    }

    /// `TYPECAST` for a tree-sitter cast whose operand has already been built.
    fn cast_node(&mut self, n: Node, value: N) -> N {
        let k = kids(n);
        let cast = self.tok(Tok::TYPECAST, k[0]);
        let mut i = 1;
        while k[i].kind_s() != ")" {
            let part = if k[i].kind_s() == "&" { self.tok(Tok::TYPE_EXTENSION_AND, k[i]) } else { self.type_type(k[i]) };
            self.add(cast, part);
            i += 1;
        }
        let r = self.punct(k[i]);
        self.add(cast, r);
        self.add(cast, value);
        cast
    }

    // ---- casts that tree-sitter reads with the wrong reach ----
    //
    // 1. `(a) + b` is a sum of the parenthesized `a` and `b`: a cast to a reference type cannot be
    //    followed by a unary `+` or `-`, and ANTLR's grammar takes the parenthesized reading. Inside
    //    a longer arithmetic chain tree-sitter reads it as a cast of `+b` instead, which also pulls
    //    the operators around it out of order (`x * (a) + b * c`).
    // 2. A cast binds tighter than every binary operator, but after `k * (a) (b) + c` tree-sitter
    //    gives the cast of `(a)` the operand `(b) + c`.
    //
    // The tokens are the same either way, so for such an expression the binary operators are
    // flattened (casts become prefix operators) and the precedence applied again.

    fn flatten_binary<'t>(&self, n: Node<'t>, out: &mut Vec<Item<'t>>) {
        if n.kind_s() == "binary_expression" {
            self.flatten_binary(field(n, "left").unwrap(), out);
            out.push(Item::Op(field(n, "operator").unwrap()));
            self.flatten_binary(field(n, "right").unwrap(), out);
        } else if is_sign_cast(n) {
            let unary = field(n, "value").unwrap();
            out.push(Item::Paren(n));
            out.push(Item::Op(field(unary, "operator").unwrap()));
            self.flatten_binary(field(unary, "operand").unwrap(), out);
        } else if n.kind_s() == "cast_expression" {
            out.push(Item::Cast(n));
            self.flatten_binary(field(n, "value").unwrap(), out);
        } else {
            out.push(Item::Operand(n));
        }
    }

    fn reassociate(&mut self, n: Node) -> N {
        let mut items = Vec::new();
        self.flatten_binary(n, &mut items);
        let mut pos = 0;
        self.climb(&items, &mut pos, 0)
    }

    /// An operand with the casts in front of it.
    fn unary(&mut self, items: &[Item], pos: &mut usize) -> N {
        let item = &items[*pos];
        *pos += 1;
        if let Item::Cast(cast) = *item {
            let inner = self.unary(items, pos);
            self.cast_node(cast, inner)
        } else {
            self.operand(item)
        }
    }

    fn climb(&mut self, items: &[Item], pos: &mut usize, min_prec: u8) -> N {
        let mut lhs = self.unary(items, pos);
        while *pos < items.len() {
            let Item::Op(op) = items[*pos] else { unreachable!("operators alternate with operands") };
            let prec = binary_precedence(op.kind_s());
            if prec < min_prec {
                break;
            }
            *pos += 1;
            let rhs = self.climb(items, pos, prec + 1);
            let bop = self.punct(op);
            self.add(bop, lhs);
            self.add(bop, rhs);
            lhs = bop;
        }
        lhs
    }

    fn operand(&mut self, item: &Item) -> N {
        match *item {
            Item::Operand(n) => self.expr(n),
            Item::Paren(cast) => {
                let k = kids(cast);
                let l = self.punct(k[0]);
                let name = self.class_type(k[1]);
                let r = self.punct(k[2]);
                self.chain(&[l, name, r])
            }
            Item::Op(_) | Item::Cast(_) => unreachable!(),
        }
    }

    fn string_literal(&mut self, n: Node) -> N {
        let (s, e) = (n.start_byte(), n.end_byte());
        if e - s >= 6 && &self.src[s..s + 3] == b"\"\"\"" {
            let begin = self.tok_range(Tok::TEXT_BLOCK_LITERAL_BEGIN, s, s + 3);
            let content = self.tok_range(Tok::TEXT_BLOCK_CONTENT, s + 3, e - 3);
            let end = self.tok_range(Tok::TEXT_BLOCK_LITERAL_END, e - 3, e);
            self.add(begin, content);
            self.add(begin, end);
            begin
        } else {
            self.tok(Tok::STRING_LITERAL, n)
        }
    }

    fn field_access(&mut self, n: Node) -> N {
        let k = kids(n);
        let obj = field(n, "object").unwrap();
        let fld = field(n, "field").unwrap();
        let dots: Vec<Node> = k.iter().copied().filter(|x| x.kind_s() == ".").collect();
        let has_super = k.iter().any(|x| x.kind_s() == "super" && x.id() != obj.id());
        if has_super {
            // X.super.f
            let bop = self.punct(dots[0]);
            let o = self.expr(obj);
            self.add(bop, o);
            let sup = k.iter().find(|x| x.kind_s() == "super" && x.id() != obj.id()).unwrap();
            let s = self.punct(*sup);
            self.add(bop, s);
            let dot2 = self.punct(dots[1]);
            self.add(dot2, bop);
            let id = self.ident(fld);
            self.add(dot2, id);
            return dot2;
        }
        let dot = self.punct(dots[0]);
        let o = self.expr(obj);
        self.add(dot, o);
        // `A.super::m` reaches here with the `super` read as an identifier.
        let f = if fld.kind_s() == "this" {
            self.punct(fld)
        } else if self.text(fld) == "super" {
            self.tok(Tok::LITERAL_SUPER, fld)
        } else {
            self.ident(fld)
        };
        self.add(dot, f);
        dot
    }

    fn method_invocation(&mut self, n: Node) -> N {
        let args = field(n, "arguments").unwrap();
        let name = field(n, "name").unwrap();
        let lparen = kids(args)[0];
        let mc = self.tok(Tok::METHOD_CALL, lparen);
        let rparen = *kids(args).last().unwrap();
        if let Some(obj) = field(n, "object") {
            let k = kids(n);
            let dots: Vec<Node> = k.iter().copied().filter(|x| x.kind_s() == ".").collect();
            let sup = k.iter().find(|x| x.kind_s() == "super" && x.id() != obj.id()).copied();
            let targs = field(n, "type_arguments");
            let callee = if let Some(sup) = sup {
                // X.super.m(...)
                let bop = self.punct(dots[0]);
                let o = self.expr(obj);
                self.add(bop, o);
                let s = self.punct(sup);
                self.add(bop, s);
                let dot2 = self.punct(dots[1]);
                self.add(dot2, bop);
                if let Some(t) = targs {
                    // `X.super.<T>m()` is `invOp` on the expression `X.super`.
                    let ta = self.type_arguments(t);
                    self.add(dot2, ta);
                }
                let id = self.ident(name);
                self.add(dot2, id);
                dot2
            } else {
                let dot = self.punct(dots[0]);
                let o = self.expr(obj);
                self.add(dot, o);
                if let Some(t) = targs {
                    let ta = self.type_arguments(t);
                    self.add(dot, ta);
                }
                let id = self.ident(name);
                self.add(dot, id);
                dot
            };
            self.add(mc, callee);
        } else {
            let id = self.ident(name);
            self.add(mc, id);
        }
        let el = self.elist(args);
        self.add(mc, el);
        let r = self.punct(rparen);
        self.add(mc, r);
        mc
    }

    fn instanceof(&mut self, n: Node) -> N {
        let k = kids(n);
        let io = self.punct(k[1]);
        let l = self.expr(k[0]);
        self.add(io, l);
        let mods = k.iter().find(|x| x.kind_s() == "modifiers").copied();
        if let Some(pat) = field(n, "pattern") {
            let p = self.pattern(pat);
            self.add(io, p);
        } else {
            let right = field(n, "right").unwrap();
            if let Some(name) = field(n, "name") {
                let def = self.imag(Tok::PATTERN_VARIABLE_DEF);
                let m = self.modifiers(mods);
                self.add(def, m);
                let ty = self.type_type(right);
                self.add(def, ty);
                let id = self.ident(name);
                self.add(def, id);
                self.add(io, def);
            } else {
                let ty = self.type_type(right);
                self.add(io, ty);
            }
        }
        io
    }

    fn lambda(&mut self, n: Node) -> N {
        let k = kids(n);
        let arrow = k.iter().find(|x| x.kind_s() == "->").unwrap();
        let lam = self.punct(*arrow);
        let params = field(n, "parameters").unwrap();
        let p = match params.kind_s() {
            "formal_parameters" => self.formal_parameters(params),
            "inferred_parameters" => {
                let pk = kids(params);
                let l = self.punct(pk[0]);
                let ps = self.imag(Tok::PARAMETERS);
                for x in &pk[1..pk.len() - 1] {
                    let part = if x.kind_s() == "," {
                        self.punct(*x)
                    } else {
                        let id = self.ident(*x);
                        let def = self.imag(Tok::PARAMETER_DEF);
                        let m = self.imag(Tok::MODIFIERS);
                        let t = self.imag(Tok::TYPE);
                        self.add(def, m);
                        self.add(def, t);
                        self.add(def, id);
                        def
                    };
                    self.add(ps, part);
                }
                let r = self.punct(*pk.last().unwrap());
                self.chain(&[l, ps, r])
            }
            _ => self.ident(params),
        };
        self.add(lam, p);
        let body = field(n, "body").unwrap();
        let b = if body.kind_s() == "block" { self.block(body) } else { self.expression(body) };
        self.add(lam, b);
        lam
    }

    fn object_creation(&mut self, n: Node) -> N {
        let k = kids(n);
        let new_pos = k.iter().position(|x| x.kind_s() == "new").unwrap();
        let new = self.punct(k[new_pos]);
        let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        if let Some(t) = field(n, "type_arguments") {
            let ta = self.type_arguments(t);
            self.add(new, ta);
        }
        let an = self.annotations(&annos, false);
        self.add(new, an);
        let ty = field(n, "type").unwrap();
        let name = self.class_type(ty);
        self.add(new, name);
        let a = self.arguments(field(n, "arguments").unwrap());
        self.add(new, a);
        if let Some(body) = k.iter().find(|x| x.kind_s() == "class_body") {
            let ob = self.obj_block(*body);
            self.add(new, ob);
        }
        if new_pos > 0 {
            // outer.new Inner()
            let dot = self.punct(k[new_pos - 1]);
            let outer = self.expr(k[0]);
            self.add(dot, outer);
            self.add(dot, new);
            return dot;
        }
        new
    }

    fn array_creation(&mut self, n: Node) -> N {
        let k = kids(n);
        let new = self.punct(k[0]);
        let ty = field(n, "type").unwrap();
        let tpos = k.iter().position(|x| x.id() == ty.id()).unwrap();
        let annos: Vec<Node> = k[1..tpos].iter().copied().filter(|x| is_annotation(*x)).collect();
        let an = self.annotations(&annos, false);
        self.add(new, an);
        let name = if matches!(ty.kind_s(), "integral_type" | "floating_point_type" | "boolean_type") {
            let text = self.text(ty);
            let t = super::tok_for_kind(&text).unwrap();
            self.tok(t, ty)
        } else {
            self.class_type(ty)
        };
        self.add(new, name);
        for x in &k[tpos + 1..] {
            match x.kind_s() {
                "dimensions_expr" => {
                    let dk = kids(*x);
                    let dannos: Vec<Node> = dk.iter().copied().filter(|a| is_annotation(*a)).collect();
                    let lb = dk.iter().find(|a| a.kind_s() == "[").unwrap();
                    let decl = self.tok(Tok::ARRAY_DECLARATOR, *lb);
                    let e = self.expression(dk[dk.len() - 2]);
                    self.add(decl, e);
                    let r = self.punct(*dk.last().unwrap());
                    self.add(decl, r);
                    let da = self.annotations(&dannos, false);
                    self.add(new, da);
                    self.add(new, decl);
                }
                "dimensions" => {
                    for d in self.dimensions(*x) {
                        self.add(new, d);
                    }
                }
                "array_initializer" => {
                    let init = self.array_initializer(*x);
                    self.add(new, init);
                }
                _ => {}
            }
        }
        new
    }

    fn class_literal(&mut self, n: Node) -> N {
        let k = kids(n);
        let dot = self.punct(k[1]);
        let class = self.punct(k[2]);
        let ty = k[0];
        let mut base_node = ty;
        let mut dim_nodes: Vec<Node> = Vec::new();
        while base_node.kind_s() == "array_type" {
            let ak = kids(base_node);
            dim_nodes.insert(0, ak[1]);
            base_node = ak[0];
        }
        let primitive = matches!(base_node.kind_s(), "integral_type" | "floating_point_type" | "boolean_type" | "void_type");
        let base = if primitive {
            let text = self.text(base_node);
            let t = super::tok_for_kind(&text).unwrap();
            self.tok(t, base_node)
        } else {
            self.class_type(base_node)
        };
        self.add(dot, base);
        for d in dim_nodes {
            for decl in self.dimensions(d) {
                if primitive || self.a.ty(base) == Tok::DOT {
                    if primitive {
                        self.add(dot, decl);
                    } else {
                        self.add(base, decl);
                    }
                } else {
                    self.last_sibling(base, decl);
                }
            }
        }
        self.add(dot, class);
        dot
    }

    fn method_reference(&mut self, n: Node) -> N {
        let k = kids(n);
        let cc = k.iter().position(|x| x.kind_s() == "::").unwrap();
        let mr = self.tok(Tok::METHOD_REF, k[cc]);
        let first = k[0];
        let f = if is_type_kind(first.kind_s()) { self.type_content(first) } else { self.expr(first) };
        self.add(mr, f);
        for x in &k[cc + 1..] {
            let part = match x.kind_s() {
                "type_arguments" => self.type_arguments(*x),
                "new" => self.punct(*x),
                _ => self.ident(*x),
            };
            self.add(mr, part);
        }
        let _ = NIL;
        mr
    }
}
