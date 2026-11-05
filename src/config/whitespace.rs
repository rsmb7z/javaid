//! Whitespace, padding, wrapping and braces.

use super::*;

/// `Indentation`: the offsets Checkstyle's handlers work with.
#[derive(Clone, Debug)]
pub struct Indentation {
    pub basic_offset: i32,
    pub brace_adjustment: i32,
    pub case_indent: i32,
    pub throws_indent: i32,
    pub array_init_indent: i32,
    pub line_wrapping_indentation: i32,
    pub force_strict_condition: bool,
}

/// `EmptyLineSeparator`, a module ported onto the emulated Checkstyle AST.
#[derive(Clone, Debug)]
pub struct EmptyLineSeparator {
    pub tokens: Vec<crate::csast::Tok>,
    pub allow_no_empty_line_between_fields: bool,
    pub allow_multiple_empty_lines: bool,
    pub allow_multiple_empty_lines_inside_class_members: bool,
}

/// The tokens `EmptyLineSeparator` accepts (and checks by default).
const EMPTY_LINE_SEPARATOR_TOKENS: &[(&str, CsTok)] = &[
    ("PACKAGE_DEF", CsTok::PACKAGE_DEF),
    ("IMPORT", CsTok::IMPORT),
    ("STATIC_IMPORT", CsTok::STATIC_IMPORT),
    ("MODULE_IMPORT", CsTok::MODULE_IMPORT),
    ("CLASS_DEF", CsTok::CLASS_DEF),
    ("INTERFACE_DEF", CsTok::INTERFACE_DEF),
    ("ENUM_DEF", CsTok::ENUM_DEF),
    ("ENUM_CONSTANT_DEF", CsTok::ENUM_CONSTANT_DEF),
    ("STATIC_INIT", CsTok::STATIC_INIT),
    ("INSTANCE_INIT", CsTok::INSTANCE_INIT),
    ("METHOD_DEF", CsTok::METHOD_DEF),
    ("CTOR_DEF", CsTok::CTOR_DEF),
    ("VARIABLE_DEF", CsTok::VARIABLE_DEF),
    ("RECORD_DEF", CsTok::RECORD_DEF),
    ("COMPACT_CTOR_DEF", CsTok::COMPACT_CTOR_DEF),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RightCurlyOption {
    Same,
    Alone,
    AloneOrSingleline,
}

#[derive(Clone, Debug)]
pub struct RightCurly {
    pub option: RightCurlyOption,
    pub tokens: TokenSet,
}

const RIGHT_CURLY_TOKENS: &str = "LITERAL_TRY, LITERAL_CATCH, LITERAL_FINALLY, LITERAL_IF, LITERAL_ELSE";

const RIGHT_CURLY_ACCEPTED: &str = "LITERAL_TRY, LITERAL_CATCH, LITERAL_FINALLY, LITERAL_IF, LITERAL_ELSE, CLASS_DEF, METHOD_DEF, CTOR_DEF, LITERAL_FOR, \
    LITERAL_WHILE, LITERAL_DO, STATIC_INIT, INSTANCE_INIT, ANNOTATION_DEF, ENUM_DEF, INTERFACE_DEF, RECORD_DEF, COMPACT_CTOR_DEF, LITERAL_SWITCH, \
    LITERAL_CASE, LITERAL_DEFAULT";

#[derive(Clone, Debug)]
pub struct ParenPad {
    /// The constructs whose parentheses are checked.
    pub tokens: TokenSet,
    /// `option=space`: padding is required, not forbidden.
    pub space: bool,
}

/// The tokens `ParenPad` checks when `tokens` is not set.
const PAREN_PAD_TOKENS: &str = "ANNOTATION, ANNOTATION_FIELD_DEF, CTOR_CALL, CTOR_DEF, DOT, ENUM_CONSTANT_DEF, EXPR, LITERAL_CATCH, \
    LITERAL_DO, LITERAL_FOR, LITERAL_IF, LITERAL_NEW, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_WHILE, METHOD_CALL, METHOD_DEF, \
    QUESTION, RESOURCE_SPECIFICATION, SUPER_CTOR_CALL, LAMBDA, RECORD_DEF, RECORD_PATTERN_DEF";

#[derive(Clone, Debug)]
pub struct MethodParamPad {
    pub tokens: TokenSet,
    /// `allowLineBreaks`: the `(` may start a line.
    pub allow_line_breaks: bool,
    pub space: bool,
}

const METHOD_PARAM_PAD_TOKENS: &str =
    "CTOR_DEF, CTOR_CALL, LITERAL_NEW, METHOD_CALL, METHOD_DEF, SUPER_CTOR_CALL, ENUM_CONSTANT_DEF, RECORD_DEF, RECORD_PATTERN_DEF";

#[derive(Clone, Debug)]
pub struct NoWhitespaceBefore {
    pub tokens: TokenSet,
    /// `allowLineBreaks`: a token that starts its line is fine.
    pub allow_line_breaks: bool,
}

const NO_WHITESPACE_BEFORE_TOKENS: &str = "COMMA, SEMI, POST_INC, POST_DEC, ELLIPSIS, LABELED_STAT";

/// What javaid models beyond the defaults.
const NO_WHITESPACE_BEFORE_ACCEPTED: &str = "COMMA, SEMI, POST_INC, POST_DEC, ELLIPSIS, LABELED_STAT, DOT, METHOD_REF";

#[derive(Clone, Debug)]
pub struct NoWhitespaceAfter {
    pub tokens: TokenSet,
    /// `allowLineBreaks`: a token that ends its line is fine.
    pub allow_line_breaks: bool,
}

const NO_WHITESPACE_AFTER_TOKENS: &str = "ARRAY_INIT, AT, INC, DEC, UNARY_MINUS, UNARY_PLUS, BNOT, LNOT, DOT, ARRAY_DECLARATOR, INDEX_OP";

const NO_WHITESPACE_AFTER_ACCEPTED: &str = "ARRAY_INIT, AT, INC, DEC, UNARY_MINUS, UNARY_PLUS, BNOT, LNOT, DOT, TYPECAST, ARRAY_DECLARATOR, INDEX_OP, \
    DO_WHILE, LITERAL_IF, LITERAL_SYNCHRONIZED, METHOD_REF, LITERAL_FOR, LITERAL_WHILE, LITERAL_CATCH";

#[derive(Clone, Debug)]
pub struct WhitespaceAfter {
    pub tokens: TokenSet,
}

const WHITESPACE_AFTER_TOKENS: &str = "COMMA, SEMI, TYPECAST, LITERAL_IF, LITERAL_ELSE, LITERAL_WHILE, LITERAL_DO, LITERAL_FOR, LITERAL_FINALLY, \
    LITERAL_RETURN, LITERAL_YIELD, LITERAL_CATCH, DO_WHILE, ELLIPSIS, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_TRY, LITERAL_CASE, LAMBDA, LITERAL_WHEN";

/// What the module accepts beyond its defaults.
const WHITESPACE_AFTER_ACCEPTED: &str = concat!(
    "COMMA, SEMI, TYPECAST, LITERAL_IF, LITERAL_ELSE, LITERAL_WHILE, LITERAL_DO, LITERAL_FOR, LITERAL_FINALLY, ",
    "LITERAL_RETURN, LITERAL_YIELD, LITERAL_CATCH, DO_WHILE, ELLIPSIS, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_TRY, ",
    "LITERAL_CASE, LAMBDA, LITERAL_WHEN, ANNOTATIONS, SINGLE_LINE_COMMENT"
);

#[derive(Clone, Debug)]
pub struct WhitespaceAround {
    pub tokens: TokenSet,
    pub allow_empty_constructors: bool,
    pub allow_empty_methods: bool,
    pub allow_empty_types: bool,
    pub allow_empty_loops: bool,
    pub allow_empty_lambdas: bool,
    pub allow_empty_catches: bool,
    pub allow_empty_switch_block_statements: bool,
    /// Leave the `:` of an enhanced `for` alone. On by default.
    pub ignore_enhanced_for_colon: bool,
}

const WHITESPACE_AROUND_TOKENS: &str = "ASSIGN, BAND, BAND_ASSIGN, BOR, BOR_ASSIGN, BSR, BSR_ASSIGN, BXOR, BXOR_ASSIGN, COLON, DIV, DIV_ASSIGN, \
    DO_WHILE, EQUAL, GE, GT, LAMBDA, LAND, LCURLY, LE, LITERAL_CATCH, LITERAL_DO, LITERAL_ELSE, LITERAL_FINALLY, LITERAL_FOR, LITERAL_IF, \
    LITERAL_RETURN, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_TRY, LITERAL_WHILE, LOR, LT, MINUS, MINUS_ASSIGN, MOD, MOD_ASSIGN, NOT_EQUAL, \
    PLUS, PLUS_ASSIGN, QUESTION, RCURLY, SL, SLIST, SL_ASSIGN, SR, SR_ASSIGN, STAR, STAR_ASSIGN, LITERAL_ASSERT, TYPE_EXTENSION_AND, LITERAL_WHEN";

/// What the module accepts beyond its defaults.
const WHITESPACE_AROUND_ACCEPTED: &str = "ASSIGN, ARRAY_INIT, BAND, BAND_ASSIGN, BOR, BOR_ASSIGN, BSR, BSR_ASSIGN, BXOR, BXOR_ASSIGN, COLON, DIV, DIV_ASSIGN, \
    DO_WHILE, EQUAL, GE, GT, LAMBDA, LAND, LCURLY, LE, LITERAL_CATCH, LITERAL_DO, LITERAL_ELSE, LITERAL_FINALLY, LITERAL_FOR, LITERAL_IF, \
    LITERAL_RETURN, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_TRY, LITERAL_WHILE, LOR, LT, MINUS, MINUS_ASSIGN, MOD, MOD_ASSIGN, NOT_EQUAL, \
    PLUS, PLUS_ASSIGN, QUESTION, RCURLY, SL, SLIST, SL_ASSIGN, SR, SR_ASSIGN, STAR, STAR_ASSIGN, LITERAL_ASSERT, TYPE_EXTENSION_AND, LITERAL_WHEN, \
    WILDCARD_TYPE, GENERIC_START, GENERIC_END, ELLIPSIS";

/// One `OperatorWrap` instance. A configuration may hold several, each under its own `id`.
#[derive(Clone, Debug)]
pub struct OperatorWrap {
    pub name: &'static str,
    pub tokens: TokenSet,
    /// `option=eol`: the operator ends its line. Otherwise (`nl`, the default) it starts the next.
    pub end_of_line: bool,
}

const OPERATOR_WRAP_TOKENS: &str = "QUESTION, COLON, EQUAL, NOT_EQUAL, DIV, PLUS, MINUS, STAR, MOD, SR, BSR, GE, GT, SL, LE, LT, BXOR, BOR, LOR, BAND, \
    LAND, TYPE_EXTENSION_AND, LITERAL_INSTANCEOF";

/// What the module accepts beyond its defaults.
const OPERATOR_WRAP_ACCEPTED: &str = "QUESTION, COLON, EQUAL, NOT_EQUAL, DIV, PLUS, MINUS, STAR, MOD, SR, BSR, GE, GT, SL, LE, LT, BXOR, BOR, LOR, BAND, \
    LAND, TYPE_EXTENSION_AND, LITERAL_INSTANCEOF, ASSIGN, DIV_ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN, STAR_ASSIGN, MOD_ASSIGN, SR_ASSIGN, BSR_ASSIGN, \
    SL_ASSIGN, BXOR_ASSIGN, BOR_ASSIGN, BAND_ASSIGN, METHOD_REF, LAMBDA";

/// One `SeparatorWrap` instance. A configuration may hold several, each under its own `id`.
#[derive(Clone, Debug)]
pub struct SeparatorWrap {
    pub name: &'static str,
    pub tokens: TokenSet,
    /// `option=eol` (the default): the separator ends its line. Otherwise (`nl`) it starts the next.
    pub end_of_line: bool,
}

const SEPARATOR_WRAP_TOKENS: &str = "DOT, COMMA";

/// What the module accepts beyond its defaults.
const SEPARATOR_WRAP_ACCEPTED: &str = "DOT, COMMA, SEMI, ELLIPSIS, ARRAY_DECLARATOR, RBRACK, METHOD_REF, LPAREN, RPAREN";

/// One `LeftCurly` instance. A configuration may hold several, each under its own `id`.
#[derive(Clone, Debug)]
pub struct LeftCurly {
    pub name: &'static str,
    pub tokens: TokenSet,
    pub option: CurlyOption,
    /// Leave the braces of an enum without annotations alone when a line break follows them. On by default.
    pub ignore_enums: bool,
}

const LEFT_CURLY_TOKENS: &str = "ANNOTATION_DEF, CLASS_DEF, CTOR_DEF, ENUM_CONSTANT_DEF, ENUM_DEF, INTERFACE_DEF, LAMBDA, LITERAL_CASE, LITERAL_CATCH, \
    LITERAL_DEFAULT, LITERAL_DO, LITERAL_ELSE, LITERAL_FINALLY, LITERAL_FOR, LITERAL_IF, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_TRY, \
    LITERAL_WHILE, METHOD_DEF, OBJBLOCK, STATIC_INIT, RECORD_DEF, COMPACT_CTOR_DEF, SWITCH_RULE";

/// `Indentation`.
pub(super) fn indentation(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let basic_offset = p.i32("basicOffset", 4)?;
    let brace_adjustment = p.i32("braceAdjustment", 0)?;
    let case_indent = p.i32("caseIndent", 4)?;
    let throws_indent = p.i32("throwsIndent", 4)?;
    let array_init_indent = p.i32("arrayInitIndent", 4)?;
    let line_wrapping_indentation = p.i32("lineWrappingIndentation", 4)?;
    let force_strict_condition = p.bool("forceStrictCondition", false)?;
    p.finish()?;
    cfg.indentation =
        Some(Indentation { basic_offset, brace_adjustment, case_indent, throws_indent, array_init_indent, line_wrapping_indentation, force_strict_condition });
    Ok(())
}

/// `RightCurly`.
pub(super) fn right_curly(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let option = match p.take("option").map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("same") => RightCurlyOption::Same,
        Some("alone") => RightCurlyOption::Alone,
        Some("alone_or_singleline") => RightCurlyOption::AloneOrSingleline,
        Some(other) => return Err(format!("property option: expected same, alone or alone_or_singleline, got '{other}'")),
    };
    let tokens = p.tokens(RIGHT_CURLY_TOKENS, RIGHT_CURLY_ACCEPTED)?;
    p.finish()?;
    cfg.right_curly = Some(RightCurly { option, tokens });
    Ok(())
}

/// `NoWhitespaceBeforeCaseDefaultColon`.
pub(super) fn no_whitespace_before_case_default_colon(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.no_whitespace_before_case_default_colon = true;
    Ok(())
}

/// `WhitespaceBeforeEmptyBody`.
pub(super) fn whitespace_before_empty_body(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let all = super::structural::WHITESPACE_BEFORE_EMPTY_BODY_TOKENS;
    let tokens = super::structural::token_names(p.take("tokens"), &all.join(", "), all)?;
    p.finish()?;
    cfg.gs.whitespace_before_empty_body = Some(tokens);
    Ok(())
}

/// `SingleSpaceSeparator`.
pub(super) fn single_space_separator(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let validate_comments = p.bool("validateComments", false)?;
    p.finish()?;
    cfg.gs.single_space_separator = Some(super::structural::SingleSpaceSeparator { validate_comments });
    Ok(())
}

/// `ArrayBracketNoWhitespace`.
pub(super) fn array_bracket_no_whitespace(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.array_bracket_no_whitespace = true;
    Ok(())
}

/// `NoLineWrap`.
pub(super) fn no_line_wrap(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let skip_annotations = p.bool("skipAnnotations", true)?;
    let tokens = super::structural::token_names(p.take("tokens"), super::structural::NO_LINE_WRAP_DEFAULT, super::structural::NO_LINE_WRAP_ACCEPTED)?;
    p.finish()?;
    cfg.gs.no_line_wrap = Some(super::structural::NoLineWrap { skip_annotations, tokens });
    Ok(())
}

/// `ParenPad`.
pub(super) fn paren_pad(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let tokens = p.tokens(PAREN_PAD_TOKENS, PAREN_PAD_TOKENS)?;
    let space = p.pad_option()?;
    p.finish()?;
    cfg.paren_pad = Some(ParenPad { tokens, space });
    Ok(())
}

/// `TypecastParenPad`.
pub(super) fn typecast_paren_pad(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    // The tokens are fixed (TYPECAST, RPAREN); only the option varies.
    let space = p.pad_option()?;
    p.finish()?;
    cfg.typecast_paren_pad = Some(space);
    Ok(())
}

/// `EmptyForIteratorPad`.
pub(super) fn empty_for_iterator_pad(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let space = p.pad_option()?;
    p.finish()?;
    cfg.empty_for_iterator_pad = Some(space);
    Ok(())
}

/// `MethodParamPad`.
pub(super) fn method_param_pad(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_line_breaks = p.bool("allowLineBreaks", false)?;
    let tokens = p.tokens(METHOD_PARAM_PAD_TOKENS, METHOD_PARAM_PAD_TOKENS)?;
    let space = p.pad_option()?;
    p.finish()?;
    cfg.method_param_pad = Some(MethodParamPad { tokens, allow_line_breaks, space });
    Ok(())
}

/// `NoWhitespaceBefore`.
pub(super) fn no_whitespace_before(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_line_breaks = p.bool("allowLineBreaks", false)?;
    let tokens = p.tokens(NO_WHITESPACE_BEFORE_TOKENS, NO_WHITESPACE_BEFORE_ACCEPTED)?;
    p.finish()?;
    cfg.no_whitespace_before = Some(NoWhitespaceBefore { tokens, allow_line_breaks });
    Ok(())
}

/// `NoWhitespaceAfter`.
pub(super) fn no_whitespace_after(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_line_breaks = p.bool("allowLineBreaks", true)?;
    let tokens = p.tokens(NO_WHITESPACE_AFTER_TOKENS, NO_WHITESPACE_AFTER_ACCEPTED)?;
    p.finish()?;
    cfg.no_whitespace_after = Some(NoWhitespaceAfter { tokens, allow_line_breaks });
    Ok(())
}

/// `WhitespaceAfter`.
pub(super) fn whitespace_after(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let tokens = p.tokens(WHITESPACE_AFTER_TOKENS, WHITESPACE_AFTER_ACCEPTED)?;
    p.finish()?;
    cfg.whitespace_after = Some(WhitespaceAfter { tokens });
    Ok(())
}

/// `OperatorWrap`.
pub(super) fn operator_wrap(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let end_of_line = p.wrap_option(false)?;
    let tokens = p.tokens(OPERATOR_WRAP_TOKENS, OPERATOR_WRAP_ACCEPTED)?;
    p.finish()?;
    let name = id.map_or("OperatorWrap", leak);
    cfg.operator_wrap.push(OperatorWrap { name, tokens, end_of_line });
    Ok(())
}

/// `SeparatorWrap`.
pub(super) fn separator_wrap(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let end_of_line = p.wrap_option(true)?;
    let tokens = p.tokens(SEPARATOR_WRAP_TOKENS, SEPARATOR_WRAP_ACCEPTED)?;
    p.finish()?;
    let name = id.map_or("SeparatorWrap", leak);
    cfg.separator_wrap.push(SeparatorWrap { name, tokens, end_of_line });
    Ok(())
}

/// `LeftCurly`.
pub(super) fn left_curly(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let option = p.curly_option()?;
    let ignore_enums = p.bool("ignoreEnums", true)?;
    let tokens = p.tokens(LEFT_CURLY_TOKENS, LEFT_CURLY_TOKENS)?;
    p.finish()?;
    let name = id.map_or("LeftCurly", leak);
    cfg.left_curly.push(LeftCurly { name, tokens, option, ignore_enums });
    Ok(())
}

/// `GenericWhitespace`.
pub(super) fn generic_whitespace(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.generic_whitespace = true;
    Ok(())
}

/// `WhitespaceAround`.
pub(super) fn whitespace_around(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_empty_constructors = p.bool("allowEmptyConstructors", false)?;
    let allow_empty_methods = p.bool("allowEmptyMethods", false)?;
    let allow_empty_types = p.bool("allowEmptyTypes", false)?;
    let allow_empty_loops = p.bool("allowEmptyLoops", false)?;
    let allow_empty_lambdas = p.bool("allowEmptyLambdas", false)?;
    let allow_empty_catches = p.bool("allowEmptyCatches", false)?;
    let allow_empty_switch_block_statements = p.bool("allowEmptySwitchBlockStatements", false)?;
    let ignore_enhanced_for_colon = p.bool("ignoreEnhancedForColon", true)?;
    let tokens = p.tokens(WHITESPACE_AROUND_TOKENS, WHITESPACE_AROUND_ACCEPTED)?;
    p.finish()?;
    cfg.whitespace_around = Some(WhitespaceAround {
        tokens,
        allow_empty_constructors,
        allow_empty_methods,
        allow_empty_types,
        allow_empty_loops,
        allow_empty_lambdas,
        allow_empty_catches,
        allow_empty_switch_block_statements,
        ignore_enhanced_for_colon,
    });
    Ok(())
}

/// `EmptyLineSeparator`.
pub(super) fn empty_line_separator(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_no_empty_line_between_fields = p.bool("allowNoEmptyLineBetweenFields", false)?;
    let allow_multiple_empty_lines = p.bool("allowMultipleEmptyLines", true)?;
    let allow_multiple_empty_lines_inside_class_members = p.bool("allowMultipleEmptyLinesInsideClassMembers", true)?;
    let tokens = match p.take("tokens") {
        None => EMPTY_LINE_SEPARATOR_TOKENS.iter().map(|(_, t)| *t).collect(),
        Some(list) => {
            let mut out = Vec::new();
            for name in list.split(',').map(str::trim).filter(|n| !n.is_empty()) {
                match EMPTY_LINE_SEPARATOR_TOKENS.iter().find(|(n, _)| *n == name) {
                    Some((_, t)) => out.push(*t),
                    None => return Err(format!("tokens: {name} is not an acceptable token of EmptyLineSeparator")),
                }
            }
            out
        }
    };
    p.finish()?;
    cfg.empty_line_separator =
        Some(EmptyLineSeparator { tokens, allow_no_empty_line_between_fields, allow_multiple_empty_lines, allow_multiple_empty_lines_inside_class_members });
    Ok(())
}
