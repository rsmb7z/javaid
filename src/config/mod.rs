//! What to check and how: Checkstyle's own defaults, or a `checkstyle.xml`.
//!
//! The rule is *never skip silently*. A module javaid does not implement, or a property it cannot
//! honour, is reported by name with the reason, and that module does not run.

use crate::tokens::{parse_tokens, TokenSet};

mod props;
pub mod structural;
mod coding;
mod filters;
mod imports;
mod javadoc;
mod misc;
mod naming;
mod text;
mod whitespace;

pub use props::{Extensions, Format};
use props::*;
pub use coding::*;
pub use filters::*;
pub use imports::*;
pub use javadoc::*;
pub use misc::*;
pub use naming::*;
pub use text::*;
pub use whitespace::*;

/// Reads one module's properties into the configuration: the registry row's `apply`.
pub type Apply = fn(&mut Config, &str, Props, Option<&str>) -> Result<(), String>;

/// The four access levels of the `applyTo*` properties, in that order.
const ACCESS: [&str; 4] = ["applyToPublic", "applyToProtected", "applyToPackage", "applyToPrivate"];

/// One implemented module. This table is the single source of truth for which modules exist and
/// how a configuration may use them. Adding a module means adding a row here, its options type and
/// `apply` function in the family file (`config/<family>.rs`), and its rule.
#[derive(Clone, Copy, Debug)]
pub struct Module {
    /// Checkstyle's spelling.
    pub name: &'static str,
    pub family: &'static str,
    /// A configuration may list it more than once (`TypeName` once per type kind).
    pub multi: bool,
    /// Each instance is labelled with its own `id`, instead of one id for the module.
    pub own_name: bool,
    /// Reads the module's properties into the configuration, or says why it cannot.
    pub apply: Apply,
}

pub const REGISTRY: &[Module] = &[
    Module { name: "EmptyCatchBlock", family: "coding and blocks", multi: false, own_name: false, apply: coding::empty_catch_block },
    Module { name: "AvoidNestedBlocks", family: "coding and blocks", multi: false, own_name: false, apply: coding::avoid_nested_blocks },
    Module { name: "FinalParameters", family: "coding and blocks", multi: false, own_name: false, apply: coding::final_parameters },
    Module { name: "MagicNumber", family: "coding and blocks", multi: false, own_name: false, apply: coding::magic_number },
    Module { name: "FinalClass", family: "coding and blocks", multi: false, own_name: false, apply: coding::final_class },
    Module { name: "OneTopLevelClass", family: "coding and blocks", multi: false, own_name: false, apply: coding::one_top_level_class },
    Module { name: "NoFinalizer", family: "coding and blocks", multi: false, own_name: false, apply: coding::no_finalizer },
    Module { name: "PackageDeclaration", family: "coding and blocks", multi: false, own_name: false, apply: coding::package_declaration },
    Module { name: "OuterTypeFilename", family: "misc", multi: false, own_name: false, apply: misc::outer_type_filename },
    Module { name: "NoLineWrap", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::no_line_wrap },
    Module { name: "IllegalTokenText", family: "coding and blocks", multi: true, own_name: true, apply: coding::illegal_token_text },
    Module { name: "ArrayBracketNoWhitespace", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::array_bracket_no_whitespace },
    Module { name: "SingleSpaceSeparator", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::single_space_separator },
    Module { name: "AvoidModuleImport", family: "imports", multi: false, own_name: false, apply: imports::avoid_module_import },
    Module { name: "ModuleDirectiveOrder", family: "imports", multi: false, own_name: false, apply: imports::module_directive_order },
    Module { name: "AnnotationLocation", family: "modifiers and small checks", multi: true, own_name: true, apply: misc::annotation_location },
    Module { name: "WhitespaceBeforeEmptyBody", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::whitespace_before_empty_body },
    Module { name: "NoWhitespaceBeforeCaseDefaultColon", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::no_whitespace_before_case_default_colon },
    Module { name: "ConstructorsDeclarationGrouping", family: "coding and blocks", multi: false, own_name: false, apply: coding::constructors_declaration_grouping },
    Module { name: "OverloadMethodsDeclarationOrder", family: "coding and blocks", multi: false, own_name: false, apply: coding::overload_methods_declaration_order },
    Module { name: "MissingOverrideOnRecordAccessor", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::missing_override_on_record_accessor },
    Module { name: "HideUtilityClassConstructor", family: "coding and blocks", multi: false, own_name: false, apply: coding::hide_utility_class_constructor },
    Module { name: "InterfaceIsType", family: "coding and blocks", multi: false, own_name: false, apply: coding::interface_is_type },
    Module { name: "HiddenField", family: "coding and blocks", multi: false, own_name: false, apply: coding::hidden_field },
    Module { name: "RedundantModifier", family: "coding and blocks", multi: false, own_name: false, apply: coding::redundant_modifier },
    Module { name: "DeclarationOrder", family: "coding and blocks", multi: false, own_name: false, apply: coding::declaration_order },
    Module { name: "DesignForExtension", family: "coding and blocks", multi: false, own_name: false, apply: coding::design_for_extension },
    Module { name: "VisibilityModifier", family: "coding and blocks", multi: false, own_name: false, apply: coding::visibility_modifier },
    Module { name: "EmptyBlock", family: "coding and blocks", multi: false, own_name: false, apply: coding::empty_block },
    Module { name: "NeedBraces", family: "coding and blocks", multi: false, own_name: false, apply: coding::need_braces },
    Module { name: "EqualsHashCode", family: "coding and blocks", multi: false, own_name: false, apply: coding::equals_hash_code },
    Module { name: "RedundantImport", family: "imports", multi: false, own_name: false, apply: imports::redundant_import },
    Module { name: "IllegalInstantiation", family: "imports", multi: false, own_name: false, apply: imports::illegal_instantiation },
    Module { name: "IllegalImport", family: "imports", multi: false, own_name: false, apply: imports::illegal_import },
    Module { name: "UnusedImports", family: "imports", multi: false, own_name: false, apply: imports::unused_imports },
    Module { name: "TypeName", family: "naming", multi: true, own_name: false, apply: naming::type_name },
    Module { name: "MethodName", family: "naming", multi: false, own_name: false, apply: naming::method_name },
    Module { name: "MemberName", family: "naming", multi: false, own_name: false, apply: naming::member_name },
    Module { name: "ConstantName", family: "naming", multi: false, own_name: false, apply: naming::member_name },
    Module { name: "StaticVariableName", family: "naming", multi: false, own_name: false, apply: naming::member_name },
    Module { name: "ParameterName", family: "naming", multi: false, own_name: false, apply: naming::parameter_name },
    Module { name: "LocalVariableName", family: "naming", multi: false, own_name: false, apply: naming::local_variable_name },
    Module { name: "LocalFinalVariableName", family: "naming", multi: false, own_name: false, apply: naming::local_final_variable_name },
    Module { name: "PackageName", family: "naming", multi: false, own_name: false, apply: naming::package_name },
    Module { name: "ClassTypeParameterName", family: "naming", multi: false, own_name: false, apply: naming::class_type_parameter_name },
    Module { name: "InterfaceTypeParameterName", family: "naming", multi: false, own_name: false, apply: naming::class_type_parameter_name },
    Module { name: "MethodTypeParameterName", family: "naming", multi: false, own_name: false, apply: naming::class_type_parameter_name },
    Module { name: "RecordTypeParameterName", family: "naming", multi: false, own_name: false, apply: naming::class_type_parameter_name },
    Module { name: "CatchParameterName", family: "naming", multi: false, own_name: false, apply: naming::catch_parameter_name },
    Module { name: "LambdaParameterName", family: "naming", multi: false, own_name: false, apply: naming::local_final_variable_name },
    Module { name: "PatternVariableName", family: "naming", multi: false, own_name: false, apply: naming::local_final_variable_name },
    Module { name: "RecordComponentName", family: "naming", multi: false, own_name: false, apply: naming::record_component_name },
    Module { name: "FileTabCharacter", family: "text", multi: false, own_name: false, apply: text::file_tab_character },
    Module { name: "NewlineAtEndOfFile", family: "text", multi: false, own_name: false, apply: text::newline_at_end_of_file },
    Module { name: "FileLength", family: "text", multi: false, own_name: false, apply: text::file_length },
    Module { name: "LineLength", family: "text", multi: false, own_name: false, apply: text::line_length },
    Module { name: "RegexpSingleline", family: "text", multi: true, own_name: true, apply: text::regexp_singleline },
    Module { name: "BeforeExecutionExclusionFileFilter", family: "filters", multi: true, own_name: true, apply: filters::before_execution_exclusion_file_filter },
    Module { name: "SuppressionFilter", family: "filters", multi: true, own_name: true, apply: filters::suppression_filter },
    Module { name: "SuppressWithNearbyTextFilter", family: "filters", multi: true, own_name: true, apply: filters::suppress_with_nearby_text_filter },
    Module { name: "SuppressionCommentFilter", family: "filters", multi: true, own_name: true, apply: filters::suppression_comment_filter },
    Module { name: "SuppressWithNearbyCommentFilter", family: "filters", multi: true, own_name: true, apply: filters::suppress_with_nearby_comment_filter },
    Module { name: "SuppressWithPlainTextCommentFilter", family: "filters", multi: true, own_name: true, apply: filters::suppress_with_plain_text_comment_filter },
    Module { name: "SuppressionXpathFilter", family: "filters", multi: true, own_name: true, apply: filters::suppression_filter },
    Module { name: "SuppressionXpathSingleFilter", family: "filters", multi: true, own_name: true, apply: filters::suppression_xpath_single_filter },
    Module { name: "SuppressWarningsHolder", family: "filters", multi: false, own_name: false, apply: filters::suppress_warnings_holder },
    Module { name: "SuppressWarningsFilter", family: "filters", multi: false, own_name: false, apply: filters::suppress_warnings_filter },
    Module { name: "MatchXpath", family: "xpath", multi: true, own_name: true, apply: filters::match_xpath },
    Module { name: "NonEmptyAtclauseDescription", family: "javadoc", multi: false, own_name: false, apply: javadoc::non_empty_atclause_description },
    Module { name: "AtclauseOrder", family: "javadoc", multi: false, own_name: false, apply: javadoc::atclause_order },
    Module { name: "SingleLineJavadoc", family: "javadoc", multi: false, own_name: false, apply: javadoc::single_line_javadoc },
    Module { name: "JavadocMissingLeadingAsterisk", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_missing_leading_asterisk },
    Module { name: "AbbreviationAsWordInName", family: "naming", multi: false, own_name: false, apply: naming::abbreviation_as_word_in_name },
    Module { name: "GoogleMethodName", family: "naming", multi: false, own_name: false, apply: naming::google_method_name },
    Module { name: "GoogleNonConstantFieldName", family: "naming", multi: false, own_name: false, apply: naming::google_non_constant_field_name },
    Module { name: "MultilineCommentLeadingAsteriskPresence", family: "misc", multi: false, own_name: false, apply: misc::multiline_comment_leading_asterisk_presence },
    Module { name: "AvoidEscapedUnicodeCharacters", family: "misc", multi: false, own_name: false, apply: misc::avoid_escaped_unicode_characters },
    Module { name: "CustomImportOrder", family: "imports", multi: false, own_name: false, apply: imports::custom_import_order },
    Module { name: "JavadocContentLocation", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_content_location },
    Module { name: "JavadocLeadingAsteriskAlign", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_leading_asterisk_align },
    Module { name: "RequireEmptyLineBeforeBlockTagGroup", family: "javadoc", multi: false, own_name: false, apply: javadoc::require_empty_line_before_block_tag_group },
    Module { name: "SummaryJavadoc", family: "javadoc", multi: false, own_name: false, apply: javadoc::summary_javadoc },
    Module { name: "JavadocParagraph", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_paragraph },
    Module { name: "JavadocTagContinuationIndentation", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_tag_continuation_indentation },
    Module { name: "MissingJavadocType", family: "javadoc", multi: false, own_name: false, apply: javadoc::missing_javadoc_type },
    Module { name: "JavadocPackage", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_package },
    Module { name: "Translation", family: "misc", multi: false, own_name: false, apply: misc::translation },
    Module { name: "MultiFileRegexpHeader", family: "header", multi: false, own_name: false, apply: text::multi_file_regexp_header },
    Module { name: "JavadocMethod", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_method },
    Module { name: "JavadocType", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_type },
    Module { name: "InvalidJavadocPosition", family: "javadoc", multi: false, own_name: false, apply: javadoc::invalid_javadoc_position },
    Module { name: "MissingJavadocMethod", family: "javadoc", multi: false, own_name: false, apply: javadoc::missing_javadoc_method },
    Module { name: "JavadocVariable", family: "javadoc", multi: false, own_name: false, apply: javadoc::javadoc_variable },
    Module { name: "DescendantToken", family: "coding and blocks", multi: true, own_name: true, apply: coding::descendant_token },
    Module { name: "TodoComment", family: "text", multi: true, own_name: true, apply: text::todo_comment },
    Module { name: "ArrayTypeStyle", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::array_type_style },
    Module { name: "EmptyStatement", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::empty_statement },
    Module { name: "VariableDeclarationUsageDistance", family: "coding and blocks", multi: false, own_name: false, apply: coding::variable_declaration_usage_distance },
    Module { name: "CommentsIndentation", family: "misc", multi: false, own_name: false, apply: misc::comments_indentation },
    Module { name: "MethodLength", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::method_length },
    Module { name: "ParameterNumber", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::parameter_number },
    Module { name: "UpperEll", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::upper_ell },
    Module { name: "AvoidStarImport", family: "imports", multi: false, own_name: false, apply: imports::avoid_star_import },
    Module { name: "SimplifyBooleanReturn", family: "coding and blocks", multi: false, own_name: false, apply: coding::simplify_boolean_return },
    Module { name: "SimplifyBooleanExpression", family: "coding and blocks", multi: false, own_name: false, apply: coding::simplify_boolean_expression },
    Module { name: "InnerAssignment", family: "coding and blocks", multi: false, own_name: false, apply: coding::inner_assignment },
    Module { name: "MissingSwitchDefault", family: "coding and blocks", multi: false, own_name: false, apply: coding::missing_switch_default },
    Module { name: "ModifierOrder", family: "modifiers and small checks", multi: false, own_name: false, apply: misc::modifier_order },
    Module { name: "MultipleVariableDeclarations", family: "coding and blocks", multi: false, own_name: false, apply: coding::multiple_variable_declarations },
    Module { name: "OneStatementPerLine", family: "coding and blocks", multi: false, own_name: false, apply: coding::one_statement_per_line },
    Module { name: "FallThrough", family: "coding and blocks", multi: false, own_name: false, apply: coding::fall_through },
    Module { name: "ParenPad", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::paren_pad },
    Module { name: "TypecastParenPad", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::typecast_paren_pad },
    Module { name: "EmptyForIteratorPad", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::empty_for_iterator_pad },
    Module { name: "MethodParamPad", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::method_param_pad },
    Module { name: "NoWhitespaceBefore", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::no_whitespace_before },
    Module { name: "NoWhitespaceAfter", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::no_whitespace_after },
    Module { name: "WhitespaceAfter", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::whitespace_after },
    Module { name: "WhitespaceAround", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::whitespace_around },
    Module { name: "GenericWhitespace", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::generic_whitespace },
    Module { name: "OperatorWrap", family: "whitespace and wrapping", multi: true, own_name: true, apply: whitespace::operator_wrap },
    Module { name: "SeparatorWrap", family: "whitespace and wrapping", multi: true, own_name: true, apply: whitespace::separator_wrap },
    Module { name: "RightCurly", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::right_curly },
    Module { name: "LeftCurly", family: "whitespace and wrapping", multi: true, own_name: true, apply: whitespace::left_curly },
    Module { name: "Indentation", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::indentation },
    Module { name: "TextBlockGoogleStyleFormatting", family: "coding and blocks", multi: false, own_name: false, apply: coding::text_block_google_style_formatting },
    Module { name: "GoogleRightCurly", family: "coding and blocks", multi: false, own_name: false, apply: coding::google_right_curly },
    Module { name: "EmptyLineSeparator", family: "whitespace and wrapping", multi: false, own_name: false, apply: whitespace::empty_line_separator },
];

/// Every module name, in registry order.
pub fn module_names() -> impl Iterator<Item = &'static str> {
    REGISTRY.iter().map(|m| m.name)
}

/// An `id` from the configuration, kept for the life of the process. Configurations are read once
/// and have a handful of ids, so leaking them is the simplest way to hand out `&'static str`.
fn leak(id: &str) -> &'static str {
    Box::leak(id.to_string().into_boxed_str())
}

pub fn find_module(name: &str) -> Option<&'static Module> {
    REGISTRY.iter().find(|m| m.name == name)
}

use crate::csast::Tok as CsTok;

/// `LeftCurly`'s `option`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurlyOption {
    /// The brace ends the line of the declaration and something follows on the next. The default.
    Eol,
    /// The brace starts its line.
    Nl,
    /// The brace starts its line when the declaration spans lines, and stays on it otherwise.
    Nlow,
    /// The brace starts its line unless the block ends on the line it opens.
    NlOrSingleline,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub empty_catch_block: Option<EmptyCatchBlock>,
    pub need_braces: bool,
    pub equals_hash_code: bool,
    pub unused_imports: Option<UnusedImports>,
    pub naming: Naming,
    pub file_tab_character: Option<FileTabCharacter>,
    pub newline_at_end_of_file: Option<NewlineAtEndOfFile>,
    pub file_length: Option<FileLength>,
    pub line_length: Option<LineLength>,
    pub regexp_singleline: Vec<RegexpSingleline>,
    pub todo_comments: Vec<TodoComment>,
    pub descendant_tokens: Vec<DescendantToken>,
    pub javadoc_variable: Option<JavadocVariable>,
    pub missing_javadoc_method: Option<MissingJavadocMethod>,
    pub invalid_javadoc_position: Option<()>,
    pub javadoc_type: Option<JavadocType>,
    pub javadoc_method: Option<JavadocMethod>,
    pub javadoc_package: Option<JavadocPackage>,
    pub non_empty_atclause_description: bool,
    pub atclause_order: Option<AtclauseOrder>,
    pub single_line_javadoc: Option<SingleLineJavadoc>,
    pub javadoc_missing_leading_asterisk: bool,
    /// `JavadocContentLocation.location`: true for `first_line`.
    pub javadoc_content_location: Option<bool>,
    pub abbreviation_as_word_in_name: Option<AbbreviationAsWordInName>,
    pub multiline_comment_asterisk: bool,
    pub text_block_google: bool,
    pub google_right_curly: bool,
    pub empty_line_separator: Option<EmptyLineSeparator>,
    pub custom_import_order: Option<CustomImportOrder>,
    pub avoid_escaped_unicode: Option<AvoidEscapedUnicode>,
    pub google_method_name: bool,
    pub google_non_constant_field_name: bool,
    /// `JavadocLeadingAsteriskAlign.indentation`.
    pub javadoc_asterisk_align: Option<usize>,
    pub require_empty_line_before_tag_group: bool,
    pub summary_javadoc: Option<SummaryJavadoc>,
    /// `JavadocParagraph.allowNewlineParagraph`.
    pub javadoc_paragraph: Option<bool>,
    /// `JavadocTagContinuationIndentation.offset`.
    pub javadoc_tag_continuation_indent: Option<usize>,
    pub missing_javadoc_type: Option<MissingJavadocType>,
    pub translation: Option<Translation>,
    pub multi_file_regexp_header: Option<MultiFileRegexpHeader>,
    pub array_type_style: bool,
    pub upper_ell: bool,
    pub empty_statement: bool,
    pub avoid_star_import: Option<AvoidStarImport>,
    pub missing_switch_default: bool,
    pub inner_assignment: bool,
    pub simplify_boolean_return: bool,
    pub simplify_boolean_expression: bool,
    pub modifier_order: bool,
    pub multiple_variable_declarations: bool,
    pub one_statement_per_line: Option<OneStatementPerLine>,
    pub fall_through: Option<FallThrough>,
    pub paren_pad: Option<ParenPad>,
    pub avoid_nested_blocks: Option<AvoidNestedBlocks>,
    pub empty_block: Option<EmptyBlock>,
    pub redundant_import: bool,
    /// `IllegalInstantiation`: the fully qualified classes that must not be instantiated.
    pub illegal_instantiation: Option<Vec<String>>,
    pub illegal_import: Option<IllegalImport>,
    pub final_class: bool,
    pub magic_number: Option<MagicNumber>,
    pub final_parameters: Option<FinalParameters>,
    pub hide_utility_class_constructor: bool,
    /// `InterfaceIsType`: whether marker interfaces are allowed.
    pub interface_is_type: Option<bool>,
    pub visibility_modifier: Option<VisibilityModifier>,
    pub hidden_field: Option<HiddenField>,
    pub declaration_order: Option<DeclarationOrder>,
    pub design_for_extension: Option<DesignForExtension>,
    pub redundant_modifier: Option<RedundantModifier>,
    pub right_curly: Option<RightCurly>,
    pub method_length: Option<MethodLength>,
    pub variable_declaration_usage_distance: Option<VariableDeclarationUsageDistance>,
    pub comments_indentation: Option<CommentsIndentation>,
    pub parameter_number: Option<ParameterNumber>,
    pub indentation: Option<Indentation>,
    /// `TypecastParenPad`: whether padding is required (`option=space`).
    pub typecast_paren_pad: Option<bool>,
    /// `EmptyForIteratorPad`: whether padding is required (`option=space`).
    pub empty_for_iterator_pad: Option<bool>,
    pub method_param_pad: Option<MethodParamPad>,
    pub no_whitespace_before: Option<NoWhitespaceBefore>,
    pub no_whitespace_after: Option<NoWhitespaceAfter>,
    pub whitespace_after: Option<WhitespaceAfter>,
    pub whitespace_around: Option<WhitespaceAround>,
    pub generic_whitespace: bool,
    pub operator_wrap: Vec<OperatorWrap>,
    pub separator_wrap: Vec<SeparatorWrap>,
    pub left_curly: Vec<LeftCurly>,
    /// Filters that remove findings (see `suppress`). None are implemented yet.
    pub filters: Vec<String>,
    /// `BeforeExecutionExclusionFileFilter`: files whose path matches are not read at all.
    pub file_excludes: Vec<Format>,
    /// `SuppressWithNearbyTextFilter` and `SuppressWithPlainTextCommentFilter` instances.
    pub text_filters: Vec<crate::suppress::TextFilter>,
    pub comment_filters: Vec<crate::suppress::CommentFilter>,
    /// The elements of every `SuppressionFilter` file.
    pub suppressions: Vec<crate::suppress::Suppression>,
    /// `SuppressionXpathSingleFilter` instances and the `<suppress-xpath>` elements of every `SuppressionXpathFilter` file.
    pub xpath_filters: Vec<XpathFilter>,
    pub match_xpaths: Vec<MatchXpath>,
    /// `SuppressWarningsHolder` with its `aliasList` entries (source name, alias), if configured.
    pub suppress_warnings_holder: Option<Vec<(String, String)>>,
    pub suppress_warnings_filter: bool,
    /// Whether some TreeWalker check asks for comment nodes, which then are in the AST the XPath filters see.
    pub needs_comment_nodes: bool,
    /// The Checker's own `fileExtensions`, if the configuration sets one.
    pub checker_extensions: Option<Vec<String>>,
    /// The Checker's `tabWidth`: how far a tab advances when a line is measured. Default 8.
    pub tab_width: usize,
    /// The TreeWalker's `tabWidth`, which, when it has one, is what the columns of findings on the
    /// syntax tree use instead of the Checker's.
    pub walker_tab_width: Option<usize>,
    /// The small structural modules of `rules::gs` (see `structural`).
    pub gs: structural::Options,
    /// `id`s that single-instance modules were given. Checkstyle prints the id in brackets where it
    /// would print the module name, so javaid does too (see `display`).
    pub ids: Vec<(&'static str, &'static str)>,
    /// The `<message>` overrides of the modules (see `messages`).
    pub custom_messages: Vec<crate::messages::CustomMessage>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            empty_catch_block: None,
            need_braces: false,
            equals_hash_code: false,
            unused_imports: None,
            naming: Naming::default(),
            file_tab_character: None,
            newline_at_end_of_file: None,
            file_length: None,
            line_length: None,
            regexp_singleline: Vec::new(),
            todo_comments: Vec::new(),
            descendant_tokens: Vec::new(),
            javadoc_variable: None,
            missing_javadoc_method: None,
            invalid_javadoc_position: None,
            javadoc_type: None,
            javadoc_method: None,
            javadoc_package: None,
            non_empty_atclause_description: false,
            atclause_order: None,
            single_line_javadoc: None,
            javadoc_missing_leading_asterisk: false,
            javadoc_content_location: None,
            abbreviation_as_word_in_name: None,
            multiline_comment_asterisk: false,
            text_block_google: false,
            google_right_curly: false,
            empty_line_separator: None,
            custom_import_order: None,
            avoid_escaped_unicode: None,
            google_method_name: false,
            google_non_constant_field_name: false,
            javadoc_asterisk_align: None,
            require_empty_line_before_tag_group: false,
            summary_javadoc: None,
            javadoc_paragraph: None,
            javadoc_tag_continuation_indent: None,
            missing_javadoc_type: None,
            translation: None,
            multi_file_regexp_header: None,
            array_type_style: false,
            upper_ell: false,
            empty_statement: false,
            avoid_star_import: None,
            missing_switch_default: false,
            inner_assignment: false,
            simplify_boolean_return: false,
            simplify_boolean_expression: false,
            modifier_order: false,
            multiple_variable_declarations: false,
            one_statement_per_line: None,
            fall_through: None,
            paren_pad: None,
            avoid_nested_blocks: None,
            empty_block: None,
            redundant_import: false,
            illegal_instantiation: None,
            illegal_import: None,
            final_class: false,
            magic_number: None,
            final_parameters: None,
            hide_utility_class_constructor: false,
            interface_is_type: None,
            visibility_modifier: None,
            hidden_field: None,
            declaration_order: None,
            design_for_extension: None,
            redundant_modifier: None,
            right_curly: None,
            method_length: None,
            variable_declaration_usage_distance: None,
            comments_indentation: None,
            parameter_number: None,
            indentation: None,
            typecast_paren_pad: None,
            empty_for_iterator_pad: None,
            method_param_pad: None,
            no_whitespace_before: None,
            no_whitespace_after: None,
            whitespace_after: None,
            whitespace_around: None,
            generic_whitespace: false,
            operator_wrap: Vec::new(),
            separator_wrap: Vec::new(),
            left_curly: Vec::new(),
            filters: Vec::new(),
            file_excludes: Vec::new(),
            suppressions: Vec::new(),
            xpath_filters: Vec::new(),
            match_xpaths: Vec::new(),
            suppress_warnings_holder: None,
            suppress_warnings_filter: false,
            needs_comment_nodes: false,
            text_filters: Vec::new(),
            comment_filters: Vec::new(),
            checker_extensions: None,
            tab_width: 8,
            walker_tab_width: None,
            custom_messages: Vec::new(),
            gs: Default::default(),
            ids: Vec::new(),
        }
    }
}

impl Config {
    /// Do any of the enabled rules look at tokens inside `import` and `package` declarations?
    pub fn looks_inside_declarations(&self) -> bool {
        self.empty_statement
            || self.paren_pad.is_some()
            || self.typecast_paren_pad.is_some()
            || self.no_whitespace_before.is_some()
            || self.no_whitespace_after.is_some()
            || self.whitespace_after.is_some()
            || !self.operator_wrap.is_empty()
            || !self.separator_wrap.is_empty()
    }

    /// What Checkstyle prints in brackets for a module's findings: its `id` if it has one.
    /// The tab width of the checks that run on the syntax tree: the TreeWalker's, else the Checker's.
    pub fn walker_tab(&self) -> usize {
        self.walker_tab_width.unwrap_or(self.tab_width)
    }

    pub fn display(&self, module: &'static str) -> &'static str {
        self.ids.iter().find(|(m, _)| *m == module).map_or(module, |(_, id)| id)
    }

    /// Does a file-level module with these own extensions read a file named `*.ext`? Checkstyle
    /// requires the file to pass the Checker's `fileExtensions` and then the module's own, and reads
    /// every file when neither is set. javaid reads only `.java` then, rather than every file type.
    pub fn reads(&self, own: &Extensions, ext: &str) -> bool {
        let named = |set: &Vec<String>| set.iter().any(|e| e == ext);
        let checker_ok = self.checker_extensions.as_ref().is_none_or(named);
        let own_ok = own.as_ref().is_none_or(named);
        let unconstrained = self.checker_extensions.is_none() && own.is_none();
        checker_ok && own_ok && (!unconstrained || ext == "java")
    }

    /// Every file extension some enabled module reads: what the file search must collect.
    pub fn extensions(&self) -> Vec<String> {
        let mut found = vec!["java".to_string()];
        let mut add = |own: &Extensions| {
            for e in own.iter().chain(self.checker_extensions.iter()).flatten() {
                if !found.contains(e) && self.reads(own, e) {
                    found.push(e.clone());
                }
            }
        };
        if let Some(m) = &self.file_tab_character {
            add(&m.extensions);
        }
        if let Some(m) = &self.translation {
            add(&m.extensions);
        }
        if let Some(m) = &self.multi_file_regexp_header {
            add(&m.extensions);
        }
        if let Some(m) = &self.newline_at_end_of_file {
            add(&m.extensions);
        }
        if let Some(m) = &self.file_length {
            add(&m.extensions);
        }
        if let Some(m) = &self.line_length {
            add(&m.extensions);
        }
        for m in &self.regexp_singleline {
            add(&m.extensions);
        }
        found
    }
}

/// The outcome of reading a `checkstyle.xml`.
pub struct Loaded {
    pub config: Config,
    pub enabled: Vec<String>,
    /// Module name and the reason it did not run.
    pub skipped: Vec<(String, String)>,
    /// Things that do not stop a module running but change what the user sees, e.g. a custom
    /// message template that javaid does not apply.
    pub notes: Vec<String>,
}

/// One `<module>` as written: its name, its properties and its `<message>` overrides.
struct RawModule {
    name: String,
    props: Vec<(String, String)>,
    messages: Vec<(String, String)>,
}

const SIMPLE_LOWER: &str = r"^[a-z][a-zA-Z0-9]*$";
const LOWER_OR_UNDERSCORE: &str = r"^([a-z][a-zA-Z0-9]*|_)$";
const TYPE_PARAMETER: &str = r"^[A-Z]$";

impl Config {
    /// Every implemented module at Checkstyle's default settings.
    pub fn checkstyle_defaults(only: Option<&[String]>) -> Config {
        let mut cfg = Config::default();
        for m in module_names() {
            if allowed(only, m) {
                cfg.apply(m, Props { items: Vec::new() }, None).expect("default settings are always valid");
            }
        }
        cfg
    }

    /// Read a Checkstyle configuration. Modules and properties javaid cannot honour are listed in
    /// `skipped` with the reason, and do not run.
    pub fn from_checkstyle_xml(text: &str, only: Option<&[String]>) -> Result<Loaded, String> {
        let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
        let doc = roxmltree::Document::parse_with_options(text, opts).map_err(|e| format!("cannot parse the configuration: {e}"))?;

        let mut raw = Vec::new();
        collect_modules(doc.root_element(), &mut raw);

        let mut cfg = Config::default();
        let mut enabled: Vec<String> = Vec::new();
        let mut skipped = Vec::new();
        let mut notes = Vec::new();
        for RawModule { name: full_name, props: items, messages } in raw {
            let name = short_name(&full_name).to_string();
            let mut props = Props { items };
            if name == "Checker" {
                // The Checker's own settings that decide which files are read and how lines are measured.
                if let Some(e) = props.extensions() {
                    cfg.checker_extensions = Some(e);
                }
                match props.usize("tabWidth", 8) {
                    Ok(w) if w > 0 => cfg.tab_width = w,
                    _ => notes.push("Checker: tabWidth is not a positive integer; javaid measures tabs as 8 columns".to_string()),
                }
                continue;
            }
            if name == "TreeWalker" {
                // The TreeWalker's own tabWidth sets the columns of the findings on the syntax tree.
                match props.take("tabWidth").map(|w| w.trim().parse::<usize>()) {
                    Some(Ok(w)) if w > 0 => cfg.walker_tab_width = Some(w),
                    Some(_) => notes.push("TreeWalker: tabWidth is not a positive integer; javaid uses the Checker's".to_string()),
                    None => {}
                }
                continue;
            }
            if !allowed(only, &name) {
                continue;
            }
            let id = props.take("id");
            if props.take("severity").is_some_and(|s| s.trim() == "ignore") {
                continue;
            }
            if props.items.iter().any(|(_, v)| v.contains("${")) {
                skipped.push((name, "uses an unresolved ${...} property".to_string()));
                continue;
            }
            if !find_module(&name).is_some_and(|m| m.multi) && enabled.contains(&name) {
                skipped.push((name, "a second instance of the same module is not supported".to_string()));
                continue;
            }
            match cfg.apply(&name, props, id.as_deref()) {
                Ok(()) => {
                    for (key, value) in messages {
                        match crate::messages::custom(&name, id.as_deref().unwrap_or(&name), &key, &value) {
                            Some(m) => cfg.custom_messages.push(m),
                            None => notes.push(format!("{name}: the custom message for '{key}' is not applied; javaid prints its own wording")),
                        }
                    }
                    // Modules that keep one name per instance record their own id (see `apply`).
                    if let (Some(id), Some(module)) = (&id, find_module(&name)) {
                        if !module.own_name {
                            let module = module.name;
                            match cfg.ids.iter().find(|(m, _)| *m == module) {
                                Some((_, first)) if first != id => notes.push(format!("{name}: its instances have different ids; javaid prints '{first}' for all")),
                                Some(_) => {}
                                None => cfg.ids.push((module, leak(id))),
                            }
                        }
                    }
                    enabled.push(name);
                }
                Err(why) => skipped.push((name, why)),
            }
        }
        // Checkstyle hands a file-level module every file under the paths when no `fileExtensions`
        // narrows it. javaid reads Java files only, and says so rather than skipping silently.
        let unconstrained = |own: &Extensions| own.is_none();
        let file_level_without_extensions = cfg.file_tab_character.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.newline_at_end_of_file.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.file_length.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.line_length.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.regexp_singleline.iter().any(|m| unconstrained(&m.extensions));
        if cfg.checker_extensions.is_none() && file_level_without_extensions {
            notes.push("no fileExtensions are set: Checkstyle would read every file under the paths for FileTabCharacter, LineLength and RegexpSingleline, javaid reads .java files only".to_string());
        }
        cfg.needs_comment_nodes = enabled.iter().any(|m| COMMENT_NODE_MODULES.contains(&m.as_str()));
        Ok(Loaded { config: cfg, enabled, skipped, notes })
    }

    fn apply(&mut self, module: &str, p: Props, id: Option<&str>) -> Result<(), String> {
        match find_module(module) {
            Some(m) => (m.apply)(self, module, p, id),
            None => Err("not implemented".to_string()),
        }
    }
}

fn allowed(only: Option<&[String]>, module: &str) -> bool {
    only.is_none_or(|names| names.iter().any(|n| n.eq_ignore_ascii_case(module)))
}

/// TreeWalker checks that ask for comment nodes (`isCommentNodesRequired`). When one is configured
/// the AST the filters see has comment nodes in it.
const COMMENT_NODE_MODULES: &[&str] = &[
    "MatchXpath",
    "TodoComment",
    "TrailingComment",
    "CommentsIndentation",
    "AvoidEscapedUnicodeCharacters",
    "MultilineCommentLeadingAsteriskPresence",
    "NonEmptyAtclauseDescription",
    "AtclauseOrder",
    "SingleLineJavadoc",
    "JavadocMissingLeadingAsterisk",
    "JavadocContentLocation",
    "JavadocLeadingAsteriskAlign",
    "RequireEmptyLineBeforeBlockTagGroup",
    "SummaryJavadoc",
    "JavadocParagraph",
    "JavadocTagContinuationIndentation",
    "InvalidJavadocPosition",
    "JavadocMethod",
    "JavadocType",
    "MissingJavadocMethod",
    "MissingJavadocType",
    "JavadocVariable",
    "JavadocStyle",
    "JavadocBlockTagLocation",
    "JavadocMissingWhitespaceAfterAsterisk",
    // Found by probing: with any of these configured the filters' AST has comment nodes.
    "EmptyCatchBlock",
    "DesignForExtension",
    "UnusedImports",
    "FallThrough",
    "WhitespaceAfter",
    "MissingDeprecated",
    "MissingOverride",
    "NoWhitespaceBeforeCaseDefaultColon",
    "IllegalTokenText",
    "EmptyLineSeparator",
    "WriteTag",
];

/// `com.puppycrawl.tools.checkstyle.checks.naming.TypeNameCheck` and `TypeName` are the same module.
fn short_name(full: &str) -> &str {
    let last = full.rsplit('.').next().unwrap_or(full);
    last.strip_suffix("Check").filter(|s| !s.is_empty()).unwrap_or(last)
}

fn collect_modules(node: roxmltree::Node, out: &mut Vec<RawModule>) {
    let is_module = |n: &roxmltree::Node| n.is_element() && n.tag_name().name() == "module";
    if is_module(&node) {
        let children = |tag: &'static str| node.children().filter(move |c| c.is_element() && c.tag_name().name() == tag);
        // `value="${name}" default="x"`: javaid has no property source, so every placeholder is unset,
        // and Checkstyle (probed, bench/oracle/probes/Probe7) then uses the default in place of the
        // whole value, even when the placeholder is only part of it. Without a default, Checkstyle
        // refuses to load the file; javaid skips just that module and says so (see `from_checkstyle_xml`).
        let props = children("property")
            .filter_map(|p| {
                let value = p.attribute("value").unwrap_or("");
                let value = match p.attribute("default") {
                    Some(default) if value.contains("${") => default,
                    _ => value,
                };
                Some((p.attribute("name")?.to_string(), value.to_string()))
            })
            .collect();
        let mut props: Vec<(String, String)> = props;
        let mut messages: Vec<(String, String)> = children("message").filter_map(|m| Some((m.attribute("key")?.to_string(), m.attribute("value").unwrap_or("").to_string()))).collect();
        // MatchXpath's custom message is its output, so it travels as a pseudo-property.
        if short_name(node.attribute("name").unwrap_or("")) == "MatchXpath" {
            for (k, v) in &messages {
                if k == "matchxpath.match" {
                    props.push(("@matchxpath.match".to_string(), v.clone()));
                }
            }
            messages.retain(|(k, _)| k != "matchxpath.match");
        }
        out.push(RawModule { name: node.attribute("name").unwrap_or("").to_string(), props, messages });
    }
    for child in node.children().filter(is_module) {
        collect_modules(child, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_module_is_listed_once_and_loads_at_its_defaults() {
        let mut seen = std::collections::HashSet::new();
        for m in REGISTRY {
            assert!(seen.insert(m.name), "{} is listed twice", m.name);
        }
        // Every row's `apply` accepts the module with no properties: loading all defaults panics otherwise.
        let cfg = Config::checkstyle_defaults(None);
        assert!(cfg.need_braces && cfg.fall_through.is_some());
        assert!(find_module("TodoComment").unwrap().own_name && !find_module("MethodName").unwrap().multi);
    }

    #[test]
    fn java_shorthand_classes_are_ascii() {
        let f = Format::new(r"^[a-z]\w*$").unwrap();
        assert!(f.matches(b"abc_9"));
        assert!(!f.matches("größe".as_bytes()), "Java's \\w is ASCII-only");
        assert!(Format::new(r"^\d+$").unwrap().matches(b"123"));
        assert!(Format::new(r"^[\w.]+$").unwrap().matches(b"a.b_c"));
    }

    #[test]
    fn match_is_a_search_like_javas_find() {
        assert!(Format::new("bc").unwrap().matches(b"abcd"));
    }

    #[test]
    fn lookahead_patterns_fall_back_to_the_slower_engine() {
        // Google's naming formats use lookahead, which the fast engine cannot compile.
        let f = Format::new(r"^(_|[a-z](?=[a-z0-9]|$)(?:[a-zA-Z]|[0-9]+(?:_[0-9]+)*)*)$").unwrap();
        assert!(f.matches(b"ab") && f.matches(b"a") && f.matches(b"_") && f.matches(b"a1_2"));
        assert!(!f.matches(b"aB"), "a second letter must be lower-case or a digit");
        assert!(!f.matches(b"Ab"));
    }

    fn only_module(xml: &str) -> Loaded {
        let wrapped = format!(r#"<module name="Checker"><module name="TreeWalker">{xml}</module></module>"#);
        Config::from_checkstyle_xml(&wrapped, None).unwrap()
    }

    #[test]
    fn tokens_are_accepted_at_a_modules_default_and_refused_otherwise() {
        let same = only_module(r#"<module name="NeedBraces"><property name="tokens" value="LITERAL_WHILE, LITERAL_IF,
            LITERAL_FOR, LITERAL_ELSE, LITERAL_DO"/></module>"#);
        assert!(same.skipped.is_empty() && same.config.need_braces, "order, spacing and newlines do not matter");
        let narrower = only_module(r#"<module name="NeedBraces"><property name="tokens" value="LITERAL_IF"/></module>"#);
        assert!(!narrower.config.need_braces);
        assert!(narrower.skipped[0].1.contains("tokens"));
    }

    #[test]
    fn type_name_can_be_configured_once_per_type_kind() {
        let two = only_module(
            r#"<module name="TypeName"><property name="format" value="^C"/><property name="tokens" value="CLASS_DEF"/></module>
               <module name="TypeName"><property name="tokens" value="INTERFACE_DEF, ENUM_DEF, ANNOTATION_DEF, RECORD_DEF"/></module>"#,
        );
        assert!(two.skipped.is_empty(), "{:?}", two.skipped);
        let t = &two.config.naming.type_name;
        assert_eq!(t.class.as_ref().unwrap().source, "^C");
        assert_eq!(t.record.as_ref().unwrap().source, r"^[A-Z][a-zA-Z0-9]*$");

        let overlap = only_module(
            r#"<module name="TypeName"/>
               <module name="TypeName"><property name="tokens" value="CLASS_DEF"/></module>"#,
        );
        assert_eq!(overlap.skipped.len(), 1);
        assert!(overlap.skipped[0].1.contains("already configured"));
    }

    #[test]
    fn defaults_enable_everything() {
        let cfg = Config::checkstyle_defaults(None);
        assert!(cfg.empty_catch_block.is_some() && cfg.need_braces && cfg.equals_hash_code);
        assert!(cfg.unused_imports.as_ref().unwrap().process_javadoc);
        assert!(cfg.naming.type_name.class.is_some() && cfg.naming.record_component_name.is_some());
    }

    #[test]
    fn an_unset_placeholder_takes_its_default_as_checkstyle_does() {
        // Behaviour pinned by Probe7 against Checkstyle 14.4: the default replaces the whole value.
        let with = only_module(r#"<module name="MethodName"><property name="format" value="${unset}" default="^[A-Z]+$"/></module>"#);
        assert!(with.skipped.is_empty(), "{:?}", with.skipped);
        assert_eq!(with.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$");

        let partial = only_module(r#"<module name="MethodName"><property name="format" value="^[a-z]+${unset}$" default="^[A-Z]+$"/></module>"#);
        assert_eq!(partial.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$");

        let unused = only_module(r#"<module name="MethodName"><property name="format" value="^[A-Z]+$" default="^[a-z]+$"/></module>"#);
        assert_eq!(unused.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$", "a default is ignored when nothing is unset");

        // Checkstyle fails the whole load here; javaid skips the one module and names the reason.
        let without = only_module(r#"<module name="MethodName"><property name="format" value="${unset}"/></module>"#);
        assert!(without.config.naming.method_name.is_none());
        assert!(without.skipped[0].1.contains("unresolved"));
    }

    #[test]
    fn file_level_modules_read_the_extensions_the_config_names() {
        let both = Config::from_checkstyle_xml(
            r#"<module name="Checker"><property name="fileExtensions" value="java, .properties, xml"/>
               <module name="FileTabCharacter"/>
               <module name="LineLength"><property name="fileExtensions" value="java"/></module></module>"#,
            None,
        )
        .unwrap();
        let cfg = &both.config;
        assert!(cfg.reads(&cfg.file_tab_character.as_ref().unwrap().extensions, "properties"), "a leading dot is accepted");
        assert!(cfg.reads(&cfg.file_tab_character.as_ref().unwrap().extensions, "xml"));
        assert!(!cfg.reads(&cfg.line_length.as_ref().unwrap().extensions, "xml"), "the module's own list narrows the Checker's");
        assert!(!cfg.reads(&None, "yaml"), "the Checker's list is a ceiling");
        let mut found = cfg.extensions();
        found.sort();
        assert_eq!(found, ["java", "properties", "xml"]);

        // Nothing narrows anything: Checkstyle would read every file, javaid reads Java and says so.
        let bare = Config::from_checkstyle_xml(r#"<module name="Checker"><module name="FileTabCharacter"/></module>"#, None).unwrap();
        assert!(bare.config.reads(&None, "java") && !bare.config.reads(&None, "xml"));
        assert_eq!(bare.config.extensions(), ["java"]);
        assert!(bare.notes.iter().any(|n| n.contains("fileExtensions")), "{:?}", bare.notes);
    }

    #[test]
    fn options_javaid_does_not_model_are_refused_by_name() {
        let refused = |xml: &str| only_module(xml).skipped.into_iter().map(|(n, why)| format!("{n}: {why}")).collect::<Vec<_>>();
        assert!(refused(r#"<module name="ArrayTypeStyle"><property name="javaStyle" value="false"/></module>"#)[0].contains("javaStyle"));
        assert!(refused(r#"<module name="ModifierOrder"><property name="modifiersOrder" value="public, static"/></module>"#)[0].contains("modifiersOrder"));
        assert!(refused(r#"<module name="FallThrough"><property name="reliefPattern" value="fall"/></module>"#)[0].contains("reliefPattern"));
        assert!(refused(r#"<module name="ParenPad"><property name="tokens" value="NOPE"/></module>"#)[0].contains("NOPE"));
        assert!(refused(r#"<module name="ParenPad"><property name="tokens" value="COMMA"/></module>"#)[0].contains("COMMA is not supported"));
        assert!(refused(r#"<module name="ParenPad"><property name="option" value="maybe"/></module>"#)[0].contains("option"));
        assert!(refused(r#"<module name="OperatorWrap"><property name="option" value="maybe"/></module>"#)[0].contains("expected nl or eol"));
        assert!(refused(r#"<module name="OperatorWrap"><property name="tokens" value="COMMA"/></module>"#)[0].contains("COMMA is not supported"));
        assert!(refused(r#"<module name="SeparatorWrap"><property name="tokens" value="LITERAL_IF"/></module>"#)[0].contains("LITERAL_IF is not supported"));
        assert!(refused(r#"<module name="LeftCurly"><property name="option" value="maybe"/></module>"#)[0].contains("expected eol, nl, nlow"));
        assert!(refused(r#"<module name="AvoidStarImport"><property name="maxAllowedStarImports" value="2"/></module>"#)[0].contains("maxAllowedStarImports"));
        // ...while the defaults, spelled out, are fine.
        for fine in [
            r#"<module name="ArrayTypeStyle"><property name="javaStyle" value="true"/></module>"#,
            r#"<module name="FallThrough"><property name="reliefPattern" value="falls?[ -]?thr(u|ough)"/><property name="checkLastCaseGroup" value="true"/></module>"#,
            r#"<module name="OneStatementPerLine"><property name="treatTryResourcesAsStatement" value="true"/></module>"#,
        ] {
            assert!(refused(fine).is_empty(), "{fine}");
        }
    }

    #[test]
    fn ids_name_the_findings_and_instances_keep_their_own() {
        let loaded = only_module(
            r#"<module name="NeedBraces"><property name="id" value="braces"/></module>
               <module name="TodoComment"><property name="id" value="first"/></module>
               <module name="TodoComment"/>"#,
        );
        assert!(loaded.skipped.is_empty(), "{:?}", loaded.skipped);
        assert_eq!(loaded.config.display("NeedBraces"), "braces");
        assert_eq!(loaded.config.display("MethodName"), "MethodName", "a module without an id keeps its name");
        let names: Vec<_> = loaded.config.todo_comments.iter().map(|t| t.name).collect();
        assert_eq!(names, ["first", "TodoComment"]);
    }

    #[test]
    fn reads_a_checkstyle_xml_and_reports_what_it_skips() {
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <module name="TreeWalker">
    <module name="UnusedImports"><property name="processJavadoc" value="false"/></module>
    <module name="MethodName"><property name="format" value="^[a-z]+$"/></module>
    <module name="NeedBraces"><property name="allowSingleLineStatement" value="true"/></module>
    <module name="AnonInnerLength"/>
    <module name="com.puppycrawl.tools.checkstyle.checks.naming.TypeNameCheck"/>
    <module name="EqualsHashCode"><property name="severity" value="ignore"/></module>
  </module>
</module>"#;
        let loaded = Config::from_checkstyle_xml(xml, None).unwrap();
        assert!(!loaded.config.unused_imports.as_ref().unwrap().process_javadoc);
        assert_eq!(loaded.config.naming.method_name.as_ref().unwrap().format.source, "^[a-z]+$");
        assert!(loaded.config.naming.type_name.class.is_some(), "fully-qualified names are understood");
        assert!(!loaded.config.equals_hash_code, "severity=ignore disables a module");
        let skipped: Vec<_> = loaded.skipped.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(skipped, vec!["NeedBraces", "AnonInnerLength"], "unsupported property and unimplemented module are named");
        assert!(loaded.skipped[0].1.contains("allowSingleLineStatement"));
    }
}
