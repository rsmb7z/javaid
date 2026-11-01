//! The stage between the rules and the output: filters that remove findings, and the file filters
//! that keep a file from being read at all.
//!
//! Checkstyle's filters do not find anything; they decide which findings (or files) stand. Done
//! here: `BeforeExecutionExclusionFileFilter` and `SuppressionFilter`. The XPath, comment and
//! `@SuppressWarnings` filters are not, and a configuration that names them says so when it loads.

use crate::config::{Config, Format};
use crate::engine::Diagnostic;
use std::path::Path;

/// One `<suppress .../>` element. Every attribute present must match for a finding to go.
#[derive(Clone, Debug, Default)]
pub struct Suppression {
    pub files: Option<Format>,
    pub checks: Option<Format>,
    pub message: Option<Format>,
    pub id: Option<Format>,
    pub lines: Option<Vec<(u32, u32)>>,
    pub columns: Option<Vec<(u32, u32)>>,
}

/// Parse a suppressions file. Elements javaid cannot honour (`suppress-xpath`) come back in the
/// second part, to be reported.
pub fn parse_suppressions(text: &str) -> Result<(Vec<Suppression>, Vec<String>), String> {
    let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
    let doc = roxmltree::Document::parse_with_options(text, opts).map_err(|e| format!("cannot parse the suppressions file: {e}"))?;
    let mut out = Vec::new();
    let mut unsupported = Vec::new();
    for e in doc.root_element().children().filter(|c| c.is_element()) {
        match e.tag_name().name() {
            "suppress" => {
                let attr = |n: &str| e.attribute(n);
                let format = |n: &str| attr(n).map(Format::new).transpose();
                let ranges = |n: &str| attr(n).map(parse_ranges).transpose();
                if attr("files").is_none() && attr("checks").is_none() && attr("message").is_none() && attr("id").is_none() {
                    return Err("a <suppress> element needs at least one of files, checks, message or id".to_string());
                }
                out.push(Suppression {
                    files: format("files")?,
                    checks: format("checks")?,
                    message: format("message")?,
                    id: format("id")?,
                    lines: ranges("lines")?,
                    columns: ranges("columns")?,
                });
            }
            other => unsupported.push(other.to_string()),
        }
    }
    Ok((out, unsupported))
}

fn parse_ranges(text: &str) -> Result<Vec<(u32, u32)>, String> {
    text.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(|p| {
            let bad = || format!("'{p}' is not a number or a range of numbers");
            match p.split_once('-') {
                Some((a, b)) => Ok((a.trim().parse().map_err(|_| bad())?, b.trim().parse().map_err(|_| bad())?)),
                None => p.parse().map(|n| (n, n)).map_err(|_| bad()),
            }
        })
        .collect()
}

fn within(ranges: &[(u32, u32)], n: u32) -> bool {
    ranges.iter().any(|&(a, b)| a <= n && n <= b)
}

/// The path Checkstyle matches file patterns against: absolute, as the file system names it.
fn absolute(path: &Path) -> String {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf()).to_string_lossy().into_owned()
}

/// Whether a file filter keeps the file out of the run altogether.
pub fn file_excluded(cfg: &Config, path: &Path) -> bool {
    if cfg.file_excludes.is_empty() {
        return false;
    }
    let name = absolute(path);
    cfg.file_excludes.iter().any(|f| f.matches(name.as_bytes()))
}

/// Remove the findings of a file that the `SuppressionFilter` suppresses.
pub fn by_file(cfg: &Config, path: &Path, src: &[u8], findings: &mut Vec<Diagnostic>) {
    if cfg.suppressions.is_empty() || findings.is_empty() {
        return;
    }
    let name = absolute(path);
    // Elements whose `files` do not match this file can never suppress anything in it.
    let relevant: Vec<&Suppression> = cfg.suppressions.iter().filter(|s| s.files.as_ref().is_none_or(|f| f.matches(name.as_bytes()))).collect();
    if relevant.is_empty() {
        return;
    }
    let lines: Vec<&[u8]> = if relevant.iter().any(|s| s.columns.is_some()) { src.split(|&b| b == b'\n').collect() } else { Vec::new() };
    findings.retain(|d| {
        let (module, id) = module_of(cfg, d.rule);
        let column = expanded_column(&lines, d.line, d.col, cfg.tab_width);
        let source = source_name(module);
        !relevant.iter().any(|s| {
            s.checks.as_ref().is_none_or(|c| c.matches(source.as_bytes()))
                && s.message.as_ref().is_none_or(|m| m.matches(d.message.as_bytes()))
                && s.id.as_ref().is_none_or(|i| id.is_some_and(|id| i.matches(id.as_bytes())))
                && position_matches(s, d.line, column)
        })
    });
}

/// Checkstyle accepts a position when it is in the lines *or* in the columns, if both are given
/// (probed: a finding on a listed line goes whatever its column, and one in a listed column goes
/// whatever its line).
fn position_matches(s: &Suppression, line: u32, column: u32) -> bool {
    match (&s.lines, &s.columns) {
        (None, None) => true,
        (Some(l), None) => within(l, line),
        (None, Some(c)) => within(c, column),
        (Some(l), Some(c)) => within(l, line) || within(c, column),
    }
}

/// The module behind a finding's printed name, and the `id` it was given if that is what was printed.
fn module_of(cfg: &Config, display: &str) -> (String, Option<&'static str>) {
    if let Some((module, id)) = cfg.ids.iter().find(|(_, id)| *id == display) {
        return (module.to_string(), Some(id));
    }
    if let Some(t) = cfg.todo_comments.iter().find(|t| t.name == display) {
        return ("TodoComment".to_string(), (t.name != "TodoComment").then_some(t.name));
    }
    if let Some(t) = cfg.regexp_singleline.iter().find(|t| t.name == display) {
        return ("RegexpSingleline".to_string(), (t.name != "RegexpSingleline").then_some(t.name));
    }
    if let Some(t) = cfg.descendant_tokens.iter().find(|t| t.name == display) {
        return ("DescendantToken".to_string(), (t.name != "DescendantToken").then_some(t.name));
    }
    (display.to_string(), None)
}

/// `com.puppycrawl.tools.checkstyle.checks.<package>.<Module>Check`, the name `checks=` matches.
pub fn source_name(module: String) -> String {
    let package = match module.as_str() {
        "AnnotationLocation" | "AnnotationOnSameLine" | "AnnotationUseStyle" | "MissingDeprecated" | "MissingOverride" | "PackageAnnotation" | "SuppressWarnings" => "annotation",
        "AvoidNestedBlocks" | "EmptyBlock" | "EmptyCatchBlock" | "LeftCurly" | "NeedBraces" | "RightCurly" => "blocks",
        "DesignForExtension" | "FinalClass" | "HideUtilityClassConstructor" | "InnerTypeLast" | "InterfaceIsType" | "MutableException" | "OneTopLevelClass" | "SealedShouldHavePermitsList"
        | "ThrowsCount" | "VisibilityModifier" => "design",
        "AvoidStarImport" | "AvoidStaticImport" | "CustomImportOrder" | "IllegalImport" | "ImportControl" | "ImportOrder" | "RedundantImport" | "UnusedImports" | "AvoidModuleImport" => "imports",
        "Indentation" => "indentation",
        "AtclauseOrder" | "InvalidJavadocPosition" | "JavadocBlockTagLocation" | "JavadocContentLocation" | "JavadocLeadingAsteriskAlign" | "JavadocMethod" | "JavadocMissingLeadingAsterisk"
        | "JavadocMissingWhitespaceAfterAsterisk" | "JavadocPackage" | "JavadocParagraph" | "JavadocStyle" | "JavadocTagContinuationIndentation" | "JavadocType" | "JavadocVariable"
        | "MissingJavadocMethod" | "MissingJavadocPackage" | "MissingJavadocType" | "NonEmptyAtclauseDescription" | "RequireEmptyLineBeforeBlockTagGroup" | "SingleLineJavadoc"
        | "SummaryJavadoc" | "WriteTag" => "javadoc",
        "BooleanExpressionComplexity" | "ClassDataAbstractionCoupling" | "ClassFanOutComplexity" | "CyclomaticComplexity" | "JavaNCSS" | "NPathComplexity" => "metrics",
        "ModifierOrder" | "RedundantModifier" => "modifier",
        "AbbreviationAsWordInName" | "CatchParameterName" | "ClassTypeParameterName" | "ConstantName" | "IllegalIdentifierName" | "InterfaceTypeParameterName" | "LambdaParameterName"
        | "LocalFinalVariableName" | "LocalVariableName" | "MemberName" | "MethodName" | "MethodTypeParameterName" | "PackageName" | "ParameterName" | "PatternVariableName"
        | "RecordComponentName" | "RecordTypeParameterName" | "StaticVariableName" | "TypeName" => "naming",
        "Regexp" | "RegexpMultiline" | "RegexpOnFilename" | "RegexpSingleline" | "RegexpSinglelineJava" => "regexp",
        "AnonInnerLength" | "ExecutableStatementCount" | "FileLength" | "LambdaBodyLength" | "LineLength" | "MethodCount" | "MethodLength" | "OuterTypeNumber" | "ParameterNumber"
        | "RecordComponentNumber" => "sizes",
        "EmptyForInitializerPad" | "EmptyForIteratorPad" | "EmptyLineSeparator" | "FileTabCharacter" | "GenericWhitespace" | "MethodParamPad" | "NoLineWrap" | "NoWhitespaceAfter"
        | "NoWhitespaceBefore" | "NoWhitespaceBeforeCaseDefaultColon" | "OperatorWrap" | "ParenPad" | "SeparatorWrap" | "SingleSpaceSeparator" | "TypecastParenPad" | "WhitespaceAfter"
        | "WhitespaceAround" => "whitespace",
        "Header" | "MultiFileRegexpHeader" | "RegexpHeader" => "header",
        "ArrayTypeStyle" | "AvoidEscapedUnicodeCharacters" | "CommentsIndentation" | "DescendantToken" | "FinalParameters" | "NewlineAtEndOfFile" | "OuterTypeFilename" | "TodoComment"
        | "Translation" | "TrailingComment" | "UncommentedMain" | "UpperEll" | "OrderedProperties" | "ConstructorsDeclarationGrouping" => "misc",
        // The rest are the coding checks.
        _ => "coding",
    };
    format!("com.puppycrawl.tools.checkstyle.checks.{package}.{module}Check")
}

/// The column the filter compares: Checkstyle counts a tab as running to the next multiple of the
/// tab width, so a finding after tab indentation sits further right than its character index.
fn expanded_column(lines: &[&[u8]], line: u32, col: u32, tab_width: usize) -> u32 {
    let Some(text) = lines.get((line as usize).wrapping_sub(1)) else { return col };
    let mut width = 0usize;
    for &b in text.iter().take((col as usize).saturating_sub(1)) {
        if b == b'\t' {
            width = (width / tab_width + 1) * tab_width;
        } else if b & 0xC0 != 0x80 {
            width += 1;
        }
    }
    width as u32 + 1
}
