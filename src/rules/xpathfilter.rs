//! `MatchXpath`, the XPath suppression filters (`SuppressionXpathSingleFilter`,
//! `SuppressionXpathFilter`) and the `SuppressWarningsHolder` / `SuppressWarningsFilter` pair.
//!
//! All of them work on the emulated Checkstyle AST (`csast`) and are applied to a file's findings
//! after the tree walk (`suppress::by_file`); `MatchXpath` itself produces findings at the end of
//! the walk (`engine::lint`).
//!
//! Known approximation: Checkstyle's XPath filters match a finding to a selected node by token
//! type, line and column, where the token type is that of the AST node the check logged. javaid's
//! findings carry no token type, so a selected node matches a finding at the same line and column
//! whatever its type. Findings Checkstyle logs by position only (token type 0) can never match, so
//! the modules and messages known to do that are listed in `tokenless` and are left alone.

use crate::config::{Config, Format};
use crate::csast::{CsAst, Tok, N, NIL};
use crate::engine::{Ctx, Diagnostic};
use crate::rules::commentfilter::CHECKER_LEVEL;
use crate::xpath::{XPath, XTree};
use std::collections::HashSet;
use std::path::Path;

/// One `SuppressionXpathSingleFilter` instance or one `<suppress-xpath>` element.
#[derive(Clone, Debug)]
pub struct XpathFilter {
    pub files: Option<Format>,
    pub checks: Option<Format>,
    pub message: Option<Format>,
    /// The module `id` a finding must carry.
    pub id: Option<String>,
    pub query: Option<XPath>,
}

/// One `MatchXpath` instance.
#[derive(Clone, Debug)]
pub struct MatchXpath {
    pub name: &'static str,
    pub query: XPath,
    pub message: String,
}

/// The default `matchxpath.match` message.
pub const MATCH_MESSAGE: &str = "Illegal code structure detected.";

/// Parse the `<suppress-xpath>` elements of a `SuppressionXpathFilter` file.
pub fn parse_xpath_suppressions(text: &str) -> Result<Vec<XpathFilter>, String> {
    let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
    let doc = roxmltree::Document::parse_with_options(text, opts).map_err(|e| format!("cannot parse the suppressions file: {e}"))?;
    let mut out = Vec::new();
    for e in doc.root_element().children().filter(|c| c.is_element()) {
        if e.tag_name().name() != "suppress-xpath" {
            return Err(format!("the XPath suppressions file has an unexpected <{}> element", e.tag_name().name()));
        }
        let query = match e.attribute("query") {
            Some(q) => Some(XPath::compile(q).map_err(|err| format!("incorrect xpath query '{q}': {err}"))?),
            None => None,
        };
        let format = |n: &str| e.attribute(n).map(Format::new).transpose();
        out.push(XpathFilter { files: format("files")?, checks: format("checks")?, message: format("message")?, id: e.attribute("id").map(String::from), query });
    }
    Ok(out)
}

/// `MessageFormat` on a message without arguments: a doubled quote is one quote.
pub fn format_message(m: &str) -> String {
    m.replace("''", "'")
}

/// Whether Checkstyle logs this finding by position only, so that an XPath filter never sees a
/// node for it. Measured against Checkstyle 14.4.0 with a `//*` suppression.
fn tokenless(module: &str, message: &str) -> bool {
    const MODULES: &[&str] = &[
        "NonEmptyAtclauseDescription",
        "AtclauseOrder",
        "SingleLineJavadoc",
        "JavadocMissingLeadingAsterisk",
        "JavadocLeadingAsteriskAlign",
        "RequireEmptyLineBeforeBlockTagGroup",
        "SummaryJavadoc",
        "JavadocParagraph",
        "JavadocTagContinuationIndentation",
        "JavadocBlockTagLocation",
        "JavadocMissingWhitespaceAfterAsterisk",
        "JavadocStyle",
    ];
    const MESSAGES: &[&str] = &["Unknown tag ", "Unused Javadoc tag", "Unused @param tag", "Duplicate @", "@return tag should be present", "Javadoc comment at column"];
    MODULES.contains(&module) || (module.starts_with("Javadoc") && MESSAGES.iter().any(|m| message.starts_with(m)))
}

/// Whether a module can log a node of this type. Checkstyle matches a finding to a selected node by
/// type as well as position; javaid cannot see the type of the node a check logged, so for the
/// modules that only ever log a token it rules out the structural nodes that start at the same
/// place as a token (`OBJBLOCK` and its `LCURLY`, `EXPR` and its first operand...).
fn can_be_logged(module: &str, ty: Tok) -> bool {
    const TOKEN_LOGGERS: &[&str] = &[
        "WhitespaceAround",
        "WhitespaceAfter",
        "NoWhitespaceBefore",
        "NoWhitespaceAfter",
        "ParenPad",
        "TypecastParenPad",
        "MethodParamPad",
        "EmptyForIteratorPad",
        "EmptyForInitializerPad",
        "GenericWhitespace",
        "OperatorWrap",
        "SeparatorWrap",
        "LeftCurly",
        "RightCurly",
        "NoLineWrap",
        "SingleSpaceSeparator",
    ];
    const STRUCTURAL: &[Tok] = &[
        Tok::COMPILATION_UNIT,
        Tok::OBJBLOCK,
        Tok::MODIFIERS,
        Tok::TYPE,
        Tok::EXPR,
        Tok::ELIST,
        Tok::PARAMETERS,
        Tok::METHOD_DEF,
        Tok::CTOR_DEF,
        Tok::COMPACT_CTOR_DEF,
        Tok::VARIABLE_DEF,
        Tok::CLASS_DEF,
        Tok::INTERFACE_DEF,
        Tok::ENUM_DEF,
        Tok::RECORD_DEF,
        Tok::ANNOTATION_DEF,
        Tok::PARAMETER_DEF,
        Tok::ANNOTATIONS,
        Tok::TYPE_ARGUMENTS,
        Tok::TYPE_PARAMETERS,
    ];
    !(TOKEN_LOGGERS.contains(&module) && STRUCTURAL.contains(&ty))
}

/// `MatchXpath`: one finding per selected node.
pub fn match_xpath(cx: &mut Ctx, cache: &crate::csast::CsCache) {
    let cfg = cx.cfg;
    if cfg.match_xpaths.is_empty() {
        return;
    }
    let Some(ast) = cache.get(true) else { return };
    let tree = XTree::new(ast);
    for m in &cfg.match_xpaths {
        match m.query.select(&tree) {
            Ok(hits) => {
                let mut seen: HashSet<N> = HashSet::new();
                for h in hits {
                    if seen.insert(h.node) {
                        cx.diag_at(m.name, ast.line_no(h.node) as u32, ast.column_no(h.node) as u32 + 1, m.message.clone());
                    }
                }
            }
            Err(e) => eprintln!("javaid: MatchXpath: {e}"),
        }
    }
}

/// The 0-based character index (what `DetailAST.getColumnNo` counts) of a 1-based byte column.
/// The character index (0-based, as the syntax tree counts columns) of a finding printed at the
/// tab-expanded column `col`.
fn char_index(lines: &[&[u8]], line: u32, col: u32, tab_width: usize) -> i32 {
    let Some(text) = lines.get((line as usize).wrapping_sub(1)) else { return col as i32 - 1 };
    if text.is_ascii() {
        // One pass: the printed column of the prefix of `i` characters is its tab-expanded width + 1.
        let (mut width, mut best, mut exact) = (0usize, 0i32, None);
        for i in 0..=text.len() {
            let printed = width as u32 + 1;
            if printed <= col {
                best = i as i32;
            }
            if printed == col {
                exact = Some(i as i32);
            }
            if let Some(&b) = text.get(i) {
                width = if b == b'\t' { crate::columns::tab_stop(width, tab_width) } else { width + 1 };
            }
        }
        return exact.unwrap_or(best);
    }
    let mut chars = 0i32;
    let mut best = 0i32;
    for (i, ch) in String::from_utf8_lossy(text).char_indices() {
        if crate::columns::expanded_column(&text[..i], tab_width) <= col {
            best = chars;
        }
        let _ = ch;
        chars += 1;
    }
    if crate::columns::expanded_column(text, tab_width) <= col {
        best = chars;
    }
    // Several prefixes can share a column only when a tab sits between them; take the last that fits exactly.
    let mut exact = None;
    let mut n = 0i32;
    for (i, _) in String::from_utf8_lossy(text).char_indices() {
        if crate::columns::expanded_column(&text[..i], tab_width) == col {
            exact = Some(n);
        }
        n += 1;
    }
    if crate::columns::expanded_column(text, tab_width) == col {
        exact = Some(n);
    }
    exact.unwrap_or(best)
}

struct SwEntry {
    name: String,
    first: (i32, i32),
    last: (i32, i32),
}

fn preorder(ast: &CsAst) -> Vec<N> {
    let mut out = Vec::new();
    if ast.root == NIL {
        return out;
    }
    let mut tops = vec![ast.root];
    while let Some(s) = ast.next_sibling(*tops.last().unwrap()) {
        tops.push(s);
    }
    let mut stack: Vec<N> = tops.into_iter().rev().collect();
    while let Some(n) = stack.pop() {
        out.push(n);
        let kids: Vec<N> = ast.children(n).collect();
        stack.extend(kids.into_iter().rev());
    }
    out
}

fn nth_child(ast: &CsAst, n: N, index: usize) -> Option<N> {
    let mut c = ast.first_child(n);
    for _ in 0..index {
        c = c.and_then(|x| ast.next_sibling(x));
    }
    c
}

fn identifier(ast: &CsAst, n: Option<N>) -> Option<String> {
    let n = n?;
    if ast.ty(n) == Tok::IDENT {
        Some(ast.text(n).to_string())
    } else {
        Some(format!("{}.{}", identifier(ast, ast.first_child(n))?, identifier(ast, ast.last_child(n))?))
    }
}

fn string_expr(ast: &CsAst, expr: N) -> String {
    let Some(first) = ast.first_child(expr) else { return String::new() };
    match ast.ty(first) {
        Tok::STRING_LITERAL => {
            let t: Vec<char> = ast.text(first).chars().collect();
            if t.len() >= 2 { t[1..t.len() - 1].iter().collect() } else { String::new() }
        }
        Tok::IDENT => ast.text(first).to_string(),
        Tok::DOT => ast.last_child(first).map(|l| ast.text(l).to_string()).unwrap_or_default(),
        Tok::TEXT_BLOCK_LITERAL_BEGIN => ast.first_child(first).map(|c| ast.text(c).chars().filter(|c| !matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r')).collect()).unwrap_or_default(),
        _ => String::new(),
    }
}

fn annotation_values(ast: &CsAst, n: N) -> Option<Vec<String>> {
    match ast.ty(n) {
        Tok::EXPR => Some(vec![string_expr(ast, n)]),
        Tok::ANNOTATION_ARRAY_INIT => Some(ast.children(n).filter(|&c| ast.ty(c) == Tok::EXPR).map(|c| string_expr(ast, c)).collect()),
        _ => None,
    }
}

/// `SuppressWarningsHolder.visitToken` over the whole tree.
fn sw_entries(ast: &CsAst) -> Vec<SwEntry> {
    let mut entries = Vec::new();
    for n in preorder(ast) {
        if ast.ty(n) != Tok::ANNOTATION {
            continue;
        }
        let Some(mut id) = identifier(ast, nth_child(ast, n, 1)) else { continue };
        if let Some(rest) = id.strip_prefix("java.lang.") {
            id = rest.to_string();
        }
        if id != "SuppressWarnings" {
            continue;
        }
        // The target.
        let mut cur = ast.parent(n);
        while let Some(c) = cur.filter(|&c| ast.ty(c) == Tok::ANNOTATION_ARRAY_INIT) {
            cur = ast.parent(c);
        }
        let Some(cur) = cur else { continue };
        let target = match ast.ty(cur) {
            Tok::MODIFIERS | Tok::ANNOTATIONS | Tok::ANNOTATION | Tok::ANNOTATION_MEMBER_VALUE_PAIR => ast.parent(cur),
            _ => None,
        };
        let Some(target) = target else { continue };
        let values = match ast.find_first_token(n, Tok::LPAREN) {
            None => Vec::new(),
            Some(lp) => match ast.next_sibling(lp) {
                None => continue,
                Some(next) => match ast.ty(next) {
                    Tok::EXPR | Tok::ANNOTATION_ARRAY_INIT => annotation_values(ast, next).unwrap_or_default(),
                    Tok::ANNOTATION_MEMBER_VALUE_PAIR => match nth_child(ast, next, 2).and_then(|v| annotation_values(ast, v)) {
                        Some(v) => v,
                        None => continue,
                    },
                    Tok::RPAREN => Vec::new(),
                    _ => continue,
                },
            },
        };
        let first = (ast.line_no(target), ast.column_no(target));
        let last = match ast.next_sibling(target) {
            None => (i32::MAX, i32::MAX),
            Some(nx) => (ast.line_no(nx), ast.column_no(nx)),
        };
        for v in values {
            let name = v.strip_prefix("checkstyle:").unwrap_or(&v).to_string();
            entries.push(SwEntry { name, first, last });
        }
    }
    entries
}

fn default_alias(source: &str) -> String {
    let lower = source.to_lowercase();
    let mut end = lower.len();
    if lower.ends_with("check") {
        end -= 5;
    }
    let start = lower.rfind('.').map_or(0, |i| i + 1);
    if start > end { String::new() } else { lower[start..end].to_string() }
}

fn alias_of(source: &str, aliases: &[(String, String)]) -> String {
    let get = |k: &str| aliases.iter().rev().find(|(n, _)| n == k).map(|(_, v)| v.clone());
    if let Some(a) = get(source) {
        return a;
    }
    let base = source.rsplit('.').next().unwrap_or(source);
    if let Some(a) = get(base) {
        return a;
    }
    if let Some(stem) = base.strip_suffix("Check") {
        if let Some(a) = get(stem) {
            return a;
        }
    }
    default_alias(source)
}

/// Apply the XPath filters and the `@SuppressWarnings` filter to a Java file's findings.
pub fn apply(cfg: &Config, path: &Path, cache: &crate::csast::CsCache, findings: &mut Vec<Diagnostic>, rules: &mut crate::suppress::RuleCache) {
    let src = cache.src();
    let sw = cfg.suppress_warnings_filter && cfg.suppress_warnings_holder.is_some();
    if (cfg.xpath_filters.is_empty() && !sw) || findings.is_empty() || path.extension().and_then(|e| e.to_str()) != Some("java") {
        return;
    }
    let lines: Vec<&[u8]> = src.split(|&b| b == b'\n').collect();

    if sw {
        // Only an annotation named `SuppressWarnings` makes an entry, and its token is in the source.
        let named = memchr::memmem::find(src, b"SuppressWarnings").is_some();
        if let Some(ast) = if named { cache.get(false) } else { None } {
            let entries = sw_entries(ast);
            if !entries.is_empty() {
                let aliases = cfg.suppress_warnings_holder.as_deref().unwrap_or(&[]);
                findings.retain(|d| {
                    let info = rules.get(d.rule);
                    let (module, id, source) = (&info.module, info.id, &info.source);
                    let alias = alias_of(source, aliases);
                    // The file-level modules log a line, not a column (javaid prints a 1 for some).
                    let no_column = d.col == 0 || (CHECKER_LEVEL.contains(&module.as_str()) && module != "FileTabCharacter");
                    let column = if no_column { 0 } else { d.col as i32 };
                    let line = d.line as i32;
                    !entries.iter().any(|e| {
                        let after = e.first.0 < line || (e.first.0 == line && (column == 0 || e.first.1 <= column));
                        let before = e.last.0 > line || (e.last.0 == line && e.last.1 >= column);
                        let name_matches = e.name == "all"
                            || e.name.eq_ignore_ascii_case(&alias)
                            || default_alias(&e.name).eq_ignore_ascii_case(&alias)
                            || default_alias(&source).eq_ignore_ascii_case(&e.name);
                        after && before && (name_matches || id.is_some_and(|i| i == e.name))
                    })
                });
            }
        }
    }

    if cfg.xpath_filters.is_empty() || findings.is_empty() {
        return;
    }
    let Some(ast) = cache.get(cfg.needs_comment_nodes) else { return };
    let tree = XTree::new(ast);
    // Positions selected by each filter's query, computed when a finding first reaches it.
    type Selected = std::collections::HashMap<(i32, i32), Vec<Tok>>;
    let mut selected: Vec<Option<Selected>> = vec![None; cfg.xpath_filters.len()];
    let file = crate::suppress::absolute(path);
    findings.retain(|d| {
        let info = rules.get(d.rule);
        let (module, id) = (&info.module, info.id);
        if CHECKER_LEVEL.contains(&module.as_str()) || tokenless(module, &d.message) || d.col == 0 {
            return true;
        }
        let source_module = module;
        let source = &info.source;
        let pos = (d.line as i32, char_index(&lines, d.line, d.col, cfg.walker_tab()));
        for (i, f) in cfg.xpath_filters.iter().enumerate() {
            if f.files.as_ref().is_some_and(|p| !p.matches(file.as_bytes()))
                || f.id.as_ref().is_some_and(|want| id != Some(want.as_str()))
                || f.checks.as_ref().is_some_and(|p| !p.matches(source.as_bytes()))
                || f.message.as_ref().is_some_and(|p| !p.matches(d.message.as_bytes()))
            {
                continue;
            }
            // A filter with nothing configured accepts every finding.
            if f.files.is_none() && f.checks.is_none() && f.message.is_none() && f.id.is_none() && f.query.is_none() {
                continue;
            }
            let Some(q) = &f.query else { return false };
            let set = selected[i].get_or_insert_with(|| match q.select(&tree) {
                Ok(hits) => {
                    let mut m = Selected::new();
                    for h in hits {
                        m.entry((ast.line_no(h.node), ast.column_no(h.node))).or_default().push(ast.ty(h.node));
                    }
                    m
                }
                Err(e) => {
                    eprintln!("javaid: {e}");
                    Selected::new()
                }
            });
            if set.get(&pos).is_some_and(|types| types.iter().any(|&ty| can_be_logged(source_module, ty))) {
                return false;
            }
        }
        true
    });
}
