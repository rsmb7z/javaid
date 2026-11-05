//! The stage between the rules and the output: filters that remove findings, and the file filters
//! that keep a file from being read at all.
//!
//! Checkstyle's filters do not find anything; they decide which findings (or files) stand. Done
//! here: `BeforeExecutionExclusionFileFilter` and `SuppressionFilter`. The XPath filters and
//! `@SuppressWarnings` live in `rules::xpathfilter` (called from `by_file`), the comment filters in
//! `rules::commentfilter`.

use crate::config::{Config, Format};
use crate::engine::Diagnostic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

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
pub fn absolute(path: &Path) -> String {
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

/// A filter that reads the file's text for its instructions.
#[derive(Clone, Debug)]
pub enum TextFilter {
    /// `SUPPRESS CHECKSTYLE name` in a line suppresses the lines around it.
    Nearby { pattern: Format, check: String, message: Option<String>, id: Option<String>, range: String },
    /// An "off" line suppresses everything after it until an "on" line.
    Plain { off: Format, on: Format, check: String, message: Option<String>, id: Option<String> },
}

/// `SuppressionCommentFilter` and `SuppressWithNearbyCommentFilter`: filters of the TreeWalker's
/// findings that read the file's comments.
#[derive(Clone, Debug)]
pub enum CommentFilter {
    OnOff { off: Format, on: Format, check: String, message: Option<String>, id: Option<String>, check_c: bool, check_cpp: bool },
    Nearby { comment: Format, check: String, message: Option<String>, id: Option<String>, influence: String, check_c: bool, check_cpp: bool },
}

/// `fillTemplateWithStringsByRegexp`: `$n` becomes group `n` of the first match of `re` in `text`.
pub fn fill(template: &str, text: &str, re: &Format) -> String {
    let mut out = template.to_string();
    if let Some(groups) = re.captures(text) {
        for (i, g) in groups.iter().enumerate() {
            if let Some(g) = g {
                out = out.replace(&format!("${i}"), g);
            }
        }
    }
    out
}

/// The part of a finding the text filters look at.
struct Event<'a> {
    line: u32,
    source: &'a str,
    id: Option<&'static str>,
    message: &'a str,
}

/// What a filter's instruction line says to do, and for which lines.
struct Span {
    first: u32,
    last: u32,
    on: bool,
}

/// The finding-matching half of the instructions that share it, and where those instructions are.
/// Most filters give every instruction the same check, message and id (no `$n` in the templates),
/// so one `Cond` stands for all of them and a finding is tested against it once, not once per line
/// of the file.
struct Cond {
    source: Option<Rc<Format>>,
    message: Option<Rc<Format>>,
    id: Option<Rc<Format>>,
    /// In the order the lines come.
    spans: Vec<Span>,
}

impl Cond {
    fn matches(&self, e: &Event) -> bool {
        self.source.as_ref().is_none_or(|f| f.matches(e.source.as_bytes()))
            && self.id.as_ref().is_none_or(|f| e.id.is_some_and(|id| f.matches(id.as_bytes())))
            && self.message.as_ref().is_none_or(|f| f.matches(e.message.as_bytes()))
    }
}

thread_local! {
    /// Patterns filled in from the file's text, compiled once per thread: the same few come up
    /// on line after line of file after file.
    static FORMATS: RefCell<HashMap<String, Option<Rc<Format>>>> = RefCell::new(HashMap::new());
}

/// `Format::new(pattern).ok()`, remembered.
fn compiled(pattern: &str) -> Option<Rc<Format>> {
    FORMATS.with(|f| {
        let mut f = f.borrow_mut();
        if let Some(found) = f.get(pattern) {
            return found.clone();
        }
        if f.len() >= 4096 {
            f.clear();
        }
        let made = Format::new(pattern).ok().map(Rc::new);
        f.insert(pattern.to_string(), made.clone());
        made
    })
}

fn is_template(s: &str) -> bool {
    s.contains('$')
}

type Key = (String, Option<String>, Option<String>);

fn text_rules(filter: &TextFilter, lines: &[&str]) -> Vec<Cond> {
    let (check, message, id) = match filter {
        TextFilter::Nearby { check, message, id, .. } | TextFilter::Plain { check, message, id, .. } => (check, message, id),
    };
    let mut conds: Vec<Cond> = Vec::new();
    let mut index: HashMap<Key, usize> = HashMap::new();
    // No `$n` in the check, message or id: they read the same on every line.
    let fixed = !is_template(check) && !message.as_deref().is_some_and(is_template) && !id.as_deref().is_some_and(is_template);
    let mut spot = |key: Key, span: Span| {
        let at = *index.entry(key).or_insert_with_key(|k| {
            conds.push(Cond { source: compiled(&k.0), message: k.1.as_deref().and_then(compiled), id: k.2.as_deref().and_then(compiled), spans: Vec::new() });
            conds.len() - 1
        });
        conds[at].spans.push(span);
    };
    let fixed_key = || (check.clone(), message.clone(), id.clone());
    for (i, line) in lines.iter().enumerate() {
        let line_no = i as u32 + 1;
        match filter {
            TextFilter::Nearby { pattern, check, message, id, range } => {
                if !pattern.matches(line.as_bytes()) {
                    continue;
                }
                let Some(groups) = pattern.captures(line) else { continue };
                let text = groups[0].clone().unwrap_or_default();
                let Ok(range) = fill(range, &text, pattern).trim().parse::<i64>() else { continue };
                let (first, last) = ((line_no as i64).min(line_no as i64 + range), (line_no as i64).max(line_no as i64 + range));
                let key = if fixed {
                    fixed_key()
                } else {
                    (fill(check, &text, pattern), message.as_ref().map(|m| fill(m, &text, pattern)), id.as_ref().map(|m| fill(m, &text, pattern)))
                };
                spot(key, Span { first: first.max(0) as u32, last: last.max(0) as u32, on: false });
            }
            TextFilter::Plain { off, on, check, message, id } => {
                // `.captures` only where a template needs the groups.
                let mut found: Option<(bool, &Format, String)> = None;
                if on.matches(line.as_bytes()) {
                    found = Some((true, on, if fixed { String::new() } else { on.captures(line).and_then(|g| g[0].clone()).unwrap_or_default() }));
                }
                if off.matches(line.as_bytes()) {
                    found = Some((false, off, if fixed { String::new() } else { off.captures(line).and_then(|g| g[0].clone()).unwrap_or_default() }));
                }
                let Some((is_on, format, text)) = found else { continue };
                let key = if fixed {
                    fixed_key()
                } else {
                    (fill(check, &text, format), message.as_ref().map(|m| fill(m, &text, format)), id.as_ref().map(|m| fill(m, &text, format)))
                };
                spot(key, Span { first: line_no, last: u32::MAX, on: is_on });
            }
        }
    }
    conds
}

/// Remove the findings that the text-reading filters suppress.
fn by_text(cfg: &Config, src: &[u8], findings: &mut Vec<Diagnostic>, rules: &mut RuleCache) {
    if cfg.text_filters.is_empty() || findings.is_empty() {
        return;
    }
    let text = String::from_utf8_lossy(src);
    // FileText splits on \n, \r and \r\n.
    let normalized = if text.contains('\r') { text.replace("\r\n", "\n").replace('\r', "\n") } else { text.into_owned() };
    let lines: Vec<&str> = normalized.split('\n').collect();
    let lines = if lines.last().is_some_and(|l| l.is_empty()) { &lines[..lines.len() - 1] } else { &lines[..] };
    let per_filter: Vec<(bool, Vec<Cond>)> = cfg.text_filters.iter().map(|f| (matches!(f, TextFilter::Plain { .. }), text_rules(f, lines))).collect();
    findings.retain(|d| {
        let info = rules.get(d.rule);
        let event = Event { line: d.line, source: &info.source, id: info.id, message: &d.message };
        !per_filter.iter().any(|(plain, conds)| {
            let mut hits = conds.iter().filter(|c| c.matches(&event));
            if *plain {
                // The nearest matching instruction before the finding decides; "on" lifts the suppression.
                hits.filter_map(|c| c.spans[..c.spans.partition_point(|r| r.first <= event.line)].last()).max_by_key(|r| r.first).is_some_and(|r| !r.on)
            } else {
                hits.any(|c| c.spans.iter().any(|r| r.first <= event.line && event.line <= r.last))
            }
        })
    });
}

/// Remove the findings of a file that the `SuppressionFilter` suppresses.
pub fn by_file(cfg: &Config, path: &Path, src: &[u8], findings: &mut Vec<Diagnostic>) {
    by_file_cached(cfg, path, &crate::csast::CsCache::detached(src), findings);
}

/// `by_file`, sharing the emulated ASTs already built for the file (or building them for the filters).
pub fn by_file_cached(cfg: &Config, path: &Path, cache: &crate::csast::CsCache, findings: &mut Vec<Diagnostic>) {
    let src = cache.src();
    let mut rules = RuleCache::new(cfg);
    by_text(cfg, src, findings, &mut rules);
    crate::rules::xpathfilter::apply(cfg, path, cache, findings, &mut rules);
    if cfg.suppressions.is_empty() || findings.is_empty() {
        return;
    }
    let name = absolute(path);
    // Elements whose `files` do not match this file can never suppress anything in it.
    let relevant: Vec<&Suppression> = cfg.suppressions.iter().filter(|s| s.files.as_ref().is_none_or(|f| f.matches(name.as_bytes()))).collect();
    if relevant.is_empty() {
        return;
    }
    findings.retain(|d| {
        let info = rules.get(d.rule);
        let (id, column, source) = (info.id, d.col, &info.source);
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
pub fn module_of(cfg: &Config, display: &str) -> (String, Option<&'static str>) {
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
    // Modules whose instances each carry their own `id`.
    let per_instance: [(&str, Option<&'static str>); 3] = [
        ("LeftCurly", cfg.left_curly.iter().find(|t| t.name == display).map(|t| t.name)),
        ("OperatorWrap", cfg.operator_wrap.iter().find(|t| t.name == display).map(|t| t.name)),
        ("SeparatorWrap", cfg.separator_wrap.iter().find(|t| t.name == display).map(|t| t.name)),
    ];
    for (module, found) in per_instance {
        if let Some(name) = found {
            return (module.to_string(), (name != module).then_some(name));
        }
    }
    if let Some(t) = cfg.match_xpaths.iter().find(|t| t.name == display) {
        return ("MatchXpath".to_string(), (t.name != "MatchXpath").then_some(t.name));
    }
    (display.to_string(), None)
}

/// What the filters want to know about the module behind a printed rule name.
pub struct RuleInfo {
    pub module: String,
    pub id: Option<&'static str>,
    /// The source name `checks=` patterns match (see [`source_name`]).
    pub source: String,
}

/// [`module_of`] and [`source_name`] for each rule name a file's findings carry, worked out once
/// per name instead of once per finding and filter.
pub struct RuleCache<'c> {
    cfg: &'c Config,
    known: HashMap<&'static str, Rc<RuleInfo>>,
}

impl<'c> RuleCache<'c> {
    pub fn new(cfg: &'c Config) -> Self {
        RuleCache { cfg, known: HashMap::new() }
    }

    pub fn get(&mut self, display: &'static str) -> Rc<RuleInfo> {
        let cfg = self.cfg;
        self.known
            .entry(display)
            .or_insert_with(|| {
                let (module, id) = module_of(cfg, display);
                Rc::new(RuleInfo { source: source_name(module.clone()), module, id })
            })
            .clone()
    }
}

/// `com.puppycrawl.tools.checkstyle.checks.<package>.<Module>Check`, the name `checks=` matches.
pub fn source_name(module: String) -> String {
    let package = match module.as_str() {
        "AnnotationLocation"
        | "AnnotationOnSameLine"
        | "AnnotationUseStyle"
        | "MissingDeprecated"
        | "MissingOverride"
        | "PackageAnnotation"
        | "SuppressWarnings" => "annotation",
        "AvoidNestedBlocks" | "GoogleRightCurly" | "EmptyBlock" | "EmptyCatchBlock" | "LeftCurly" | "NeedBraces" | "RightCurly" => "blocks",
        "DesignForExtension"
        | "FinalClass"
        | "HideUtilityClassConstructor"
        | "InnerTypeLast"
        | "InterfaceIsType"
        | "MutableException"
        | "OneTopLevelClass"
        | "SealedShouldHavePermitsList"
        | "ThrowsCount"
        | "VisibilityModifier" => "design",
        "AvoidStarImport" | "AvoidStaticImport" | "CustomImportOrder" | "IllegalImport" | "ImportControl" | "ImportOrder" | "RedundantImport"
        | "UnusedImports" | "AvoidModuleImport" => "imports",
        "Indentation" => "indentation",
        "AtclauseOrder"
        | "InvalidJavadocPosition"
        | "JavadocBlockTagLocation"
        | "JavadocContentLocation"
        | "JavadocLeadingAsteriskAlign"
        | "JavadocMethod"
        | "JavadocMissingLeadingAsterisk"
        | "JavadocMissingWhitespaceAfterAsterisk"
        | "JavadocPackage"
        | "JavadocParagraph"
        | "JavadocStyle"
        | "JavadocTagContinuationIndentation"
        | "JavadocType"
        | "JavadocVariable"
        | "MissingJavadocMethod"
        | "MissingJavadocPackage"
        | "MissingJavadocType"
        | "NonEmptyAtclauseDescription"
        | "RequireEmptyLineBeforeBlockTagGroup"
        | "SingleLineJavadoc"
        | "SummaryJavadoc"
        | "WriteTag" => "javadoc",
        "BooleanExpressionComplexity" | "ClassDataAbstractionCoupling" | "ClassFanOutComplexity" | "CyclomaticComplexity" | "JavaNCSS" | "NPathComplexity" => {
            "metrics"
        }
        "ModifierOrder" | "RedundantModifier" => "modifier",
        "AbbreviationAsWordInName"
        | "CatchParameterName"
        | "ClassTypeParameterName"
        | "ConstantName"
        | "IllegalIdentifierName"
        | "InterfaceTypeParameterName"
        | "LambdaParameterName"
        | "LocalFinalVariableName"
        | "LocalVariableName"
        | "MemberName"
        | "MethodName"
        | "MethodTypeParameterName"
        | "PackageName"
        | "ParameterName"
        | "PatternVariableName"
        | "RecordComponentName"
        | "RecordTypeParameterName"
        | "StaticVariableName"
        | "TypeName" => "naming",
        "Regexp" | "RegexpMultiline" | "RegexpOnFilename" | "RegexpSingleline" | "RegexpSinglelineJava" => "regexp",
        "AnonInnerLength"
        | "ExecutableStatementCount"
        | "FileLength"
        | "LambdaBodyLength"
        | "LineLength"
        | "MethodCount"
        | "MethodLength"
        | "OuterTypeNumber"
        | "ParameterNumber"
        | "RecordComponentNumber" => "sizes",
        "EmptyForInitializerPad"
        | "EmptyForIteratorPad"
        | "EmptyLineSeparator"
        | "FileTabCharacter"
        | "GenericWhitespace"
        | "MethodParamPad"
        | "NoLineWrap"
        | "NoWhitespaceAfter"
        | "NoWhitespaceBefore"
        | "NoWhitespaceBeforeCaseDefaultColon"
        | "OperatorWrap"
        | "ParenPad"
        | "SeparatorWrap"
        | "SingleSpaceSeparator"
        | "TypecastParenPad"
        | "WhitespaceAfter"
        | "WhitespaceAround" => "whitespace",
        "Header" | "MultiFileRegexpHeader" | "RegexpHeader" => "header",
        "ArrayTypeStyle"
        | "AvoidEscapedUnicodeCharacters"
        | "CommentsIndentation"
        | "DescendantToken"
        | "FinalParameters"
        | "NewlineAtEndOfFile"
        | "OuterTypeFilename"
        | "TodoComment"
        | "Translation"
        | "TrailingComment"
        | "UncommentedMain"
        | "UpperEll"
        | "OrderedProperties"
        | "ConstructorsDeclarationGrouping" => "misc",
        // The rest are the coding checks.
        _ => "coding",
    };
    format!("com.puppycrawl.tools.checkstyle.checks.{package}.{module}Check")
}
