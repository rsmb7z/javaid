//! What a module's properties are read with: `Props`, and the regular expressions they hold.

use super::*;
use regex::bytes::{Regex, RegexBuilder};

/// A naming pattern. Java and Rust regexes differ on shorthand classes (`\w` is ASCII-only in
/// Java and Unicode in Rust), so patterns are translated before compiling. Names are matched with
/// `find` semantics, exactly as Checkstyle matches them.
///
/// Most patterns compile with the fast `regex` crate and match raw bytes. Patterns that need
/// lookahead or backreferences (Google's naming formats use lookahead) fall back to `fancy-regex`.
#[derive(Clone, Debug)]
pub struct Format {
    pub source: String,
    engine: Engine,
}

#[derive(Clone, Debug)]
enum Engine {
    Fast(Regex),
    Fancy(fancy_regex::Regex),
}

impl Format {
    pub fn new(source: &str) -> Result<Format, String> {
        Format::with_case(source, false)
    }

    /// `ignore_case` is Java's `CASE_INSENSITIVE` flag (`RegexpSingleline.ignoreCase`).
    pub fn with_case(source: &str, ignore_case: bool) -> Result<Format, String> {
        let translated = java_regex_to_rust(source);
        let engine = match RegexBuilder::new(&translated).case_insensitive(ignore_case).build() {
            Ok(re) => Engine::Fast(re),
            Err(_) => {
                let pattern = if ignore_case { format!("(?i){translated}") } else { translated };
                match fancy_regex::Regex::new(&pattern) {
                    Ok(re) => Engine::Fancy(re),
                    Err(e) => return Err(format!("regular expression '{source}' is not supported: {e}")),
                }
            }
        };
        Ok(Format { source: source.to_string(), engine })
    }

    /// The groups of the first match (`Matcher.find()`), group 0 first; an unmatched group is `None`.
    pub fn captures(&self, text: &str) -> Option<Vec<Option<String>>> {
        match &self.engine {
            Engine::Fast(re) => {
                let c = re.captures(text.as_bytes())?;
                Some((0..c.len()).map(|i| c.get(i).map(|m| String::from_utf8_lossy(m.as_bytes()).into_owned())).collect())
            }
            Engine::Fancy(re) => {
                let c = re.captures(text).ok()??;
                Some((0..c.len()).map(|i| c.get(i).map(|m| m.as_str().to_string())).collect())
            }
        }
    }

    /// Every non-overlapping match as `(start, length)` in bytes, like `Matcher.find()` in a loop.
    pub fn find_all(&self, text: &str) -> Vec<(usize, usize)> {
        match &self.engine {
            Engine::Fast(re) => re.find_iter(text.as_bytes()).map(|m| (m.start(), m.end() - m.start())).collect(),
            Engine::Fancy(re) => re.find_iter(text).filter_map(Result::ok).map(|m| (m.start(), m.end() - m.start())).collect(),
        }
    }

    pub fn matches(&self, name: &[u8]) -> bool {
        match &self.engine {
            Engine::Fast(re) => re.is_match(name),
            // A backtracking error (pathological pattern) counts as no match, never a panic.
            Engine::Fancy(re) => std::str::from_utf8(name).is_ok_and(|s| re.is_match(s).unwrap_or(false)),
        }
    }
}

/// Rewrite Java's ASCII shorthand classes into explicit ASCII classes.
pub(super) fn java_regex_to_rust(java: &str) -> String {
    let mut out = String::with_capacity(java.len() + 16);
    let mut class_depth = 0usize;
    let mut chars = java.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('w') => out.push_str(if class_depth > 0 { "a-zA-Z0-9_" } else { "[a-zA-Z0-9_]" }),
                Some('d') => out.push_str(if class_depth > 0 { "0-9" } else { "[0-9]" }),
                Some('s') => out.push_str(if class_depth > 0 { r" \t\n\x0B\f\r" } else { r"[ \t\n\x0B\f\r]" }),
                Some('W') if class_depth == 0 => out.push_str("[^a-zA-Z0-9_]"),
                Some('D') if class_depth == 0 => out.push_str("[^0-9]"),
                Some('S') if class_depth == 0 => out.push_str(r"[^ \t\n\x0B\f\r]"),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            },
            '[' => {
                class_depth += 1;
                out.push('[');
            }
            ']' => {
                class_depth = class_depth.saturating_sub(1);
                out.push(']');
            }
            _ => out.push(c),
        }
    }
    out
}

/// The extra file extensions a file-level module reads, from its own `fileExtensions` property.
/// `None` means the property is not set.
pub type Extensions = Option<Vec<String>>;

/// Properties of one `<module>`, consumed as they are understood; anything left over is a
/// property javaid cannot honour.
pub struct Props {
    pub(super) items: Vec<(String, String)>,
}

/// A comma-separated list compared as a set: order, spacing and case do not matter.
pub(super) fn normalize(s: &str) -> String {
    let mut parts: Vec<String> = s.split(',').map(|p| p.trim().to_ascii_lowercase()).filter(|p| !p.is_empty()).collect();
    parts.sort();
    parts.join(",")
}

impl Props {
    pub(super) fn take(&mut self, name: &str) -> Option<String> {
        let i = self.items.iter().position(|(k, _)| k == name)?;
        Some(self.items.remove(i).1)
    }

    /// The property may be present, but only at its default value.
    pub(super) fn expect_default(&mut self, name: &str, default: &str) -> Result<(), String> {
        match self.take(name) {
            Some(v) if normalize(&v) != normalize(default) => Err(format!("property {name}={v} is not supported yet")),
            _ => Ok(()),
        }
    }

    pub(super) fn bool(&mut self, name: &str, default: bool) -> Result<bool, String> {
        match self.take(name) {
            None => Ok(default),
            Some(v) => match v.trim() {
                "true" => Ok(true),
                "false" => Ok(false),
                other => Err(format!("property {name}: expected true or false, got '{other}'")),
            },
        }
    }

    pub(super) fn i32(&mut self, name: &str, default: i32) -> Result<i32, String> {
        match self.take(name) {
            None => Ok(default),
            Some(v) => v.trim().parse().map_err(|_| format!("property {name}: expected an integer, got '{}'", v.trim())),
        }
    }

    pub(super) fn usize(&mut self, name: &str, default: usize) -> Result<usize, String> {
        match self.take(name) {
            None => Ok(default),
            Some(v) => v.trim().parse().map_err(|_| format!("property {name}: expected a non-negative integer, got '{}'", v.trim())),
        }
    }

    /// The `tokens` property: `default` when it is absent, and only tokens in `accepted` otherwise.
    /// Checkstyle refuses a token its module does not accept (TYPECAST has a module of its own,
    /// TypecastParenPad); javaid also refuses the ones it has not modelled, by name.
    pub(super) fn tokens(&mut self, default: &str, accepted: &str) -> Result<TokenSet, String> {
        let list = self.take("tokens");
        let tokens = parse_tokens(list.as_deref().unwrap_or(default)).map_err(|t| format!("tokens: {t} is not a token javaid knows"))?;
        let allowed = parse_tokens(accepted)?;
        if let Some(list) = list.filter(|_| !tokens.is_subset_of(allowed)) {
            let name = list.split(',').map(str::trim).find(|n| parse_tokens(n).is_ok_and(|t| !t.is_subset_of(allowed))).unwrap_or("?");
            return Err(format!("tokens: {name} is not supported by this module"));
        }
        Ok(tokens)
    }

    /// A `Scope` property: nothing, public, protected, package, private or anoninner.
    pub(super) fn scope(&mut self, name: &str) -> Result<Option<u8>, String> {
        let Some(v) = self.take(name) else { return Ok(None) };
        Ok(Some(match v.trim().to_ascii_lowercase().as_str() {
            "nothing" => 0,
            "public" => 1,
            "protected" => 2,
            "package" => 3,
            "private" => 4,
            "anoninner" => 5,
            other => return Err(format!("property {name}: '{other}' is not a scope")),
        }))
    }

    /// `accessModifiers`: a list of public, protected, package, private; all four when absent.
    pub(super) fn access_modifiers(&mut self) -> Result<[bool; 4], String> {
        let Some(list) = self.take("accessModifiers") else { return Ok([true; 4]) };
        let mut out = [false; 4];
        for word in list.split(',').map(|w| w.trim().to_ascii_lowercase()).filter(|w| !w.is_empty()) {
            match word.as_str() {
                "public" => out[0] = true,
                "protected" => out[1] = true,
                "package" => out[2] = true,
                "private" => out[3] = true,
                other => return Err(format!("property accessModifiers: '{other}' is not public, protected, package or private")),
            }
        }
        Ok(out)
    }

    /// `option`: `nospace` (the default) or `space`; true for `space`.
    pub(super) fn pad_option(&mut self) -> Result<bool, String> {
        match self.take("option").as_deref().map(str::trim) {
            None | Some("nospace") => Ok(false),
            Some("space") => Ok(true),
            Some(other) => Err(format!("property option: expected nospace or space, got '{other}'")),
        }
    }

    /// `option` of the wrap modules: `nl` or `eol`, in any case, and `default_eol` when absent;
    /// true for `eol`.
    pub(super) fn wrap_option(&mut self, default_eol: bool) -> Result<bool, String> {
        match self.take("option").map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            None => Ok(default_eol),
            Some("nl") => Ok(false),
            Some("eol") => Ok(true),
            Some(other) => Err(format!("property option: expected nl or eol, got '{other}'")),
        }
    }

    /// `option` of `LeftCurly`, in any case.
    pub(super) fn curly_option(&mut self) -> Result<CurlyOption, String> {
        match self.take("option").map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            None | Some("eol") => Ok(CurlyOption::Eol),
            Some("nl") => Ok(CurlyOption::Nl),
            Some("nlow") => Ok(CurlyOption::Nlow),
            Some("nl_or_singleline") => Ok(CurlyOption::NlOrSingleline),
            Some(other) => Err(format!("property option: expected eol, nl, nlow or nl_or_singleline, got '{other}'")),
        }
    }

    pub(super) fn extensions(&mut self) -> Extensions {
        self.take("fileExtensions").map(|v| parse_extensions(&v))
    }

    pub(super) fn format(&mut self, default: &str) -> Result<Format, String> {
        let src = self.take("format").unwrap_or_else(|| default.to_string());
        Format::new(&src)
    }

    pub(super) fn finish(self) -> Result<(), String> {
        if self.items.is_empty() {
            Ok(())
        } else {
            let names: Vec<&str> = self.items.iter().map(|(k, _)| k.as_str()).collect();
            Err(format!("unsupported property: {}", names.join(", ")))
        }
    }
}

/// Checkstyle spells extensions with or without the dot; the comparison here is on the bare name.
pub(super) fn parse_extensions(list: &str) -> Vec<String> {
    list.split(',').map(|e| e.trim().trim_start_matches('.').to_string()).filter(|e| !e.is_empty()).collect()
}
