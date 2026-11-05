//! The modules that read a file's lines, and the multi-file header check.

use super::*;

#[derive(Clone, Debug)]
pub struct FileTabCharacter {
    /// Report every line that has a tab, instead of only the first one in the file.
    pub each_line: bool,
    pub extensions: Extensions,
}

/// What `NewlineAtEndOfFile` wants a file to end with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineSeparator {
    Lf,
    Cr,
    Crlf,
    /// Any of the three (the default).
    LfCrCrlf,
}

#[derive(Clone, Debug)]
pub struct NewlineAtEndOfFile {
    pub separator: LineSeparator,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct FileLength {
    pub max: usize,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct LineLength {
    pub max: usize,
    /// A line this matches (with `find` semantics) is never reported. The default is
    /// `^(package|import) .*`. An empty pattern matches every line, so nothing is reported.
    pub ignore_pattern: Format,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct RegexpSingleline {
    /// What Checkstyle prints in brackets: the module's `id` if it has one, else its name.
    pub name: &'static str,
    pub format: Format,
    /// The `message` property, which replaces the default wording.
    pub message: Option<String>,
    pub minimum: usize,
    pub maximum: usize,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct MultiFileRegexpHeader {
    /// The header file names as written, joined with ", " for messages.
    pub names: String,
    /// One list of line patterns per header file, in order, with the file's name.
    pub headers: Vec<(String, Vec<Format>)>,
    pub extensions: Extensions,
}

/// What Checkstyle ships as `/com/puppycrawl/tools/checkstyle/any.header`: one line that matches anything.
pub const ANY_HEADER: &str = "/com/puppycrawl/tools/checkstyle/any.header";

#[derive(Clone, Debug)]
pub struct TodoComment {
    pub name: &'static str,
    pub format: Format,
}

/// `FileTabCharacter`.
pub(super) fn file_tab_character(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let each_line = p.bool("eachLine", false)?;
    let extensions = p.extensions();
    p.finish()?;
    cfg.file_tab_character = Some(FileTabCharacter { each_line, extensions });
    Ok(())
}

/// `NewlineAtEndOfFile`.
pub(super) fn newline_at_end_of_file(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    // `system` is whatever the machine Checkstyle runs on uses, which javaid cannot know.
    let separator = match p.take("lineSeparator").map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("lf_cr_crlf") => LineSeparator::LfCrCrlf,
        Some("lf") => LineSeparator::Lf,
        Some("cr") => LineSeparator::Cr,
        Some("crlf") => LineSeparator::Crlf,
        Some("system") => return Err("property lineSeparator: system depends on the machine and is not supported".to_string()),
        Some(other) => return Err(format!("property lineSeparator: expected lf, cr, crlf or lf_cr_crlf, got '{other}'")),
    };
    let extensions = p.extensions();
    p.finish()?;
    cfg.newline_at_end_of_file = Some(NewlineAtEndOfFile { separator, extensions });
    Ok(())
}

/// `FileLength`.
pub(super) fn file_length(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let max = p.usize("max", 2000)?;
    let extensions = p.extensions();
    p.finish()?;
    cfg.file_length = Some(FileLength { max, extensions });
    Ok(())
}

/// `LineLength`.
pub(super) fn line_length(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let max = p.usize("max", 80)?;
    let ignore_pattern = Format::new(&p.take("ignorePattern").unwrap_or_else(|| "^(package|import) .*".to_string()))?;
    let extensions = p.extensions();
    p.finish()?;
    cfg.line_length = Some(LineLength { max, ignore_pattern, extensions });
    Ok(())
}

/// `RegexpSingleline`.
pub(super) fn regexp_singleline(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let ignore_case = p.bool("ignoreCase", false)?;
    // The default can never match: `$` is the end of the line, and a character must follow.
    let format = Format::with_case(&p.take("format").unwrap_or_else(|| "$.".to_string()), ignore_case)?;
    let message = p.take("message").filter(|m| !m.is_empty());
    let minimum = p.usize("minimum", 0)?;
    let maximum = p.usize("maximum", 0)?;
    let extensions = p.extensions();
    p.finish()?;
    let name = id.map_or("RegexpSingleline", leak);
    cfg.regexp_singleline.push(RegexpSingleline { name, format, message, minimum, maximum, extensions });
    Ok(())
}

/// `MultiFileRegexpHeader`.
pub(super) fn multi_file_regexp_header(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let list = p.take("headerFiles");
    let extensions = p.extensions();
    p.finish()?;
    let Some(list) = list else { return Ok(()) };
    let mut headers = Vec::new();
    for name in list.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        let text = if name == ANY_HEADER {
            "^.*$\n".to_string()
        } else {
            std::fs::read_to_string(name).map_err(|e| format!("unable to load header file {name}: {e}"))?
        };
        let patterns = text.lines().map(|l| Format::new(if l.is_empty() { "^$" } else { l })).collect::<Result<Vec<_>, _>>()?;
        if patterns.is_empty() {
            return Err(format!("Header file is empty: {name}"));
        }
        headers.push((name.to_string(), patterns));
    }
    let names = headers.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", ");
    cfg.multi_file_regexp_header = Some(MultiFileRegexpHeader { names, headers, extensions });
    Ok(())
}

/// `TodoComment`.
pub(super) fn todo_comment(cfg: &mut Config, _module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
    let format = p.format("TODO:")?;
    p.finish()?;
    let name = id.map_or("TodoComment", leak);
    cfg.todo_comments.push(TodoComment { name, format });
    Ok(())
}
