//! A small XPath 3.1 evaluator over the emulated Checkstyle AST (`csast`).
//!
//! Checkstyle evaluates `MatchXpath`, `SuppressionXpathFilter` and `SuppressionXpathSingleFilter`
//! queries with Saxon over a thin wrapper of its `DetailAST` (`xpath/*Node.java`). The wrapper
//! decides what a query sees, and this module reproduces that, not generic XML:
//!
//! * the context is a document node whose children are the AST's top-level nodes; every `DetailAST`
//!   is an element named after its token type (`METHOD_DEF`, `IDENT`, ...);
//! * the only attribute is `@text`, on IDENT, STRING_LITERAL (quotes removed), CHAR_LITERAL,
//!   NUM_LONG, NUM_INT, NUM_DOUBLE, NUM_FLOAT, TEXT_BLOCK_CONTENT and COMMENT_CONTENT; a newline or
//!   carriage return in it reads as the two characters `\n` / `\r`;
//! * elements cannot be atomized: `//IDENT = 'x'` or `string(.)` on an element fail in Checkstyle
//!   (Saxon's `getStringValue` is unsupported on the wrapper) and fail here as well;
//! * the descendant axes walk breadth-first, `preceding` also returns the ancestors, and union
//!   orders nodes by depth first (`AbstractElementNode.compareOrder`). All reproduced.
//!
//! # Supported
//!
//! Paths with `/`, `//`, `.`, `..`, all axes but `namespace` (`child`, `descendant`,
//! `descendant-or-self`, `parent`, `ancestor`, `ancestor-or-self`, `following-sibling`,
//! `preceding-sibling`, `following`, `preceding`, `self`, `attribute`/`@`), name tests, `*`, `@*`,
//! `node()`, `element()`, `element(NAME)`, `attribute()`, `document-node()`, `text()`/`comment()`/
//! `processing-instruction()` (which select nothing); predicates, positional ones included;
//! parenthesised sequences and `,`; string, integer, decimal and double literals; `or`, `and`,
//! general comparisons (`= != < <= > >=`), value comparisons (`eq ne lt le gt ge`), `||`, `to`,
//! `+ - * div idiv mod`, unary `+ -`, `| union intersect except`, the simple map `!`,
//! `if/then/else`, `for`, `let`, `some`/`every`, variables bound by those, and the functions
//! `true false not boolean string concat string-join string-length substring substring-before
//! substring-after contains starts-with ends-with normalize-space upper-case lower-case translate
//! matches number sum avg min max abs ceiling floor round count empty exists position last
//! distinct-values reverse head tail subsequence data local-name` (without a prefix: Checkstyle's context declares none).
//!
//! # Refused when the query is compiled
//!
//! Everything else: the `namespace` axis, prefixed names, `cast`, `treat`, `instance of`, `castable`,
//! `is`, `<<`, `>>`, `=>`, inline functions, maps, arrays, `?`, and every function not listed above
//! (among them `replace`, `tokenize`, `name`, `string-to-codepoints`, `id`, `doc`, `root`).
//!
//! Dynamic errors (a type error, an atomized element, a navigation from an attribute...) are
//! returned as `Err`; Checkstyle aborts the run with an `IllegalStateException` for those.

use crate::csast::{CsAst, Tok, N, NIL};
use std::collections::VecDeque;

/// A compiled query.
#[derive(Clone, Debug)]
pub struct XPath {
    pub source: String,
    expr: Expr,
}

/// A node of the XPath tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeRef {
    /// The document node, parent of the AST's top-level nodes.
    Doc,
    El(N),
    /// The `@text` attribute of an element.
    Attr(N),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Int,
    Dec,
    Dbl,
}

#[derive(Clone, Debug)]
enum Item {
    Node(NodeRef),
    Str(String),
    Untyped(String),
    Num(f64, Kind),
    Bool(bool),
}

type Seq = Vec<Item>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Child,
    Descendant,
    DescendantOrSelf,
    Parent,
    Ancestor,
    AncestorOrSelf,
    FollowingSibling,
    PrecedingSibling,
    Following,
    Preceding,
    SelfAxis,
    Attribute,
}

#[derive(Clone, Debug)]
enum Test {
    Name(String),
    AnyName,
    AnyNode,
    Never,
    Element(Option<String>),
    Document,
    Attribute,
}

#[derive(Clone, Debug)]
enum Step {
    Axis { axis: Axis, test: Test, preds: Vec<Expr> },
    Expr(Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arith {
    Add,
    Sub,
    Mul,
    Div,
    IDiv,
    Mod,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SetOp {
    Union,
    Intersect,
    Except,
}

#[derive(Clone, Debug)]
enum Expr {
    Str(String),
    Num(f64, Kind),
    Var(String),
    Context,
    Seq(Vec<Expr>),
    Path { absolute: bool, steps: Vec<Step> },
    Filter(Box<Expr>, Vec<Expr>),
    Call(&'static str, Vec<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    General(Cmp, Box<Expr>, Box<Expr>),
    Value(Cmp, Box<Expr>, Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    Range(Box<Expr>, Box<Expr>),
    Arith(Arith, Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Plus(Box<Expr>),
    Set(SetOp, Box<Expr>, Box<Expr>),
    Map(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Quant(bool, String, Box<Expr>, Box<Expr>),
}

// ---------------------------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum Tk {
    Str(String),
    Num(f64, Kind),
    Name(String),
    Var(String),
    Sym(&'static str),
    End,
}

const SYMS: &[&str] = &[
    "//", "::", "..", "||", "!=", "<=", ">=", "<<", ">>", ":=", "=>", "/", "(", ")", "[", "]", "@", ",", ".", "|", "+", "-", "*", "=", "<", ">", "!", "?", "{", "}", "#", "$",
];

fn is_name_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '\u{b7}')
}

fn lex(src: &str) -> Result<Vec<Tk>, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '(' && chars.get(i + 1) == Some(&':') {
            // A comment, which nests.
            let mut depth = 1;
            i += 2;
            while depth > 0 {
                match (chars.get(i), chars.get(i + 1)) {
                    (Some('('), Some(':')) => {
                        depth += 1;
                        i += 2;
                    }
                    (Some(':'), Some(')')) => {
                        depth -= 1;
                        i += 2;
                    }
                    (Some(_), _) => i += 1,
                    (None, _) => return Err("unterminated comment".to_string()),
                }
            }
        } else if c == '"' || c == '\'' {
            let mut s = String::new();
            i += 1;
            loop {
                match chars.get(i) {
                    None => return Err("unterminated string literal".to_string()),
                    Some(&q) if q == c => {
                        if chars.get(i + 1) == Some(&c) {
                            s.push(c);
                            i += 2;
                        } else {
                            i += 1;
                            break;
                        }
                    }
                    Some(&ch) => {
                        s.push(ch);
                        i += 1;
                    }
                }
            }
            out.push(Tk::Str(s));
        } else if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit())) {
            let start = i;
            let mut kind = Kind::Int;
            while chars.get(i).is_some_and(|d| d.is_ascii_digit()) {
                i += 1;
            }
            if chars.get(i) == Some(&'.') {
                kind = Kind::Dec;
                i += 1;
                while chars.get(i).is_some_and(|d| d.is_ascii_digit()) {
                    i += 1;
                }
            }
            if matches!(chars.get(i), Some('e' | 'E')) {
                let mut j = i + 1;
                if matches!(chars.get(j), Some('+' | '-')) {
                    j += 1;
                }
                if chars.get(j).is_some_and(|d| d.is_ascii_digit()) {
                    while chars.get(j).is_some_and(|d| d.is_ascii_digit()) {
                        j += 1;
                    }
                    kind = Kind::Dbl;
                    i = j;
                }
            }
            let text: String = chars[start..i].iter().collect();
            let v: f64 = text.parse().map_err(|_| format!("bad number '{text}'"))?;
            out.push(Tk::Num(v, kind));
        } else if is_name_start(c) {
            let start = i;
            while chars.get(i).is_some_and(|&d| is_name_char(d)) {
                i += 1;
            }
            // A name must not end in '.': `a.` is `a` then `.`? XPath lets names contain dots; keep them.
            // A prefix: `fn:name` (but not `axis::`).
            if chars.get(i) == Some(&':') && chars.get(i + 1) != Some(&':') && chars.get(i + 1).is_some_and(|&d| is_name_start(d)) {
                i += 1;
                while chars.get(i).is_some_and(|&d| is_name_char(d)) {
                    i += 1;
                }
            }
            out.push(Tk::Name(chars[start..i].iter().collect()));
        } else if c == '$' {
            i += 1;
            let start = i;
            while chars.get(i).is_some_and(|&d| is_name_char(d)) {
                i += 1;
            }
            if start == i {
                return Err("'$' must be followed by a variable name".to_string());
            }
            out.push(Tk::Var(chars[start..i].iter().collect()));
        } else {
            let rest: String = chars[i..chars.len().min(i + 2)].iter().collect();
            match SYMS.iter().find(|s| rest.starts_with(**s)) {
                Some(s) => {
                    out.push(Tk::Sym(s));
                    i += s.chars().count();
                }
                None => return Err(format!("unexpected character '{c}'")),
            }
        }
    }
    out.push(Tk::End);
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------------------------

/// `(name, minimum arguments, maximum arguments)`
const FUNCTIONS: &[(&str, usize, usize)] = &[
    ("true", 0, 0),
    ("false", 0, 0),
    ("not", 1, 1),
    ("boolean", 1, 1),
    ("string", 0, 1),
    ("concat", 2, usize::MAX),
    ("string-join", 1, 2),
    ("string-length", 0, 1),
    ("substring", 2, 3),
    ("substring-before", 2, 2),
    ("substring-after", 2, 2),
    ("contains", 2, 2),
    ("starts-with", 2, 2),
    ("ends-with", 2, 2),
    ("normalize-space", 0, 1),
    ("upper-case", 1, 1),
    ("lower-case", 1, 1),
    ("translate", 3, 3),
    ("matches", 2, 3),
    ("number", 0, 1),
    ("sum", 1, 2),
    ("avg", 1, 1),
    ("min", 1, 1),
    ("max", 1, 1),
    ("abs", 1, 1),
    ("ceiling", 1, 1),
    ("floor", 1, 1),
    ("round", 1, 2),
    ("count", 1, 1),
    ("empty", 1, 1),
    ("exists", 1, 1),
    ("position", 0, 0),
    ("last", 0, 0),
    ("distinct-values", 1, 1),
    ("reverse", 1, 1),
    ("head", 1, 1),
    ("tail", 1, 1),
    ("subsequence", 2, 3),
    ("data", 0, 1),
    ("local-name", 0, 1),
];

struct Parser {
    toks: Vec<Tk>,
    pos: usize,
}

fn boxed(e: Expr) -> Box<Expr> {
    Box::new(e)
}

impl Parser {
    fn peek(&self) -> &Tk {
        &self.toks[self.pos]
    }

    fn peek_at(&self, n: usize) -> &Tk {
        self.toks.get(self.pos + n).unwrap_or(&Tk::End)
    }

    fn bump(&mut self) -> Tk {
        let t = self.toks[self.pos].clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn sym(&self, s: &str) -> bool {
        matches!(self.peek(), Tk::Sym(x) if *x == s)
    }

    fn eat_sym(&mut self, s: &str) -> bool {
        if self.sym(s) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect_sym(&mut self, s: &str) -> Result<(), String> {
        if self.eat_sym(s) { Ok(()) } else { Err(format!("expected '{s}', found {}", self.describe())) }
    }

    fn word(&self, w: &str) -> bool {
        matches!(self.peek(), Tk::Name(n) if n == w)
    }

    fn eat_word(&mut self, w: &str) -> bool {
        if self.word(w) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect_word(&mut self, w: &str) -> Result<(), String> {
        if self.eat_word(w) { Ok(()) } else { Err(format!("expected '{w}', found {}", self.describe())) }
    }

    fn describe(&self) -> String {
        match self.peek() {
            Tk::Str(s) => format!("string '{s}'"),
            Tk::Num(v, _) => format!("number {v}"),
            Tk::Name(n) => format!("'{n}'"),
            Tk::Var(v) => format!("'${v}'"),
            Tk::Sym(s) => format!("'{s}'"),
            Tk::End => "the end of the query".to_string(),
        }
    }

    fn expr(&mut self) -> Result<Expr, String> {
        let first = self.expr_single()?;
        if !self.sym(",") {
            return Ok(first);
        }
        let mut all = vec![first];
        while self.eat_sym(",") {
            all.push(self.expr_single()?);
        }
        Ok(Expr::Seq(all))
    }

    fn var_binding(&mut self) -> Result<String, String> {
        match self.bump() {
            Tk::Var(v) => Ok(v),
            _ => Err("expected a variable".to_string()),
        }
    }

    fn expr_single(&mut self) -> Result<Expr, String> {
        // `for $x in` / `let $x :=` / `some $x in` / `every $x in` / `if (`
        if let Tk::Name(n) = self.peek().clone() {
            let next_is_var = matches!(self.peek_at(1), Tk::Var(_));
            match n.as_str() {
                "for" if next_is_var => {
                    self.bump();
                    let v = self.var_binding()?;
                    self.expect_word("in")?;
                    let seq = self.expr_single()?;
                    // `for $a in x, $b in y return` is sugar for nesting.
                    if self.eat_sym(",") {
                        self.toks.insert(self.pos, Tk::Name("for".to_string()));
                        let rest = self.expr_single()?;
                        return Ok(Expr::For(v, boxed(seq), boxed(rest)));
                    }
                    self.expect_word("return")?;
                    let ret = self.expr_single()?;
                    return Ok(Expr::For(v, boxed(seq), boxed(ret)));
                }
                "let" if next_is_var => {
                    self.bump();
                    let v = self.var_binding()?;
                    self.expect_sym(":=")?;
                    let value = self.expr_single()?;
                    if self.eat_sym(",") {
                        self.toks.insert(self.pos, Tk::Name("let".to_string()));
                        let rest = self.expr_single()?;
                        return Ok(Expr::Let(v, boxed(value), boxed(rest)));
                    }
                    self.expect_word("return")?;
                    let ret = self.expr_single()?;
                    return Ok(Expr::Let(v, boxed(value), boxed(ret)));
                }
                "some" | "every" if next_is_var => {
                    self.bump();
                    let v = self.var_binding()?;
                    self.expect_word("in")?;
                    let seq = self.expr_single()?;
                    if self.eat_sym(",") {
                        self.toks.insert(self.pos, Tk::Name(n.clone()));
                        let rest = self.expr_single()?;
                        return Ok(Expr::Quant(n == "every", v, boxed(seq), boxed(rest)));
                    }
                    self.expect_word("satisfies")?;
                    let sat = self.expr_single()?;
                    return Ok(Expr::Quant(n == "every", v, boxed(seq), boxed(sat)));
                }
                "if" if matches!(self.peek_at(1), Tk::Sym("(")) => {
                    self.bump();
                    self.bump();
                    let cond = self.expr()?;
                    self.expect_sym(")")?;
                    self.expect_word("then")?;
                    let a = self.expr_single()?;
                    self.expect_word("else")?;
                    let b = self.expr_single()?;
                    return Ok(Expr::If(boxed(cond), boxed(a), boxed(b)));
                }
                _ => {}
            }
        }
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.and_expr()?;
        while self.eat_word("or") {
            left = Expr::Or(boxed(left), boxed(self.and_expr()?));
        }
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;
        while self.eat_word("and") {
            left = Expr::And(boxed(left), boxed(self.comparison()?));
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let left = self.concat()?;
        let general = match self.peek() {
            Tk::Sym("=") => Some(Cmp::Eq),
            Tk::Sym("!=") => Some(Cmp::Ne),
            Tk::Sym("<") => Some(Cmp::Lt),
            Tk::Sym("<=") => Some(Cmp::Le),
            Tk::Sym(">") => Some(Cmp::Gt),
            Tk::Sym(">=") => Some(Cmp::Ge),
            _ => None,
        };
        if let Some(op) = general {
            self.bump();
            return Ok(Expr::General(op, boxed(left), boxed(self.concat()?)));
        }
        let value = match self.peek() {
            Tk::Name(n) => match n.as_str() {
                "eq" => Some(Cmp::Eq),
                "ne" => Some(Cmp::Ne),
                "lt" => Some(Cmp::Lt),
                "le" => Some(Cmp::Le),
                "gt" => Some(Cmp::Gt),
                "ge" => Some(Cmp::Ge),
                "is" => return Err("the 'is' operator is not supported".to_string()),
                _ => None,
            },
            Tk::Sym("<<") | Tk::Sym(">>") => return Err("node order comparison ('<<', '>>') is not supported".to_string()),
            _ => None,
        };
        if let Some(op) = value {
            self.bump();
            return Ok(Expr::Value(op, boxed(left), boxed(self.concat()?)));
        }
        Ok(left)
    }

    fn concat(&mut self) -> Result<Expr, String> {
        let mut left = self.range()?;
        while self.eat_sym("||") {
            left = Expr::Concat(boxed(left), boxed(self.range()?));
        }
        Ok(left)
    }

    fn range(&mut self) -> Result<Expr, String> {
        let left = self.additive()?;
        if self.eat_word("to") {
            return Ok(Expr::Range(boxed(left), boxed(self.additive()?)));
        }
        Ok(left)
    }

    fn additive(&mut self) -> Result<Expr, String> {
        let mut left = self.multiplicative()?;
        loop {
            let op = if self.eat_sym("+") {
                Arith::Add
            } else if self.eat_sym("-") {
                Arith::Sub
            } else {
                return Ok(left);
            };
            left = Expr::Arith(op, boxed(left), boxed(self.multiplicative()?));
        }
    }

    fn multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.union()?;
        loop {
            let op = if self.eat_sym("*") {
                Arith::Mul
            } else if self.eat_word("div") {
                Arith::Div
            } else if self.eat_word("idiv") {
                Arith::IDiv
            } else if self.eat_word("mod") {
                Arith::Mod
            } else {
                return Ok(left);
            };
            left = Expr::Arith(op, boxed(left), boxed(self.union()?));
        }
    }

    fn union(&mut self) -> Result<Expr, String> {
        let mut left = self.intersect()?;
        while self.eat_sym("|") || self.eat_word("union") {
            left = Expr::Set(SetOp::Union, boxed(left), boxed(self.intersect()?));
        }
        Ok(left)
    }

    fn intersect(&mut self) -> Result<Expr, String> {
        let mut left = self.unary_no_cast()?;
        loop {
            let op = if self.eat_word("intersect") {
                SetOp::Intersect
            } else if self.eat_word("except") {
                SetOp::Except
            } else {
                return Ok(left);
            };
            left = Expr::Set(op, boxed(left), boxed(self.unary_no_cast()?));
        }
    }

    fn unary_no_cast(&mut self) -> Result<Expr, String> {
        let e = self.unary()?;
        for w in ["instance", "treat", "castable", "cast"] {
            if self.word(w) {
                return Err(format!("'{w}' expressions are not supported"));
            }
        }
        Ok(e)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.eat_sym("-") {
            return Ok(Expr::Neg(boxed(self.unary()?)));
        }
        if self.eat_sym("+") {
            return Ok(Expr::Plus(boxed(self.unary()?)));
        }
        self.simple_map()
    }

    fn simple_map(&mut self) -> Result<Expr, String> {
        let mut left = self.path()?;
        while self.sym("!") {
            self.bump();
            left = Expr::Map(boxed(left), boxed(self.path()?));
        }
        if self.sym("=>") {
            return Err("the arrow operator '=>' is not supported".to_string());
        }
        Ok(left)
    }

    fn starts_step(&self) -> bool {
        match self.peek() {
            Tk::Str(_) | Tk::Num(..) | Tk::Var(_) | Tk::Name(_) => true,
            Tk::Sym(s) => matches!(*s, "(" | "@" | "." | ".." | "*" | "$"),
            Tk::End => false,
        }
    }

    fn path(&mut self) -> Result<Expr, String> {
        let mut steps = Vec::new();
        let mut absolute = false;
        if self.eat_sym("//") {
            absolute = true;
            steps.push(descendant_or_self_node());
            steps.push(self.step()?);
        } else if self.eat_sym("/") {
            absolute = true;
            if !self.starts_step() {
                return Ok(Expr::Path { absolute, steps });
            }
            steps.push(self.step()?);
        } else {
            steps.push(self.step()?);
        }
        loop {
            if self.eat_sym("//") {
                steps.push(descendant_or_self_node());
                steps.push(self.step()?);
            } else if self.eat_sym("/") {
                steps.push(self.step()?);
            } else {
                break;
            }
        }
        if !absolute && steps.len() == 1 {
            match steps.pop().unwrap() {
                Step::Expr(e) => return Ok(*e),
                s => steps.push(s),
            }
        }
        Ok(Expr::Path { absolute, steps })
    }

    fn predicates(&mut self) -> Result<Vec<Expr>, String> {
        let mut preds = Vec::new();
        while self.eat_sym("[") {
            preds.push(self.expr()?);
            self.expect_sym("]")?;
        }
        Ok(preds)
    }

    fn step(&mut self) -> Result<Step, String> {
        // Abbreviated steps.
        if self.eat_sym("..") {
            let preds = self.predicates()?;
            return Ok(Step::Axis { axis: Axis::Parent, test: Test::AnyNode, preds });
        }
        if self.eat_sym("@") {
            let test = self.node_test(true)?;
            let preds = self.predicates()?;
            return Ok(Step::Axis { axis: Axis::Attribute, test, preds });
        }
        if let Tk::Name(n) = self.peek().clone() {
            if matches!(self.peek_at(1), Tk::Sym("::")) {
                self.bump();
                self.bump();
                let axis = match n.as_str() {
                    "child" => Axis::Child,
                    "descendant" => Axis::Descendant,
                    "descendant-or-self" => Axis::DescendantOrSelf,
                    "parent" => Axis::Parent,
                    "ancestor" => Axis::Ancestor,
                    "ancestor-or-self" => Axis::AncestorOrSelf,
                    "following-sibling" => Axis::FollowingSibling,
                    "preceding-sibling" => Axis::PrecedingSibling,
                    "following" => Axis::Following,
                    "preceding" => Axis::Preceding,
                    "self" => Axis::SelfAxis,
                    "attribute" => Axis::Attribute,
                    "namespace" => return Err("the namespace axis is not supported".to_string()),
                    other => return Err(format!("'{other}' is not an axis")),
                };
                let test = self.node_test(axis == Axis::Attribute)?;
                let preds = self.predicates()?;
                return Ok(Step::Axis { axis, test, preds });
            }
            // A name that is not a call is a name test.
            if !matches!(self.peek_at(1), Tk::Sym("(")) {
                let test = self.node_test(false)?;
                let preds = self.predicates()?;
                return Ok(Step::Axis { axis: Axis::Child, test, preds });
            }
            // A kind test is an axis step; any other `name(` is a function call.
            if is_kind_test(&n) {
                let test = self.node_test(false)?;
                let preds = self.predicates()?;
                return Ok(Step::Axis { axis: Axis::Child, test, preds });
            }
        }
        if self.sym("*") {
            self.bump();
            let preds = self.predicates()?;
            return Ok(Step::Axis { axis: Axis::Child, test: Test::AnyName, preds });
        }
        // A postfix expression: primary with predicates.
        let primary = self.primary()?;
        let preds = self.predicates()?;
        Ok(Step::Expr(boxed(if preds.is_empty() { primary } else { Expr::Filter(boxed(primary), preds) })))
    }

    fn node_test(&mut self, attribute: bool) -> Result<Test, String> {
        let _ = attribute;
        match self.bump() {
            Tk::Sym("*") => {
                if self.sym(":") {
                    return Err("prefixed wildcards are not supported".to_string());
                }
                Ok(Test::AnyName)
            }
            Tk::Name(n) => {
                if matches!(self.peek(), Tk::Sym("(")) && is_kind_test(&n) {
                    self.bump();
                    let test = match n.as_str() {
                        "node" => Test::AnyNode,
                        "text" | "comment" | "processing-instruction" | "namespace-node" => Test::Never,
                        "document-node" => Test::Document,
                        "attribute" => Test::Attribute,
                        "element" => {
                            let mut name = None;
                            match self.peek().clone() {
                                Tk::Name(x) => {
                                    self.bump();
                                    name = Some(x);
                                }
                                Tk::Sym("*") => {
                                    self.bump();
                                }
                                _ => {}
                            }
                            Test::Element(name)
                        }
                        other => return Err(format!("the kind test {other}() is not supported")),
                    };
                    if matches!(n.as_str(), "node" | "text" | "comment" | "document-node" | "attribute" | "processing-instruction") && !self.sym(")") {
                        return Err(format!("arguments of the kind test {n}() are not supported"));
                    }
                    self.expect_sym(")")?;
                    return Ok(test);
                }
                if n.contains(':') {
                    return Err(format!("the prefixed name '{n}' is not supported"));
                }
                Ok(Test::Name(n))
            }
            _ => Err("expected a node test".to_string()),
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.bump() {
            Tk::Str(s) => Ok(Expr::Str(s)),
            Tk::Num(v, k) => Ok(Expr::Num(v, k)),
            Tk::Var(v) => Ok(Expr::Var(v)),
            Tk::Sym(".") => Ok(Expr::Context),
            Tk::Sym("(") => {
                if self.eat_sym(")") {
                    return Ok(Expr::Seq(Vec::new()));
                }
                let e = self.expr()?;
                self.expect_sym(")")?;
                Ok(match e {
                    Expr::Seq(_) => e,
                    other => Expr::Seq(vec![other]),
                })
            }
            Tk::Sym("?") => Err("the lookup operator '?' is not supported".to_string()),
            Tk::Name(n) => {
                if !self.sym("(") {
                    if matches!(n.as_str(), "map" | "array") && self.sym("{") {
                        return Err(format!("{n} constructors are not supported"));
                    }
                    return Err(format!("unexpected name '{n}'"));
                }
                self.bump();
                let mut args = Vec::new();
                if !self.eat_sym(")") {
                    loop {
                        args.push(self.expr_single()?);
                        if self.eat_sym(")") {
                            break;
                        }
                        self.expect_sym(",")?;
                    }
                }
                // Checkstyle's static context declares no prefix, not even `fn`.
                let short = n.as_str();
                if short.contains(':') {
                    return Err(format!("the function '{n}' is not supported"));
                }
                if short == "function" {
                    return Err("inline functions are not supported".to_string());
                }
                match FUNCTIONS.iter().find(|(f, _, _)| *f == short) {
                    Some((f, min, max)) => {
                        if args.len() < *min || args.len() > *max {
                            return Err(format!("the function {f}() cannot take {} arguments", args.len()));
                        }
                        Ok(Expr::Call(f, args))
                    }
                    None => Err(format!("the function {short}() is not supported")),
                }
            }
            Tk::Sym(s) => Err(format!("unexpected '{s}'")),
            Tk::End => Err("the query ends where an expression was expected".to_string()),
        }
    }
}

fn is_kind_test(n: &str) -> bool {
    matches!(
        n,
        "node" | "text" | "comment" | "processing-instruction" | "namespace-node" | "document-node" | "element" | "attribute" | "schema-element" | "schema-attribute"
    )
}

fn descendant_or_self_node() -> Step {
    Step::Axis { axis: Axis::DescendantOrSelf, test: Test::AnyNode, preds: Vec::new() }
}

impl XPath {
    /// Compile a query. Anything outside the supported subset is an error naming the construct.
    pub fn compile(source: &str) -> Result<XPath, String> {
        let toks = lex(source)?;
        let mut p = Parser { toks, pos: 0 };
        let expr = p.expr()?;
        if *p.peek() != Tk::End {
            return Err(format!("unexpected {} after the end of the expression", p.describe()));
        }
        Ok(XPath { source: source.to_string(), expr })
    }
}

// ---------------------------------------------------------------------------------------------
// The tree
// ---------------------------------------------------------------------------------------------

/// The AST seen through Checkstyle's XPath wrapper, with what the evaluator needs precomputed.
pub struct XTree<'a> {
    ast: &'a CsAst,
    depth: Vec<u32>,
    pre: Vec<u32>,
    index: Vec<u32>,
    tops: Vec<N>,
}

impl<'a> XTree<'a> {
    pub fn new(ast: &'a CsAst) -> Self {
        let n = ast.len();
        let mut t = XTree { ast, depth: vec![0; n], pre: vec![0; n], index: vec![0; n], tops: Vec::new() };
        if ast.root != NIL {
            let mut counter = 0u32;
            let mut c = ast.root;
            let mut i = 0;
            loop {
                t.tops.push(c);
                t.index[c as usize] = i;
                i += 1;
                match ast.next_sibling(c) {
                    Some(x) => c = x,
                    None => break,
                }
            }
            // Iterative pre-order, so deep trees cannot overflow the stack.
            let mut stack: Vec<(N, u32)> = t.tops.iter().rev().map(|&c| (c, 1)).collect();
            while let Some((node, depth)) = stack.pop() {
                t.depth[node as usize] = depth;
                t.pre[node as usize] = counter;
                counter += 1;
                let kids: Vec<N> = ast.children(node).collect();
                for (i, &k) in kids.iter().enumerate() {
                    t.index[k as usize] = i as u32;
                }
                for &k in kids.iter().rev() {
                    stack.push((k, depth + 1));
                }
            }
        }
        t
    }

    fn children(&self, n: NodeRef) -> Vec<NodeRef> {
        match n {
            NodeRef::Doc => self.tops.iter().map(|&c| NodeRef::El(c)).collect(),
            NodeRef::El(e) => self.ast.children(e).map(NodeRef::El).collect(),
            NodeRef::Attr(_) => Vec::new(),
        }
    }

    fn parent(&self, n: NodeRef) -> Option<NodeRef> {
        match n {
            NodeRef::Doc => None,
            NodeRef::El(e) => Some(self.ast.parent(e).map_or(NodeRef::Doc, NodeRef::El)),
            NodeRef::Attr(e) => Some(NodeRef::El(e)),
        }
    }

    fn siblings(&self, n: NodeRef) -> Vec<NodeRef> {
        match n {
            NodeRef::El(e) => match self.ast.parent(e) {
                Some(p) => self.ast.children(p).map(NodeRef::El).collect(),
                None => self.tops.iter().map(|&c| NodeRef::El(c)).collect(),
            },
            _ => Vec::new(),
        }
    }

    fn index_of(&self, e: N) -> usize {
        self.index[e as usize] as usize
    }

    fn has_text(&self, e: N) -> bool {
        matches!(
            self.ast.ty(e),
            Tok::IDENT | Tok::STRING_LITERAL | Tok::CHAR_LITERAL | Tok::NUM_LONG | Tok::NUM_INT | Tok::NUM_DOUBLE | Tok::NUM_FLOAT | Tok::TEXT_BLOCK_CONTENT | Tok::COMMENT_CONTENT
        )
    }

    /// `XpathUtil.getTextAttributeValue`.
    fn text_value(&self, e: N) -> String {
        let mut text = self.ast.text(e);
        if self.ast.ty(e) == Tok::STRING_LITERAL {
            let chars: Vec<char> = text.chars().collect();
            let inner: String = if chars.len() >= 2 { chars[1..chars.len() - 1].iter().collect() } else { String::new() };
            return inner.replace('\r', "\\r").replace('\n', "\\n");
        }
        if text.is_empty() {
            text = "";
        }
        text.replace('\r', "\\r").replace('\n', "\\n")
    }

    /// Checkstyle's `compareOrder`: depth first, then the order among siblings of the ancestors
    /// below the common one (for equal depth that is document order).
    fn order_key(&self, n: NodeRef) -> Result<(u32, u32), String> {
        match n {
            NodeRef::El(e) => Ok((self.depth[e as usize], self.pre[e as usize])),
            NodeRef::Doc => Ok((0, 0)),
            NodeRef::Attr(_) => Err("cannot order the document or an attribute node (unsupported operation)".to_string()),
        }
    }

    fn bfs(&self, start: NodeRef, include_self: bool, reverse: bool) -> Vec<NodeRef> {
        let kids = |n: NodeRef| {
            let mut k = self.children(n);
            if reverse {
                k.reverse();
            }
            k
        };
        let mut cur: VecDeque<NodeRef> = if include_self { VecDeque::from([start]) } else { kids(start).into() };
        let mut queue: VecDeque<NodeRef> = VecDeque::new();
        let mut out = Vec::new();
        loop {
            if let Some(x) = cur.pop_front() {
                out.push(x);
                queue.push_back(x);
            } else if let Some(q) = queue.pop_front() {
                cur = kids(q).into();
            } else {
                return out;
            }
        }
    }

    fn ancestors(&self, n: NodeRef) -> Vec<NodeRef> {
        let mut out = Vec::new();
        let mut cur = n;
        while let Some(p) = self.parent(cur) {
            out.push(p);
            cur = p;
        }
        out
    }

    fn following_siblings(&self, n: NodeRef) -> Vec<NodeRef> {
        match n {
            NodeRef::El(e) => self.siblings(n).into_iter().skip(self.index_of(e) + 1).collect(),
            _ => Vec::new(),
        }
    }

    fn preceding_siblings(&self, n: NodeRef) -> Vec<NodeRef> {
        match n {
            NodeRef::El(e) => {
                let mut v: Vec<NodeRef> = self.siblings(n).into_iter().take(self.index_of(e)).collect();
                v.reverse();
                v
            }
            _ => Vec::new(),
        }
    }

    /// The nodes of an axis in the order Checkstyle's iterators produce them.
    fn axis(&self, axis: Axis, n: NodeRef) -> Result<Vec<NodeRef>, String> {
        if let NodeRef::Attr(_) = n {
            return match axis {
                Axis::SelfAxis | Axis::DescendantOrSelf => Ok(vec![n]),
                Axis::Attribute | Axis::Descendant => Ok(Vec::new()),
                _ => Err("navigating from an attribute node is unsupported (Operation is not supported)".to_string()),
            };
        }
        Ok(match axis {
            Axis::Child => self.children(n),
            Axis::Descendant => self.bfs(n, false, false),
            Axis::DescendantOrSelf => self.bfs(n, true, false),
            Axis::Parent => self.parent(n).into_iter().collect(),
            Axis::Ancestor => self.ancestors(n),
            Axis::AncestorOrSelf => {
                let mut v = vec![n];
                v.extend(self.ancestors(n));
                v
            }
            Axis::FollowingSibling => self.following_siblings(n),
            Axis::PrecedingSibling => self.preceding_siblings(n),
            Axis::SelfAxis => vec![n],
            Axis::Attribute => match n {
                NodeRef::El(e) if self.has_text(e) => vec![NodeRef::Attr(e)],
                NodeRef::El(_) => Vec::new(),
                _ => return Err("the attribute axis of the document node is unsupported (Operation is not supported)".to_string()),
            },
            Axis::Following => {
                let mut out = Vec::new();
                let mut sibs = self.following_siblings(n);
                let mut ancestors = self.ancestors(n).into_iter();
                loop {
                    for s in sibs {
                        out.push(s);
                        out.extend(self.bfs(s, false, false));
                    }
                    match ancestors.next() {
                        Some(a) => sibs = self.following_siblings(a),
                        None => break,
                    }
                }
                out
            }
            Axis::Preceding => {
                let mut out = Vec::new();
                let mut sibs = self.preceding_siblings(n);
                let mut ancestors = self.ancestors(n).into_iter();
                loop {
                    for s in sibs {
                        out.push(s);
                        out.extend(self.bfs(s, false, true));
                    }
                    match ancestors.next() {
                        Some(a) => {
                            // Checkstyle's PrecedingIterator hands back the ancestor itself.
                            out.push(a);
                            sibs = self.preceding_siblings(a);
                        }
                        None => break,
                    }
                }
                out
            }
        })
    }

    fn matches(&self, test: &Test, axis: Axis, n: NodeRef) -> bool {
        let principal_attr = axis == Axis::Attribute;
        match test {
            Test::AnyNode => true,
            Test::Never => false,
            Test::AnyName => matches!((n, principal_attr), (NodeRef::El(_), false) | (NodeRef::Attr(_), true)),
            Test::Name(name) => match (n, principal_attr) {
                (NodeRef::El(e), false) => self.ast.ty(e).name() == name,
                (NodeRef::Attr(_), true) => name == "text",
                _ => false,
            },
            Test::Element(name) => match n {
                NodeRef::El(e) => name.as_ref().is_none_or(|x| self.ast.ty(e).name() == x),
                _ => false,
            },
            Test::Document => n == NodeRef::Doc,
            Test::Attribute => matches!(n, NodeRef::Attr(_)),
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Evaluation
// ---------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Focus {
    item: Item,
    pos: usize,
    size: usize,
}

type Env = Vec<(String, Seq)>;

struct Ev<'a> {
    t: &'a XTree<'a>,
    /// Compiled regular expressions of `matches()`, by pattern and flags.
    regexes: std::cell::RefCell<Vec<(String, crate::config::Format)>>,
}

fn num_string(v: f64, kind: Kind) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "INF" } else { "-INF" }.to_string();
    }
    if v == 0.0 {
        return "0".to_string();
    }
    if kind == Kind::Dbl && !(1e-6..1e6).contains(&v.abs()) {
        // XPath prints doubles outside this range in scientific form: 1.0E7.
        let s = format!("{:E}", v);
        return match s.split_once('E') {
            Some((m, e)) => {
                let m = if m.contains('.') { m.to_string() } else { format!("{m}.0") };
                format!("{m}E{e}")
            }
            None => s,
        };
    }
    if v.fract() == 0.0 && v.abs() < 1e18 {
        return format!("{}", v as i64);
    }
    format!("{v}")
}

fn item_string(i: &Item) -> String {
    match i {
        Item::Str(s) | Item::Untyped(s) => s.clone(),
        Item::Num(v, k) => num_string(*v, *k),
        Item::Bool(b) => b.to_string(),
        Item::Node(_) => String::new(),
    }
}

impl Ev<'_> {
    fn atomize(&self, i: &Item) -> Result<Item, String> {
        match i {
            Item::Node(NodeRef::Attr(e)) => Ok(Item::Untyped(self.t.text_value(*e))),
            Item::Node(_) => Err("an element or document node cannot be atomized (getStringValue is unsupported)".to_string()),
            other => Ok(other.clone()),
        }
    }

    fn atomize_seq(&self, s: &Seq) -> Result<Seq, String> {
        s.iter().map(|i| self.atomize(i)).collect()
    }

    fn ebv(&self, s: &Seq) -> Result<bool, String> {
        match s.first() {
            None => Ok(false),
            Some(Item::Node(_)) => Ok(true),
            Some(first) if s.len() == 1 => Ok(match first {
                Item::Bool(b) => *b,
                Item::Str(x) | Item::Untyped(x) => !x.is_empty(),
                Item::Num(v, _) => !(*v == 0.0 || v.is_nan()),
                Item::Node(_) => true,
            }),
            Some(_) => Err("effective boolean value of a sequence of several atomic values".to_string()),
        }
    }

    /// A single string argument: empty is "", several items are a type error.
    fn string_arg(&self, s: &Seq) -> Result<String, String> {
        let a = self.atomize_seq(s)?;
        match a.len() {
            0 => Ok(String::new()),
            1 => Ok(item_string(&a[0])),
            _ => Err("a sequence of more than one item is not allowed as a string argument".to_string()),
        }
    }

    fn to_double(&self, i: &Item) -> Result<f64, String> {
        match i {
            Item::Num(v, _) => Ok(*v),
            Item::Untyped(s) => parse_double(s).ok_or_else(|| format!("cannot convert '{s}' to a number")),
            Item::Str(_) => Err("a string is not a number".to_string()),
            Item::Bool(_) => Err("a boolean is not a number".to_string()),
            Item::Node(_) => Err("a node is not a number".to_string()),
        }
    }

    fn eval(&self, e: &Expr, f: Option<&Focus>, env: &mut Env) -> Result<Seq, String> {
        Ok(match e {
            Expr::Str(s) => vec![Item::Str(s.clone())],
            Expr::Num(v, k) => vec![Item::Num(*v, *k)],
            Expr::Var(name) => match env.iter().rev().find(|(n, _)| n == name) {
                Some((_, v)) => v.clone(),
                None => return Err(format!("the variable ${name} is not defined")),
            },
            Expr::Context => vec![f.ok_or("the context item is undefined")?.item.clone()],
            Expr::Seq(items) => {
                let mut out = Vec::new();
                for i in items {
                    out.extend(self.eval(i, f, env)?);
                }
                out
            }
            Expr::Path { absolute, steps } => self.path(*absolute, steps, f, env)?,
            Expr::Filter(base, preds) => {
                let seq = self.eval(base, f, env)?;
                self.filter(seq, preds, env)?
            }
            Expr::Call(name, args) => self.call(name, args, f, env)?,
            Expr::Or(a, b) => {
                let l = self.eval(a, f, env)?;
                if self.ebv(&l)? {
                    vec![Item::Bool(true)]
                } else {
                    let r = self.eval(b, f, env)?;
                    vec![Item::Bool(self.ebv(&r)?)]
                }
            }
            Expr::And(a, b) => {
                let l = self.eval(a, f, env)?;
                if !self.ebv(&l)? {
                    vec![Item::Bool(false)]
                } else {
                    let r = self.eval(b, f, env)?;
                    vec![Item::Bool(self.ebv(&r)?)]
                }
            }
            Expr::General(op, a, b) => {
                let (l, r) = (self.eval(a, f, env)?, self.eval(b, f, env)?);
                if l.is_empty() || r.is_empty() {
                    return Ok(vec![Item::Bool(false)]);
                }
                let (l, r) = (self.atomize_seq(&l)?, self.atomize_seq(&r)?);
                let mut result = false;
                'outer: for x in &l {
                    for y in &r {
                        if self.compare(*op, x, y)? {
                            result = true;
                            break 'outer;
                        }
                    }
                }
                vec![Item::Bool(result)]
            }
            Expr::Value(op, a, b) => {
                let (l, r) = (self.eval(a, f, env)?, self.eval(b, f, env)?);
                if l.is_empty() || r.is_empty() {
                    return Ok(Vec::new());
                }
                let (l, r) = (self.atomize_seq(&l)?, self.atomize_seq(&r)?);
                if l.len() > 1 || r.len() > 1 {
                    return Err("a value comparison needs single items".to_string());
                }
                match (l.first(), r.first()) {
                    (Some(x), Some(y)) => {
                        let cast = |i: &Item| match i {
                            Item::Untyped(s) => Item::Str(s.clone()),
                            other => other.clone(),
                        };
                        vec![Item::Bool(self.compare(*op, &cast(x), &cast(y))?)]
                    }
                    _ => Vec::new(),
                }
            }
            Expr::Concat(a, b) => {
                let l = self.string_arg(&self.eval(a, f, env)?)?;
                let r = self.string_arg(&self.eval(b, f, env)?)?;
                vec![Item::Str(l + &r)]
            }
            Expr::Range(a, b) => {
                let l = self.atomize_seq(&self.eval(a, f, env)?)?;
                let r = self.atomize_seq(&self.eval(b, f, env)?)?;
                match (l.first(), r.first()) {
                    (Some(x), Some(y)) if l.len() == 1 && r.len() == 1 => {
                        let (lo, hi) = (self.to_double(x)?, self.to_double(y)?);
                        if lo.fract() != 0.0 || hi.fract() != 0.0 {
                            return Err("the operands of 'to' must be integers".to_string());
                        }
                        ((lo as i64)..=(hi as i64)).map(|v| Item::Num(v as f64, Kind::Int)).collect()
                    }
                    _ => Vec::new(),
                }
            }
            Expr::Arith(op, a, b) => {
                let (l, r) = (self.eval(a, f, env)?, self.eval(b, f, env)?);
                if l.is_empty() || r.is_empty() {
                    return Ok(Vec::new());
                }
                let (l, r) = (self.atomize_seq(&l)?, self.atomize_seq(&r)?);
                match (l.first(), r.first()) {
                    (Some(x), Some(y)) if l.len() == 1 && r.len() == 1 => vec![self.arith(*op, x, y)?],
                    (None, _) | (_, None) => Vec::new(),
                    _ => return Err("arithmetic needs single items".to_string()),
                }
            }
            Expr::Neg(a) | Expr::Plus(a) => {
                let v = self.atomize_seq(&self.eval(a, f, env)?)?;
                match v.first() {
                    None => Vec::new(),
                    Some(x) if v.len() == 1 => {
                        let kind = match x {
                            Item::Num(_, k) => *k,
                            _ => Kind::Dbl,
                        };
                        let n = self.to_double(x)?;
                        vec![Item::Num(if matches!(e, Expr::Neg(_)) { -n } else { n }, kind)]
                    }
                    _ => return Err("unary arithmetic needs a single item".to_string()),
                }
            }
            Expr::Set(op, a, b) => {
                let l = self.nodes_of(self.eval(a, f, env)?)?;
                let r = self.nodes_of(self.eval(b, f, env)?)?;
                let keep: Vec<NodeRef> = match op {
                    SetOp::Union => l.iter().chain(r.iter()).copied().collect(),
                    SetOp::Intersect => l.iter().filter(|x| r.contains(x)).copied().collect(),
                    SetOp::Except => l.iter().filter(|x| !r.contains(x)).copied().collect(),
                };
                // Comparing the document node with another node is unsupported in Checkstyle's wrapper.
                let distinct: std::collections::HashSet<&NodeRef> = keep.iter().collect();
                if distinct.len() > 1 && distinct.contains(&NodeRef::Doc) {
                    return Err("ordering the document node against another node is unsupported".to_string());
                }
                self.sorted(keep)?.into_iter().map(Item::Node).collect()
            }
            Expr::Map(a, b) => {
                let seq = self.eval(a, f, env)?;
                let size = seq.len();
                let mut out = Vec::new();
                for (i, item) in seq.into_iter().enumerate() {
                    let focus = Focus { item, pos: i + 1, size };
                    out.extend(self.eval(b, Some(&focus), env)?);
                }
                out
            }
            Expr::If(c, a, b) => {
                let cond = self.eval(c, f, env)?;
                if self.ebv(&cond)? { self.eval(a, f, env)? } else { self.eval(b, f, env)? }
            }
            Expr::For(v, seq, ret) => {
                let items = self.eval(seq, f, env)?;
                let mut out = Vec::new();
                for item in items {
                    env.push((v.clone(), vec![item]));
                    let r = self.eval(ret, f, env);
                    env.pop();
                    out.extend(r?);
                }
                out
            }
            Expr::Let(v, value, ret) => {
                let value = self.eval(value, f, env)?;
                env.push((v.clone(), value));
                let r = self.eval(ret, f, env);
                env.pop();
                r?
            }
            Expr::Quant(every, v, seq, sat) => {
                let items = self.eval(seq, f, env)?;
                let mut result = *every;
                for item in items {
                    env.push((v.clone(), vec![item]));
                    let r = self.eval(sat, f, env).and_then(|s| self.ebv(&s));
                    env.pop();
                    let r = r?;
                    if *every && !r {
                        result = false;
                        break;
                    }
                    if !*every && r {
                        result = true;
                        break;
                    }
                }
                vec![Item::Bool(result)]
            }
        })
    }

    fn nodes_of(&self, s: Seq) -> Result<Vec<NodeRef>, String> {
        s.into_iter()
            .map(|i| match i {
                Item::Node(n) => Ok(n),
                _ => Err("a set operator needs nodes".to_string()),
            })
            .collect()
    }

    /// Document order as Checkstyle's wrapper defines it, without duplicates.
    fn sorted(&self, mut nodes: Vec<NodeRef>) -> Result<Vec<NodeRef>, String> {
        let mut seen = std::collections::HashSet::new();
        nodes.retain(|n| seen.insert(*n));
        if nodes.len() < 2 {
            return Ok(nodes);
        }
        let mut keyed: Vec<((u32, u32), NodeRef)> = Vec::with_capacity(nodes.len());
        for n in nodes.drain(..) {
            keyed.push((self.t.order_key(n)?, n));
        }
        keyed.sort_by_key(|(k, _)| *k);
        keyed.dedup_by_key(|(k, _)| *k);
        Ok(keyed.into_iter().map(|(_, n)| n).collect())
    }

    fn compare(&self, op: Cmp, x: &Item, y: &Item) -> Result<bool, String> {
        use std::cmp::Ordering;
        let ord: Option<Ordering> = match (x, y) {
            (Item::Untyped(a), Item::Untyped(b)) | (Item::Untyped(a), Item::Str(b)) | (Item::Str(a), Item::Untyped(b)) | (Item::Str(a), Item::Str(b)) => Some(a.cmp(b)),
            (Item::Untyped(_), Item::Num(..)) | (Item::Num(..), Item::Untyped(_)) | (Item::Num(..), Item::Num(..)) => {
                let (a, b) = (self.to_double(x)?, self.to_double(y)?);
                a.partial_cmp(&b)
            }
            (Item::Untyped(s), Item::Bool(b)) | (Item::Bool(b), Item::Untyped(s)) => {
                let v = match s.trim() {
                    "true" | "1" => true,
                    "false" | "0" => false,
                    other => return Err(format!("cannot convert '{other}' to a boolean")),
                };
                return Ok(match (x, y) {
                    (Item::Untyped(_), _) => cmp_result(op, Some(v.cmp(b))),
                    _ => cmp_result(op, Some(b.cmp(&v))),
                });
            }
            (Item::Bool(a), Item::Bool(b)) => Some(a.cmp(b)),
            _ => return Err("the operands of the comparison have incompatible types".to_string()),
        };
        Ok(cmp_result(op, ord))
    }

    fn arith(&self, op: Arith, x: &Item, y: &Item) -> Result<Item, String> {
        let kind_of = |i: &Item| match i {
            Item::Num(_, k) => *k,
            _ => Kind::Dbl,
        };
        let (a, b) = (self.to_double(x)?, self.to_double(y)?);
        let (ka, kb) = (kind_of(x), kind_of(y));
        let kind = if ka == Kind::Dbl || kb == Kind::Dbl {
            Kind::Dbl
        } else if ka == Kind::Dec || kb == Kind::Dec {
            Kind::Dec
        } else {
            Kind::Int
        };
        Ok(match op {
            Arith::Add => Item::Num(a + b, kind),
            Arith::Sub => Item::Num(a - b, kind),
            Arith::Mul => Item::Num(a * b, kind),
            Arith::Div => {
                if b == 0.0 && kind != Kind::Dbl {
                    return Err("division by zero".to_string());
                }
                Item::Num(a / b, if kind == Kind::Int { Kind::Dec } else { kind })
            }
            Arith::IDiv => {
                if b == 0.0 {
                    return Err("integer division by zero".to_string());
                }
                Item::Num((a / b).trunc(), Kind::Int)
            }
            Arith::Mod => {
                if b == 0.0 && kind != Kind::Dbl {
                    return Err("modulo by zero".to_string());
                }
                Item::Num(a % b, kind)
            }
        })
    }

    fn filter(&self, mut seq: Seq, preds: &[Expr], env: &mut Env) -> Result<Seq, String> {
        for p in preds {
            let size = seq.len();
            let mut kept = Vec::new();
            for (i, item) in seq.into_iter().enumerate() {
                let focus = Focus { item: item.clone(), pos: i + 1, size };
                let r = self.eval(p, Some(&focus), env)?;
                let keep = match r.as_slice() {
                    [Item::Num(v, _)] => *v == (i + 1) as f64,
                    _ => self.ebv(&r)?,
                };
                if keep {
                    kept.push(item);
                }
            }
            seq = kept;
        }
        Ok(seq)
    }

    fn path(&self, absolute: bool, steps: &[Step], f: Option<&Focus>, env: &mut Env) -> Result<Seq, String> {
        let mut current: Seq = if absolute {
            match f.map(|f| &f.item) {
                Some(Item::Node(_)) => vec![Item::Node(NodeRef::Doc)],
                _ => return Err("a path starting with '/' needs a node as the context item".to_string()),
            }
        } else {
            vec![f.ok_or("the context item is undefined")?.item.clone()]
        };
        for step in steps {
            let size = current.len();
            let mut next: Seq = Vec::new();
            let mut raw_nodes = true;
            for (i, item) in current.iter().enumerate() {
                match step {
                    Step::Axis { axis, test, preds } => {
                        let Item::Node(n) = item else { return Err("a path step needs a node as the context item".to_string()) };
                        let mut cand: Seq = Vec::new();
                        for m in self.t.axis(*axis, *n)? {
                            if self.t.matches(test, *axis, m) {
                                cand.push(Item::Node(m));
                            }
                        }
                        let cand = self.filter(cand, preds, env)?;
                        next.extend(cand);
                    }
                    Step::Expr(e) => {
                        let focus = Focus { item: item.clone(), pos: i + 1, size };
                        let r = self.eval(e, Some(&focus), env)?;
                        raw_nodes = true;
                        next.extend(r);
                    }
                }
            }
            let _ = raw_nodes;
            let all_nodes = next.iter().all(|i| matches!(i, Item::Node(_)));
            let any_node = next.iter().any(|i| matches!(i, Item::Node(_)));
            if any_node && !all_nodes {
                return Err("a path step returned both nodes and atomic values".to_string());
            }
            // Saxon puts a step's result in document order unless it can tell the order already holds.
            if all_nodes && !next.is_empty() {
                let reorder = match step {
                    Step::Axis { axis, .. } => match axis {
                        Axis::Child | Axis::Attribute | Axis::SelfAxis => false,
                        Axis::Descendant | Axis::DescendantOrSelf => size > 1,
                        _ => true,
                    },
                    Step::Expr(_) => true,
                };
                if reorder {
                    let nodes: Vec<NodeRef> = next
                        .iter()
                        .map(|i| match i {
                            Item::Node(n) => *n,
                            _ => unreachable!(),
                        })
                        .collect();
                    // A single result needs no ordering (and so no ordering key for the document node).
                    if nodes.len() > 1 {
                        next = self.sorted(nodes)?.into_iter().map(Item::Node).collect();
                    }
                }
            }
            current = next;
        }
        Ok(current)
    }

    fn call(&self, name: &str, args: &[Expr], f: Option<&Focus>, env: &mut Env) -> Result<Seq, String> {
        let mut a: Vec<Seq> = Vec::with_capacity(args.len());
        for e in args {
            a.push(self.eval(e, f, env)?);
        }
        let ctx = || -> Result<Seq, String> { Ok(vec![f.ok_or("the context item is undefined")?.item.clone()]) };
        let one = |s: Seq| -> Seq { s };
        let num_arg = |s: &Seq| -> Result<Option<(f64, Kind)>, String> {
            let at = self.atomize_seq(s)?;
            match at.as_slice() {
                [] => Ok(None),
                [x] => Ok(Some((self.to_double(x)?, if let Item::Num(_, k) = x { *k } else { Kind::Dbl }))),
                _ => Err("a sequence of more than one item is not allowed as a numeric argument".to_string()),
            }
        };
        Ok(match name {
            "true" => vec![Item::Bool(true)],
            "false" => vec![Item::Bool(false)],
            "not" => vec![Item::Bool(!self.ebv(&a[0])?)],
            "boolean" => vec![Item::Bool(self.ebv(&a[0])?)],
            "string" => {
                let s = if a.is_empty() { ctx()? } else { one(a[0].clone()) };
                // Saxon takes the string value of a node itself, which the wrapper does not provide.
                if s.iter().any(|i| matches!(i, Item::Node(_))) {
                    return Err("string() of a node is unsupported (getUnicodeStringValue)".to_string());
                }
                vec![Item::Str(self.string_arg(&s)?)]
            }
            "data" => {
                let s = if a.is_empty() { ctx()? } else { a[0].clone() };
                self.atomize_seq(&s)?
            }
            "concat" => {
                let mut out = String::new();
                for s in &a {
                    out += &self.string_arg(s)?;
                }
                vec![Item::Str(out)]
            }
            "string-join" => {
                let parts: Vec<String> = self.atomize_seq(&a[0])?.iter().map(item_string).collect();
                let sep = if a.len() > 1 { self.string_arg(&a[1])? } else { String::new() };
                vec![Item::Str(parts.join(&sep))]
            }
            "string-length" => {
                let s = if a.is_empty() { self.string_arg(&ctx()?)? } else { self.string_arg(&a[0])? };
                vec![Item::Num(s.chars().count() as f64, Kind::Int)]
            }
            "normalize-space" => {
                let s = if a.is_empty() { self.string_arg(&ctx()?)? } else { self.string_arg(&a[0])? };
                vec![Item::Str(s.split_whitespace().collect::<Vec<_>>().join(" "))]
            }
            "upper-case" => vec![Item::Str(self.string_arg(&a[0])?.to_uppercase())],
            "lower-case" => vec![Item::Str(self.string_arg(&a[0])?.to_lowercase())],
            "contains" => vec![Item::Bool(self.string_arg(&a[0])?.contains(&self.string_arg(&a[1])?))],
            "starts-with" => vec![Item::Bool(self.string_arg(&a[0])?.starts_with(&self.string_arg(&a[1])?))],
            "ends-with" => vec![Item::Bool(self.string_arg(&a[0])?.ends_with(&self.string_arg(&a[1])?))],
            "substring-before" => {
                let (s, t) = (self.string_arg(&a[0])?, self.string_arg(&a[1])?);
                vec![Item::Str(s.find(&t).map(|i| s[..i].to_string()).unwrap_or_default())]
            }
            "substring-after" => {
                let (s, t) = (self.string_arg(&a[0])?, self.string_arg(&a[1])?);
                vec![Item::Str(s.find(&t).map(|i| s[i + t.len()..].to_string()).unwrap_or_default())]
            }
            "substring" => {
                let s: Vec<char> = self.string_arg(&a[0])?.chars().collect();
                let start = match num_arg(&a[1])? {
                    Some((v, _)) => round_half_up(v),
                    None => f64::NAN,
                };
                let len = if a.len() > 2 {
                    match num_arg(&a[2])? {
                        Some((v, _)) => round_half_up(v),
                        None => f64::NAN,
                    }
                } else {
                    f64::INFINITY
                };
                let mut out = String::new();
                for (i, c) in s.iter().enumerate() {
                    let p = (i + 1) as f64;
                    if p >= start && p < start + len {
                        out.push(*c);
                    }
                }
                vec![Item::Str(out)]
            }
            "translate" => {
                let (s, from, to) = (self.string_arg(&a[0])?, self.string_arg(&a[1])?, self.string_arg(&a[2])?);
                let from: Vec<char> = from.chars().collect();
                let to: Vec<char> = to.chars().collect();
                let out: String = s
                    .chars()
                    .filter_map(|c| match from.iter().position(|x| *x == c) {
                        Some(i) => to.get(i).copied(),
                        None => Some(c),
                    })
                    .collect();
                vec![Item::Str(out)]
            }
            "matches" => {
                let (s, pat) = (self.string_arg(&a[0])?, self.string_arg(&a[1])?);
                let flags = if a.len() > 2 { self.string_arg(&a[2])? } else { String::new() };
                let key = format!("{flags}\u{0}{pat}");
                let mut cache = self.regexes.borrow_mut();
                if !cache.iter().any(|(k, _)| *k == key) {
                    let mut ignore_case = false;
                    for c in flags.chars() {
                        match c {
                            'i' => ignore_case = true,
                            other => return Err(format!("the regular expression flag '{other}' is not supported")),
                        }
                    }
                    let re = crate::config::Format::with_case(&pat, ignore_case)?;
                    cache.push((key.clone(), re));
                }
                let re = &cache.iter().find(|(k, _)| *k == key).unwrap().1;
                vec![Item::Bool(re.matches(s.as_bytes()))]
            }
            "number" => {
                let s = if a.is_empty() { ctx()? } else { a[0].clone() };
                let at = self.atomize_seq(&s)?;
                match at.as_slice() {
                    [] => vec![Item::Num(f64::NAN, Kind::Dbl)],
                    [x] => {
                        let v = match x {
                            Item::Num(v, _) => *v,
                            Item::Untyped(t) | Item::Str(t) => parse_double(t).unwrap_or(f64::NAN),
                            Item::Bool(b) => *b as i32 as f64,
                            Item::Node(_) => f64::NAN,
                        };
                        vec![Item::Num(v, Kind::Dbl)]
                    }
                    _ => return Err("number() needs a single item".to_string()),
                }
            }
            "sum" | "avg" | "min" | "max" => {
                let at = self.atomize_seq(&a[0])?;
                if at.is_empty() {
                    return Ok(match name {
                        "sum" => {
                            if a.len() > 1 {
                                a[1].clone()
                            } else {
                                vec![Item::Num(0.0, Kind::Int)]
                            }
                        }
                        _ => Vec::new(),
                    });
                }
                let nums: Vec<f64> = at.iter().map(|i| self.to_double(i)).collect::<Result<_, _>>()?;
                let kind = if at.iter().all(|i| matches!(i, Item::Num(_, Kind::Int))) { Kind::Int } else { Kind::Dbl };
                let v = match name {
                    "sum" => nums.iter().sum(),
                    "avg" => nums.iter().sum::<f64>() / nums.len() as f64,
                    "min" => nums.iter().copied().fold(f64::INFINITY, f64::min),
                    _ => nums.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                };
                vec![Item::Num(v, if name == "avg" { Kind::Dbl } else { kind })]
            }
            "abs" | "ceiling" | "floor" | "round" => match num_arg(&a[0])? {
                None => Vec::new(),
                Some((v, k)) => vec![Item::Num(
                    match name {
                        "abs" => v.abs(),
                        "ceiling" => v.ceil(),
                        "floor" => v.floor(),
                        _ => round_half_up(v),
                    },
                    k,
                )],
            },
            "count" => vec![Item::Num(a[0].len() as f64, Kind::Int)],
            "empty" => vec![Item::Bool(a[0].is_empty())],
            "exists" => vec![Item::Bool(!a[0].is_empty())],
            "position" => vec![Item::Num(f.ok_or("the context position is undefined")?.pos as f64, Kind::Int)],
            "last" => vec![Item::Num(f.ok_or("the context size is undefined")?.size as f64, Kind::Int)],
            "distinct-values" => {
                let at = self.atomize_seq(&a[0])?;
                let mut out: Seq = Vec::new();
                for i in at {
                    let dup = out.iter().any(|o| self.compare(Cmp::Eq, o, &i).unwrap_or(false));
                    if !dup {
                        out.push(i);
                    }
                }
                out
            }
            "reverse" => {
                let mut s = a[0].clone();
                s.reverse();
                s
            }
            "head" => a[0].first().cloned().into_iter().collect(),
            "tail" => a[0].iter().skip(1).cloned().collect(),
            "subsequence" => {
                let start = num_arg(&a[1])?.map_or(f64::NAN, |(v, _)| round_half_up(v));
                let len = if a.len() > 2 { num_arg(&a[2])?.map_or(f64::NAN, |(v, _)| round_half_up(v)) } else { f64::INFINITY };
                a[0].iter().enumerate().filter(|(i, _)| ((*i + 1) as f64) >= start && ((*i + 1) as f64) < start + len).map(|(_, x)| x.clone()).collect()
            }
            "local-name" => {
                let s = if a.is_empty() { ctx()? } else { a[0].clone() };
                match s.as_slice() {
                    [] => vec![Item::Str(String::new())],
                    [Item::Node(NodeRef::El(e))] => vec![Item::Str(self.t.ast.ty(*e).name().to_string())],
                    [Item::Node(NodeRef::Attr(_))] => vec![Item::Str("text".to_string())],
                    [Item::Node(NodeRef::Doc)] => vec![Item::Str("ROOT".to_string())],
                    _ => return Err("local-name() needs a single node".to_string()),
                }
            }
            other => return Err(format!("the function {other}() is not supported")),
        })
    }
}

fn round_half_up(v: f64) -> f64 {
    if v.is_nan() || v.is_infinite() { v } else { (v + 0.5).floor() }
}

fn cmp_result(op: Cmp, ord: Option<std::cmp::Ordering>) -> bool {
    use std::cmp::Ordering::*;
    match (op, ord) {
        (Cmp::Ne, None) => true,
        (_, None) => false,
        (Cmp::Eq, Some(o)) => o == Equal,
        (Cmp::Ne, Some(o)) => o != Equal,
        (Cmp::Lt, Some(o)) => o == Less,
        (Cmp::Le, Some(o)) => o != Greater,
        (Cmp::Gt, Some(o)) => o == Greater,
        (Cmp::Ge, Some(o)) => o != Less,
    }
}

/// `xs:double` lexical form.
fn parse_double(s: &str) -> Option<f64> {
    let t = s.trim();
    match t {
        "INF" | "+INF" => return Some(f64::INFINITY),
        "-INF" => return Some(f64::NEG_INFINITY),
        "NaN" => return Some(f64::NAN),
        _ => {}
    }
    let ok = !t.is_empty() && t.chars().all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-')) && t.chars().any(|c| c.is_ascii_digit());
    if ok { t.parse().ok() } else { None }
}

/// One node a query selected.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// The `DetailAST` the node wraps (the first top-level one for the document node).
    pub node: N,
}

impl XPath {
    /// The nodes the query selects in the tree, in the order Saxon would return them. An item that
    /// is not an element or document node is an error, as it is in Checkstyle.
    pub fn select(&self, t: &XTree) -> Result<Vec<Hit>, String> {
        if t.ast.root == NIL {
            return Ok(Vec::new());
        }
        let ev = Ev { t, regexes: Default::default() };
        let focus = Focus { item: Item::Node(NodeRef::Doc), pos: 1, size: 1 };
        let result = ev.eval(&self.expr, Some(&focus), &mut Vec::new()).map_err(|e| format!("evaluation of the XPath query '{}' failed: {e}", self.source))?;
        result
            .into_iter()
            .map(|i| match i {
                Item::Node(NodeRef::Doc) => Ok(Hit { node: t.ast.root }),
                Item::Node(NodeRef::El(e)) => Ok(Hit { node: e }),
                _ => Err(format!("the XPath query '{}' selected something that is not an element (Checkstyle cannot use it)", self.source)),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select(src: &str, query: &str) -> Vec<String> {
        let ast = crate::csast::try_build_source(src.as_bytes(), crate::syntax::Grammar::default(), true).unwrap();
        let tree = XTree::new(&ast);
        XPath::compile(query).unwrap().select(&tree).unwrap().into_iter().map(|h| crate::csast::node_line(&ast, h.node)).collect()
    }

    const SRC: &str = "class A {\n  @Test void m(int a) { if (a > 1) { } }\n  void n() { }\n}\n";

    #[test]
    fn selects_by_name_attribute_and_predicate() {
        assert_eq!(select(SRC, "//METHOD_DEF/IDENT").len(), 2);
        assert_eq!(select(SRC, "//IDENT[@text='n']").len(), 1);
        assert_eq!(select(SRC, "//METHOD_DEF[./MODIFIERS/ANNOTATION//IDENT[contains(@text, 'Test')]]/IDENT").len(), 1);
        assert_eq!(select(SRC, "//SLIST[count(./*)=1]").len(), 2);
        assert_eq!(select(SRC, "//LITERAL_IF/preceding-sibling::*").len(), 0);
    }

    #[test]
    fn unsupported_constructs_are_refused_at_compile_time() {
        for q in ["//A[replace(@text, 'a', 'b')]", "//A cast as xs:string", "//A[. is .]", "//A/namespace::*", "fn:count(//A)", "//A => f()", "//A[function($x) { $x }(1)]", "//A[tokenize(@text, ',')]"] {
            assert!(XPath::compile(q).is_err(), "{q} must be refused");
        }
    }

    #[test]
    fn elements_cannot_be_atomized_as_in_checkstyle() {
        let ast = crate::csast::try_build_source(SRC.as_bytes(), crate::syntax::Grammar::default(), true).unwrap();
        let tree = XTree::new(&ast);
        assert!(XPath::compile("//IDENT[. = 'm']").unwrap().select(&tree).is_err());
        assert!(XPath::compile("//IDENT[string(@text) = 'm']").unwrap().select(&tree).is_err());
    }
}
