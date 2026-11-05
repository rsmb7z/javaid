//! The options of the small structural modules in `rules::gs`, and what their properties accept.

use super::Format;

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

pub const WHITESPACE_BEFORE_EMPTY_BODY_TOKENS: &[&str] = &[
    "METHOD_DEF", "CTOR_DEF", "COMPACT_CTOR_DEF", "CLASS_DEF", "INTERFACE_DEF", "RECORD_DEF", "ENUM_DEF", "ANNOTATION_DEF", "LITERAL_NEW", "LITERAL_DO",
    "LITERAL_WHILE", "LITERAL_FOR", "LITERAL_IF", "LITERAL_ELSE", "STATIC_INIT", "LITERAL_TRY", "LITERAL_CATCH", "LITERAL_FINALLY", "LITERAL_SYNCHRONIZED",
    "LITERAL_SWITCH", "LAMBDA",
];

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

#[derive(Clone, Debug)]
pub struct AvoidModuleImport {
    pub excludes: Vec<String>,
    pub max_allowed: usize,
}

#[derive(Clone, Debug)]
pub struct ModuleDirectiveOrder {
    pub order: Vec<String>,
    pub validate_block_separation: bool,
}

pub const MODULE_DIRECTIVES: &[&str] = &["requires", "exports", "opens", "uses", "provides"];

#[derive(Clone, Debug)]
pub struct SingleSpaceSeparator {
    pub validate_comments: bool,
}

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
