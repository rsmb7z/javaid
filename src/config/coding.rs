//! Coding, blocks and class design.

use super::*;

/// `None` means Checkstyle's default for that property (`^$` and `.*`), which needs no work at all.
#[derive(Clone, Debug, Default)]
pub struct EmptyCatchBlock {
    /// A block is allowed if the catch variable's name matches this.
    pub exception_variable_name: Option<Format>,
    /// A block is allowed if its *first* comment's text matches this.
    pub comment_format: Option<Format>,
}

#[derive(Clone, Debug, Default)]
pub struct OneStatementPerLine {
    /// Two resources of one `try (...)` on a line count as two statements.
    pub treat_try_resources_as_statement: bool,
}

#[derive(Clone, Debug)]
pub struct VariableDeclarationUsageDistance {
    pub allowed_distance: usize,
    /// `ignoreVariablePattern`, wrapped to match the whole name.
    pub ignore: Format,
    pub validate_between_scopes: bool,
    pub ignore_final: bool,
}

#[derive(Clone, Debug)]
pub struct DesignForExtension {
    pub ignored_annotations: Vec<String>,
    pub required_javadoc_phrase: Format,
}

#[derive(Clone, Debug)]
pub struct DeclarationOrder {
    pub ignore_modifiers: bool,
    pub ignore_constructors: bool,
}

#[derive(Clone, Debug)]
pub struct RedundantModifier {
    pub jdk_version: u32,
}

#[derive(Clone, Debug)]
pub struct HiddenField {
    pub ignore_format: Option<Format>,
    pub ignore_constructor_parameter: bool,
    pub ignore_setter: bool,
    pub setter_can_return_its_class: bool,
    pub ignore_abstract_methods: bool,
}

#[derive(Clone, Debug)]
pub struct VisibilityModifier {
    pub package_allowed: bool,
    pub protected_allowed: bool,
    pub allow_public_final_fields: bool,
    pub public_member_pattern: Format,
    /// Annotation names, as written: the canonical names and the short name of each.
    pub ignore_annotations: Vec<String>,
}

const VISIBILITY_IGNORED_ANNOTATIONS: &str = "org.junit.Rule, org.junit.ClassRule, com.google.common.annotations.VisibleForTesting";

#[derive(Clone, Debug)]
pub struct FinalParameters {
    pub tokens: TokenSet,
    pub ignore_primitive_types: bool,
}

const FINAL_PARAMETERS_TOKENS: &str = "METHOD_DEF, CTOR_DEF";

const FINAL_PARAMETERS_ACCEPTED: &str = "METHOD_DEF, CTOR_DEF, LITERAL_CATCH, FOR_EACH_CLAUSE";

#[derive(Clone, Debug)]
pub struct MagicNumber {
    pub ignore_numbers: Vec<f64>,
    pub ignore_hash_code_method: bool,
    pub ignore_annotation: bool,
    pub ignore_field_declaration: bool,
    pub ignore_annotation_element_defaults: bool,
}

#[derive(Clone, Debug)]
pub struct EmptyBlock {
    pub tokens: TokenSet,
    /// `option=text`: a block holding only a comment is not empty.
    pub text: bool,
}

const EMPTY_BLOCK_TOKENS: &str = "LITERAL_WHILE, LITERAL_TRY, LITERAL_FINALLY, LITERAL_DO, LITERAL_IF, LITERAL_ELSE, LITERAL_FOR, INSTANCE_INIT, \
    STATIC_INIT, LITERAL_SWITCH, LITERAL_SYNCHRONIZED";

const EMPTY_BLOCK_ACCEPTED: &str = "LITERAL_WHILE, LITERAL_TRY, LITERAL_CATCH, LITERAL_FINALLY, LITERAL_DO, LITERAL_IF, LITERAL_ELSE, LITERAL_FOR, \
    INSTANCE_INIT, STATIC_INIT, LITERAL_SWITCH, LITERAL_SYNCHRONIZED, LITERAL_CASE, LITERAL_DEFAULT, ARRAY_INIT";

#[derive(Clone, Debug)]
pub struct AvoidNestedBlocks {
    /// `allowInSwitchCase`: a block that is a `case`'s only statement is fine.
    pub allow_in_switch_case: bool,
}

#[derive(Clone, Debug)]
pub struct FallThrough {
    /// Hold the last case group to the same rule; it falls out of the switch, not into a case.
    pub check_last_case_group: bool,
    /// A comment matching this on the line before the next case excuses a group. Only the
    /// default, `falls?[ -]?thr(u|ough)`, is supported.
    pub relief: Format,
}

/// `DescendantToken` as the bundled configurations use it: commas in a `for` header.
#[derive(Clone, Debug)]
pub struct DescendantToken {
    pub name: &'static str,
    pub for_init: bool,
    pub for_iterator: bool,
    pub min_depth: u32,
    pub max_depth: u32,
    pub min_number: u32,
    pub max_number: u32,
    pub minimum_message: Option<String>,
    pub maximum_message: Option<String>,
}

/// `SimplifyBooleanReturn`.
pub(super) fn simplify_boolean_return(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.simplify_boolean_return = true;
    Ok(())
}

/// `SimplifyBooleanExpression`.
pub(super) fn simplify_boolean_expression(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.simplify_boolean_expression = true;
    Ok(())
}

/// `InnerAssignment`.
pub(super) fn inner_assignment(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.inner_assignment = true;
    Ok(())
}

/// `MissingSwitchDefault`.
pub(super) fn missing_switch_default(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.missing_switch_default = true;
    Ok(())
}

/// `MultipleVariableDeclarations`.
pub(super) fn multiple_variable_declarations(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.multiple_variable_declarations = true;
    Ok(())
}

/// `FallThrough`.
pub(super) fn fall_through(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let check_last_case_group = p.bool("checkLastCaseGroup", false)?;
    const DEFAULT_RELIEF: &str = "falls?[ -]?thr(u|ough)";
    if p.take("reliefPattern").is_some_and(|v| v != DEFAULT_RELIEF) {
        return Err("property reliefPattern is not supported yet".to_string());
    }
    p.finish()?;
    cfg.fall_through = Some(FallThrough { check_last_case_group, relief: Format::new(DEFAULT_RELIEF)? });
    Ok(())
}

/// `VariableDeclarationUsageDistance`.
pub(super) fn variable_declaration_usage_distance(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allowed_distance = p.usize("allowedDistance", 3)?;
    let ignore = Format::new(&format!("^(?:{})$", p.take("ignoreVariablePattern").unwrap_or_default()))?;
    let validate_between_scopes = p.bool("validateBetweenScopes", false)?;
    let ignore_final = p.bool("ignoreFinal", true)?;
    p.finish()?;
    cfg.variable_declaration_usage_distance = Some(VariableDeclarationUsageDistance { allowed_distance, ignore, validate_between_scopes, ignore_final });
    Ok(())
}

/// `FinalParameters`.
pub(super) fn final_parameters(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let ignore_primitive_types = p.bool("ignorePrimitiveTypes", false)?;
    let tokens = p.tokens(FINAL_PARAMETERS_TOKENS, FINAL_PARAMETERS_ACCEPTED)?;
    p.finish()?;
    cfg.final_parameters = Some(FinalParameters { tokens, ignore_primitive_types });
    Ok(())
}

/// `MagicNumber`.
pub(super) fn magic_number(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    for unsupported in ["constantWaiverParentToken", "tokens"] {
        if p.take(unsupported).is_some() {
            return Err(format!("property {unsupported} is not supported yet"));
        }
    }
    let ignore_numbers = match p.take("ignoreNumbers") {
        None => vec![-1.0, 0.0, 1.0, 2.0],
        Some(v) => v
            .split(',')
            .map(str::trim)
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<f64>().map_err(|_| format!("property ignoreNumbers: '{e}' is not a number")))
            .collect::<Result<Vec<_>, _>>()?,
    };
    let ignore_hash_code_method = p.bool("ignoreHashCodeMethod", false)?;
    let ignore_annotation = p.bool("ignoreAnnotation", false)?;
    let ignore_field_declaration = p.bool("ignoreFieldDeclaration", false)?;
    let ignore_annotation_element_defaults = p.bool("ignoreAnnotationElementDefaults", true)?;
    p.finish()?;
    cfg.magic_number =
        Some(MagicNumber { ignore_numbers, ignore_hash_code_method, ignore_annotation, ignore_field_declaration, ignore_annotation_element_defaults });
    Ok(())
}

/// `FinalClass`.
pub(super) fn final_class(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.final_class = true;
    Ok(())
}

/// `HideUtilityClassConstructor`.
pub(super) fn hide_utility_class_constructor(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.hide_utility_class_constructor = true;
    Ok(())
}

/// `DesignForExtension`.
pub(super) fn design_for_extension(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let ignored_annotations = match p.take("ignoredAnnotations") {
        None => ["Test", "Before", "After", "BeforeClass", "AfterClass"].map(String::from).to_vec(),
        Some(v) => v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect(),
    };
    let phrase = p.take("requiredJavadocPhrase").unwrap_or_else(|| ".*".to_string());
    let required_javadoc_phrase = Format::new(&phrase)?;
    p.finish()?;
    cfg.design_for_extension = Some(DesignForExtension { ignored_annotations, required_javadoc_phrase });
    Ok(())
}

/// `OverloadMethodsDeclarationOrder`.
pub(super) fn overload_methods_declaration_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let order = p.bool("orderByIncreasingParameterCount", false)?;
    p.finish()?;
    cfg.gs.overload_methods = Some(order);
    Ok(())
}

/// `ConstructorsDeclarationGrouping`.
pub(super) fn constructors_declaration_grouping(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let order = p.bool("orderByIncreasingParameterCount", false)?;
    p.finish()?;
    cfg.gs.constructors_grouping = Some(order);
    Ok(())
}

/// `IllegalTokenText`.
pub(super) fn illegal_token_text(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    use super::structural::{message_format, token_names, ILLEGAL_TOKEN_TEXT_ACCEPTED};
    let tokens = token_names(p.take("tokens"), "", ILLEGAL_TOKEN_TEXT_ACCEPTED)?;
    let ignore_case = p.bool("ignoreCase", false)?;
    let pattern = p.take("format").unwrap_or_else(|| "^$".to_string());
    let format = Format::with_case(&pattern, ignore_case)?;
    let custom = p.take("message").filter(|m| !m.is_empty());
    p.finish()?;
    // Without a message, Checkstyle's own: `Token text matches the illegal pattern ''{0}''.`
    let message = message_format(custom.as_deref().unwrap_or("Token text matches the illegal pattern ''{0}''."), &pattern)?;
    let name = id.map_or("IllegalTokenText", leak);
    cfg.gs.illegal_token_text.push(super::structural::IllegalTokenText { name, tokens, format, pattern, message });
    Ok(())
}

/// `PackageDeclaration`.
pub(super) fn package_declaration(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let match_directory = p.bool("matchDirectoryStructure", true)?;
    p.finish()?;
    cfg.gs.package_declaration = Some(match_directory);
    Ok(())
}

/// `NoFinalizer`.
pub(super) fn no_finalizer(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.no_finalizer = true;
    Ok(())
}

/// `OneTopLevelClass`.
pub(super) fn one_top_level_class(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.gs.one_top_level_class = true;
    Ok(())
}

/// `DeclarationOrder`.
pub(super) fn declaration_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let ignore_modifiers = p.bool("ignoreModifiers", false)?;
    let ignore_constructors = p.bool("ignoreConstructors", false)?;
    p.finish()?;
    cfg.declaration_order = Some(DeclarationOrder { ignore_modifiers, ignore_constructors });
    Ok(())
}

/// `RedundantModifier`.
pub(super) fn redundant_modifier(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default(
        "tokens",
        "METHOD_DEF, VARIABLE_DEF, ANNOTATION_FIELD_DEF, INTERFACE_DEF, CTOR_DEF, CLASS_DEF, ENUM_DEF, RESOURCE, ANNOTATION_DEF, RECORD_DEF, \
         PATTERN_VARIABLE_DEF, LITERAL_CATCH, LAMBDA",
    )?;
    let jdk_version = match p.take("jdkVersion") {
        None => 22,
        Some(v) => v
            .trim()
            .trim_start_matches("1.")
            .parse()
            .map_err(|_| format!("property jdkVersion: '{}' is not a Java version", v.trim()))?,
    };
    p.finish()?;
    cfg.redundant_modifier = Some(RedundantModifier { jdk_version });
    Ok(())
}

/// `HiddenField`.
pub(super) fn hidden_field(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "VARIABLE_DEF, PARAMETER_DEF, PATTERN_VARIABLE_DEF, LAMBDA, RECORD_COMPONENT_DEF")?;
    let ignore_format = p.take("ignoreFormat").map(|v| Format::new(&v)).transpose()?;
    let ignore_constructor_parameter = p.bool("ignoreConstructorParameter", false)?;
    let ignore_setter = p.bool("ignoreSetter", false)?;
    let setter_can_return_its_class = p.bool("setterCanReturnItsClass", false)?;
    let ignore_abstract_methods = p.bool("ignoreAbstractMethods", false)?;
    p.finish()?;
    cfg.hidden_field =
        Some(HiddenField { ignore_format, ignore_constructor_parameter, ignore_setter, setter_can_return_its_class, ignore_abstract_methods });
    Ok(())
}

/// `VisibilityModifier`.
pub(super) fn visibility_modifier(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("allowPublicImmutableFields", "false")?;
    // Only read when immutable fields are allowed, which is refused above.
    p.take("immutableClassCanonicalNames");
    p.expect_default("tokens", "VARIABLE_DEF")?;
    let package_allowed = p.bool("packageAllowed", false)?;
    let protected_allowed = p.bool("protectedAllowed", false)?;
    let allow_public_final_fields = p.bool("allowPublicFinalFields", false)?;
    let pattern = p.take("publicMemberPattern").unwrap_or_else(|| "^serialVersionUID$".to_string());
    let public_member_pattern = Format::new(&pattern)?;
    let list = p.take("ignoreAnnotationCanonicalNames").unwrap_or_else(|| VISIBILITY_IGNORED_ANNOTATIONS.to_string());
    let mut ignore_annotations = Vec::new();
    for name in list.split(',').map(|n| n.split_whitespace().collect::<String>()).filter(|n| !n.is_empty()) {
        if let Some(short) = name.rsplit('.').next().filter(|s| *s != name) {
            ignore_annotations.push(short.to_string());
        }
        ignore_annotations.push(name);
    }
    p.finish()?;
    cfg.visibility_modifier =
        Some(VisibilityModifier { package_allowed, protected_allowed, allow_public_final_fields, public_member_pattern, ignore_annotations });
    Ok(())
}

/// `InterfaceIsType`.
pub(super) fn interface_is_type(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow = p.bool("allowMarkerInterfaces", true)?;
    p.finish()?;
    cfg.interface_is_type = Some(allow);
    Ok(())
}

/// `EmptyBlock`.
pub(super) fn empty_block(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let text = match p.take("option").map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("statement") => false,
        Some("text") => true,
        Some(other) => return Err(format!("property option: expected statement or text, got '{other}'")),
    };
    let tokens = p.tokens(EMPTY_BLOCK_TOKENS, EMPTY_BLOCK_ACCEPTED)?;
    p.finish()?;
    cfg.empty_block = Some(EmptyBlock { tokens, text });
    Ok(())
}

/// `AvoidNestedBlocks`.
pub(super) fn avoid_nested_blocks(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_in_switch_case = p.bool("allowInSwitchCase", false)?;
    p.finish()?;
    cfg.avoid_nested_blocks = Some(AvoidNestedBlocks { allow_in_switch_case });
    Ok(())
}

/// `OneStatementPerLine`.
pub(super) fn one_statement_per_line(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let treat_try_resources_as_statement = p.bool("treatTryResourcesAsStatement", false)?;
    p.finish()?;
    cfg.one_statement_per_line = Some(OneStatementPerLine { treat_try_resources_as_statement });
    Ok(())
}

/// `GoogleRightCurly`.
pub(super) fn google_right_curly(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.google_right_curly = true;
    Ok(())
}

/// `TextBlockGoogleStyleFormatting`.
pub(super) fn text_block_google_style_formatting(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.text_block_google = true;
    Ok(())
}

/// `DescendantToken`.
pub(super) fn descendant_token(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    // Without `tokens` Checkstyle's module looks at nothing.
    let Some(tokens) = p.take("tokens") else {
        p.finish()?;
        return Ok(());
    };
    let (mut for_init, mut for_iterator) = (false, false);
    for t in tokens.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        match t {
            "FOR_INIT" => for_init = true,
            "FOR_ITERATOR" => for_iterator = true,
            other => return Err(format!("tokens: {other} is not supported yet (only FOR_INIT and FOR_ITERATOR)")),
        }
    }
    match p.take("limitedTokens").as_deref().map(str::trim) {
        Some("COMMA") => {}
        _ => return Err("property limitedTokens: only COMMA is supported yet".to_string()),
    }
    p.expect_default("sumTokenCounts", "false")?;
    let max_depth = match p.take("maximumDepth") {
        Some(v) => v.trim().parse::<u32>().ok().filter(|d| *d <= 2).ok_or("property maximumDepth: only 0 to 2 is supported yet")?,
        None => return Err("property maximumDepth: a value of 2 or less is required for now".to_string()),
    };
    let min_depth = p.usize("minimumDepth", 0)? as u32;
    let min_number = p.usize("minimumNumber", 0)? as u32;
    let max_number = p.usize("maximumNumber", i32::MAX as usize)? as u32;
    let minimum_message = p.take("minimumMessage");
    let maximum_message = p.take("maximumMessage");
    p.finish()?;
    let name = id.map_or("DescendantToken", leak);
    cfg.descendant_tokens.push(DescendantToken { name, for_init, for_iterator, min_depth, max_depth, min_number, max_number, minimum_message, maximum_message });
    Ok(())
}

/// `EmptyCatchBlock`.
pub(super) fn empty_catch_block(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let non_default = |v: String, default: &str| (v != default).then_some(v);
    let exception_variable_name = p.take("exceptionVariableName").and_then(|v| non_default(v, "^$")).map(|v| Format::new(&v)).transpose()?;
    let comment_format = p.take("commentFormat").and_then(|v| non_default(v, ".*")).map(|v| Format::new(&v)).transpose()?;
    p.finish()?;
    cfg.empty_catch_block = Some(EmptyCatchBlock { exception_variable_name, comment_format });
    Ok(())
}

/// `NeedBraces`.
pub(super) fn need_braces(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("tokens", "LITERAL_DO,LITERAL_ELSE,LITERAL_FOR,LITERAL_IF,LITERAL_WHILE")?;
    p.expect_default("allowSingleLineStatement", "false")?;
    p.expect_default("allowEmptyLoopBody", "false")?;
    p.expect_default("allowEmptyLambdas", "false")?;
    p.expect_default("allowEmptyCatch", "false")?;
    p.expect_default("allowEmptyCase", "false")?;
    p.finish()?;
    cfg.need_braces = true;
    Ok(())
}

/// `EqualsHashCode`.
pub(super) fn equals_hash_code(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.equals_hash_code = true;
    Ok(())
}
