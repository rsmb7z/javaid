//! Small structural modules, each a few lines over the tree: `OneTopLevelClass` and its neighbours.
//!
//! One hook, `on_node`, sees every node outside package and import declarations; each module picks
//! out the node kinds it cares about. Findings go where Checkstyle logs them: on the declaration's
//! first token, annotations included.

use crate::config::Format;
use crate::engine::Ctx;
use crate::rules::util::has_modifier;
use tree_sitter::Node;

/// Which of the modules in this file are on, and their settings.
#[derive(Clone, Debug, Default)]
pub struct Options {
    pub one_top_level_class: bool,
    pub no_finalizer: bool,
    /// `PackageDeclaration`: whether `matchDirectoryStructure` is on.
    pub package_declaration: Option<bool>,
    pub outer_type_filename: bool,
    pub no_line_wrap: Option<NoLineWrap>,
    /// `ConstructorsDeclarationGrouping`: whether `orderByIncreasingParameterCount` is on.
    pub constructors_grouping: Option<bool>,
    /// `OverloadMethodsDeclarationOrder`: whether `orderByIncreasingParameterCount` is on.
    pub overload_methods: Option<bool>,
    pub missing_override_on_record_accessor: bool,
    pub no_whitespace_before_case_default_colon: bool,
    pub whitespace_before_empty_body: Option<Vec<&'static str>>,
    pub annotation_location: Vec<AnnotationLocation>,
    pub avoid_module_import: Option<AvoidModuleImport>,
    pub module_directive_order: Option<ModuleDirectiveOrder>,
    pub single_space_separator: Option<SingleSpaceSeparator>,
    pub array_bracket_no_whitespace: bool,
    pub illegal_token_text: Vec<IllegalTokenText>,
}

/// The names of a `tokens` property, each one of `accepted` (Checkstyle refuses any other).
pub fn token_names(list: Option<String>, default: &str, accepted: &[&'static str]) -> Result<Vec<&'static str>, String> {
    let list = list.unwrap_or_else(|| default.to_string());
    let mut out = Vec::new();
    for name in list.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        match accepted.iter().find(|a| **a == name) {
            Some(a) => out.push(*a),
            None => return Err(format!("tokens: {name} is not supported by this module")),
        }
    }
    Ok(out)
}

#[derive(Clone, Debug)]
pub struct NoLineWrap {
    pub skip_annotations: bool,
    pub tokens: Vec<&'static str>,
}

pub const NO_LINE_WRAP_DEFAULT: &str = "PACKAGE_DEF, IMPORT, STATIC_IMPORT, MODULE_IMPORT, MODULE_DEF";
pub const NO_LINE_WRAP_ACCEPTED: &[&str] =
    &["IMPORT", "STATIC_IMPORT", "MODULE_IMPORT", "PACKAGE_DEF", "CLASS_DEF", "METHOD_DEF", "CTOR_DEF", "ENUM_DEF", "INTERFACE_DEF", "RECORD_DEF", "COMPACT_CTOR_DEF", "MODULE_DEF"];

pub fn on_node(cx: &mut Ctx, node: Node) {
    if cx.cfg.gs.one_top_level_class && node.kind() == "program" {
        one_top_level_class(cx, node);
    }
    if cx.cfg.gs.no_finalizer && node.kind() == "method_declaration" {
        no_finalizer(cx, node);
    }
    if let Some(order) = cx.cfg.gs.constructors_grouping {
        if matches!(node.kind(), "class_declaration" | "enum_declaration" | "record_declaration") {
            constructors_grouping(cx, node, order);
        }
    }
    if let Some(order) = cx.cfg.gs.overload_methods {
        let body = match node.kind() {
            "class_declaration" | "interface_declaration" | "record_declaration" | "enum_declaration" => node.child_by_field_id(cx.syn.f.body),
            "object_creation_expression" => child_of_kind(node, "class_body"),
            _ => None,
        };
        if let Some(body) = body {
            overload_order(cx, body, order);
        }
    }
    if cx.cfg.gs.missing_override_on_record_accessor && node.kind() == "method_declaration" {
        record_accessor(cx, node);
    }
    if cx.cfg.gs.no_whitespace_before_case_default_colon && node.kind() == ":" {
        case_default_colon(cx, node);
    }
    if let Some(tokens) = &cx.cfg.gs.whitespace_before_empty_body {
        whitespace_before_empty_body(cx, node, tokens);
    }
    for i in 0..cx.cfg.gs.annotation_location.len() {
        let opts = &cx.cfg.gs.annotation_location[i];
        annotation_location(cx, node, opts);
    }
    if let Some(opts) = &cx.cfg.gs.avoid_module_import {
        if node.kind() == "program" {
            avoid_module_import(cx, node, opts);
        }
    }
    if let Some(opts) = &cx.cfg.gs.module_directive_order {
        if node.kind() == "module_declaration" {
            module_directive_order(cx, node, opts);
        }
    }
    if let Some(opts) = &cx.cfg.gs.single_space_separator {
        if matches!(node.kind(), "package_declaration" | "import_declaration") {
            // The walk does not enter these: look at their tokens here.
            let mut stack = vec![node];
            while let Some(n) = stack.pop() {
                single_space_separator(cx, n, opts.validate_comments);
                let mut cur = n.walk();
                let kids: Vec<Node> = n.children(&mut cur).collect();
                stack.extend(kids.into_iter().rev());
            }
        } else {
            single_space_separator(cx, node, opts.validate_comments);
        }
    }
    if cx.cfg.gs.array_bracket_no_whitespace && matches!(node.kind(), "[" | "]") {
        array_bracket_no_whitespace(cx, node);
    }
    if !cx.cfg.gs.illegal_token_text.is_empty() {
        if matches!(node.kind(), "package_declaration" | "import_declaration") {
            illegal_token_text_inside(cx, node);
        } else {
            illegal_token_text(cx, node);
        }
    }
    if let Some(opts) = &cx.cfg.gs.no_line_wrap {
        no_line_wrap(cx, node, opts);
    }
    if cx.cfg.gs.outer_type_filename && node.kind() == "program" {
        outer_type_filename(cx, node);
    }
    if let Some(match_directory) = cx.cfg.gs.package_declaration {
        match node.kind() {
            "program" => missing_package(cx, node),
            "package_declaration" if match_directory => package_directory(cx, node),
            _ => {}
        }
    }
}

/// The dotted text of a name, without the spaces and comments the source put inside it:
/// Checkstyle's `FullIdent`.
pub fn full_ident(cx: &Ctx, name: Node) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut stack = vec![name];
    while let Some(n) = stack.pop() {
        if n.child_count() == 0 {
            if !n.is_extra() && n.kind() != "." {
                parts.push(String::from_utf8_lossy(cx.text(n)).into_owned());
            }
            continue;
        }
        let mut cur = n.walk();
        let kids: Vec<Node> = n.children(&mut cur).collect();
        stack.extend(kids.into_iter().rev());
    }
    parts.join(".")
}

/// `PackageDeclaration`, missing half: a file with code and no `package` (a module declaration
/// is exempt), reported on its first token. A file of nothing but comments has no tree to report on.
fn missing_package(cx: &mut Ctx, program: Node) {
    let mut cur = program.walk();
    let kids: Vec<Node> = program.children(&mut cur).filter(|c| !c.is_extra()).collect();
    let Some(first) = kids.first() else { return };
    if kids.iter().any(|c| matches!(c.kind(), "package_declaration" | "module_declaration")) {
        return;
    }
    cx.diag("PackageDeclaration", *first, "Missing package declaration.".into());
}

/// `PackageDeclaration`, directory half: the file's directory must end with the package name.
fn package_directory(cx: &mut Ctx, decl: Node) {
    let Some(path) = cx.path else { return };
    let mut cur = decl.walk();
    let kids: Vec<Node> = decl.children(&mut cur).collect();
    let Some(name) = kids.iter().rev().find(|c| matches!(c.kind(), "scoped_identifier" | "identifier")) else { return };
    let Some(keyword) = kids.iter().find(|c| c.kind() == "package") else { return };
    let package = full_ident(cx, *name).replace('.', "/");
    let absolute = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let dir = absolute.parent().map(|d| d.to_string_lossy().into_owned()).unwrap_or_default();
    if !dir.ends_with(&package) {
        cx.diag("PackageDeclaration", *keyword, "Package name is not same as directory.".into());
    }
}

/// The parameters of a method or constructor, receiver parameters included: Checkstyle counts all
/// of them as `PARAMETER_DEF`.
fn parameter_count(cx: &Ctx, decl: Node) -> usize {
    let Some(params) = decl.child_by_field_id(cx.syn.f.parameters) else { return 0 };
    let mut cur = params.walk();
    let n = params.children(&mut cur).filter(|c| matches!(c.kind(), "formal_parameter" | "spread_parameter" | "receiver_parameter")).count();
    n
}

/// `NoFinalizer`: a method called `finalize` that takes nothing.
fn no_finalizer(cx: &mut Ctx, method: Node) {
    let Some(name) = method.child_by_field_id(cx.syn.f.name) else { return };
    if cx.text(name) == b"finalize" && parameter_count(cx, method) == 0 {
        cx.diag("NoFinalizer", method, "Avoid using finalizer method.".into());
    }
}

pub fn is_type_declaration(kind: &str) -> bool {
    matches!(kind, "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration")
}

fn type_name<'a>(cx: &Ctx<'a>, ty: Node) -> String {
    ty.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).into_owned()).unwrap_or_default()
}

/// `OneTopLevelClass`: only the first top-level type may be there, unless a public one exists, in
/// which case the first is reported too when it is not the public one.
fn one_top_level_class(cx: &mut Ctx, program: Node) {
    let mut public_found = false;
    let mut first: Option<Node> = None;
    let mut cur = program.walk();
    let types: Vec<Node> = program.children(&mut cur).filter(|c| is_type_declaration(c.kind())).collect();
    for ty in types {
        let public = has_modifier(cx, ty, b"public");
        if public {
            public_found = true;
        }
        match first {
            None => first = Some(ty),
            Some(_) if !public => {
                let name = type_name(cx, ty);
                cx.diag("OneTopLevelClass", ty, format!("Top-level class {name} has to reside in its own source file."));
            }
            Some(_) => {}
        }
    }
    if let Some(first) = first {
        if public_found && !has_modifier(cx, first, b"public") {
            let name = type_name(cx, first);
            cx.diag("OneTopLevelClass", first, format!("Top-level class {name} has to reside in its own source file."));
        }
    }
}

/// `OuterTypeFilename`: the first type of the file carries the file's name, unless another top-level
/// type is public (the first type is not counted: a public first type with the wrong name is reported
/// when nothing else in the file is public).
fn outer_type_filename(cx: &mut Ctx, program: Node) {
    let Some(path) = cx.path else { return };
    let mut cur = program.walk();
    let types: Vec<Node> = program.children(&mut cur).filter(|c| is_type_declaration(c.kind())).collect();
    let Some(first) = types.first() else { return };
    let file = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    // Java's `\.[^.]*$`: the last dot and what follows it.
    let stem = file.rfind('.').map_or(file.as_str(), |i| &file[..i]);
    let public_elsewhere = types[1..].iter().any(|t| has_modifier(cx, *t, b"public"));
    if !public_elsewhere && type_name(cx, *first) != stem {
        cx.diag("OuterTypeFilename", *first, "The name of the outer type and the file do not match.".into());
    }
}

/// The first child of `node` of this kind.
fn child_of_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cur = node.walk();
    let found = node.children(&mut cur).find(|c| c.kind() == kind);
    found
}

/// Does the declaration carry an annotation among its modifiers? Returns the node after them.
fn after_annotated_modifiers<'a>(decl: Node<'a>) -> Option<Node<'a>> {
    let mods = child_of_kind(decl, "modifiers")?;
    let mut cur = mods.walk();
    let annotated = mods.children(&mut cur).any(|m| matches!(m.kind(), "annotation" | "marker_annotation"));
    if !annotated {
        return None;
    }
    let mut next = mods.next_sibling();
    while let Some(n) = next.filter(|n| n.is_extra()) {
        next = n.next_sibling();
    }
    next
}

/// `NoLineWrap`: a package, import or declaration header written on one line, from its first token
/// (past the annotations, unless `skipAnnotations` is off) to the `;` or `{` that ends it.
fn no_line_wrap(cx: &mut Ctx, node: Node, opts: &NoLineWrap) {
    let kind = node.kind();
    let token = match kind {
        "package_declaration" => "PACKAGE_DEF",
        "import_declaration" => {
            let mut cur = node.walk();
            let mut modifier = "IMPORT";
            for c in node.children(&mut cur) {
                match c.kind() {
                    "static" => modifier = "STATIC_IMPORT",
                    "module" => modifier = "MODULE_IMPORT",
                    _ => {}
                }
            }
            modifier
        }
        "module_declaration" => "MODULE_DEF",
        "class_declaration" => "CLASS_DEF",
        "interface_declaration" => "INTERFACE_DEF",
        "enum_declaration" => "ENUM_DEF",
        "record_declaration" => "RECORD_DEF",
        "method_declaration" => "METHOD_DEF",
        "constructor_declaration" => "CTOR_DEF",
        "compact_constructor_declaration" => "COMPACT_CTOR_DEF",
        _ => return,
    };
    if !opts.tokens.contains(&token) {
        return;
    }
    let row = |n: Node| n.start_position().row;
    let last = |n: Node| -> Option<usize> {
        let mut cur = n.walk();
        let kids: Vec<Node> = n.children(&mut cur).filter(|c| !c.is_extra()).collect();
        kids.last().map(|k| row(*k))
    };
    let (at, first_row, last_row) = match token {
        "PACKAGE_DEF" | "IMPORT" | "STATIC_IMPORT" | "MODULE_IMPORT" => {
            let keyword = child_of_kind(node, if token == "PACKAGE_DEF" { "package" } else { "import" });
            let Some(keyword) = keyword else { return };
            let mut cur = node.walk();
            let annotated = node.children(&mut cur).any(|c| matches!(c.kind(), "annotation" | "marker_annotation"));
            let first = if token == "PACKAGE_DEF" && annotated && opts.skip_annotations {
                // The name's outermost token: Checkstyle's `DOT` sits at its last dot.
                let name = node.children(&mut cur).filter(|c| matches!(c.kind(), "scoped_identifier" | "identifier")).last();
                match name {
                    Some(n) => child_of_kind(n, ".").map_or(row(n), row),
                    None => row(keyword),
                }
            } else {
                row(keyword)
            };
            (keyword, first, last(node))
        }
        "MODULE_DEF" => {
            // The annotations are children of the declaration itself; the first line is that of the
            // first token after them.
            let mut cur = node.walk();
            let kids: Vec<Node> = node.children(&mut cur).filter(|c| !c.is_extra()).collect();
            let annotated = kids.iter().any(|c| matches!(c.kind(), "annotation" | "marker_annotation"));
            let first = match kids.iter().find(|c| !matches!(c.kind(), "annotation" | "marker_annotation")).filter(|_| annotated && opts.skip_annotations) {
                Some(n) => row(*n),
                None => row(node),
            };
            (node, first, node.child_by_field_id(cx.syn.f.body).map(row))
        }
        _ => {
            let first = match after_annotated_modifiers(node).filter(|_| opts.skip_annotations) {
                Some(n) => row(n),
                None => row(node),
            };
            let end = node.child_by_field_id(cx.syn.f.body).map(row).or_else(|| last(node));
            (node, first, end)
        }
    };
    if Some(first_row) != last_row {
        let text = match token {
            "PACKAGE_DEF" => "package",
            "IMPORT" | "STATIC_IMPORT" | "MODULE_IMPORT" => "import",
            other => other,
        };
        cx.diag("NoLineWrap", at, format!("{text} statement should not be line-wrapped."));
    }
}

/// The members of a type body as Checkstyle's `OBJBLOCK` lists them, comments and the braces left
/// out: for an enum the constants, the separator and the members all sit in one list.
fn members<'a>(body: Node<'a>) -> Vec<Node<'a>> {
    let mut out = Vec::new();
    let mut cur = body.walk();
    for c in body.children(&mut cur) {
        if c.is_extra() || matches!(c.kind(), "{" | "}") {
            continue;
        }
        if c.kind() == "enum_body_declarations" {
            let mut inner = c.walk();
            out.extend(c.children(&mut inner).filter(|m| !m.is_extra()));
        } else {
            out.push(c);
        }
    }
    out
}

fn is_constructor(n: Node) -> bool {
    matches!(n.kind(), "constructor_declaration" | "compact_constructor_declaration")
}

/// `ConstructorsDeclarationGrouping`: once the first constructor is met, every member up to the
/// last constructor is a constructor, or the stragglers are reported with the line of the last
/// constructor of the first group. With `orderByIncreasingParameterCount`, a constructor with fewer
/// parameters than the one before it, or after one that was out of order, is reported too.
fn constructors_grouping(cx: &mut Ctx, ty: Node, order_by_count: bool) {
    let Some(body) = ty.child_by_field_id(cx.syn.f.body) else { return };
    let children = members(body);
    let Some(first) = children.iter().position(|c| is_constructor(*c)) else { return };
    let after_first = &children[first..];
    let split = after_first.iter().position(|c| !is_constructor(*c));
    if let Some(split) = split {
        let last_grouped = after_first[split - 1];
        let line = last_grouped.start_position().row + 1;
        for ctor in after_first[split..].iter().filter(|c| is_constructor(**c)) {
            cx.diag("ConstructorsDeclarationGrouping", *ctor, format!("Constructors should be grouped together. The last grouped constructor is declared at line '{line}'."));
        }
    }
    if order_by_count {
        let mut previous = 0usize;
        let mut ordered = true;
        for ctor in children.iter().filter(|c| is_constructor(**c)) {
            let count = if ctor.kind() == "constructor_declaration" { parameter_count(cx, *ctor) } else { 0 };
            ordered = ordered && count >= previous;
            previous = count;
            if !ordered {
                cx.diag("ConstructorsDeclarationGrouping", *ctor, "Constructors should be ordered by increasing parameter count.".into());
            }
        }
    }
}

/// `OverloadMethodsDeclarationOrder`: the methods of one name sit next to each other (no member of
/// any other kind, and no method of another name, between them). With `orderByIncreasingParameterCount`
/// they are also in order of parameter count, and once one is out of order all the rest of that name are.
fn overload_order(cx: &mut Ctx, body: Node, order_by_count: bool) {
    use std::collections::HashMap;
    let children = members(body);
    // name -> (index among methods, line, parameter count, ordered so far)
    let mut seen: HashMap<Vec<u8>, (usize, u32, usize, bool)> = HashMap::new();
    let mut index = 0usize;
    for (i, child) in children.iter().enumerate() {
        if child.kind() != "method_declaration" {
            continue;
        }
        index += 1;
        let Some(name) = child.child_by_field_id(cx.syn.f.name) else { continue };
        let name = cx.text(name).to_vec();
        let count = parameter_count(cx, *child);
        let line = child.start_position().row as u32 + 1;
        let mut ordered = true;
        if let Some(&(previous_index, previous_line, previous_count, previous_ordered)) = seen.get(&name) {
            let previous_is_method = i > 0 && children[i - 1].kind() == "method_declaration";
            if !previous_is_method || index - previous_index > 1 {
                cx.diag("OverloadMethodsDeclarationOrder", *child, format!("All overloaded methods should be placed next to each other. Previous overloaded method located at line '{}'.", crate::engine::grouped(previous_line)));
            }
            ordered = previous_ordered;
            if order_by_count {
                ordered = previous_ordered && count >= previous_count;
                if !ordered {
                    cx.diag("OverloadMethodsDeclarationOrder", *child, "Overloaded methods should be ordered by increasing parameter count.".into());
                }
            }
        }
        seen.insert(name, (index, line, count, ordered));
    }
}

/// The names of the annotations among a declaration's modifiers, as written without blanks.
fn annotation_names(cx: &Ctx, decl: Node) -> Vec<String> {
    let Some(mods) = child_of_kind(decl, "modifiers") else { return Vec::new() };
    let mut cur = mods.walk();
    let kids: Vec<Node> = mods.children(&mut cur).filter(|m| matches!(m.kind(), "annotation" | "marker_annotation")).collect();
    kids.into_iter().filter_map(|a| a.child_by_field_id(cx.syn.f.name)).map(|n| full_ident(cx, n)).collect()
}

/// `MissingOverrideOnRecordAccessor`: a method with no parameters, named like a component of the
/// record it is declared in, needs `@Override`.
fn record_accessor(cx: &mut Ctx, method: Node) {
    let (Some(body), Some(record)) = (cx.ancestor(0), cx.ancestor(1)) else { return };
    if body.kind() != "class_body" || record.kind() != "record_declaration" || parameter_count(cx, method) != 0 {
        return;
    }
    let Some(name) = method.child_by_field_id(cx.syn.f.name) else { return };
    let Some(components) = record.child_by_field_id(cx.syn.f.parameters) else { return };
    let mut cur = components.walk();
    let is_component = components.children(&mut cur).any(|c| {
        let declared = match c.kind() {
            "formal_parameter" => c.child_by_field_id(cx.syn.f.name),
            "spread_parameter" => child_of_kind(c, "variable_declarator").and_then(|d| d.child_by_field_id(cx.syn.f.name)),
            _ => None,
        };
        declared.is_some_and(|n| cx.text(n) == cx.text(name))
    });
    if is_component && !annotation_names(cx, method).iter().any(|a| a == "Override" || a == "java.lang.Override") {
        cx.diag("MissingOverrideOnRecordAccessor", method, "Record component accessor method must include @java.lang.Override annotation.".into());
    }
}

/// `NoWhitespaceBeforeCaseDefaultColon`: the `:` that ends a `case` or `default` label touches the
/// label. The tests Checkstyle makes, in its own terms: the colon is on the line of the label's last
/// element (of the keyword, for `default`) and starts where that element ends. A block comment
/// between them counts as the thing the colon must touch, and it must begin on the colon's line; a
/// line comment never can. A label that runs over several lines is judged by the line it starts on.
fn case_default_colon(cx: &mut Ctx, colon: Node) {
    if cx.parent().is_none_or(|p| p.kind() != "switch_block_statement_group") {
        return;
    }
    let row = colon.start_position().row;
    let mut prev = colon.prev_sibling();
    let comment = prev.filter(|p| p.is_extra());
    // The label is the nearest sibling before the colon that is not a comment.
    while let Some(p) = prev.filter(|p| p.is_extra()) {
        prev = p.prev_sibling();
    }
    let Some(label) = prev.filter(|l| l.kind() == "switch_label") else { return };
    let mut cur = label.walk();
    let kids: Vec<Node> = label.children(&mut cur).filter(|c| !c.is_extra()).collect();
    let is_default = kids.len() == 1 && kids[0].kind() == "default";
    let flagged = if is_default && kids[0].start_position().row != row {
        true
    } else if let Some(c) = comment {
        c.kind() == "line_comment" || c.start_position().row != row || c.end_byte() != colon.start_byte()
    } else if is_default {
        label.end_byte() != colon.start_byte()
    } else {
        label.end_byte() != colon.start_byte() || kids.last().is_none_or(|last| last.start_position().row != row)
    };
    if flagged {
        cx.diag("NoWhitespaceBeforeCaseDefaultColon", colon, "':' is preceded with whitespace.".into());
    }
}

pub const WHITESPACE_BEFORE_EMPTY_BODY_TOKENS: &[&str] = &[
    "METHOD_DEF", "CTOR_DEF", "COMPACT_CTOR_DEF", "CLASS_DEF", "INTERFACE_DEF", "RECORD_DEF", "ENUM_DEF", "ANNOTATION_DEF", "LITERAL_NEW", "LITERAL_DO",
    "LITERAL_WHILE", "LITERAL_FOR", "LITERAL_IF", "LITERAL_ELSE", "STATIC_INIT", "LITERAL_TRY", "LITERAL_CATCH", "LITERAL_FINALLY", "LITERAL_SYNCHRONIZED",
    "LITERAL_SWITCH", "LAMBDA",
];

/// Is the body, an opening brace and a closing one with nothing but comments between, empty?
fn is_empty_body(body: Node) -> bool {
    let mut cur = body.walk();
    let n = body.children(&mut cur).filter(|c| !c.is_extra() && !matches!(c.kind(), "{" | "}")).count();
    n == 0 && body.child(0).is_some_and(|c| c.kind() == "{")
}

/// `WhitespaceBeforeEmptyBody`: the `{` of an empty body has a blank, a line break or a comment
/// before it. The body must be empty, and the `{` is judged against the token before it: only a
/// token touching it is a finding.
fn whitespace_before_empty_body(cx: &mut Ctx, node: Node, tokens: &[&'static str]) {
    let on = |t: &str| tokens.contains(&t);
    fn block(n: Option<Node<'_>>) -> Option<Node<'_>> {
        n.filter(|b| b.kind() == "block")
    }
    let mut bodies: Vec<Node> = Vec::new();
    match node.kind() {
        "method_declaration" if on("METHOD_DEF") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "constructor_declaration" if on("CTOR_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "compact_constructor_declaration" if on("COMPACT_CTOR_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "class_declaration" if on("CLASS_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "interface_declaration" if on("INTERFACE_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "record_declaration" if on("RECORD_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "enum_declaration" if on("ENUM_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "annotation_type_declaration" if on("ANNOTATION_DEF") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "object_creation_expression" if on("LITERAL_NEW") => bodies.extend(child_of_kind(node, "class_body")),
        "do_statement" if on("LITERAL_DO") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "while_statement" if on("LITERAL_WHILE") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "for_statement" | "enhanced_for_statement" if on("LITERAL_FOR") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "if_statement" => {
            if on("LITERAL_IF") {
                bodies.extend(block(node.child_by_field_id(cx.syn.f.consequence)));
            }
            if on("LITERAL_ELSE") {
                bodies.extend(block(node.child_by_field_id(cx.syn.f.alternative)));
            }
        }
        "static_initializer" if on("STATIC_INIT") => bodies.extend(child_of_kind(node, "block")),
        "try_statement" | "try_with_resources_statement" if on("LITERAL_TRY") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "catch_clause" if on("LITERAL_CATCH") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "finally_clause" if on("LITERAL_FINALLY") => bodies.extend(child_of_kind(node, "block")),
        "synchronized_statement" if on("LITERAL_SYNCHRONIZED") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        "switch_expression" if on("LITERAL_SWITCH") => bodies.extend(node.child_by_field_id(cx.syn.f.body)),
        "lambda_expression" if on("LAMBDA") => bodies.extend(block(node.child_by_field_id(cx.syn.f.body))),
        _ => {}
    }
    for body in bodies {
        if !is_empty_body(body) {
            continue;
        }
        let at = body.start_byte();
        let touching = at > 0 && !cx.src[at - 1].is_ascii_whitespace() && !cx.src[..at].ends_with(b"*/");
        if touching {
            let open = body.child(0).unwrap_or(body);
            cx.diag("WhitespaceBeforeEmptyBody", open, "'{' is not preceded with whitespace.".into());
        }
    }
}

/// One `AnnotationLocation` instance.
#[derive(Clone, Debug)]
pub struct AnnotationLocation {
    pub name: &'static str,
    pub tokens: Vec<&'static str>,
    pub same_line_single: bool,
    pub same_line_parameterized: bool,
    pub same_line_multiple: bool,
}

pub const ANNOTATION_LOCATION_DEFAULT: &str = "CLASS_DEF, INTERFACE_DEF, PACKAGE_DEF, ENUM_CONSTANT_DEF, ENUM_DEF, METHOD_DEF, CTOR_DEF, VARIABLE_DEF, RECORD_DEF, COMPACT_CTOR_DEF, MODULE_DEF";
pub const ANNOTATION_LOCATION_ACCEPTED: &[&str] = &[
    "CLASS_DEF", "INTERFACE_DEF", "PACKAGE_DEF", "ENUM_CONSTANT_DEF", "ENUM_DEF", "METHOD_DEF", "CTOR_DEF", "VARIABLE_DEF", "ANNOTATION_DEF", "ANNOTATION_FIELD_DEF",
    "RECORD_DEF", "COMPACT_CTOR_DEF", "MODULE_DEF",
];

fn is_annotation(n: Node) -> bool {
    matches!(n.kind(), "annotation" | "marker_annotation")
}

/// The 0-based column of a node in characters, as Checkstyle counts it.
fn char_column(cx: &Ctx, n: Node) -> usize {
    let start = n.start_byte();
    let line_start = cx.src[..start].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    String::from_utf8_lossy(&cx.src[line_start..start]).chars().count()
}

/// The name Checkstyle prints for an annotation: its identifier, or for a qualified name the first
/// identifier directly under the last `.` (the first segment for `@a.B`, the last for `@a.b.C`).
fn annotation_name(cx: &Ctx, a: Node) -> String {
    let Some(name) = a.child_by_field_id(cx.syn.f.name) else { return String::new() };
    let parts: Vec<String> = full_ident(cx, name).split('.').map(String::from).collect();
    if parts.len() == 2 { parts[0].clone() } else { parts.last().cloned().unwrap_or_default() }
}

/// Does the annotation have an argument that is a value or a `name = value` pair? An array
/// initialiser or a nested annotation as the only argument does not count.
fn is_parameterized(a: Node) -> bool {
    let Some(args) = a.child_by_field_name("arguments") else { return false };
    let mut cur = args.walk();
    let kids: Vec<Node> = args.children(&mut cur).collect();
    kids.iter().any(|c| !c.is_extra() && !matches!(c.kind(), "(" | ")" | "," | "element_value_array_initializer") && !is_annotation(*c))
}

fn annotation_location(cx: &mut Ctx, node: Node, opts: &AnnotationLocation) {
    let token = match node.kind() {
        "package_declaration" => "PACKAGE_DEF",
        "module_declaration" => "MODULE_DEF",
        "class_declaration" => "CLASS_DEF",
        "interface_declaration" => "INTERFACE_DEF",
        "enum_declaration" => "ENUM_DEF",
        "record_declaration" => "RECORD_DEF",
        "annotation_type_declaration" => "ANNOTATION_DEF",
        "enum_constant" => "ENUM_CONSTANT_DEF",
        "annotation_type_element_declaration" => "ANNOTATION_FIELD_DEF",
        "method_declaration" => "METHOD_DEF",
        "constructor_declaration" => "CTOR_DEF",
        "compact_constructor_declaration" => "COMPACT_CTOR_DEF",
        "field_declaration" | "constant_declaration" => "VARIABLE_DEF",
        _ => return,
    };
    if !opts.tokens.contains(&token) {
        return;
    }
    // The siblings the annotations sit among: the modifiers, or the declaration itself for a package
    // and a module. Only the annotations that come first count.
    let holder = if matches!(token, "PACKAGE_DEF" | "MODULE_DEF") { Some(node) } else { child_of_kind(node, "modifiers") };
    let Some(holder) = holder else { return };
    let mut cur = holder.walk();
    let kids: Vec<Node> = holder.children(&mut cur).filter(|c| !c.is_extra()).collect();
    let leading: Vec<Node> = kids.iter().copied().take_while(|c| is_annotation(*c)).collect();
    if leading.is_empty() {
        return;
    }
    let row = |n: Node| n.start_position().row;
    let correct = char_column(cx, leading[0]);
    for (i, ann) in leading.iter().enumerate() {
        let params = is_parameterized(*ann);
        let before = i > 0 && row(leading[i - 1]) == row(*ann);
        // The node after: the next modifier, or the first thing after the modifiers.
        let last = i + 1 == leading.len();
        let after = if token == "PACKAGE_DEF" && last {
            // Checkstyle's next sibling is the name, at its last dot.
            kids.iter().rev().find(|c| matches!(c.kind(), "scoped_identifier" | "identifier")).map(|n| child_of_kind(*n, ".").map_or(row(*n), row))
        } else if let Some(next) = kids.get(i + 1) {
            Some(row(*next))
        } else {
            let mut next = holder.next_sibling();
            while let Some(n) = next.filter(|n| n.is_extra()) {
                next = n.next_sibling();
            }
            next.map(row)
        };
        let beside = before || after == Some(row(*ann));
        let allowing = if params { opts.same_line_parameterized } else { opts.same_line_single };
        let correct_location = opts.same_line_multiple || (allowing && !before) || !beside;
        let name = annotation_name(cx, *ann);
        if !correct_location {
            cx.diag(opts.name, *ann, format!("Annotation '{name}' should be alone on line."));
        } else if char_column(cx, *ann) != correct && !before {
            let column = char_column(cx, *ann);
            cx.diag(opts.name, *ann, format!("Annotation '{name}' have incorrect indentation level {column}, expected level should be {correct}."));
        }
    }
}

// ---- AvoidModuleImport -------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct AvoidModuleImport {
    pub excludes: Vec<String>,
    pub max_allowed: usize,
}

/// `AvoidModuleImport`: `import module m;` is reported, unless `m` is excluded or the file has not yet
/// used up the `maxAllowedModuleImports` it may have. Every module import counts, excluded or not.
fn avoid_module_import(cx: &mut Ctx, program: Node, opts: &AvoidModuleImport) {
    let mut cur = program.walk();
    let imports: Vec<Node> = program.children(&mut cur).filter(|c| c.kind() == "import_declaration" && child_of_kind(*c, "module").is_some()).collect();
    for (i, import) in imports.iter().enumerate() {
        let mut inner = import.walk();
        let name = import.children(&mut inner).find(|c| matches!(c.kind(), "scoped_identifier" | "identifier")).map(|n| full_ident(cx, n)).unwrap_or_default();
        if i + 1 > opts.max_allowed && !opts.excludes.contains(&name) {
            let message = if opts.max_allowed > 0 {
                format!("Only '{}' module import is allowed per file.", opts.max_allowed)
            } else {
                format!("Using the 'import module' form of import should be avoided - {name}.")
            };
            cx.diag("AvoidModuleImport", *import, message);
        }
    }
}

// ---- ModuleDirectiveOrder ----------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModuleDirectiveOrder {
    pub order: Vec<String>,
    pub validate_block_separation: bool,
}

pub const MODULE_DIRECTIVES: &[&str] = &["requires", "exports", "opens", "uses", "provides"];

fn directive_kind(n: Node) -> Option<&'static str> {
    Some(match n.kind() {
        "requires_module_directive" => "requires",
        "exports_module_directive" => "exports",
        "opens_module_directive" => "opens",
        "uses_module_directive" => "uses",
        "provides_module_directive" => "provides",
        _ => return None,
    })
}

/// `ModuleDirectiveOrder`: the directives of a module declaration come in the configured order,
/// each kind in one block, and (with `validateBlockSeparation`) blocks are one empty line apart
/// while the directives of a block are not.
fn module_directive_order(cx: &mut Ctx, module: Node, opts: &ModuleDirectiveOrder) {
    let Some(body) = module.child_by_field_id(cx.syn.f.body) else { return };
    let mut cur = body.walk();
    let directives: Vec<Node> = body.children(&mut cur).filter(|c| directive_kind(*c).is_some()).collect();
    let mut seen: Vec<&str> = Vec::new();
    let mut previous: Option<(Node, &str)> = None;
    for directive in directives {
        let kind = directive_kind(directive).unwrap_or("");
        if !opts.order.iter().any(|o| o == kind) {
            continue;
        }
        if let Some((prev, prev_kind)) = previous {
            // The empty lines strictly between the previous directive's last line and this one's first.
            let from = prev.end_position().row + 1;
            let to = directive.start_position().row;
            let blank_lines = (from..to).filter(|&row| line_is_blank(cx.src, row)).count();
            let msg = if kind == prev_kind {
                (opts.validate_block_separation && blank_lines > 0).then(|| format!("Empty line not allowed inside '{kind}' directive block."))
            } else if seen.contains(&kind) {
                Some(format!("All '{kind}' directives should be in a single block."))
            } else if opts.order.iter().take_while(|o| *o != prev_kind).any(|o| o == kind) {
                Some(format!("'{kind}' directive should be before '{prev_kind}' directive."))
            } else {
                (opts.validate_block_separation && blank_lines != 1).then(|| format!("'{kind}' directive block should be separated from the previous block by exactly one empty line."))
            };
            if let Some(msg) = msg {
                cx.diag("ModuleDirectiveOrder", directive, msg);
            }
        }
        seen.push(kind);
        previous = Some((directive, kind));
    }
}

/// Is line `row` (0-based) empty or only blanks?
fn line_is_blank(src: &[u8], row: usize) -> bool {
    src.split(|&b| b == b'\n').nth(row).is_none_or(|l| String::from_utf8_lossy(l).chars().all(java_whitespace))
}

/// Java's `Character.isWhitespace`.
fn java_whitespace(c: char) -> bool {
    match c {
        '\u{00A0}' | '\u{2007}' | '\u{202F}' | '\u{0085}' => false,
        '\u{1C}'..='\u{1F}' => true,
        c => c.is_whitespace(),
    }
}

// ---- SingleSpaceSeparator and ArrayBracketNoWhitespace -----------------------------------------

/// The characters of the line before byte offset `at`.
fn line_before(src: &[u8], at: usize) -> Vec<char> {
    let start = src[..at].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    String::from_utf8_lossy(&src[start..at]).chars().collect()
}

#[derive(Clone, Debug)]
pub struct SingleSpaceSeparator {
    pub validate_comments: bool,
}

/// `SingleSpaceSeparator`, for the token starting at byte `at`: the blank before it, when there is
/// one, is a single space, or ends the indentation, or follows a block comment's `*/`.
fn check_single_space(cx: &mut Ctx, at: Node, byte: usize, validate_comments: bool) {
    let before = line_before(cx.src, byte);
    let n = before.len();
    // The column of the character before the token must be at least 2.
    if n < 3 {
        return;
    }
    let previous = before[n - 1];
    if !java_whitespace(previous) {
        return;
    }
    if previous == ' ' && !java_whitespace(before[n - 2]) {
        return;
    }
    let head = &before[..n - 1];
    if head.iter().all(|c| java_whitespace(*c)) {
        return;
    }
    if !validate_comments {
        let end = head.iter().rposition(|c| !java_whitespace(*c)).map_or(0, |i| i + 1);
        if head[..end].ends_with(&['*', '/']) {
            return;
        }
    }
    let row = cx.src[..byte].iter().filter(|&&b| b == b'\n').count() as u32 + 1;
    let col = (byte - cx.src[..byte].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1)) as u32 + 1;
    let _ = at;
    cx.diag_at("SingleSpaceSeparator", row, col, "Use a single space to separate non-whitespace characters.".into());
}

fn single_space_separator(cx: &mut Ctx, node: Node, validate_comments: bool) {
    match node.kind() {
        "line_comment" if validate_comments => check_single_space(cx, node, node.start_byte(), true),
        "block_comment" if validate_comments => {
            check_single_space(cx, node, node.start_byte(), true);
            // Checkstyle puts the end of a one-line block comment one character before its `*/`.
            let multi_line = cx.text(node).contains(&b'\n');
            check_single_space(cx, node, node.end_byte() - if multi_line { 2 } else { 3 }, true);
        }
        "line_comment" | "block_comment" => {}
        _ if node.child_count() > 0 => {
            // The closing delimiter of a text block is a token of its own; a string is one token.
            if node.kind() == "string_literal" && cx.text(node).starts_with(b"\"\"\"") {
                let end = node.end_byte() - 3;
                check_single_space(cx, node, end, validate_comments);
            }
        }
        _ => {
            if cx.parent().is_some_and(|p| matches!(p.kind(), "string_literal")) {
                // Pieces of a string: the opening quote starts the literal, the rest are not tokens.
                if node.prev_sibling().is_some() {
                    return;
                }
            }
            check_single_space(cx, node, node.start_byte(), validate_comments);
        }
    }
}

/// The character at `byte` if it is on the same line as the token (not a line end).
fn char_at(src: &[u8], byte: usize) -> Option<char> {
    let rest = src.get(byte..)?;
    let end = rest.len().min(4);
    let text = match std::str::from_utf8(&rest[..end]) {
        Ok(t) => t,
        Err(e) => std::str::from_utf8(&rest[..e.valid_up_to()]).ok()?,
    };
    text.chars().next().filter(|c| *c != '\n' && *c != '\r')
}

/// The character just before `byte`, if there is one on the same line.
fn char_before(src: &[u8], byte: usize) -> Option<char> {
    let head = &src[..byte];
    let start = head.len().saturating_sub(4);
    let tail = String::from_utf8_lossy(&head[start..]);
    tail.chars().next_back().filter(|c| *c != '\n' && *c != '\r')
}

/// Does `]` at `bracket` stand before a token that may touch it? `None` if no token follows on the
/// line. Comments that end on the line are not tokens.
fn needs_space_after_bracket(cx: &Ctx, bracket: Node) -> Option<bool> {
    let src = cx.src;
    let mut at = bracket.end_byte();
    loop {
        match src.get(at)? {
            b' ' | b'\t' | 0x0c => at += 1,
            b'\n' | b'\r' => return None,
            b'/' if src.get(at + 1) == Some(&b'/') => return None,
            b'/' if src.get(at + 1) == Some(&b'*') => {
                let close = src[at + 2..].windows(2).position(|w| w == b"*/")?;
                let comment = &src[at..at + 2 + close + 2];
                if comment.contains(&b'\n') || comment.contains(&b'\r') {
                    return None;
                }
                at += comment.len();
            }
            _ => break,
        }
    }
    let rest = &src[at..];
    let starts = |s: &[u8]| rest.starts_with(s);
    let valid = if starts(b"[") || starts(b".") || starts(b"]") || starts(b")") || starts(b"}") || starts(b",") || starts(b";") || starts(b"::") || starts(b"++") || starts(b"--") {
        true
    } else if starts(b">") {
        // A generic closer, not a comparison or a shift.
        let mut root = bracket;
        while let Some(p) = root.parent() {
            root = p;
        }
        root.descendant_for_byte_range(at, at + 1).and_then(|n| n.parent()).is_some_and(|p| matches!(p.kind(), "type_arguments" | "type_parameters"))
    } else {
        false
    };
    Some(!valid)
}

fn array_bracket_no_whitespace(cx: &mut Ctx, node: Node) {
    let bracket = node.kind();
    let start = node.start_byte();
    let ws_before = char_before(cx.src, start).is_some_and(java_whitespace);
    let ws_after = char_at(cx.src, node.end_byte()).is_some_and(java_whitespace);
    if bracket == "[" {
        let annotation_before = node.prev_sibling().is_some_and(|p| {
            let mut p = p;
            while p.is_extra() {
                match p.prev_sibling() {
                    Some(q) => p = q,
                    None => return false,
                }
            }
            is_annotation(p)
        });
        if !annotation_before && ws_before {
            cx.diag("ArrayBracketNoWhitespace", node, "'[' is preceded with whitespace.".into());
        } else if annotation_before && !ws_before {
            cx.diag("ArrayBracketNoWhitespace", node, "'[' is not preceded with whitespace.".into());
        }
        if ws_after {
            cx.diag("ArrayBracketNoWhitespace", node, "'[' is followed by whitespace.".into());
        }
    } else {
        if ws_before {
            cx.diag("ArrayBracketNoWhitespace", node, "']' is preceded with whitespace.".into());
        }
        if let Some(requires) = needs_space_after_bracket(cx, node) {
            if requires && !ws_after {
                cx.diag("ArrayBracketNoWhitespace", node, "']' is not followed by whitespace.".into());
            } else if !requires && ws_after {
                cx.diag("ArrayBracketNoWhitespace", node, "']' is followed by whitespace.".into());
            }
        }
    }
}

// ---- IllegalTokenText --------------------------------------------------------------------------

/// One `IllegalTokenText` instance.
#[derive(Clone, Debug)]
pub struct IllegalTokenText {
    pub name: &'static str,
    pub tokens: Vec<&'static str>,
    pub format: Format,
    /// The pattern as written, which the message names.
    pub pattern: String,
    /// The message, with its `{0}` already replaced by the pattern.
    pub message: String,
}

pub const ILLEGAL_TOKEN_TEXT_ACCEPTED: &[&str] =
    &["NUM_DOUBLE", "NUM_FLOAT", "NUM_INT", "NUM_LONG", "IDENT", "COMMENT_CONTENT", "STRING_LITERAL", "CHAR_LITERAL", "TEXT_BLOCK_CONTENT"];

/// Java's `MessageFormat` over one argument: `{0}` is the argument, `''` a quote, and text in quotes
/// is literal. Any other argument index is refused, since there is no argument to put there.
pub fn message_format(pattern: &str, argument: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut chars = pattern.chars().peekable();
    let mut quoted = false;
    while let Some(c) = chars.next() {
        match c {
            '\'' if chars.peek() == Some(&'\'') => {
                chars.next();
                out.push('\'');
            }
            '\'' => quoted = !quoted,
            '{' if !quoted => {
                let mut spec = String::new();
                for d in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    spec.push(d);
                }
                if spec.trim() != "0" {
                    return Err(format!("message: the argument {{{spec}}} is not supported"));
                }
                out.push_str(argument);
            }
            c => out.push(c),
        }
    }
    Ok(out)
}

fn illegal_token_text(cx: &mut Ctx, node: Node) {
    let kind = node.kind();
    // (token, text, position of the token)
    let (token, text, row, col): (&str, std::borrow::Cow<[u8]>, usize, usize) = match kind {
        "identifier" | "type_identifier" => ("IDENT", cx.text(node).into(), node.start_position().row, node.start_position().column),
        "string_literal" => {
            let text = cx.text(node);
            if text.starts_with(b"\"\"\"") {
                let p = node.start_position();
                ("TEXT_BLOCK_CONTENT", text[3..text.len().saturating_sub(3).max(3)].into(), p.row, p.column + 3)
            } else {
                ("STRING_LITERAL", text.into(), node.start_position().row, node.start_position().column)
            }
        }
        "character_literal" => ("CHAR_LITERAL", cx.text(node).into(), node.start_position().row, node.start_position().column),
        "decimal_integer_literal" | "hex_integer_literal" | "octal_integer_literal" | "binary_integer_literal" => {
            let t = cx.text(node);
            let token = if matches!(t.last(), Some(b'l' | b'L')) { "NUM_LONG" } else { "NUM_INT" };
            (token, t.into(), node.start_position().row, node.start_position().column)
        }
        "decimal_floating_point_literal" | "hex_floating_point_literal" => {
            let t = cx.text(node);
            let token = if matches!(t.last(), Some(b'f' | b'F')) { "NUM_FLOAT" } else { "NUM_DOUBLE" };
            (token, t.into(), node.start_position().row, node.start_position().column)
        }
        "line_comment" => {
            let t = cx.text(node);
            let mut content = t[2..].to_vec();
            if cx.src.get(node.end_byte()).is_some() {
                content.push(b'\n');
            }
            ("COMMENT_CONTENT", content.into(), node.start_position().row, node.start_position().column + 2)
        }
        "block_comment" => {
            let t = cx.text(node);
            let content = t[2..t.len().saturating_sub(2).max(2)].to_vec();
            ("COMMENT_CONTENT", content.into(), node.start_position().row, node.start_position().column + 2)
        }
        _ => return,
    };
    for i in 0..cx.cfg.gs.illegal_token_text.len() {
        let opts = &cx.cfg.gs.illegal_token_text[i];
        if opts.tokens.contains(&token) && opts.format.matches(&text) {
            let (name, message) = (opts.name, opts.message.clone());
            cx.diag_at(name, row as u32 + 1, col as u32 + 1, message);
        }
    }
}

/// An `IllegalTokenText` has no node for the identifiers, strings and numbers inside a package or
/// import declaration, which the walk does not enter: look at them here.
fn illegal_token_text_inside(cx: &mut Ctx, decl: Node) {
    let mut stack = vec![decl];
    while let Some(n) = stack.pop() {
        illegal_token_text(cx, n);
        let mut cur = n.walk();
        let kids: Vec<Node> = n.children(&mut cur).collect();
        stack.extend(kids.into_iter().rev());
    }
}
