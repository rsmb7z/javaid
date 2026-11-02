//! Custom messages: the `<message key="..." value="..."/>` elements of a module in the configuration.
//!
//! A rule builds its finding's message in Checkstyle's own wording. When the configuration gives the
//! module a message for a key, Checkstyle prints that pattern instead, filled in with the same
//! arguments (`java.text.MessageFormat`: `''` is one quote, `{0}` the first argument). The rules do
//! not know their keys, so the substitution happens afterwards: the module's default pattern for the
//! key, read from `messages_table`, is turned into a regular expression that takes the arguments
//! back out of the finding, and they go into the custom pattern.

use crate::config::Config;
use crate::engine::Diagnostic;
use crate::messages_table::DEFAULTS;
use regex::Regex;

/// One module instance's replacement for one message key.
#[derive(Clone, Debug)]
pub struct CustomMessage {
    /// The name the module's findings are printed with: its `id` if it has one, else its name.
    pub printed: String,
    /// The default pattern with its arguments as capture groups, in the order they appear.
    from: Regex,
    /// The argument number each capture group holds.
    order: Vec<usize>,
    /// The configured pattern.
    to: String,
}

/// A parsed `MessageFormat` pattern: literal text, and `{n}` references.
enum Piece {
    Text(String),
    Arg(usize),
}

/// Split a `MessageFormat` pattern. A quote starts a quoted section that runs to the next quote
/// (`''` is a quote on its own, inside or outside one); `{n}` and `{n,type,style}` are arguments.
fn parse(pattern: &str) -> Vec<Piece> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut chars = pattern.chars().peekable();
    let mut quoted = false;
    while let Some(c) = chars.next() {
        match c {
            '\'' if chars.peek() == Some(&'\'') => {
                chars.next();
                text.push('\'');
            }
            '\'' => quoted = !quoted,
            '{' if !quoted => {
                let mut inner = String::new();
                for d in chars.by_ref() {
                    if d == '}' {
                        break;
                    }
                    inner.push(d);
                }
                match inner.split(',').next().and_then(|n| n.trim().parse::<usize>().ok()) {
                    Some(n) => {
                        if !text.is_empty() {
                            out.push(Piece::Text(std::mem::take(&mut text)));
                        }
                        out.push(Piece::Arg(n));
                    }
                    None => {
                        text.push('{');
                        text.push_str(&inner);
                        text.push('}');
                    }
                }
            }
            c => text.push(c),
        }
    }
    if !text.is_empty() {
        out.push(Piece::Text(text));
    }
    out
}

/// The default pattern of `key` for `module`, from the properties file of the module's package.
fn default_pattern(module: &str, key: &str) -> Option<&'static str> {
    let source = crate::suppress::source_name(module.to_string());
    let package = source.split('.').nth_back(1).unwrap_or("");
    let found = |pkg: Option<&str>| DEFAULTS.iter().find(|(p, k, _)| *k == key && pkg.is_none_or(|pkg| *p == pkg)).map(|(_, _, v)| *v);
    found(Some(package)).or_else(|| found(None))
}

/// The replacement for `key` of `module`, whose findings are printed as `printed`; `None` when
/// javaid has no default pattern for the key to recognise the finding by.
pub fn custom(module: &str, printed: &str, key: &str, value: &str) -> Option<CustomMessage> {
    let default = default_pattern(module, key)?;
    let mut rx = String::from("^");
    let mut order = Vec::new();
    for piece in parse(default) {
        match piece {
            Piece::Text(t) => rx.push_str(&regex::escape(&t)),
            Piece::Arg(n) => {
                rx.push_str("(.*)");
                order.push(n);
            }
        }
    }
    rx.push('$');
    Some(CustomMessage { printed: printed.to_string(), from: Regex::new(&rx).ok()?, order, to: value.to_string() })
}

impl CustomMessage {
    fn rewrite(&self, message: &str) -> Option<String> {
        let caps = self.from.captures(message)?;
        let mut args: Vec<Option<&str>> = Vec::new();
        for (i, &n) in self.order.iter().enumerate() {
            if args.len() <= n {
                args.resize(n + 1, None);
            }
            args[n] = caps.get(i + 1).map(|m| m.as_str());
        }
        let mut out = String::new();
        for piece in parse(&self.to) {
            match piece {
                Piece::Text(t) => out.push_str(&t),
                // An argument the call did not pass is printed as written, `{n}`.
                Piece::Arg(n) => match args.get(n).copied().flatten() {
                    Some(a) => out.push_str(a),
                    None => out.push_str(&format!("{{{n}}}")),
                },
            }
        }
        Some(out)
    }
}

/// Put the configured messages into the findings from `from` on.
pub fn apply(cfg: &Config, out: &mut [Diagnostic], from: usize) {
    if cfg.custom_messages.is_empty() {
        return;
    }
    for d in &mut out[from..] {
        if let Some(m) = cfg.custom_messages.iter().filter(|m| m.printed == d.rule).find_map(|m| m.rewrite(&d.message)) {
            d.message = m;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pattern_is_read_the_way_message_format_reads_it() {
        let m = custom("WhitespaceAround", "WhitespaceAround", "ws.notFollowed", "WhitespaceAround: ''{0}'' is not followed by whitespace.").unwrap();
        assert_eq!(m.rewrite("'->' is not followed by whitespace.").unwrap(), "WhitespaceAround: '->' is not followed by whitespace.");
        let m = custom("TypeName", "TypeName", "name.invalidPattern", "Type name ''{0}'' must match pattern ''{1}''.").unwrap();
        assert_eq!(m.rewrite("Name 'Foo_Bar' must match pattern '^[A-Z]+$'.").unwrap(), "Type name 'Foo_Bar' must match pattern '^[A-Z]+$'.");
        // A quote alone opens a quoted section; `{0}` inside it stays text.
        let m = custom("TodoComment", "TodoComment", "todo.match", "'{0}' is quoted, don't {0}").unwrap();
        assert_eq!(m.rewrite("Comment matches to-do format 'x'.").unwrap(), "{0} is quoted, dont {0}");
    }
}
