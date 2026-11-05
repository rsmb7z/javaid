// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/checks/coding/VariableDeclarationUsageDistanceCheck.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! `VariableDeclarationUsageDistance`, ported from Checkstyle 14.4.0 onto the emulated AST.
//!
//! The method names follow `VariableDeclarationUsageDistanceCheck`. A Java `NullPointerException`
//! (Checkstyle would fail the file) is answered with "nothing found" here.

use crate::config::VariableDeclarationUsageDistance as Opts;
use crate::csast::{CsAst, N, Tok};
use crate::engine::Diagnostic;

const ZERO_DISTANCE: [Tok; 9] = [
    Tok::VARIABLE_DEF,
    Tok::TYPE,
    Tok::MODIFIERS,
    Tok::RESOURCE,
    Tok::EXTENDS_CLAUSE,
    Tok::IMPLEMENTS_CLAUSE,
    Tok::TYPE_PARAMETERS,
    Tok::PARAMETERS,
    Tok::LITERAL_THROWS,
];

const TYPE_DEFINITIONS: [Tok; 6] = [Tok::CLASS_DEF, Tok::INTERFACE_DEF, Tok::ANNOTATION_DEF, Tok::ENUM_DEF, Tok::RECORD_DEF, Tok::COMPACT_COMPILATION_UNIT];

/// The Java `MessageFormat` rendering of an integer argument: thousands separated by commas.
pub fn grouped(n: i64) -> String {
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 { format!("-{out}") } else { out }
}

pub fn run(a: &CsAst, o: &Opts, lines: &[&[u8]], tab_width: usize, out: &mut Vec<Diagnostic>) {
    if a.root == crate::csast::NIL {
        return;
    }
    let mut stack = vec![a.root];
    while let Some(n) = stack.pop() {
        if let Some(s) = a.next_sibling(n) {
            stack.push(s);
        }
        if let Some(c) = a.first_child(n) {
            stack.push(c);
        }
        if a.ty(n) == Tok::VARIABLE_DEF {
            visit(a, o, n, lines, tab_width, out);
        }
    }
}

fn visit(a: &CsAst, o: &Opts, ast: N, lines: &[&[u8]], tab_width: usize, out: &mut Vec<Diagnostic>) {
    let Some(parent) = a.parent(ast) else { return };
    let Some(modifiers) = a.first_child(ast) else { return };
    if a.ty(parent) == Tok::OBJBLOCK || (o.ignore_final && a.find_first_token(modifiers, Tok::FINAL).is_some()) {
        return;
    }
    let Some(variable) = a.find_first_token(ast, Tok::IDENT) else { return };
    let name = a.text(variable);
    if o.ignore.matches(name.as_bytes()) {
        return;
    }
    let semicolon = a.next_sibling(ast);
    let (usage, dist) =
        if o.validate_between_scopes { distance_between_scopes(a, semicolon, variable) } else { distance_in_single_scope(a, semicolon, variable) };
    if dist > o.allowed_distance as i32 && !is_initialization_sequence(a, usage, name) {
        let tail = if o.ignore_final {
            "  Consider making that variable final if you still need to store its value in advance (before method calls that might have side effects on the original value)."
        } else {
            ""
        };
        let message = format!(
            "Distance between variable '{name}' declaration and its first usage is {}, but allowed {}.{tail}",
            grouped(dist as i64),
            grouped(o.allowed_distance as i64)
        );
        let line = a.line_no(ast);
        let col = report_col(lines, line, a.column_no(ast), tab_width);
        out.push(Diagnostic { rule: "VariableDeclarationUsageDistance", line: line as u32, col, message });
    }
}

/// `AbstractCheck.log`: the column is 1 + the tab-expanded length of the text before it.
pub fn report_col(lines: &[&[u8]], line: i32, col: i32, tab_width: usize) -> u32 {
    let Some(text) = lines.get((line - 1).max(0) as usize) else { return col as u32 + 1 };
    crate::columns::chars_width(text, col.max(0) as usize, tab_width) as u32 + 1
}

fn instance_name(a: &CsAst, method_call: N) -> String {
    let name = full_ident_below(a, method_call);
    match name.rfind('.') {
        Some(i) => name[..i].to_string(),
        None => String::new(),
    }
}

/// `FullIdent.createFullIdentBelow(ast).getText()`.
fn full_ident_below(a: &CsAst, ast: N) -> String {
    let Some(first) = a.first_child(ast) else { return String::new() };
    full_ident(a, first)
}

fn full_ident(a: &CsAst, ast: N) -> String {
    let mut node = ast;
    if a.ty(node) == Tok::SINGLE_LINE_COMMENT {
        match a.next_sibling(node) {
            Some(n) => node = n,
            None => return String::new(),
        }
    }
    let mut elements = String::new();
    let mut stack = vec![node];
    let mut brackets_exist = false;
    let mut dot_counter = 0i32;
    while let Some(cur) = stack.pop() {
        let next = a.next_sibling(cur);
        let array_start = next.is_some_and(|s| matches!(a.ty(s), Tok::ARRAY_DECLARATOR | Tok::ANNOTATIONS) && is_array_type_declaration(a, s));
        let ty = a.ty(cur);
        brackets_exist = brackets_exist || array_start;
        let first = a.first_child(cur);
        if ty == Tok::LITERAL_NEW && first.is_some() {
            stack.push(first.unwrap());
        } else if ty == Tok::DOT {
            let Some(first) = first else { continue };
            let mut sibling = a.next_sibling(first);
            if let Some(s) = sibling {
                if a.ty(s) == Tok::SINGLE_LINE_COMMENT {
                    sibling = a.next_sibling(s);
                }
            }
            if let Some(s) = sibling {
                stack.push(s);
            }
            stack.push(first);
            dot_counter += 1;
        } else if array_start {
            elements.push_str(a.text(cur));
            append_brackets(a, cur, &mut elements);
        } else if ty != Tok::ANNOTATIONS {
            elements.push_str(a.text(cur));
            if dot_counter > 0 {
                elements.push('.');
                dot_counter -= 1;
            }
            if brackets_exist {
                if let Some(p) = a.parent(cur) {
                    append_brackets(a, p, &mut elements);
                }
            }
        }
    }
    elements
}

fn is_array_type_declaration(a: &CsAst, declarator: N) -> bool {
    let mut e = Some(declarator);
    while let Some(n) = e {
        if a.ty(n) == Tok::EXPR {
            break;
        }
        e = a.first_child(n);
    }
    e.is_none()
}

fn append_brackets(a: &CsAst, ast: N, out: &mut String) {
    if let Some(p) = a.parent(ast) {
        for _ in 0..a.children(p).filter(|&c| a.ty(c) == Tok::ARRAY_DECLARATOR).count() {
            out.push_str("[]");
        }
    }
}

fn is_initialization_sequence(a: &CsAst, usage: Option<N>, variable_name: &str) -> bool {
    let mut current = usage;
    let mut result = true;
    let mut found = false;
    let mut init_instance = String::new();
    while result && !found {
        let Some(cur) = current else { break };
        if a.ty(cur) == Tok::EXPR && a.first_child(cur).is_some_and(|c| a.ty(c) == Tok::METHOD_CALL) {
            let call = a.first_child(cur).unwrap();
            let instance = instance_name(a, call);
            if instance.is_empty() {
                result = false;
            } else if instance != init_instance {
                if init_instance.is_empty() {
                    init_instance = instance;
                } else {
                    result = false;
                }
            }
        } else if a.ty(cur) == Tok::VARIABLE_DEF {
            let name = a.find_first_token(cur, Tok::IDENT).map(|i| a.text(i)).unwrap_or("");
            found = variable_name == name;
        } else {
            result = matches!(a.ty(cur), Tok::SEMI | Tok::LCURLY);
        }
        current = next_node_to_check(a, cur);
    }
    result
}

fn next_node_to_check(a: &CsAst, cur: N) -> Option<N> {
    if let Some(p) = a.prev_sibling(cur) {
        return Some(p);
    }
    let predecessor = first_predecessor(a, &[Tok::SLIST, Tok::OBJBLOCK], cur)?;
    if a.ty(predecessor) == Tok::SLIST { next_node_between_scopes_slist(a, predecessor) } else { a.parent(predecessor).and_then(|p| a.prev_sibling(p)) }
}

fn next_node_between_scopes_slist(a: &CsAst, ast: N) -> Option<N> {
    let parent = a.parent(ast)?;
    match a.ty(parent) {
        Tok::LITERAL_ELSE | Tok::LITERAL_CATCH | Tok::LITERAL_FINALLY | Tok::CASE_GROUP => a.parent(parent).and_then(|p| a.prev_sibling(p)),
        Tok::LITERAL_TRY | Tok::LITERAL_FOR | Tok::LITERAL_WHILE | Tok::LITERAL_DO | Tok::LITERAL_IF | Tok::LITERAL_SYNCHRONIZED => a.prev_sibling(parent),
        Tok::METHOD_DEF | Tok::INSTANCE_INIT => first_predecessor(a, &TYPE_DEFINITIONS, ast).and_then(|p| a.prev_sibling(p)),
        _ => a.prev_sibling(ast),
    }
}

fn first_predecessor(a: &CsAst, types: &[Tok], ast: N) -> Option<N> {
    let mut cur = Some(ast);
    while let Some(c) = cur {
        if types.contains(&a.ty(c)) {
            return Some(c);
        }
        cur = a.parent(c);
    }
    None
}

fn distance_in_single_scope(a: &CsAst, semicolon: Option<N>, ident: N) -> (Option<N>, i32) {
    let mut dist = 0;
    let mut usage = None;
    let mut cur = semicolon;
    while let Some(c) = cur {
        if a.first_child(c).is_some() {
            if is_child(a, Some(c), ident) {
                dist = dist_to_usage_in_child(a, c, dist);
                usage = Some(c);
                break;
            } else if a.ty(c) != Tok::VARIABLE_DEF {
                dist += 1;
            }
        }
        cur = a.next_sibling(c);
    }
    (usage, dist)
}

fn dist_to_usage_in_child(a: &CsAst, child: N, current: i32) -> i32 {
    match a.ty(child) {
        Tok::SLIST => 0,
        Tok::LITERAL_FOR | Tok::LITERAL_WHILE | Tok::LITERAL_DO | Tok::LITERAL_IF | Tok::LITERAL_TRY => current + 1,
        _ => {
            if a.find_first_token(child, Tok::SLIST).is_none() {
                current + 1
            } else {
                0
            }
        }
    }
}

fn distance_between_scopes(a: &CsAst, ast: Option<N>, variable: N) -> (Option<N>, i32) {
    let mut dist = 0;
    let mut scope = ast;
    let mut usage = None;
    while let Some(s) = scope {
        let (exprs, d) = search_usage_expressions(a, variable, s);
        scope = None;
        dist += d;
        if exprs.len() == 1 {
            let block = exprs[0];
            scope = match a.ty(block) {
                Tok::VARIABLE_DEF | Tok::EXPR => {
                    dist += 1;
                    None
                }
                Tok::LITERAL_FOR | Tok::LITERAL_WHILE | Tok::LITERAL_DO => first_node_inside_loop(a, block, variable),
                Tok::LITERAL_IF => first_node_inside_if(a, block, variable),
                Tok::LITERAL_SWITCH => first_node_inside_switch(a, block, variable),
                Tok::LITERAL_TRY => first_node_inside_try(a, block, variable),
                _ => a.first_child(block),
            };
            usage = Some(block);
        } else if exprs.is_empty() {
            usage = None;
            dist = 0;
        } else {
            dist += 1;
            usage = Some(exprs[0]);
        }
    }
    (usage, dist)
}

fn search_usage_expressions(a: &CsAst, variable: N, statement: N) -> (Vec<N>, i32) {
    let mut exprs = Vec::new();
    let mut distance = 0;
    let mut cur = Some(statement);
    while let Some(c) = cur {
        if a.first_child(c).is_some() {
            if is_child(a, Some(c), variable) {
                exprs.push(c);
            } else if exprs.is_empty() && !ZERO_DISTANCE.contains(&a.ty(c)) {
                distance += 1;
            }
        }
        cur = a.next_sibling(c);
    }
    (exprs, distance)
}

fn first_node_inside_loop(a: &CsAst, block: N, variable: N) -> Option<N> {
    if is_variable_in_operator_expr(a, block, variable) {
        return None;
    }
    let current = if a.ty(block) == Tok::LITERAL_DO { a.first_child(block) } else { a.find_first_token(block, Tok::RPAREN).and_then(|r| a.next_sibling(r)) }?;
    if a.ty(current) != Tok::EXPR { Some(current) } else { None }
}

fn first_node_inside_if(a: &CsAst, block: N, variable: N) -> Option<N> {
    if is_variable_in_operator_expr(a, block, variable) {
        return None;
    }
    let slist = a.children(block).find(|&c| a.ty(c) == Tok::SLIST);
    let last = a.last_child(block)?;
    let mut previous = a.prev_sibling(last);
    if slist.is_none() && a.ty(last) == Tok::LITERAL_ELSE {
        previous = previous.and_then(|p| a.prev_sibling(p));
    }
    let mut exprs = Vec::new();
    if is_child(a, previous, variable) {
        exprs.push(previous.unwrap());
    }
    if is_child(a, Some(last), variable) {
        exprs.push(last);
    }
    if exprs.len() == 1 { Some(exprs[0]) } else { None }
}

fn first_node_inside_switch(a: &CsAst, block: N, variable: N) -> Option<N> {
    let first = a.children(block).find(|&c| matches!(a.ty(c), Tok::SWITCH_RULE | Tok::CASE_GROUP));
    let mut exprs = Vec::new();
    if let Some(first) = first {
        let ty = a.ty(first);
        for child in a.children(block).filter(|&c| a.ty(c) == ty) {
            let last = a.last_child(child);
            if is_child(a, last, variable) {
                exprs.push(last.unwrap());
            }
        }
    }
    if exprs.len() == 1 { Some(exprs[0]) } else { None }
}

fn first_node_inside_try(a: &CsAst, block: N, variable: N) -> Option<N> {
    let spec = a.find_first_token(block, Tok::RESOURCE_SPECIFICATION);
    if spec.is_some_and(|s| is_variable_in_operator_expr(a, s, variable)) {
        return None;
    }
    let mut current = a.first_child(block)?;
    if a.ty(current) == Tok::RESOURCE_SPECIFICATION {
        current = a.next_sibling(current)?;
    }
    let mut exprs = Vec::new();
    if is_child(a, Some(current), variable) {
        exprs.push(current);
    }
    let mut cur = a.next_sibling(current);
    while let Some(c) = cur {
        let block = a.find_first_token(c, Tok::SLIST);
        if is_child(a, block, variable) {
            exprs.push(block.unwrap());
        }
        cur = a.next_sibling(c);
    }
    if exprs.len() == 1 { Some(exprs[0]) } else { None }
}

fn is_variable_in_operator_expr(a: &CsAst, operator: N, variable: N) -> bool {
    let mut ast = a.find_first_token(operator, Tok::LPAREN);
    while let Some(n) = ast {
        if a.ty(n) == Tok::RPAREN {
            break;
        }
        if is_child(a, Some(n), variable) {
            return true;
        }
        ast = a.next_sibling(n);
    }
    false
}

/// Whether a descendant of `parent` has the type and text of `ast` (the node itself is not looked at).
fn is_child(a: &CsAst, parent: Option<N>, ast: N) -> bool {
    let Some(parent) = parent else { return false };
    let (ty, text) = (a.ty(ast), a.text(ast));
    let mut cur = a.first_child(parent);
    while let Some(c) = cur {
        if a.ty(c) == ty && a.text(c) == text {
            return true;
        }
        let mut to_visit = a.first_child(c);
        let mut node = Some(c);
        while to_visit.is_none() {
            let Some(n) = node else { break };
            to_visit = a.next_sibling(n);
            node = a.parent(n);
            if node == Some(parent) {
                break;
            }
        }
        cur = to_visit;
    }
    false
}
