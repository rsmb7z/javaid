//! The Javadoc family: which comment is a declaration's Javadoc, and the checks that only need to
//! know whether there is one.
//!
//! Checkstyle hangs a comment on the next token it meets, and looks for a declaration's Javadoc
//! only before the first token of the declaration: `/** d */ @A public int x;` has one,
//! `@A /** d */ public int x;` and `public /** d */ int x;` do not (all probed, JvProbe.java). With
//! several comments before the declaration the last Javadoc is the one that counts.

use crate::engine::Ctx;
use crate::rules::javadoc_tags;
use crate::rules::util::has_modifier;
use tree_sitter::Node;

fn is_comment(n: Node) -> bool {
    matches!(n.kind(), "line_comment" | "block_comment")
}

/// `/** ... */` with something between the stars and the slash being optional but `/**/` not a Javadoc.
pub fn is_javadoc(cx: &Ctx, comment: Node) -> bool {
    comment.kind() == "block_comment" && {
        let t = cx.text(comment);
        t.starts_with(b"/**") && t.len() > 4
    }
}

/// The Javadoc attached to a declaration: the last Javadoc among the comments right before it.
pub fn attached<'a>(cx: &Ctx, decl: Node<'a>) -> Option<Node<'a>> {
    let mut prev = decl.prev_sibling();
    while let Some(p) = prev.filter(|p| is_comment(*p)) {
        if is_javadoc(cx, p) {
            return Some(p);
        }
        prev = p.prev_sibling();
    }
    None
}

/// Access level of a member as `accessModifiers` sees it: its own modifier, public for the members
/// of an interface or annotation, package otherwise. The enclosing types do not matter.
fn access(cx: &Ctx, decl: Node) -> u8 {
    if decl.parent().is_some_and(|p| matches!(p.kind(), "interface_body" | "annotation_type_body")) {
        return 0;
    }
    if has_modifier(cx, decl, b"public") {
        0
    } else if has_modifier(cx, decl, b"protected") {
        1
    } else if has_modifier(cx, decl, b"private") {
        3
    } else {
        2
    }
}

fn in_code_block(decl: Node) -> bool {
    let mut p = decl.parent();
    while let Some(n) = p {
        match n.kind() {
            "method_declaration" | "constructor_declaration" | "compact_constructor_declaration" | "static_initializer" => return true,
            // an instance initializer is a bare block directly in a class body
            "block" if n.parent().is_some_and(|b| b.kind() == "class_body") => return true,
            "lambda_expression" => return true,
            _ => {}
        }
        p = n.parent();
    }
    false
}

/// `JavadocVariable`
pub fn on_variable(cx: &mut Ctx, decl: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.javadoc_variable else { return };
    if in_code_block(decl) {
        return;
    }
    let (level, names): (u8, Vec<Node>) = match decl.kind() {
        "field_declaration" | "constant_declaration" => {
            let mut cur = decl.walk();
            let names = decl.children_by_field_id(cx.syn.f.declarator, &mut cur).filter_map(|d| d.child_by_field_id(cx.syn.f.name)).collect();
            (access(cx, decl), names)
        }
        "enum_constant" => {
            // A constant has the access of its enum.
            let Some(en) = decl.parent().and_then(|b| b.parent()) else { return };
            (access(cx, en), decl.child_by_field_id(cx.syn.f.name).into_iter().collect())
        }
        _ => return,
    };
    if !opts.access[level as usize] || attached(cx, decl).is_some() {
        return;
    }
    for name in names {
        let text = cx.text(name);
        if text == b"serialVersionUID" || opts.ignore_name.as_ref().is_some_and(|f| f.matches(text)) {
            continue;
        }
        let msg = format!("Missing a Javadoc comment for '{}'.", String::from_utf8_lossy(text));
        cx.diag("JavadocVariable", decl, msg);
    }
}

// ---------------------------------------------------------------------------------------------
// Scope, as Checkstyle's `Scope` orders it: public 1, protected 2, package 3, private 4, anonymous 5.

pub const PUBLIC: u8 = 1;
pub const PROTECTED: u8 = 2;
pub const PACKAGE: u8 = 3;
pub const PRIVATE: u8 = 4;
pub const ANONYMOUS: u8 = 5;

fn is_type_decl(kind: &str) -> bool {
    matches!(kind, "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration")
}

/// The modifiers' scope. What sits directly in an interface or annotation is public unless it says
/// `private`; a constructor of an enum that says nothing is private (both probed, MjmProbe.java).
pub fn declared_scope(cx: &Ctx, decl: Node) -> u8 {
    if has_modifier(cx, decl, b"private") {
        return PRIVATE;
    }
    if decl.parent().is_some_and(|p| matches!(p.kind(), "interface_body" | "annotation_type_body")) {
        return PUBLIC;
    }
    if has_modifier(cx, decl, b"public") {
        PUBLIC
    } else if has_modifier(cx, decl, b"protected") {
        PROTECTED
    } else if decl.kind() == "constructor_declaration" && decl.parent().is_some_and(|b| b.kind() == "enum_body_declarations") {
        PRIVATE
    } else {
        PACKAGE
    }
}

/// The least visible of the types around a node; an anonymous class on the way makes it 5.
pub fn surrounding_scope(cx: &Ctx, node: Node) -> Option<u8> {
    let mut scope: Option<u8> = None;
    let mut p = node.parent();
    while let Some(n) = p {
        if n.kind() == "object_creation_expression" {
            return Some(ANONYMOUS);
        }
        if is_type_decl(n.kind()) {
            let r = declared_scope(cx, n);
            scope = Some(scope.map_or(r, |s| s.max(r)));
        }
        p = n.parent();
    }
    scope
}

fn count_children(node: Node) -> usize {
    node.children(&mut node.walk()).filter(|c| !is_comment(*c) && c.kind() != "modifiers").count()
}

/// `MissingJavadocMethod`
pub fn on_method(cx: &mut Ctx, decl: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.missing_javadoc_method else { return };
    let own = declared_scope(cx, decl);
    let Some(surrounding) = surrounding_scope(cx, decl) else { return };
    if opts.exclude_scope.is_some_and(|e| own == e || surrounding == e) || own > opts.scope || surrounding > opts.scope {
        return;
    }
    if attached(cx, decl).is_some() {
        return;
    }
    let name_node = decl.child_by_field_id(cx.syn.f.name);
    let Some(name_node) = name_node else { return };
    let name = cx.text(name_node);
    let is_element = decl.kind() == "annotation_type_element_declaration";
    if opts.allow_missing_property_javadoc && (is_setter(cx, decl) || is_getter(cx, decl)) {
        return;
    }
    if opts.ignore_method_names.as_ref().is_some_and(|f| f.matches(name)) {
        return;
    }
    if !is_element && (lines_in_body(decl) <= opts.min_line_count || has_allowed_annotation(cx, decl, &opts.allowed_annotations)) {
        return;
    }
    let msg = format!("Missing a Javadoc comment for '{}'.", String::from_utf8_lossy(name));
    cx.diag("MissingJavadocMethod", decl, msg);
}

/// Lines strictly between the braces, 1 when there is no body or it sits on one line.
fn lines_in_body(decl: Node) -> i64 {
    let Some(body) = decl.child_by_field_name("body") else { return 1 };
    let (open, close) = (body.start_position().row, body.end_position().row);
    if open == close {
        1
    } else {
        close as i64 - open as i64 - 1
    }
}

pub fn has_allowed_annotation(cx: &Ctx, decl: Node, allowed: &[String]) -> bool {
    let Some(mods) = decl.children(&mut decl.walk()).find(|c| c.kind() == "modifiers") else { return false };
    mods.children(&mut mods.walk()).filter(|m| matches!(m.kind(), "marker_annotation" | "annotation")).any(|a| {
        let Some(name) = a.child_by_field_id(cx.syn.f.name) else { return false };
        let text: String = String::from_utf8_lossy(cx.text(name)).split_whitespace().collect();
        allowed.iter().any(|w| *w == text)
    })
}

fn plain_method_shape(decl: Node) -> bool {
    decl.kind() == "method_declaration" && !decl.children(&mut decl.walk()).any(|c| matches!(c.kind(), "type_parameters" | "throws")) && decl.child_by_field_name("body").is_some()
}

/// `getX`/`isX` with no parameters and a body that starts with `return`.
fn is_getter(cx: &Ctx, decl: Node) -> bool {
    if !plain_method_shape(decl) {
        return false;
    }
    let Some(name) = decl.child_by_field_id(cx.syn.f.name) else { return false };
    let n = cx.text(name);
    let prefix = if n.starts_with(b"get") { 3 } else if n.starts_with(b"is") { 2 } else { return false };
    if !n.get(prefix).is_some_and(u8::is_ascii_uppercase) {
        return false;
    }
    let Some(params) = decl.child_by_field_id(cx.syn.f.parameters) else { return false };
    if params.named_child_count() != 0 {
        return false;
    }
    let body = decl.child_by_field_name("body").unwrap();
    body.named_children(&mut body.walk()).find(|c| !is_comment(*c)).is_some_and(|c| c.kind() == "return_statement")
}

/// `setX(v)` with a body of exactly one plain assignment.
fn is_setter(cx: &Ctx, decl: Node) -> bool {
    if !plain_method_shape(decl) {
        return false;
    }
    let Some(name) = decl.child_by_field_id(cx.syn.f.name) else { return false };
    let n = cx.text(name);
    if !(n.starts_with(b"set") && n.get(3).is_some_and(u8::is_ascii_uppercase)) {
        return false;
    }
    let Some(params) = decl.child_by_field_id(cx.syn.f.parameters) else { return false };
    if params.named_children(&mut params.walk()).filter(|c| !is_comment(*c)).count() != 1 {
        return false;
    }
    let body = decl.child_by_field_name("body").unwrap();
    let statements: Vec<Node> = body.named_children(&mut body.walk()).filter(|c| !is_comment(*c)).collect();
    // One statement; an `x = y;` whose operator is a plain `=`.
    statements.len() == 1
        && statements[0].kind() == "expression_statement"
        && statements[0].named_child(0).is_some_and(|e| e.kind() == "assignment_expression" && e.child_by_field_name("operator").is_some_and(|o| cx.text(o) == b"="))
}

#[allow(dead_code)]
fn unused(node: Node) -> usize {
    count_children(node)
}

/// Whether a declaration can carry a Javadoc (`BlockCommentPosition`'s "on a type, member, package
/// or module").
fn takes_javadoc(kind: &str) -> bool {
    matches!(
        kind,
        "package_declaration"
            | "module_declaration"
            | "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "annotation_type_declaration"
            | "field_declaration"
            | "constant_declaration"
            | "method_declaration"
            | "constructor_declaration"
            | "compact_constructor_declaration"
            | "enum_constant"
            | "annotation_type_element_declaration"
    )
}

/// Whether a Javadoc is the last comment before a declaration's first token.
fn following_declaration<'a>(cx: &Ctx, comment: Node<'a>) -> Option<Node<'a>> {
    let mut next = comment.next_sibling();
    let mut valid: Option<Node<'a>> = None;
    while let Some(n) = next {
        if is_comment(n) {
            if is_javadoc(cx, n) {
                break;
            }
            next = n.next_sibling();
            continue;
        }
        valid = takes_javadoc(n.kind()).then_some(n);
        break;
    }
    valid
}

/// Every comment: `InvalidJavadocPosition`.
pub fn on_comment(cx: &mut Ctx, comment: Node) {
    let cfg = cx.cfg;
    if cfg.multiline_comment_asterisk && comment.kind() == "block_comment" && !(is_javadoc(cx, comment) && following_declaration(cx, comment).is_some()) {
        crate::rules::javadoc_style::multiline_asterisk(cx, comment);
    }
    if !is_javadoc(cx, comment) {
        return;
    }
    let Some(decl) = following_declaration(cx, comment) else {
        if cfg.invalid_javadoc_position.is_some() {
            cx.diag("InvalidJavadocPosition", comment, "Javadoc comment is placed in the wrong location.".to_string());
        }
        return;
    };
    if let Some(first_line) = cfg.javadoc_content_location {
        crate::rules::javadoc_style::content_location(cx, comment, first_line);
    }
    if crate::rules::javadoc_style::wanted(cx) {
        crate::rules::javadoc_style::on_javadoc(cx, comment, decl);
    }
}

fn type_parameter_names(cx: &Ctx, decl: Node) -> Vec<String> {
    let Some(list) = decl.children(&mut decl.walk()).find(|c| c.kind() == "type_parameters") else { return Vec::new() };
    list.named_children(&mut list.walk())
        .filter(|c| c.kind() == "type_parameter")
        .filter_map(|p| p.named_children(&mut p.walk()).find(|c| c.kind() == "type_identifier"))
        .map(|n| format!("<{}>", String::from_utf8_lossy(cx.text(n))))
        .collect()
}

fn record_component_names(cx: &Ctx, decl: Node) -> Vec<String> {
    if decl.kind() != "record_declaration" {
        return Vec::new();
    }
    let Some(list) = decl.child_by_field_id(cx.syn.f.parameters) else { return Vec::new() };
    list.named_children(&mut list.walk())
        .filter_map(|p| {
            p.child_by_field_id(cx.syn.f.name).or_else(|| p.children(&mut p.walk()).filter(|c| c.kind() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)))
        })
        .map(|n| String::from_utf8_lossy(cx.text(n)).into_owned())
        .collect()
}

/// 1-based column of a character index in a line, tabs running to the next multiple of the tab width.
fn column_of(cx: &Ctx, line: u32, index: usize) -> u32 {
    let text = cx.src.split(|&b| b == b'\n').nth(line as usize - 1).unwrap_or_default();
    let text = String::from_utf8_lossy(text);
    let mut width = 0usize;
    for c in text.chars().take(index) {
        if c == '\t' {
            width = (width / cx.cfg.walker_tab() + 1) * cx.cfg.walker_tab();
        } else {
            width += 1;
        }
    }
    width as u32 + 1
}

/// `JavadocType`
pub fn on_type(cx: &mut Ctx, decl: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.javadoc_type else { return };
    let custom = declared_scope(cx, decl);
    let surrounding = surrounding_scope(cx, decl);
    let in_scope = custom <= opts.scope && surrounding.is_none_or(|s| s <= opts.scope);
    let excluded = opts.exclude_scope.is_some_and(|e| custom <= e && !surrounding.is_some_and(|s| s > e));
    if !in_scope || excluded || has_allowed_annotation(cx, decl, &opts.allowed_annotations) {
        return;
    }
    let Some(comment) = attached(cx, decl) else { return };
    let text = String::from_utf8_lossy(cx.text(comment)).into_owned();
    let doc = javadoc_tags::parse(&text, comment.start_position().row as u32 + 1);
    // A comment the grammar cannot read is reported, and checked no further. Only the Javadoc of a
    // type that is being checked is read at all (probed: with `scope=public` the others are silent).
    if let Some(e) = doc.error {
        cx.diag_at("JavadocType", e.line, 0, format!("Javadoc comment at column {} has parse error. Details: {}", e.col, e.details));
        return;
    }
    let outermost = decl.parent().is_some_and(|p| p.kind() == "program");
    let mut found_author = false;
    let mut found_version = false;
    let mut components = record_component_names(cx, decl);
    let mut type_params = type_parameter_names(cx, decl);
    let at = |cx: &Ctx, t: &javadoc_tags::Tag| (t.line, column_of(cx, t.line, t.col));
    for tag in &doc.tags {
        match tag.name.as_str() {
            "author" | "version" if outermost => {
                let (pattern, flag) = if tag.name == "author" { (&opts.author_format, &mut found_author) } else { (&opts.version_format, &mut found_version) };
                if let Some(format) = pattern {
                    *flag = true;
                    if !format.matches(tag.first_text.as_bytes()) {
                        let msg = format!("Type Javadoc tag @{} must match pattern '{}'.", tag.name, format.source);
                        cx.diag("JavadocType", decl, msg);
                    }
                }
            }
            "param" => {
                let Some(name) = &tag.arg else {
                    let (line, col) = at(cx, tag);
                    cx.diag_at("JavadocType", line, col, "Unused Javadoc tag.".to_string());
                    continue;
                };
                let mut matched = false;
                if let Some(i) = components.iter().position(|c| c == name) {
                    components.remove(i);
                    matched = true;
                }
                if name.starts_with('<') {
                    matched = false;
                    if let Some(i) = type_params.iter().position(|c| c == name) {
                        type_params.remove(i);
                        matched = true;
                    }
                }
                if !matched {
                    let (line, col) = at(cx, tag);
                    cx.diag_at("JavadocType", line, col, format!("Unused @param tag for '{name}'."));
                }
            }
            n if !javadoc_tags::is_standard(n) && !opts.allow_unknown_tags => {
                let (line, col) = at(cx, tag);
                cx.diag_at("JavadocType", line, col, format!("Unknown tag '{n}'."));
            }
            _ => {}
        }
    }
    if outermost {
        for (pattern, found, tag) in [(&opts.author_format, found_author, "@author"), (&opts.version_format, found_version, "@version")] {
            if pattern.is_some() && !found {
                cx.diag("JavadocType", decl, format!("Type Javadoc comment is missing {tag} tag."));
            }
        }
    }
    if !opts.allow_missing_param_tags {
        for name in components.iter().chain(type_params.iter()) {
            cx.diag("JavadocType", decl, format!("Type Javadoc comment is missing @param '{name}' tag."));
        }
    }
}

// ---------------------------------------------------------------------------------------------
// JavadocMethod

struct Documented<'a> {
    /// Name node and its text, for each parameter, in order.
    params: Vec<(Node<'a>, String)>,
    type_params: Vec<(Node<'a>, String)>,
}

fn signature<'a>(cx: &Ctx, decl: Node<'a>) -> Documented<'a> {
    let mut params = Vec::new();
    if let Some(list) = decl.child_by_field_id(cx.syn.f.parameters) {
        for p in list.named_children(&mut list.walk()) {
            let name = match p.kind() {
                "formal_parameter" => p.child_by_field_id(cx.syn.f.name),
                "spread_parameter" => p.children(&mut p.walk()).filter(|c| c.kind() == "variable_declarator").find_map(|d| d.child_by_field_id(cx.syn.f.name)),
                _ => None,
            };
            if let Some(n) = name {
                params.push((n, String::from_utf8_lossy(cx.text(n)).into_owned()));
            }
        }
    }
    let mut type_params = Vec::new();
    if let Some(list) = decl.children(&mut decl.walk()).find(|c| c.kind() == "type_parameters") {
        for p in list.named_children(&mut list.walk()).filter(|c| c.kind() == "type_parameter") {
            if let Some(n) = p.named_children(&mut p.walk()).find(|c| c.kind() == "type_identifier") {
                type_params.push((n, format!("<{}>", String::from_utf8_lossy(cx.text(n)))));
            }
        }
    }
    Documented { params, type_params }
}

/// The first identifier of a (possibly qualified) type name and its simple name.
fn type_name_parts<'a>(cx: &Ctx, ty: Node<'a>) -> Option<(Node<'a>, String)> {
    let mut first = ty;
    while let Some(c) = first.named_child(0).filter(|_| matches!(first.kind(), "scoped_type_identifier" | "generic_type" | "scoped_identifier")) {
        first = c;
    }
    // The name as written, which is what the message shows (`java.io.IOException`).
    let written = match ty.kind() {
        "generic_type" => ty.named_child(0).map(|n| cx.text(n)),
        _ => Some(cx.text(ty)),
    }?;
    Some((first, String::from_utf8_lossy(written).into_owned()))
}

/// `throw new X(...)` statements that Checkstyle counts: not inside a lambda, an anonymous or local
/// class, nor in the body of a `try` that has a `catch`.
fn thrown<'a>(cx: &Ctx, body: Node<'a>, out: &mut Vec<(Node<'a>, String)>) {
    let mut stack = vec![body];
    while let Some(n) = stack.pop() {
        match n.kind() {
            "lambda_expression" | "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" => continue,
            "object_creation_expression" if n.children(&mut n.walk()).any(|c| c.kind() == "class_body") => continue,
            "throw_statement" => {
                if let Some(e) = n.named_child(0).filter(|e| e.kind() == "object_creation_expression") {
                    if let Some(ty) = e.child_by_field_id(cx.syn.f.type_) {
                        if let Some(part) = type_name_parts(cx, ty) {
                            out.push(part);
                        }
                    }
                }
                continue;
            }
            "try_statement" | "try_with_resources_statement" => {
                let has_catch = n.children(&mut n.walk()).any(|c| c.kind() == "catch_clause");
                let body_node = n.child_by_field_name("body");
                let kids: Vec<Node> = n.children(&mut n.walk()).collect();
                for c in kids.into_iter().rev() {
                    if has_catch && Some(c) == body_node {
                        continue;
                    }
                    stack.push(c);
                }
                continue;
            }
            _ => {}
        }
        let mut cur = n.walk();
        let kids: Vec<Node> = n.children(&mut cur).collect();
        stack.extend(kids.into_iter().rev());
    }
}

/// The access of the nearest class, interface, enum or annotation around a member (probed: only the
/// nearest counts, so a public class inside a package-private one is public). A record is looked
/// through, and a member of an anonymous class, or of a record with nothing around it, is not
/// checked at all: `None`.
fn surrounding_access(cx: &Ctx, node: Node) -> Option<u8> {
    let mut p = node.parent();
    while let Some(n) = p {
        if n.kind() == "object_creation_expression" {
            return None;
        }
        if is_type_decl(n.kind()) && n.kind() != "record_declaration" {
            return Some(declared_scope(cx, n));
        }
        p = n.parent();
    }
    None
}

/// `JavadocMethod`
pub fn on_javadoc_method(cx: &mut Ctx, decl: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.javadoc_method else { return };
    let own = declared_scope(cx, decl);
    let Some(surrounding) = surrounding_access(cx, decl) else { return };
    let level = |s: u8| opts.access[(s as usize).saturating_sub(1).min(3)];
    if !level(own) || !level(surrounding) {
        return;
    }
    let Some(comment) = attached(cx, decl) else { return };
    let text = String::from_utf8_lossy(cx.text(comment)).into_owned();
    let doc = javadoc_tags::parse(&text, comment.start_position().row as u32 + 1);
    if let Some(e) = doc.error {
        cx.diag_at("JavadocMethod", e.line, 0, format!("Javadoc comment at column {} has parse error. Details: {}", e.col, e.details));
        return;
    }
    let is_element = decl.kind() == "annotation_type_element_declaration";
    let is_ctor = matches!(decl.kind(), "constructor_declaration" | "compact_constructor_declaration");
    let at = |cx: &Ctx, t: &javadoc_tags::Tag| (t.line, column_of(cx, t.line, t.col));
    let decl_line = decl.start_position().row as u32 + 1;

    // The tags the check collects.
    let mut param_tags: Vec<&javadoc_tags::Tag> = doc.tags.iter().filter(|t| t.name == "param" && t.arg.is_some()).collect();
    let return_tags: Vec<&javadoc_tags::Tag> = doc.tags.iter().filter(|t| t.name == "return" && t.has_description).collect();
    let throws_tags: Vec<&javadoc_tags::Tag> = doc.tags.iter().filter(|t| matches!(t.name.as_str(), "throws" | "exception") && t.arg.is_some()).collect();
    let inherit = doc.inherit_doc;
    let only_inherit = inherit && doc.tags.is_empty();
    let invalid_inherit_target = has_modifier(cx, decl, b"private") || is_ctor || has_modifier(cx, decl, b"static");
    if only_inherit && invalid_inherit_target {
        cx.diag("JavadocMethod", decl, "Invalid use of the '@inheritDoc' tag.".to_string());
        return;
    }

    let report_expected = !inherit && !has_allowed_annotation(cx, decl, &opts.allowed_annotations);
    let non_void = is_element || (decl.kind() == "method_declaration" && !decl_is_void(cx, decl));
    // A `@return` with a description that has no return value to describe is unused.
    let leftovers: Vec<&javadoc_tags::Tag> = if non_void { Vec::new() } else { doc.tags.iter().filter(|t| t.name == "return" && t.has_description).collect() };

    if !is_element {
        let sig = signature(cx, decl);
        let record_components = if decl.kind() == "compact_constructor_declaration" { enclosing_record_components(cx, decl) } else { Vec::new() };
        // @param tags against parameters, then type parameters.
        let mut documented: Vec<String> = Vec::new();
        let known: Vec<&str> = if decl.kind() == "compact_constructor_declaration" {
            record_components.iter().map(String::as_str).collect()
        } else {
            sig.params.iter().map(|(_, n)| n.as_str()).chain(sig.type_params.iter().map(|(_, n)| n.as_str())).collect()
        };
        for tag in param_tags.drain(..) {
            let name = tag.arg.clone().unwrap_or_default();
            if documented.contains(&name) {
                let (l, c) = at(cx, tag);
                cx.diag_at("JavadocMethod", l, c, "Duplicate @param tag.".to_string());
                continue;
            }
            if known.contains(&name.as_str()) {
                documented.push(name);
            } else {
                let (l, c) = at(cx, tag);
                cx.diag_at("JavadocMethod", l, c, format!("Unused @param tag for '{name}'."));
            }
        }
        if report_expected && !opts.allow_missing_param_tags {
            if decl.kind() == "compact_constructor_declaration" {
                for name in record_components.iter().filter(|n| !documented.contains(n)) {
                    cx.diag("JavadocMethod", decl, format!("Expected @param tag for '{name}'."));
                }
            } else {
                for (node, name) in sig.params.iter().chain(sig.type_params.iter()) {
                    if !documented.contains(name) {
                        cx.diag("JavadocMethod", *node, format!("Expected @param tag for '{name}'."));
                    }
                }
            }
        }

        // Throws.
        if opts.validate_throws && report_expected {
            let mut wanted: Vec<(Node, String)> = Vec::new();
            if let Some(t) = decl.children(&mut decl.walk()).find(|c| c.kind() == "throws") {
                for ty in t.named_children(&mut t.walk()) {
                    if let Some(part) = type_name_parts(cx, ty) {
                        wanted.push(part);
                    }
                }
            }
            if let Some(body) = decl.child_by_field_name("body") {
                thrown(cx, body, &mut wanted);
            }
            // Each exception class is wanted once, wherever it is thrown (probed on Guava).
            let mut seen: Vec<String> = Vec::new();
            let simple_of = |written: &str| written.rsplit('.').next().unwrap_or_default().to_string();
            wanted.retain(|(_, written)| {
                let simple = simple_of(written);
                let fresh = !seen.contains(&simple);
                if fresh {
                    seen.push(simple);
                }
                fresh
            });
            let mut documented_throws: Vec<String> = throws_tags.iter().filter_map(|t| t.arg.as_ref().map(|a| a.rsplit('.').next().unwrap_or(a).to_string())).collect();
            for (node, written) in wanted {
                if let Some(i) = documented_throws.iter().position(|d| *d == simple_of(&written)) {
                    documented_throws.remove(i);
                } else {
                    cx.diag("JavadocMethod", node, format!("Expected @throws tag for '{written}'."));
                }
            }
        }
    }

    // Return.
    if non_void {
        let expected = (is_element || report_expected) && !opts.allow_missing_return_tag;
        let have_inline = opts.allow_inline_return && doc.inline_return;
        if return_tags.is_empty() && !have_inline {
            if expected {
                cx.diag_at("JavadocMethod", decl_line, 0, "@return tag should be present and have description.".to_string());
            }
        } else {
            for tag in return_tags.iter().skip(1) {
                let (l, c) = at(cx, tag);
                cx.diag_at("JavadocMethod", l, c, "Duplicate @return tag.".to_string());
            }
        }
    }
    // Whatever was not used.
    let mut extra: Vec<&javadoc_tags::Tag> = leftovers;
    if is_element {
        extra.extend(doc.tags.iter().filter(|t| t.name == "param" && t.arg.is_some()));
    }
    extra.sort_by_key(|t| (t.line, t.col));
    extra.dedup_by_key(|t| (t.line, t.col));
    for tag in extra {
        cx.diag_at("JavadocMethod", tag.line, 0, "Unused Javadoc tag.".to_string());
    }
}

fn decl_is_void(cx: &Ctx, decl: Node) -> bool {
    decl.child_by_field_id(cx.syn.f.type_).is_some_and(|t| t.kind() == "void_type")
}

fn enclosing_record_components(cx: &Ctx, decl: Node) -> Vec<String> {
    let mut p = decl.parent();
    while let Some(n) = p {
        if n.kind() == "record_declaration" {
            return record_component_names(cx, n);
        }
        p = n.parent();
    }
    Vec::new()
}

/// `MissingJavadocType`: the declaration's scope is the least visible of it and the types around it.
pub fn on_missing_type(cx: &mut Ctx, decl: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.missing_javadoc_type else { return };
    let surrounding = surrounding_scope(cx, decl);
    if surrounding == Some(ANONYMOUS) {
        return;
    }
    let effective = declared_scope(cx, decl).max(surrounding.unwrap_or(0));
    if effective > opts.scope || opts.exclude_scope.is_some_and(|e| effective <= e) || has_allowed_annotation(cx, decl, &opts.skip_annotations) {
        return;
    }
    if attached(cx, decl).is_some() {
        return;
    }
    let name = decl.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).into_owned()).unwrap_or_default();
    cx.diag("MissingJavadocType", decl, format!("Missing a Javadoc comment for '{name}'."));
}
