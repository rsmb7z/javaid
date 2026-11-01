// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/JavaAstVisitor.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Package, imports, type declarations, members, parameters.

use tree_sitter::Node;

use super::types::is_annotation;
use super::{B, kids};
use crate::csast::ast::{N, NIL};
use crate::csast::tok::Tok;

fn field<'t>(n: Node<'t>, name: &str) -> Option<Node<'t>> {
    n.child_by_field_name(name)
}

fn modifiers_of<'t>(n: Node<'t>) -> Option<Node<'t>> {
    kids(n).into_iter().find(|k| k.kind() == "modifiers")
}

impl<'s> B<'s> {
    pub fn package_declaration(&mut self, n: Node) -> N {
        let k = kids(n);
        let kw = k.iter().find(|x| x.kind() == "package").unwrap();
        let def = self.tok(Tok::PACKAGE_DEF, *kw);
        let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        let an = self.annotations(&annos, true);
        self.add(def, an);
        let name = k.iter().find(|x| matches!(x.kind(), "identifier" | "scoped_identifier")).unwrap();
        let q = self.qualified_name(*name);
        self.add(def, q);
        let semi = self.punct(*k.last().unwrap());
        self.add(def, semi);
        def
    }

    pub fn import_declaration(&mut self, n: Node) -> N {
        let k = kids(n);
        let root = self.tok(Tok::IMPORT, k[0]);
        let mut i = 1;
        match k[i].kind() {
            "static" => {
                self.a.set_type(root, Tok::STATIC_IMPORT);
                let s = self.punct(k[i]);
                self.add(root, s);
                i += 1;
            }
            "module" => {
                self.a.set_type(root, Tok::MODULE_IMPORT);
                let s = self.punct(k[i]);
                self.add(root, s);
                i += 1;
            }
            _ => {}
        }
        let name = k[i];
        let q = self.qualified_name(name);
        if k.get(i + 1).map(|x| x.kind()) == Some(".") {
            let dot = self.punct(k[i + 1]);
            self.add(dot, q);
            let star = self.tok(Tok::STAR, k[i + 2]);
            self.add(dot, star);
            self.add(root, dot);
        } else {
            self.add(root, q);
        }
        let semi = self.punct(*k.last().unwrap());
        self.add(root, semi);
        root
    }

    pub fn module_declaration(&mut self, n: Node) -> N {
        let def = self.imag(Tok::MODULE_DEF);
        let k = kids(n);
        let annos: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        let an = self.annotations(&annos, true);
        self.add(def, an);
        for x in k.iter().filter(|x| !is_annotation(**x)) {
            let part = match x.kind() {
                "open" => self.tok(Tok::LITERAL_OPEN, *x),
                "module" => self.tok(Tok::LITERAL_MODULE, *x),
                "module_body" => self.module_body(*x),
                _ => self.qualified_name(*x),
            };
            self.add(def, part);
        }
        def
    }

    fn module_body(&mut self, n: Node) -> N {
        let block = self.imag(Tok::DIRECTIVE_BLOCK);
        for x in kids(n) {
            let part = match x.kind() {
                "{" | "}" => self.punct(x),
                _ => self.module_directive(x),
            };
            self.add(block, part);
        }
        block
    }

    fn module_directive(&mut self, n: Node) -> N {
        let k = kids(n);
        let ty = match n.kind() {
            "requires_module_directive" => Tok::REQUIRES,
            "exports_module_directive" => Tok::EXPORTS,
            "opens_module_directive" => Tok::OPENS,
            "uses_module_directive" => Tok::USES,
            _ => Tok::PROVIDES,
        };
        let root = self.tok(ty, k[0]);
        let mut holder = root;
        for x in &k[1..] {
            let part = match x.kind() {
                "requires_modifier" => {
                    let text = self.text(*x);
                    let t = if text == "transitive" { Tok::LITERAL_TRANSITIVE } else { Tok::LITERAL_STATIC };
                    self.tok(t, *x)
                }
                "to" => {
                    let to = self.tok(Tok::TO, *x);
                    holder = to;
                    self.add(root, to);
                    continue;
                }
                "with" => {
                    let w = self.tok(Tok::WITH, *x);
                    holder = w;
                    self.add(root, w);
                    continue;
                }
                ";" => {
                    holder = root;
                    self.punct(*x)
                }
                "," => self.punct(*x),
                _ => self.qualified_name(*x),
            };
            self.add(holder, part);
        }
        root
    }

    // ---- type declarations ----

    pub fn type_declaration(&mut self, n: Node) -> N {
        let (ty, bodyless) = match n.kind() {
            "class_declaration" => (Tok::CLASS_DEF, false),
            "interface_declaration" => (Tok::INTERFACE_DEF, false),
            "enum_declaration" => (Tok::ENUM_DEF, false),
            "record_declaration" => (Tok::RECORD_DEF, false),
            _ => (Tok::ANNOTATION_DEF, false),
        };
        let _ = bodyless;
        let def = self.imag(ty);
        let mods = self.modifiers(modifiers_of(n));
        self.add(def, mods);
        let is_record = ty == Tok::RECORD_DEF;
        for x in kids(n) {
            let part = match x.kind() {
                "modifiers" => continue,
                "class" | "interface" | "enum" | "record" | "@" => self.punct(x),
                "identifier" => self.ident(x),
                "type_parameters" => self.type_parameters(x),
                "superclass" => {
                    let k = kids(x);
                    let ext = self.punct(k[0]);
                    let c = self.type_content(k[1]);
                    self.add(ext, c);
                    ext
                }
                "super_interfaces" => {
                    let k = kids(x);
                    let imp = self.tok(Tok::IMPLEMENTS_CLAUSE, k[0]);
                    self.type_list_into(imp, k[1]);
                    imp
                }
                "extends_interfaces" => {
                    let k = kids(x);
                    let ext = self.punct(k[0]);
                    self.type_list_into(ext, k[1]);
                    ext
                }
                "permits" => self.permits(x),
                "formal_parameters" if is_record => self.record_components(x),
                "class_body" | "interface_body" | "enum_body" | "annotation_type_body" => self.obj_block(x),
                other => panic!("unexpected {other} in type declaration"),
            };
            self.add(def, part);
        }
        def
    }

    fn permits(&mut self, x: Node) -> N {
        let k = kids(x);
        let p = self.tok(Tok::PERMITS_CLAUSE, k[0]);
        for y in kids(k[1]) {
            let part = if y.kind() == "," { self.punct(y) } else { self.class_type(y) };
            self.add(p, part);
        }
        p
    }

    /// `recordComponentsList`: `(` `RECORD_COMPONENTS` `)`.
    fn record_components(&mut self, x: Node) -> N {
        let k = kids(x);
        let lparen = self.punct(k[0]);
        let comps = self.imag(Tok::RECORD_COMPONENTS);
        for y in &k[1..k.len() - 1] {
            let part = match y.kind() {
                "," => self.punct(*y),
                _ => self.record_component(*y),
            };
            self.add(comps, part);
        }
        let rparen = self.punct(*k.last().unwrap());
        self.chain(&[lparen, comps, rparen])
    }

    fn record_component(&mut self, y: Node) -> N {
        let rc = self.imag(Tok::RECORD_COMPONENT_DEF);
        let ck = kids(y);
        let annos: Vec<Node> = match modifiers_of(y) {
            Some(m) => kids(m).into_iter().filter(|a| is_annotation(*a)).collect(),
            None => vec![],
        };
        let an = self.annotations(&annos, true);
        self.add(rc, an);
        if y.kind() == "spread_parameter" {
            let ty = self.imag(Tok::TYPE);
            let c = self.type_content(field(y, "type").unwrap());
            self.add(ty, c);
            let post: Vec<Node> = ck.iter().copied().filter(|a| is_annotation(*a)).collect();
            let pa = self.annotations(&post, false);
            self.add(ty, pa);
            self.add(rc, ty);
            let ell = ck.iter().find(|a| a.kind() == "...").unwrap();
            let e = self.punct(*ell);
            self.add(rc, e);
            let decl = ck.iter().find(|a| a.kind() == "variable_declarator").unwrap();
            let id = self.ident(field(*decl, "name").unwrap());
            self.add(rc, id);
        } else {
            let ty = self.type_type(field(y, "type").unwrap());
            self.add(rc, ty);
            let id = self.ident(field(y, "name").unwrap());
            self.add(rc, id);
            if let Some(d) = field(y, "dimensions") {
                for decl in self.dimensions(d) {
                    self.add(ty, decl);
                }
            }
        }
        rc
    }

    /// The `OBJBLOCK` of any class-like body.
    pub fn obj_block(&mut self, body: Node) -> N {
        let ob = self.imag(Tok::OBJBLOCK);
        for x in kids(body) {
            match x.kind() {
                "{" | "}" | "," => {
                    let t = self.punct(x);
                    self.add(ob, t);
                }
                "enum_body_declarations" => {
                    for y in kids(x) {
                        let part = self.member(y);
                        self.add(ob, part);
                    }
                }
                _ => {
                    let part = self.member(x);
                    self.add(ob, part);
                }
            }
        }
        ob
    }

    fn member(&mut self, x: Node) -> N {
        match x.kind() {
            ";" => self.punct(x),
            "field_declaration" | "constant_declaration" => self.field_declaration(x),
            "method_declaration" => self.method_declaration(x),
            "constructor_declaration" => self.constructor_declaration(x),
            "compact_constructor_declaration" => {
                let def = self.imag(Tok::COMPACT_CTOR_DEF);
                let mods = self.modifiers(modifiers_of(x));
                self.add(def, mods);
                let id = self.ident(field(x, "name").unwrap());
                self.add(def, id);
                let b = self.block(field(x, "body").unwrap());
                self.add(def, b);
                def
            }
            "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
                self.type_declaration(x)
            }
            "block" => {
                let init = self.imag(Tok::INSTANCE_INIT);
                let b = self.block(x);
                self.add(init, b);
                init
            }
            "static_initializer" => {
                let k = kids(x);
                let init = self.tok_text(Tok::STATIC_INIT, k[0], "STATIC_INIT");
                let b = self.block(k[1]);
                self.add(init, b);
                init
            }
            "enum_constant" => self.enum_constant(x),
            "annotation_type_element_declaration" => self.annotation_field(x),
            other => panic!("unexpected member {other}"),
        }
    }

    fn enum_constant(&mut self, x: Node) -> N {
        let def = self.imag(Tok::ENUM_CONSTANT_DEF);
        let annos: Vec<Node> = match modifiers_of(x) {
            Some(m) => kids(m).into_iter().filter(|a| is_annotation(*a)).collect(),
            None => vec![],
        };
        let an = self.annotations(&annos, true);
        self.add(def, an);
        for y in kids(x) {
            let part = match y.kind() {
                "modifiers" => continue,
                "identifier" => self.ident(y),
                "argument_list" => self.arguments(y),
                "class_body" => self.obj_block(y),
                other => panic!("unexpected {other} in enum constant"),
            };
            self.add(def, part);
        }
        def
    }

    fn annotation_field(&mut self, x: Node) -> N {
        let def = self.imag(Tok::ANNOTATION_FIELD_DEF);
        let mods = self.modifiers(modifiers_of(x));
        self.add(def, mods);
        let ty = self.type_type(field(x, "type").unwrap());
        self.add(def, ty);
        let k = kids(x);
        let mut seen_name = false;
        for y in &k {
            let part = match y.kind() {
                "modifiers" => continue,
                "(" | ")" => self.punct(*y),
                "identifier" if !seen_name && Some(*y) == field(x, "name") => {
                    seen_name = true;
                    self.ident(*y)
                }
                "default" => {
                    let d = self.punct(*y);
                    let v = self.default_value(&k, y.id());
                    self.add(d, v);
                    d
                }
                _ => continue,
            };
            self.add(def, part);
        }
        if let Some(d) = field(x, "dimensions") {
            for decl in self.dimensions(d) {
                self.add(ty, decl);
            }
        }
        let semi = self.punct(*k.last().unwrap());
        self.add(def, semi);
        def
    }

    fn default_value(&mut self, k: &[Node], default_id: usize) -> N {
        let pos = k.iter().position(|y| y.id() == default_id).unwrap();
        self.element_value(k[pos + 1])
    }

    // ---- fields and variables ----

    /// `fieldDeclaration` (also used for locals): a `VARIABLE_DEF` chain; the `;` hangs off the first.
    pub fn field_declaration(&mut self, n: Node) -> N {
        let chain = self.variable_declarators(n);
        let semi = kids(n).into_iter().rev().find(|x| x.kind() == ";");
        if let Some(s) = semi {
            let st = self.punct(s);
            self.add(chain, st);
        }
        chain
    }

    /// `variableDeclarators`: `VARIABLE_DEF`s with `COMMA`s between, without the `;`.
    pub fn variable_declarators(&mut self, n: Node) -> N {
        let mods = modifiers_of(n);
        let tnode = field(n, "type").unwrap();
        let mut parts = Vec::new();
        for x in kids(n) {
            match x.kind() {
                "variable_declarator" => parts.push(self.variable_def(mods, tnode, x)),
                "," => parts.push(self.punct(x)),
                _ => {}
            }
        }
        self.chain(&parts)
    }

    pub fn variable_def(&mut self, mods: Option<Node>, tnode: Node, d: Node) -> N {
        let def = self.imag(Tok::VARIABLE_DEF);
        let m = self.modifiers(mods);
        self.add(def, m);
        let ty = self.type_type(tnode);
        self.add(def, ty);
        let id = self.ident(field(d, "name").unwrap());
        self.add(def, id);
        if let Some(dims) = field(d, "dimensions") {
            for decl in self.dimensions(dims) {
                self.add(ty, decl);
            }
        }
        if let Some(v) = field(d, "value") {
            let eq = kids(d).into_iter().find(|x| x.kind() == "=").unwrap();
            let assign = self.punct(eq);
            self.add(def, assign);
            let init = self.variable_initializer(v);
            self.add(assign, init);
        }
        def
    }

    pub fn variable_initializer(&mut self, v: Node) -> N {
        if v.kind() == "array_initializer" {
            self.array_initializer(v)
        } else {
            self.expression(v)
        }
    }

    pub fn array_initializer(&mut self, v: Node) -> N {
        let k = kids(v);
        let init = self.tok_text(Tok::ARRAY_INIT, k[0], "{");
        for x in &k[1..] {
            let part = match x.kind() {
                "," | "}" => self.punct(*x),
                _ => self.variable_initializer(*x),
            };
            self.add(init, part);
        }
        init
    }

    // ---- methods ----

    pub fn method_declaration(&mut self, n: Node) -> N {
        let def = self.imag(Tok::METHOD_DEF);
        let mods = self.modifiers(modifiers_of(n));
        self.add(def, mods);
        let k = kids(n);
        let tnode = field(n, "type").unwrap();
        // Annotations that are direct children sit between the type parameters and the type.
        let post: Vec<Node> = k.iter().copied().filter(|x| is_annotation(*x)).collect();
        let mut ty = NIL;
        for x in &k {
            match x.kind() {
                "modifiers" => {}
                _ if is_annotation(*x) => {}
                "type_parameters" => {
                    let tp = self.type_parameters(*x);
                    self.add(def, tp);
                }
                _ if x.id() == tnode.id() => {
                    ty = self.imag(Tok::TYPE);
                    let an = self.annotations(&post, false);
                    self.add(ty, an);
                    let c = self.type_content(tnode);
                    self.add(ty, c);
                    self.add(def, ty);
                }
                "identifier" => {
                    let id = self.ident(*x);
                    self.add(def, id);
                }
                "formal_parameters" => {
                    let p = self.formal_parameters(*x);
                    self.add(def, p);
                }
                "dimensions" => {}
                "throws" => {
                    let t = self.throws(*x);
                    self.add(def, t);
                }
                "block" => {
                    let b = self.block(*x);
                    self.add(def, b);
                }
                ";" => {
                    let s = self.punct(*x);
                    self.add(def, s);
                }
                other => panic!("unexpected {other} in method"),
            }
        }
        if let Some(d) = field(n, "dimensions") {
            for decl in self.dimensions(d) {
                self.add(ty, decl);
            }
        }
        def
    }

    fn throws(&mut self, x: Node) -> N {
        let k = kids(x);
        let t = self.punct(k[0]);
        for y in &k[1..] {
            let part = if y.kind() == "," { self.punct(*y) } else { self.type_content(*y) };
            self.add(t, part);
        }
        t
    }

    fn constructor_declaration(&mut self, n: Node) -> N {
        let def = self.imag(Tok::CTOR_DEF);
        let mods = self.modifiers(modifiers_of(n));
        self.add(def, mods);
        for x in kids(n) {
            let part = match x.kind() {
                "modifiers" => continue,
                "type_parameters" => self.type_parameters(x),
                "identifier" => self.ident(x),
                "formal_parameters" => self.formal_parameters(x),
                "throws" => self.throws(x),
                "constructor_body" => self.constructor_body(x),
                other => panic!("unexpected {other} in constructor"),
            };
            self.add(def, part);
        }
        def
    }

    /// `formalParameters`: `(` `PARAMETERS` `)` as a chain.
    pub fn formal_parameters(&mut self, n: Node) -> N {
        let k = kids(n);
        let lparen = self.punct(k[0]);
        let params = self.imag(Tok::PARAMETERS);
        for x in &k[1..k.len() - 1] {
            let part = match x.kind() {
                "," => self.punct(*x),
                "formal_parameter" => self.formal_parameter(*x),
                "spread_parameter" => self.spread_parameter(*x),
                "receiver_parameter" => self.receiver_parameter(*x),
                other => panic!("unexpected {other} in parameters"),
            };
            self.add(params, part);
        }
        let rparen = self.punct(*k.last().unwrap());
        self.chain(&[lparen, params, rparen])
    }

    fn formal_parameter(&mut self, x: Node) -> N {
        let def = self.imag(Tok::PARAMETER_DEF);
        let mods = self.modifiers(modifiers_of(x));
        self.add(def, mods);
        let ty = self.type_type(field(x, "type").unwrap());
        self.add(def, ty);
        let name = field(x, "name").unwrap();
        // An annotated receiver parameter (`@A Foo this`) parses as a parameter named `this`.
        let id = if self.text(name) == "this" { self.tok(Tok::LITERAL_THIS, name) } else { self.ident(name) };
        self.add(def, id);
        if let Some(d) = field(x, "dimensions") {
            for decl in self.dimensions(d) {
                self.add(ty, decl);
            }
        }
        def
    }

    fn spread_parameter(&mut self, x: Node) -> N {
        let def = self.imag(Tok::PARAMETER_DEF);
        let k = kids(x);
        let mods = self.modifiers(modifiers_of(x));
        self.add(def, mods);
        let ty = self.type_type(field(x, "type").unwrap());
        self.add(def, ty);
        let decl = k.iter().find(|a| a.kind() == "variable_declarator").unwrap();
        let ell = k.iter().find(|a| a.kind() == "...").unwrap();
        let e = self.punct(*ell);
        self.add(def, e);
        let id = self.ident(field(*decl, "name").unwrap());
        self.add(def, id);
        if let Some(d) = field(*decl, "dimensions") {
            for dd in self.dimensions(d) {
                self.add(ty, dd);
            }
        }
        let post: Vec<Node> = k.iter().copied().filter(|a| is_annotation(*a)).collect();
        let pa = self.annotations(&post, false);
        self.add(ty, pa);
        def
    }

    fn receiver_parameter(&mut self, x: Node) -> N {
        let def = self.imag(Tok::PARAMETER_DEF);
        let k = kids(x);
        let annos: Vec<Node> = k.iter().copied().filter(|a| is_annotation(*a)).collect();
        let mods = self.imag(Tok::MODIFIERS);
        for a in annos {
            let an = self.annotation(a);
            self.add(mods, an);
        }
        self.add(def, mods);
        let tpos = k.iter().position(|a| !is_annotation(*a)).unwrap();
        let ty = self.type_type(k[tpos]);
        self.add(def, ty);
        let this = k.iter().find(|a| a.kind() == "this").unwrap();
        let idents: Vec<Node> = k[tpos + 1..].iter().copied().filter(|a| a.kind() == "identifier").collect();
        if idents.is_empty() {
            let t = self.punct(*this);
            self.add(def, t);
        } else {
            // qualifiedName DOT this
            let dots: Vec<Node> = k[tpos + 1..].iter().copied().filter(|a| a.kind() == ".").collect();
            let mut q = self.ident(idents[0]);
            for (i, id) in idents.iter().enumerate().skip(1) {
                let dot = self.punct(dots[i - 1]);
                self.add(dot, q);
                let idn = self.ident(*id);
                self.add(dot, idn);
                q = dot;
            }
            let dot = self.punct(*dots.last().unwrap());
            self.add(dot, q);
            let t = self.punct(*this);
            self.add(dot, t);
            self.add(def, dot);
        }
        def
    }
}
