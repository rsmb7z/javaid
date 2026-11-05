//! Filters: the modules that remove findings, or files, rather than report any.

use super::*;
use crate::xpath::XPath;

/// `BeforeExecutionExclusionFileFilter`.
pub(super) fn before_execution_exclusion_file_filter(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    // Without a pattern the filter accepts every file.
    let pattern = p.take("fileNamePattern");
    p.finish()?;
    if let Some(pattern) = pattern {
        cfg.file_excludes.push(Format::new(&pattern)?);
    }
    Ok(())
}

/// `SuppressionFilter`, `SuppressionXpathFilter`.
pub(super) fn suppression_filter(cfg: &mut Config, module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let file = p.take("file");
    let optional = p.bool("optional", false)?;
    p.finish()?;
    let Some(file) = file else { return Ok(()) };
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(_) if optional => return Ok(()),
        Err(e) => return Err(format!("cannot read the suppressions file '{file}': {e}")),
    };
    if module == "SuppressionXpathFilter" {
        cfg.xpath_filters.extend(parse_xpath_suppressions(&text)?);
        return Ok(());
    }
    let (found, unsupported) = crate::suppress::parse_suppressions(&text)?;
    if let Some(first) = unsupported.first() {
        return Err(format!("the suppressions file '{file}' has <{first}> elements, which are not implemented yet"));
    }
    cfg.suppressions.extend(found);
    Ok(())
}

/// `SuppressionXpathSingleFilter`.
pub(super) fn suppression_xpath_single_filter(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let compile = |q: Option<String>| q.map(|q| crate::xpath::XPath::compile(&q).map_err(|e| format!("incorrect xpath query '{q}': {e}"))).transpose();
    let files = p.take("files").map(|v| Format::new(&v)).transpose()?;
    let checks = p.take("checks").map(|v| Format::new(&v)).transpose()?;
    let message = p.take("message").map(|v| Format::new(&v)).transpose()?;
    let query = compile(p.take("query"))?;
    p.finish()?;
    cfg.xpath_filters.push(XpathFilter { files, checks, message, id: id.map(String::from), query });
    Ok(())
}

/// `SuppressWarningsHolder`.
pub(super) fn suppress_warnings_holder(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let mut aliases = Vec::new();
    if let Some(list) = p.take("aliasList") {
        for item in list.split(',').map(str::trim) {
            match item.find('=') {
                Some(i) if i > 0 => aliases.push((item[..i].to_string(), item[i + 1..].to_string())),
                _ if item.is_empty() => {}
                _ => return Err(format!("'=' expected in alias list item: {item}")),
            }
        }
    }
    p.finish()?;
    cfg.suppress_warnings_holder = Some(aliases);
    Ok(())
}

/// `SuppressWarningsFilter`.
pub(super) fn suppress_warnings_filter(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.suppress_warnings_filter = true;
    Ok(())
}

/// `MatchXpath`.
pub(super) fn match_xpath(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let query = p.take("query").unwrap_or_default();
    let message = p.take("@matchxpath.match").map(|m| format_message(&m)).unwrap_or_else(|| MATCH_MESSAGE.to_string());
    p.finish()?;
    if !query.is_empty() {
        let query = crate::xpath::XPath::compile(&query).map_err(|e| format!("creating Xpath expression failed: {query}: {e}"))?;
        cfg.match_xpaths.push(MatchXpath { name: id.map_or("MatchXpath", leak), query, message });
    }
    Ok(())
}

/// `SuppressWithNearbyTextFilter`.
pub(super) fn suppress_with_nearby_text_filter(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let pattern = Format::new(&p.take("nearbyTextPattern").unwrap_or_else(|| "SUPPRESS CHECKSTYLE (\\w+)".to_string()))?;
    let check = p.take("checkPattern").unwrap_or_else(|| ".*".to_string());
    let message = p.take("messagePattern");
    let id = p.take("idPattern");
    let range = p.take("lineRange").unwrap_or_else(|| "0".to_string());
    p.finish()?;
    cfg.text_filters.push(crate::suppress::TextFilter::Nearby { pattern, check, message, id, range });
    Ok(())
}

/// `SuppressionCommentFilter`.
pub(super) fn suppression_comment_filter(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let off = Format::new(&p.take("offCommentFormat").unwrap_or_else(|| "CHECKSTYLE:OFF".to_string()))?;
    let on = Format::new(&p.take("onCommentFormat").unwrap_or_else(|| "CHECKSTYLE:ON".to_string()))?;
    let check = p.take("checkFormat").unwrap_or_else(|| ".*".to_string());
    let message = p.take("messageFormat");
    let id = p.take("idFormat");
    let check_c = p.bool("checkC", true)?;
    let check_cpp = p.bool("checkCPP", true)?;
    p.finish()?;
    cfg.comment_filters.push(crate::suppress::CommentFilter::OnOff { off, on, check, message, id, check_c, check_cpp });
    Ok(())
}

/// `SuppressWithNearbyCommentFilter`.
pub(super) fn suppress_with_nearby_comment_filter(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let comment = Format::new(&p.take("commentFormat").unwrap_or_else(|| "SUPPRESS CHECKSTYLE (\\w+)".to_string()))?;
    let check = p.take("checkFormat").unwrap_or_else(|| ".*".to_string());
    let message = p.take("messageFormat");
    let id = p.take("idFormat");
    let influence = p.take("influenceFormat").unwrap_or_else(|| "0".to_string());
    let check_c = p.bool("checkC", true)?;
    let check_cpp = p.bool("checkCPP", true)?;
    p.finish()?;
    cfg.comment_filters.push(crate::suppress::CommentFilter::Nearby { comment, check, message, id, influence, check_c, check_cpp });
    Ok(())
}

/// `SuppressWithPlainTextCommentFilter`.
pub(super) fn suppress_with_plain_text_comment_filter(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let off = Format::new(&p.take("offCommentFormat").unwrap_or_else(|| "// CHECKSTYLE:OFF".to_string()))?;
    let on = Format::new(&p.take("onCommentFormat").unwrap_or_else(|| "// CHECKSTYLE:ON".to_string()))?;
    let check = p.take("checkFormat").unwrap_or_else(|| ".*".to_string());
    let message = p.take("messageFormat");
    let id = p.take("idFormat");
    p.finish()?;
    cfg.text_filters.push(crate::suppress::TextFilter::Plain { off, on, check, message, id });
    Ok(())
}

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
        out.push(XpathFilter {
            files: format("files")?,
            checks: format("checks")?,
            message: format("message")?,
            id: e.attribute("id").map(String::from),
            query,
        });
    }
    Ok(out)
}

/// `MessageFormat` on a message without arguments: a doubled quote is one quote.
pub fn format_message(m: &str) -> String {
    m.replace("''", "'")
}
