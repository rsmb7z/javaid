//! A reader for the block tags of a Javadoc comment: enough of Checkstyle's Javadoc grammar to
//! know which `@tag`s a comment has, what they name, and which errors that grammar reports.
//!
//! A block tag starts a line (after the leading `*`), or follows the opening `/**` directly. A
//! `{@...}` inline tag, which may span lines, hides whatever is inside it. Only the failures seen in
//! real code are modelled: an unclosed inline tag, and an HTML tag whose name is followed by a
//! comma (`<N, E>`, which Checkstyle's lexer cannot read). Other malformed comments are not.

/// One `@tag` at the start of a line.
#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    /// 1-based line in the file, and the 0-based character index of the `@` in that line.
    pub line: u32,
    pub col: usize,
    /// The first word after the tag name: a parameter's name, an exception's type. `<T>` for a type parameter.
    pub arg: Option<String>,
    /// The text on the tag's own line after the tag name and, for `@param`, after the name.
    pub first_text: String,
    /// Whether anything but white space follows the name on this or a later line before the next tag.
    pub has_description: bool,
    /// Whether text follows the tag's name argument (for tags without one: the tag name itself).
    pub described: bool,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: u32,
    pub col: usize,
    pub details: String,
}

#[derive(Debug, Default)]
pub struct Javadoc {
    pub tags: Vec<Tag>,
    pub inherit_doc: bool,
    /// `{@return ...}` somewhere in the text.
    pub inline_return: bool,
    pub error: Option<ParseError>,
}

/// `text` is the whole comment, `/**` to `*/`; `first_line` its first line in the file.
pub fn parse(text: &str, first_line: u32) -> Javadoc {
    let mut out = Javadoc::default();
    let lines: Vec<&str> = text.split('\n').collect();
    let mut depth = 0usize; // inside `{@...}`: brace nesting
    let mut open_inline_at: Option<(u32, usize)> = None;
    let last = lines.len() - 1;
    // HTML elements open in the current description (a block tag starts a new one).
    let mut open: Vec<String> = Vec::new();
    // A tag that wants an argument and had none on its line: the next line decides.
    let mut pending_arg: Option<usize> = None;
    for (i, raw) in lines.iter().enumerate() {
        let raw = raw.strip_suffix('\r').unwrap_or(raw);
        let line_no = first_line + i as u32;
        let chars: Vec<char> = raw.chars().collect();
        // Where this line's text starts: after `/**` on the first line, after white space and stars on others.
        let mut p = 0;
        if i == 0 {
            p = 3.min(chars.len());
        } else {
            while p < chars.len() && (chars[p] == ' ' || chars[p] == '\t') {
                p += 1;
            }
            while p < chars.len() && chars[p] == '*' {
                p += 1;
            }
        }
        // The closing `*/` ends the text of the last line.
        let mut end = chars.len();
        if i == last && end >= 2 && chars[end - 2] == '*' && chars[end - 1] == '/' {
            end -= 2;
        }
        // A block tag: first thing on the line, when not inside an inline tag.
        let mut q = p;
        while q < end && (chars[q] == ' ' || chars[q] == '\t') {
            q += 1;
        }
        // The line after a bare `@param`: another tag cannot follow it.
        let mut adopted_here = false;
        if let Some(idx) = pending_arg.take() {
            let wants_name = out.tags[idx].arg.is_none();
            if wants_name && depth == 0 {
                if q < end && chars[q] == '@' {
                    if out.error.is_none() && matches!(out.tags[idx].name.as_str(), "param" | "see" | "throws" | "exception") {
                        out.error = Some(ParseError { line: line_no, col: q, details: "token recognition error at: '@' while parsing  ".to_string() });
                    }
                } else if q < end && matches!(out.tags[idx].name.as_str(), "param" | "throws" | "exception") {
                    // The name is the first word of the next line.
                    let word: String = chars[q..end].iter().take_while(|c| !c.is_whitespace()).collect();
                    let rest_after: String = chars[q + word.chars().count()..end].iter().collect();
                    out.tags[idx].arg = Some(word);
                    out.tags[idx].has_description = true;
                    out.tags[idx].described = !rest_after.trim().is_empty();
                    adopted_here = true;
                }
            }
        }
        if depth == 0 && q < end && chars[q] == '@' {
            let mut r = q + 1;
            while r < end && chars[r].is_ascii_alphabetic() {
                r += 1;
            }
            if r == q + 1 && out.error.is_none() {
                // `@` alone: a space after it cannot be read, nor can the end of the line.
                if r < end {
                    out.error = Some(ParseError { line: line_no, col: r, details: format!("token recognition error at: '{}' while parsing @", chars[r]) });
                } else {
                    let (eline, ecol) = match lines.get(i + 1) {
                        Some(next) => (line_no + 1, next.chars().take_while(|c| c.is_whitespace()).count()),
                        None => (line_no, q + 1),
                    };
                    out.error = Some(ParseError { line: eline, col: ecol, details: "no viable alternative at input '@' while parsing BLOCK_TAG".to_string() });
                }
            }
            if r > q + 1 {
                let name: String = chars[q + 1..r].iter().collect();
                let rest: String = chars[r..end].iter().collect();
                let (arg, after) = split_arg(&name, &rest);
                open.clear();
                if arg.is_none() && rest.trim().is_empty() && matches!(name.as_str(), "param" | "see" | "throws" | "exception") {
                    pending_arg = Some(out.tags.len());
                }
                if name == "serialField" && out.error.is_none() && rest.split_whitespace().count() == 1 {
                    out.error = Some(ParseError { line: line_no, col: chars.len(), details: format!("token recognition error at: '\\n' while parsing {}", rest.trim()) });
                }
                out.tags.push(Tag { name, line: line_no, col: q, arg, first_text: after.trim().to_string(), has_description: !after.trim().is_empty(), described: !after.trim().is_empty() });
                p = r;
            }
        } else if let Some(tag) = out.tags.last_mut() {
            // A continuation line belongs to the tag above it.
            if depth == 0 && q < end {
                tag.has_description = true;
                if !adopted_here {
                    tag.described = true;
                }
            }
        }
        // Inline tags and HTML in the rest of the line.
        let mut k = p;
        while k < end {
            let c = chars[k];
            if depth > 0 {
                match c {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            open_inline_at = None;
                        }
                    }
                    _ => {}
                }
            } else if c == '{' && chars.get(k + 1) == Some(&'@') {
                let mut r = k + 2;
                while r < end && chars[r].is_ascii_alphabetic() {
                    r += 1;
                }
                let name: String = chars[k + 2..r].iter().collect();
                if name == "inheritDoc" {
                    out.inherit_doc = true;
                }
                if name == "return" {
                    out.inline_return = true;
                }
                depth = 1;
                open_inline_at = Some((line_no, k));
                k = r;
                continue;
            } else if c == '<' && out.error.is_none() {
                let closing = chars.get(k + 1) == Some(&'/');
                let name_start = k + if closing { 2 } else { 1 };
                if chars.get(name_start).is_some_and(|c| c.is_ascii_alphabetic()) {
                    let mut r = name_start;
                    while r < end && chars[r].is_ascii_alphanumeric() {
                        r += 1;
                    }
                    let name: String = chars[name_start..r].iter().collect::<String>().to_ascii_lowercase();
                    if closing {
                        match open.iter().rposition(|o| *o == name) {
                            Some(at) => open.truncate(at),
                            None => {
                                out.error = Some(ParseError { line: line_no, col: k + 1, details: "no viable alternative at input '</' while parsing HTML_ELEMENT".to_string() });
                            }
                        }
                    } else if chars.get(r) == Some(&',') {
                        let tag_name: String = chars[name_start..r].iter().collect();
                        out.error = Some(ParseError { line: line_no, col: r, details: format!("token recognition error at: ',' while parsing {tag_name}") });
                    } else {
                        let rest: String = chars[r..end].iter().collect();
                        let self_closing = rest.find('>').is_some_and(|g| rest[..g].ends_with('/'));
                        if !self_closing && !is_void(&name) {
                            open.push(name);
                        }
                    }
                }
            }
            k += 1;
        }
    }
    if depth > 0 && out.error.is_none() {
        let _ = open_inline_at;
        let end_col = lines[last].chars().count().saturating_sub(2);
        out.error = Some(ParseError {
            line: first_line + last as u32,
            col: end_col,
            details: "mismatched input '<EOF>' expecting JAVADOC_INLINE_TAG_END while parsing INLINE_TAG".to_string(),
        });
    }
    out
}

/// HTML elements that have no end tag.
fn is_void(name: &str) -> bool {
    matches!(name, "br" | "hr" | "img" | "input" | "meta" | "link" | "area" | "base" | "col" | "param" | "wbr" | "embed" | "source" | "track")
}

/// For `@param`, `@throws`, `@exception`: the word after the tag, and the text after it.
fn split_arg(name: &str, rest: &str) -> (Option<String>, String) {
    if !matches!(name, "param" | "throws" | "exception" | "serialField") {
        return (None, rest.to_string());
    }
    let trimmed = rest.trim_start();
    if trimmed.is_empty() {
        return (None, String::new());
    }
    let word_end = if trimmed.starts_with('<') { trimmed.find('>').map_or(trimmed.len(), |i| i + 1) } else { trimmed.find(char::is_whitespace).unwrap_or(trimmed.len()) };
    (Some(trimmed[..word_end].to_string()), trimmed[word_end..].to_string())
}

/// Block tags the Javadoc grammar knows. Any other name is a custom tag.
pub fn is_standard(name: &str) -> bool {
    matches!(
        name,
        "author" | "deprecated" | "exception" | "hidden" | "param" | "provides" | "return" | "see" | "serial" | "serialData" | "serialField" | "since" | "throws" | "uses" | "version"
    )
}
