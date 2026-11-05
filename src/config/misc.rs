//! Modifiers, annotations and small single-token checks.

use super::*;

#[derive(Clone, Debug)]
pub struct CommentsIndentation {
    /// `SINGLE_LINE_COMMENT` is among the tokens.
    pub single: bool,
    /// `BLOCK_COMMENT_BEGIN` is among the tokens.
    pub block: bool,
}

#[derive(Clone, Debug)]
pub struct MethodLength {
    pub max: usize,
    /// `countEmpty`: blank lines and `//` comment lines count towards the length.
    pub count_empty: bool,
    pub tokens: TokenSet,
}

const METHOD_LENGTH_TOKENS: &str = "METHOD_DEF, CTOR_DEF, COMPACT_CTOR_DEF";

#[derive(Clone, Debug)]
pub struct ParameterNumber {
    pub max: usize,
    pub ignore_overridden: bool,
    /// Annotation names, as written, that exempt a method.
    pub ignore_annotated_by: Vec<String>,
    pub tokens: TokenSet,
}

const PARAMETER_NUMBER_TOKENS: &str = "METHOD_DEF, CTOR_DEF";

#[derive(Clone, Debug)]
pub struct AvoidEscapedUnicode {
    pub allow_escapes_for_control_characters: bool,
    pub allow_by_tail_comment: bool,
    pub allow_if_all_characters_escaped: bool,
    pub allow_non_printable_escapes: bool,
}

#[derive(Clone, Debug)]
pub struct Translation {
    pub base_name: Format,
    pub required: Vec<String>,
    pub extensions: Extensions,
}

/// `ArrayTypeStyle`.
pub(super) fn array_type_style(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("javaStyle", "true")?; // C style (false) has quirks that were not modelled
    p.finish()?;
    cfg.array_type_style = true;
    Ok(())
}

/// `EmptyStatement`.
pub(super) fn empty_statement(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.empty_statement = true;
    Ok(())
}

/// `UpperEll`.
pub(super) fn upper_ell(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.upper_ell = true;
    Ok(())
}

/// `ModifierOrder`.
pub(super) fn modifier_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    // A custom order of modifiers is a different check; only the JLS order is modelled.
    if p.take("modifiersOrder").is_some() {
        return Err("property modifiersOrder is not supported yet".to_string());
    }
    p.finish()?;
    cfg.modifier_order = true;
    Ok(())
}

/// `CommentsIndentation`.
pub(super) fn comments_indentation(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let (mut single, mut block) = (true, true);
    if let Some(list) = p.take("tokens") {
        (single, block) = (false, false);
        for t in list.split(',').map(str::trim).filter(|t| !t.is_empty()) {
            match t {
                "SINGLE_LINE_COMMENT" => single = true,
                "BLOCK_COMMENT_BEGIN" => block = true,
                other => return Err(format!("tokens: {other} is not supported by this module")),
            }
        }
    }
    p.finish()?;
    cfg.comments_indentation = Some(CommentsIndentation { single, block });
    Ok(())
}

/// `MethodLength`.
pub(super) fn method_length(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let max = p.usize("max", 150)?;
    let count_empty = p.bool("countEmpty", true)?;
    let tokens = p.tokens(METHOD_LENGTH_TOKENS, METHOD_LENGTH_TOKENS)?;
    p.finish()?;
    cfg.method_length = Some(MethodLength { max, count_empty, tokens });
    Ok(())
}

/// `ParameterNumber`.
pub(super) fn parameter_number(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let max = p.usize("max", 7)?;
    let ignore_overridden = p.bool("ignoreOverriddenMethods", false)?;
    let ignore_annotated_by = p.take("ignoreAnnotatedBy").map(|v| v.split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect()).unwrap_or_default();
    let tokens = p.tokens(PARAMETER_NUMBER_TOKENS, PARAMETER_NUMBER_TOKENS)?;
    p.finish()?;
    cfg.parameter_number = Some(ParameterNumber { max, ignore_overridden, ignore_annotated_by, tokens });
    Ok(())
}

/// `MissingOverrideOnRecordAccessor`.
pub(super) fn missing_override_on_record_accessor(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.missing_override_on_record_accessor = true;
    Ok(())
}

/// `AnnotationLocation`.
pub(super) fn annotation_location(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    use super::structural::{token_names, ANNOTATION_LOCATION_ACCEPTED, ANNOTATION_LOCATION_DEFAULT};
    let same_line_single = p.bool("allowSamelineSingleParameterlessAnnotation", true)?;
    let same_line_parameterized = p.bool("allowSamelineParameterizedAnnotation", false)?;
    let same_line_multiple = p.bool("allowSamelineMultipleAnnotations", false)?;
    let tokens = token_names(p.take("tokens"), ANNOTATION_LOCATION_DEFAULT, ANNOTATION_LOCATION_ACCEPTED)?;
    p.finish()?;
    let name = id.map_or("AnnotationLocation", leak);
    cfg.gs.annotation_location.push(super::structural::AnnotationLocation { name, tokens, same_line_single, same_line_parameterized, same_line_multiple });
    Ok(())
}

/// `OuterTypeFilename`.
pub(super) fn outer_type_filename(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.outer_type_filename = true;
    Ok(())
}

/// `MultilineCommentLeadingAsteriskPresence`.
pub(super) fn multiline_comment_leading_asterisk_presence(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.multiline_comment_asterisk = true;
    Ok(())
}

/// `AvoidEscapedUnicodeCharacters`.
pub(super) fn avoid_escaped_unicode_characters(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_escapes_for_control_characters = p.bool("allowEscapesForControlCharacters", false)?;
    let allow_by_tail_comment = p.bool("allowByTailComment", false)?;
    let allow_if_all_characters_escaped = p.bool("allowIfAllCharactersEscaped", false)?;
    let allow_non_printable_escapes = p.bool("allowNonPrintableEscapes", false)?;
    p.finish()?;
    cfg.avoid_escaped_unicode =
        Some(AvoidEscapedUnicode { allow_escapes_for_control_characters, allow_by_tail_comment, allow_if_all_characters_escaped, allow_non_printable_escapes });
    Ok(())
}

/// `Translation`.
pub(super) fn translation(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let base_name = Format::new(&p.take("baseName").unwrap_or_else(|| "^messages.*$".to_string()))?;
    let required = p.take("requiredTranslations").map_or_else(Vec::new, |v| v.split(',').map(|c| c.trim().to_string()).filter(|c| !c.is_empty()).collect());
    // A bundle is made of `.properties` files unless told otherwise.
    let extensions = p.extensions().or_else(|| Some(vec!["properties".to_string()]));
    p.finish()?;
    cfg.translation = Some(Translation { base_name, required, extensions });
    Ok(())
}
