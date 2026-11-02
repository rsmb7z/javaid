// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaAstVisitor.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Statements, blocks, switch, try, patterns.

use tree_sitter::Node;
use crate::syntax::KindExt;

use super::{B, kids};
use crate::csast::ast::{N, NIL};
use crate::csast::tok::Tok;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

fn modifiers_of<'t>(n: Node<'t>) -> Option<Node<'t>> {
    kids(n).into_iter().find(|k| k.kind_s() == "modifiers")
}

impl<'s> B<'s> {
    /// `block`: an `SLIST` created from the `{`.
    pub fn block(&mut self, n: Node) -> N {
        let k = kids(n);
        let slist = self.tok(Tok::SLIST, k[0]);
        for x in &k[1..] {
            let part = if x.kind_s() == "}" { self.punct(*x) } else { self.block_statement(*x) };
            self.add(slist, part);
        }
        slist
    }

    pub fn constructor_body(&mut self, n: Node) -> N {
        let k = kids(n);
        let slist = self.tok(Tok::SLIST, k[0]);
        for x in &k[1..] {
            let part = match x.kind_s() {
                "}" => self.punct(*x),
                "explicit_constructor_invocation" => self.explicit_constructor_invocation(*x),
                _ => self.block_statement(*x),
            };
            self.add(slist, part);
        }
        slist
    }

    fn explicit_constructor_invocation(&mut self, n: Node) -> N {
        let k = kids(n);
        let ctor = field(n, "constructor").unwrap();
        let args = field(n, "arguments").unwrap();
        let targs = field(n, "type_arguments");
        let semi = *k.last().unwrap();
        if let Some(obj) = field(n, "object") {
            let root = self.tok(Tok::SUPER_CTOR_CALL, ctor);
            let o = self.expr(obj);
            self.add(root, o);
            let dot = k.iter().find(|x| x.kind_s() == ".").unwrap();
            let d = self.punct(*dot);
            self.add(root, d);
            if let Some(t) = targs {
                let ta = self.type_arguments(t);
                self.add(root, ta);
            }
            let a = self.arguments(args);
            self.add(root, a);
            let s = self.punct(semi);
            self.add(root, s);
            root
        } else {
            let ty = if ctor.kind_s() == "this" { Tok::CTOR_CALL } else { Tok::SUPER_CTOR_CALL };
            let root = self.tok(ty, ctor);
            if let Some(t) = targs {
                let ta = self.type_arguments(t);
                self.add(root, ta);
            }
            let a = self.arguments(args);
            self.add(root, a);
            let s = self.punct(semi);
            self.add(root, s);
            root
        }
    }

    /// `blockStatement`.
    pub fn block_statement(&mut self, x: Node) -> N {
        match x.kind_s() {
            "local_variable_declaration" => {
                let decls = self.variable_declarators(x);
                let semi = kids(x).into_iter().rev().find(|y| y.kind_s() == ";").unwrap();
                let s = self.punct(semi);
                self.chain(&[decls, s])
            }
            "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
                self.type_declaration(x)
            }
            ";" => self.tok(Tok::EMPTY_STAT, x),
            _ => self.statement(x),
        }
    }

    /// `parExpression`: `(`, `EXPR`, `)` as a chain.
    pub fn par_expression(&mut self, n: Node) -> N {
        let k = kids(n);
        let l = self.punct(k[0]);
        let e = self.expression(k[1]);
        let r = self.punct(k[2]);
        self.chain(&[l, e, r])
    }

    pub fn statement(&mut self, x: Node) -> N {
        match x.kind_s() {
            "block" => self.block(x),
            "expression_statement" if kids(x)[0].kind_s() == "switch_expression" => {
                // `switch (x) { ... };`: a switch statement followed by an empty statement.
                let k = kids(x);
                let sw = self.switch(k[0]);
                let semi = self.tok(Tok::EMPTY_STAT, k[1]);
                self.chain(&[sw, semi])
            }
            "expression_statement" => {
                let k = kids(x);
                let e = self.expression(k[0]);
                let s = self.punct(k[1]);
                self.chain(&[e, s])
            }
            "labeled_statement" => {
                let k = kids(x);
                let label = self.tok(Tok::LABELED_STAT, k[1]);
                let id = self.ident(k[0]);
                self.add(label, id);
                let st = self.statement(k[2]);
                self.add(label, st);
                label
            }
            "if_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                let cond = self.par_expression(k[1]);
                self.add(root, cond);
                let cons = self.statement(k[2]);
                self.add(root, cons);
                if k.len() > 3 {
                    let els = self.punct(k[3]);
                    let alt = self.statement(k[4]);
                    self.add(els, alt);
                    self.add(root, els);
                }
                root
            }
            "while_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                let cond = self.par_expression(k[1]);
                self.add(root, cond);
                let body = self.statement(k[2]);
                self.add(root, body);
                root
            }
            "do_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                let body = self.statement(k[1]);
                self.add(root, body);
                let w = self.tok(Tok::DO_WHILE, k[2]);
                self.add(root, w);
                let cond = self.par_expression(k[3]);
                self.add(root, cond);
                let semi = self.punct(k[4]);
                self.add(root, semi);
                root
            }
            "for_statement" => self.for_statement(x),
            "enhanced_for_statement" => self.enhanced_for(x),
            "assert_statement" => {
                let k = kids(x);
                let root = self.tok(Tok::LITERAL_ASSERT, k[0]);
                for y in &k[1..] {
                    let part = match y.kind_s() {
                        ":" | ";" => self.punct(*y),
                        _ => self.expression(*y),
                    };
                    self.add(root, part);
                }
                root
            }
            "break_statement" | "continue_statement" | "return_statement" | "throw_statement" | "yield_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                for y in &k[1..] {
                    let part = match y.kind_s() {
                        ";" => self.punct(*y),
                        "identifier" if matches!(x.kind_s(), "break_statement" | "continue_statement") => self.ident(*y),
                        _ => self.expression(*y),
                    };
                    self.add(root, part);
                }
                root
            }
            "synchronized_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                let cond = self.par_expression(k[1]);
                self.add(root, cond);
                let b = self.block(k[2]);
                self.add(root, b);
                root
            }
            "try_statement" | "try_with_resources_statement" => {
                let k = kids(x);
                let root = self.punct(k[0]);
                for y in &k[1..] {
                    let part = match y.kind_s() {
                        "block" => self.block(*y),
                        "catch_clause" => self.catch_clause(*y),
                        "finally_clause" => {
                            let fk = kids(*y);
                            let f = self.punct(fk[0]);
                            let b = self.block(fk[1]);
                            self.add(f, b);
                            f
                        }
                        "resource_specification" => self.resource_specification(*y),
                        other => panic!("unexpected {other} in try"),
                    };
                    self.add(root, part);
                }
                root
            }
            "switch_expression" => self.switch(x),
            "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
                self.type_declaration(x)
            }
            "local_variable_declaration" => self.block_statement(x),
            ";" => self.tok(Tok::EMPTY_STAT, x),
            other => panic!("unexpected statement {other} at byte {}", x.start_byte()),
        }
    }

    fn for_statement(&mut self, x: Node) -> N {
        let k = kids(x);
        let root = self.punct(k[0]);
        let lparen = self.punct(k[1]);
        self.add(root, lparen);
        let mut i = 2;
        // FOR_INIT and the first `;`
        let for_init = self.imag(Tok::FOR_INIT);
        let semi0;
        if k[i].kind_s() == "local_variable_declaration" {
            let decls = self.variable_declarators(k[i]);
            self.add(for_init, decls);
            let s = kids(k[i]).into_iter().rev().find(|y| y.kind_s() == ";").unwrap();
            semi0 = self.punct(s);
            i += 1;
        } else {
            let mut elist = NIL;
            while k[i].kind_s() != ";" {
                if elist == NIL {
                    elist = self.imag(Tok::ELIST);
                }
                let part = if k[i].kind_s() == "," { self.punct(k[i]) } else { self.expression(k[i]) };
                self.add(elist, part);
                i += 1;
            }
            self.add(for_init, elist);
            semi0 = self.punct(k[i]);
            i += 1;
        }
        self.add(root, for_init);
        self.add(root, semi0);
        let cond = self.imag(Tok::FOR_CONDITION);
        if k[i].kind_s() != ";" {
            let e = self.expression(k[i]);
            self.add(cond, e);
            i += 1;
        }
        self.add(root, cond);
        let semi1 = self.punct(k[i]);
        self.add(root, semi1);
        i += 1;
        let iter = self.imag(Tok::FOR_ITERATOR);
        let mut elist = NIL;
        while k[i].kind_s() != ")" {
            if elist == NIL {
                elist = self.imag(Tok::ELIST);
            }
            let part = if k[i].kind_s() == "," { self.punct(k[i]) } else { self.expression(k[i]) };
            self.add(elist, part);
            i += 1;
        }
        self.add(iter, elist);
        self.add(root, iter);
        let rparen = self.punct(k[i]);
        self.add(root, rparen);
        let body = self.statement(k[i + 1]);
        self.add(root, body);
        root
    }

    fn enhanced_for(&mut self, x: Node) -> N {
        let k = kids(x);
        let root = self.punct(k[0]);
        let lparen = self.punct(k[1]);
        self.add(root, lparen);
        let clause = self.imag(Tok::FOR_EACH_CLAUSE);
        let def = self.imag(Tok::VARIABLE_DEF);
        let mods = self.modifiers(modifiers_of(x));
        self.add(def, mods);
        let ty = self.type_type(field(x, "type").unwrap());
        self.add(def, ty);
        let id = self.ident(field(x, "name").unwrap());
        self.add(def, id);
        if let Some(d) = field(x, "dimensions") {
            for decl in self.dimensions(d) {
                self.add(ty, decl);
            }
        }
        let colon = k.iter().find(|y| y.kind_s() == ":").unwrap();
        let c = self.punct(*colon);
        self.last_sibling(def, c);
        let value = self.expression(field(x, "value").unwrap());
        self.last_sibling(def, value);
        self.add(clause, def);
        self.add(root, clause);
        let rparen = k.iter().rev().find(|y| y.kind_s() == ")").unwrap();
        // The closing paren is the last `)` before the body.
        let body = field(x, "body").unwrap();
        let rp = k.iter().filter(|y| y.kind_s() == ")" && y.end_byte() <= body.start_byte()).last().unwrap_or(rparen);
        let r = self.punct(*rp);
        self.add(root, r);
        let b = self.statement(body);
        self.add(root, b);
        root
    }

    fn catch_clause(&mut self, x: Node) -> N {
        let k = kids(x);
        let root = self.punct(k[0]);
        for y in &k[1..] {
            let part = match y.kind_s() {
                "(" | ")" => self.punct(*y),
                "catch_formal_parameter" => self.catch_parameter(*y),
                "block" => self.block(*y),
                other => panic!("unexpected {other} in catch"),
            };
            self.add(root, part);
        }
        root
    }

    fn catch_parameter(&mut self, x: Node) -> N {
        let def = self.imag(Tok::PARAMETER_DEF);
        let mods = self.modifiers(modifiers_of(x));
        self.add(def, mods);
        let k = kids(x);
        let ct = k.iter().find(|y| y.kind_s() == "catch_type").unwrap();
        let ty = self.imag(Tok::TYPE);
        for y in kids(*ct) {
            let part = if y.kind_s() == "|" {
                self.tok(Tok::BOR, y)
            } else {
                let inner = if y.kind_s() == "annotated_type" { *kids(y).last().unwrap() } else { y };
                self.class_type(inner)
            };
            self.add(ty, part);
        }
        self.add(def, ty);
        let id = self.ident(field(x, "name").unwrap_or_else(|| *k.iter().rev().find(|y| y.kind_s() == "identifier").unwrap()));
        self.add(def, id);
        def
    }

    fn resource_specification(&mut self, x: Node) -> N {
        let k = kids(x);
        let spec = self.imag(Tok::RESOURCE_SPECIFICATION);
        let lparen = self.punct(k[0]);
        self.add(spec, lparen);
        let resources = self.imag(Tok::RESOURCES);
        let mut trailing_semi = NIL;
        let last = k.len() - 1;
        for (i, y) in k.iter().enumerate().take(last).skip(1) {
            match y.kind_s() {
                ";" => {
                    let s = self.punct(*y);
                    if i == last - 1 {
                        trailing_semi = s;
                    } else {
                        self.add(resources, s);
                    }
                }
                _ => {
                    let r = self.resource(*y);
                    self.add(resources, r);
                }
            }
        }
        self.add(spec, resources);
        self.add(spec, trailing_semi);
        let rparen = self.punct(k[last]);
        self.add(spec, rparen);
        spec
    }

    fn resource(&mut self, x: Node) -> N {
        let res = self.imag(Tok::RESOURCE);
        if let Some(value) = field(x, "value") {
            let mods = self.modifiers(modifiers_of(x));
            self.add(res, mods);
            let ty = self.type_type(field(x, "type").unwrap());
            self.add(res, ty);
            let id = self.ident(field(x, "name").unwrap());
            self.add(res, id);
            let eq = kids(x).into_iter().find(|y| y.kind_s() == "=").unwrap();
            let assign = self.punct(eq);
            self.add(res, assign);
            let e = self.expression(value);
            self.add(assign, e);
        } else {
            let e = self.expr(kids(x)[0]);
            self.add(res, e);
        }
        res
    }

    // ---- switch ----

    /// `switchExpressionOrStatement`.
    pub fn switch(&mut self, x: Node) -> N {
        let k = kids(x);
        let root = self.punct(k[0]);
        let cond = self.par_expression(field(x, "condition").unwrap());
        self.add(root, cond);
        let body = field(x, "body").unwrap();
        let bk = kids(body);
        let lcurly = self.punct(bk[0]);
        self.add(root, lcurly);
        let mut pending: Vec<N> = Vec::new();
        let mut i = 1;
        while bk[i].kind_s() != "}" {
            match bk[i].kind_s() {
                "switch_rule" => {
                    let rule = self.imag(Tok::SWITCH_RULE);
                    let rk = kids(bk[i]);
                    let label = self.switch_label(rk[0], None);
                    self.add(rule, label);
                    let lambda = self.punct(rk[1]);
                    self.add(rule, lambda);
                    let b = rk[2];
                    match b.kind_s() {
                        "expression_statement" => {
                            let ek = kids(b);
                            let e = self.expression(ek[0]);
                            self.add(rule, e);
                            let s = self.punct(ek[1]);
                            self.add(rule, s);
                        }
                        "throw_statement" => {
                            let t = self.statement(b);
                            self.add(rule, t);
                        }
                        _ => {
                            let blk = self.block(b);
                            self.add(rule, blk);
                        }
                    }
                    self.add(root, rule);
                }
                "switch_block_statement_group" => {
                    // tree-sitter makes one group per label; `switchBlockStatementGroup` is
                    // `switchLabel+ blockStatement+`, so labels wait for the next statements.
                    let k = kids(bk[i]);
                    let mut j = 0;
                    while j < k.len() && k[j].kind_s() == "switch_label" {
                        let label = self.switch_label(k[j], k.get(j + 1).copied());
                        pending.push(label);
                        j += 2;
                    }
                    if j < k.len() {
                        let group = self.imag(Tok::CASE_GROUP);
                        for l in pending.drain(..) {
                            self.add(group, l);
                        }
                        let slist = self.imag(Tok::SLIST);
                        for y in &k[j..] {
                            let part = self.block_statement(*y);
                            self.add(slist, part);
                        }
                        self.add(group, slist);
                        self.add(root, group);
                    }
                }
                other => panic!("unexpected {other} in switch block"),
            }
            i += 1;
        }
        if !pending.is_empty() {
            let group = self.imag(Tok::CASE_GROUP);
            for l in pending.drain(..) {
                self.add(group, l);
            }
            self.add(root, group);
        }
        let rcurly = self.punct(bk[i]);
        self.add(root, rcurly);
        root
    }

    /// `switchLabel`: `case ...` or `default`, with its `:` (for the old style) as a child.
    fn switch_label(&mut self, l: Node, colon: Option<Node>) -> N {
        let k = kids(l);
        let root = self.punct(k[0]);
        if k[0].kind_s() == "case" {
            let mut parts: Vec<N> = Vec::new();
            for y in &k[1..] {
                match y.kind_s() {
                    "," => parts.push(self.punct(*y)),
                    "pattern" => parts.push(self.pattern(*y)),
                    "guard" => {
                        let last = parts.pop().unwrap();
                        let gk = kids(*y);
                        let when = self.punct(gk[0]);
                        self.add(when, last);
                        let e = self.expression(gk[1]);
                        self.add(when, e);
                        let pd = self.imag(Tok::PATTERN_DEF);
                        self.add(pd, when);
                        parts.push(pd);
                    }
                    // `case null, default`: tree-sitter reads the `default` as an identifier.
                    "identifier" if self.text(*y) == "default" => parts.push(self.tok(Tok::LITERAL_DEFAULT, *y)),
                    _ => parts.push(self.expression(*y)),
                }
            }
            for p in parts {
                self.add(root, p);
            }
        }
        if let Some(c) = colon {
            let cn = self.punct(c);
            self.add(root, cn);
        }
        root
    }

    // ---- patterns ----

    /// A `pattern` (or `record_pattern_component`/`record_pattern`) node.
    pub fn pattern(&mut self, x: Node) -> N {
        match x.kind_s() {
            "pattern" => self.pattern(kids(x)[0]),
            "type_pattern" => {
                let def = self.imag(Tok::PATTERN_VARIABLE_DEF);
                let k = kids(x);
                let mods = self.modifiers(modifiers_of(x));
                self.add(def, mods);
                let tpos = k.iter().position(|y| y.kind_s() != "modifiers").unwrap();
                let ty = self.type_type(k[tpos]);
                self.add(def, ty);
                let id = self.ident(*k.last().unwrap());
                self.add(def, id);
                def
            }
            "record_pattern_component" => {
                let k = kids(x);
                if k[0].kind_s() == "underscore_pattern" {
                    return self.tok(Tok::UNNAMED_PATTERN_DEF, k[0]);
                }
                let def = self.imag(Tok::PATTERN_VARIABLE_DEF);
                let mods = self.modifiers(modifiers_of(x));
                self.add(def, mods);
                let tpos = k.iter().position(|y| y.kind_s() != "modifiers").unwrap();
                let ty = self.type_type(k[tpos]);
                self.add(def, ty);
                let id = self.ident(*k.last().unwrap());
                self.add(def, id);
                def
            }
            "record_pattern" => {
                let def = self.imag(Tok::RECORD_PATTERN_DEF);
                let k = kids(x);
                let mods = self.modifiers(None);
                self.add(def, mods);
                let ty = self.type_type(k[0]);
                self.add(def, ty);
                let body = kids(k[1]);
                let lparen = self.punct(body[0]);
                self.add(def, lparen);
                if body.len() > 2 {
                    let comps = self.imag(Tok::RECORD_PATTERN_COMPONENTS);
                    for y in &body[1..body.len() - 1] {
                        let part = if y.kind_s() == "," { self.punct(*y) } else { self.pattern(*y) };
                        self.add(comps, part);
                    }
                    self.add(def, comps);
                }
                let rparen = self.punct(*body.last().unwrap());
                self.add(def, rparen);
                def
            }
            other => panic!("unexpected pattern {other}"),
        }
    }
}
