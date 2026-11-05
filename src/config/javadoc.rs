//! The Javadoc family.

use super::*;

#[derive(Clone, Debug)]
pub struct AtclauseOrder {
    /// The order, as written (`@param`, `@return`, ...).
    pub tag_order: Vec<String>,
    /// Checkstyle token names of the declarations whose Javadoc is checked.
    pub target: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SingleLineJavadoc {
    pub ignored_tags: Vec<String>,
    pub ignore_inline_tags: bool,
}

#[derive(Clone, Debug)]
pub struct SummaryJavadoc {
    pub forbidden_fragments: Format,
    pub period: String,
}

#[derive(Clone, Debug)]
pub struct MissingJavadocType {
    pub scope: u8,
    pub exclude_scope: Option<u8>,
    pub skip_annotations: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct JavadocPackage {
    pub allow_legacy: bool,
}

#[derive(Clone, Debug)]
pub struct JavadocMethod {
    pub access: [bool; 4],
    pub allow_missing_param_tags: bool,
    pub allow_missing_return_tag: bool,
    pub allowed_annotations: Vec<String>,
    pub validate_throws: bool,
    pub allow_inline_return: bool,
}

#[derive(Clone, Debug)]
pub struct JavadocType {
    pub scope: u8,
    pub exclude_scope: Option<u8>,
    pub author_format: Option<Format>,
    pub version_format: Option<Format>,
    pub allow_missing_param_tags: bool,
    pub allow_unknown_tags: bool,
    pub allowed_annotations: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct MissingJavadocMethod {
    /// Checkstyle's `Scope` rank: 0 nothing, 1 public, 2 protected, 3 package, 4 private, 5 anonymous.
    pub scope: u8,
    pub exclude_scope: Option<u8>,
    pub min_line_count: i64,
    pub allowed_annotations: Vec<String>,
    pub allow_missing_property_javadoc: bool,
    pub ignore_method_names: Option<Format>,
}

#[derive(Clone, Debug)]
pub struct JavadocVariable {
    /// Public, protected, package, private: which access levels are checked.
    pub access: [bool; 4],
    pub ignore_name: Option<Format>,
}

/// `NonEmptyAtclauseDescription`.
pub(super) fn non_empty_atclause_description(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.non_empty_atclause_description = true;
    Ok(())
}

/// `JavadocMissingLeadingAsterisk`.
pub(super) fn javadoc_missing_leading_asterisk(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.javadoc_missing_leading_asterisk = true;
    Ok(())
}

/// `JavadocContentLocation`.
pub(super) fn javadoc_content_location(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let first = match p.take("location").map(|v| v.trim().to_ascii_lowercase()) {
        None => false,
        Some(v) if v == "second_line" => false,
        Some(v) if v == "first_line" => true,
        Some(v) => return Err(format!("property location: expected first_line or second_line, got '{v}'")),
    };
    p.finish()?;
    cfg.javadoc_content_location = Some(first);
    Ok(())
}

/// `JavadocLeadingAsteriskAlign`.
pub(super) fn javadoc_leading_asterisk_align(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let indentation = p.usize("indentation", 1)?;
    p.finish()?;
    cfg.javadoc_asterisk_align = Some(indentation);
    Ok(())
}

/// `RequireEmptyLineBeforeBlockTagGroup`.
pub(super) fn require_empty_line_before_block_tag_group(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.require_empty_line_before_tag_group = true;
    Ok(())
}

/// `SummaryJavadoc`.
pub(super) fn summary_javadoc(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let forbidden_fragments = Format::new(&p.take("forbiddenSummaryFragments").unwrap_or_else(|| "^$".to_string()))?;
    let period = p.take("period").unwrap_or_else(|| ".".to_string());
    p.finish()?;
    cfg.summary_javadoc = Some(SummaryJavadoc { forbidden_fragments, period });
    Ok(())
}

/// `JavadocParagraph`.
pub(super) fn javadoc_paragraph(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_newline_paragraph = p.bool("allowNewlineParagraph", true)?;
    p.finish()?;
    cfg.javadoc_paragraph = Some(allow_newline_paragraph);
    Ok(())
}

/// `JavadocTagContinuationIndentation`.
pub(super) fn javadoc_tag_continuation_indentation(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let offset = p.usize("offset", 4)?;
    p.finish()?;
    cfg.javadoc_tag_continuation_indent = Some(offset);
    Ok(())
}

/// `AtclauseOrder`.
pub(super) fn atclause_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let list = |v: String| v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect::<Vec<_>>();
    let tag_order = p.take("tagOrder").map_or_else(
        || {
            ["@author", "@version", "@param", "@return", "@throws", "@exception", "@see", "@since", "@serial", "@serialField", "@serialData", "@deprecated"]
                .map(String::from)
                .to_vec()
        },
        list,
    );
    let target = p.take("target").map_or_else(
        || ["CLASS_DEF", "INTERFACE_DEF", "ENUM_DEF", "METHOD_DEF", "CTOR_DEF", "VARIABLE_DEF", "RECORD_DEF", "COMPACT_CTOR_DEF"].map(String::from).to_vec(),
        list,
    );
    p.finish()?;
    cfg.atclause_order = Some(AtclauseOrder { tag_order, target });
    Ok(())
}

/// `SingleLineJavadoc`.
pub(super) fn single_line_javadoc(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let ignored_tags = p.take("ignoredTags").map_or_else(Vec::new, |v| v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect());
    let ignore_inline_tags = p.bool("ignoreInlineTags", true)?;
    p.finish()?;
    cfg.single_line_javadoc = Some(SingleLineJavadoc { ignored_tags, ignore_inline_tags });
    Ok(())
}

/// `MissingJavadocType`.
pub(super) fn missing_javadoc_type(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "INTERFACE_DEF, CLASS_DEF, ENUM_DEF, ANNOTATION_DEF, RECORD_DEF")?;
    let scope = p.scope("scope")?.unwrap_or(1);
    let exclude_scope = p.scope("excludeScope")?;
    let skip_annotations = match p.take("skipAnnotations") {
        None => vec!["Generated".to_string()],
        Some(v) => v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
    };
    p.finish()?;
    cfg.missing_javadoc_type = Some(MissingJavadocType { scope, exclude_scope, skip_annotations });
    Ok(())
}

/// `JavadocPackage`.
pub(super) fn javadoc_package(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_legacy = p.bool("allowLegacy", false)?;
    p.finish()?;
    cfg.javadoc_package = Some(JavadocPackage { allow_legacy });
    Ok(())
}

/// `JavadocMethod`.
pub(super) fn javadoc_method(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "METHOD_DEF, CTOR_DEF, ANNOTATION_FIELD_DEF, COMPACT_CTOR_DEF")?;
    p.expect_default("violateExecutionOnNonTightHtml", "false")?;
    let access = p.access_modifiers()?;
    let allow_missing_param_tags = p.bool("allowMissingParamTags", false)?;
    let allow_missing_return_tag = p.bool("allowMissingReturnTag", false)?;
    let allowed_annotations = match p.take("allowedAnnotations") {
        None => vec!["Override".to_string()],
        Some(v) => v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
    };
    let validate_throws = p.bool("validateThrows", false)?;
    let allow_inline_return = p.bool("allowInlineReturn", false)?;
    p.finish()?;
    cfg.javadoc_method =
        Some(JavadocMethod { access, allow_missing_param_tags, allow_missing_return_tag, allowed_annotations, validate_throws, allow_inline_return });
    Ok(())
}

/// `JavadocType`.
pub(super) fn javadoc_type(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "INTERFACE_DEF, CLASS_DEF, ENUM_DEF, ANNOTATION_DEF, RECORD_DEF")?;
    p.expect_default("violateExecutionOnNonTightHtml", "false")?;
    let scope = p.scope("scope")?.unwrap_or(4);
    let exclude_scope = p.scope("excludeScope")?;
    let author_format = p.take("authorFormat").map(|v| Format::new(&v)).transpose()?;
    let version_format = p.take("versionFormat").map(|v| Format::new(&v)).transpose()?;
    let allow_missing_param_tags = p.bool("allowMissingParamTags", false)?;
    let allow_unknown_tags = p.bool("allowUnknownTags", false)?;
    let allowed_annotations = match p.take("allowedAnnotations") {
        None => vec!["Generated".to_string()],
        Some(v) => v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
    };
    p.finish()?;
    cfg.javadoc_type =
        Some(JavadocType { scope, exclude_scope, author_format, version_format, allow_missing_param_tags, allow_unknown_tags, allowed_annotations });
    Ok(())
}

/// `InvalidJavadocPosition`.
pub(super) fn invalid_javadoc_position(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.invalid_javadoc_position = Some(());
    Ok(())
}

/// `MissingJavadocMethod`.
pub(super) fn missing_javadoc_method(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "METHOD_DEF, CTOR_DEF, ANNOTATION_FIELD_DEF, COMPACT_CTOR_DEF")?;
    let scope = p.scope("scope")?.unwrap_or(1);
    let exclude_scope = p.scope("excludeScope")?;
    let min_line_count =
        p.take("minLineCount").map_or(Ok(-1), |v| v.trim().parse::<i64>().map_err(|_| format!("property minLineCount: '{}' is not an integer", v.trim())))?;
    let allowed_annotations = match p.take("allowedAnnotations") {
        None => vec!["Override".to_string()],
        Some(v) => v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
    };
    let allow_missing_property_javadoc = p.bool("allowMissingPropertyJavadoc", false)?;
    let ignore_method_names = p.take("ignoreMethodNamesRegex").map(|v| Format::new(&format!("^(?:{v})$"))).transpose()?;
    p.finish()?;
    cfg.missing_javadoc_method =
        Some(MissingJavadocMethod { scope, exclude_scope, min_line_count, allowed_annotations, allow_missing_property_javadoc, ignore_method_names });
    Ok(())
}

/// `JavadocVariable`.
pub(super) fn javadoc_variable(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "VARIABLE_DEF, ENUM_CONSTANT_DEF")?;
    let access = p.access_modifiers()?;
    // `matches()`, not `find()`: the whole name has to match.
    let ignore_name = p.take("ignoreNamePattern").map(|v| Format::new(&format!("^(?:{v})$"))).transpose()?;
    p.finish()?;
    cfg.javadoc_variable = Some(JavadocVariable { access, ignore_name });
    Ok(())
}
