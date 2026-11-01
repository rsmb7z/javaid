//! Which type names does a Javadoc comment refer to?
//!
//! `UnusedImports` counts an import as used when a Javadoc comment mentions it. What counts as
//! "mentions" was settled by running Checkstyle on `bench/oracle/probes/JavadocProbe*.java`, not by
//! reading the Javadoc specification:
//!
//! * the tags are `{@link}`, `{@linkplain}`, `{@value}` (inline, anywhere in the comment, also
//!   inside `<pre>`/`<code>` and inside other tags' text) and the block tags `@see`, `@throws`
//!   and `@exception` (at the start of a line);
//! * only the *first* argument of a tag is a type reference: its first dotted name, reduced to the
//!   leftmost segment (`Outer.Inner` refers to `Outer`; `java.util.List` refers to `java`);
//! * for a member reference such as `Foo#bar(Baz<Qux>, Quux[])`, every name inside the
//!   parentheses is a reference too;
//! * `{@code}`, plain prose, `<a href>` text and quoted strings are not references.

/// Call `f` with every type name the comment refers to. `scratch` is reused between calls so a
/// file with many comments does not allocate for each one.
pub fn for_each_reference(comment: &[u8], scratch: &mut Vec<u8>, f: &mut impl FnMut(&[u8])) {
    if !comment.contains(&b'@') {
        return; // no tag, no reference: the common case
    }
    normalize(comment, scratch);
    let text = &scratch[..];

    // Inline tags: {@link X ...}
    let mut i = 0;
    while let Some(pos) = find(text, b"{@", i) {
        let name_start = pos + 2;
        let mut j = name_start;
        while j < text.len() && text[j].is_ascii_alphabetic() {
            j += 1;
        }
        i = j;
        let name = &text[name_start..j];
        if matches!(name, b"link" | b"linkplain" | b"value") && text.get(j).is_some_and(u8::is_ascii_whitespace) {
            let (mut depth, mut k) = (1usize, j);
            while k < text.len() && depth > 0 {
                match text[k] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    _ => {}
                }
                k += 1;
            }
            let end = if depth == 0 { k - 1 } else { k };
            reference(&text[j..end], f);
            i = k;
        }
    }

    // Block tags: @see X, @throws X, @exception X at the start of a line.
    for line in text.split(|&b| b == b'\n') {
        let line = trim_start(line);
        for tag in [&b"@see"[..], b"@throws", b"@exception"] {
            if let Some(rest) = line.strip_prefix(tag) {
                if rest.first().is_some_and(u8::is_ascii_whitespace) {
                    reference(rest, f);
                    break;
                }
            }
        }
    }
}

/// Strip the comment delimiters and each line's leading asterisks, keeping line breaks so that a
/// tag split across lines is still one tag.
fn normalize(comment: &[u8], out: &mut Vec<u8>) {
    out.clear();
    let body = comment.strip_prefix(b"/**").unwrap_or(comment);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    for line in body.split(|&b| b == b'\n') {
        let mut l = trim_start(line);
        while let Some(rest) = l.strip_prefix(b"*") {
            l = rest;
        }
        out.extend_from_slice(l);
        out.push(b'\n');
    }
}

/// One tag argument: `Type`, `Type#member(Arg, Arg)`, `#member(Arg)`.
fn reference(arg: &[u8], f: &mut impl FnMut(&[u8])) {
    let arg = trim_start(arg);
    if let Some(name) = dotted_name(arg) {
        f(first_segment(name));
    }
    if let Some(open) = arg.iter().position(|&b| b == b'(') {
        let group = &arg[open + 1..];
        let group = &group[..group.iter().position(|&b| b == b')').unwrap_or(group.len())];
        let mut i = 0;
        while i < group.len() {
            if is_ident_start(group[i]) {
                let name = dotted_name(&group[i..]).expect("starts with an identifier");
                f(first_segment(name));
                i += name.len();
            } else {
                i += 1;
            }
        }
    }
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'$' || b >= 0x80
}

fn is_ident_part(b: u8) -> bool {
    is_ident_start(b) || b.is_ascii_digit()
}

/// `ident(.ident)*` at the start of `s`.
fn dotted_name(s: &[u8]) -> Option<&[u8]> {
    if !s.first().copied().is_some_and(is_ident_start) {
        return None;
    }
    let mut end = 0;
    loop {
        while end < s.len() && is_ident_part(s[end]) {
            end += 1;
        }
        if s.get(end) == Some(&b'.') && s.get(end + 1).copied().is_some_and(is_ident_start) {
            end += 1;
        } else {
            return Some(&s[..end]);
        }
    }
}

fn first_segment(name: &[u8]) -> &[u8] {
    name.split(|&b| b == b'.').next().unwrap_or(name)
}

fn trim_start(s: &[u8]) -> &[u8] {
    let n = s.iter().take_while(|b| b.is_ascii_whitespace()).count();
    &s[n..]
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack.get(from..)?.windows(needle.len()).position(|w| w == needle).map(|p| p + from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refs(comment: &str) -> Vec<String> {
        let mut out = Vec::new();
        for_each_reference(comment.as_bytes(), &mut Vec::new(), &mut |n| out.push(String::from_utf8_lossy(n).into_owned()));
        out
    }

    #[test]
    fn inline_tags_that_count() {
        assert_eq!(refs("/** {@link List} */"), ["List"]);
        assert_eq!(refs("/** {@linkplain Queue text} */"), ["Queue"]);
        assert_eq!(refs("/** {@value Deque#X} */"), ["Deque"]);
        assert_eq!(refs("/** {@link   Spaced   label} and {@link\tTabbed} */"), ["Spaced", "Tabbed"]);
        assert_eq!(refs("/** {@link A}{@link B} adjacent */"), ["A", "B"]);
        assert_eq!(refs("/** <pre>{@link InPre}</pre> <code>{@link InCode}</code> */"), ["InPre", "InCode"]);
    }

    #[test]
    fn tags_that_do_not_count() {
        assert!(refs("/** {@code Collection} */").is_empty());
        assert!(refs("/** plain mention of Iterator in prose, even with a @ sign */").is_empty());
        assert!(refs("/** @see <a href=\"x\">Anchor</a> */").is_empty());
        assert!(refs("/** @see \"Quoted\" */").is_empty());
        assert!(refs("/** @link NotARealBlockTag */").is_empty());
    }

    #[test]
    fn only_the_first_argument_and_only_its_leftmost_segment() {
        assert_eq!(refs("/** @see Used Ignored more words */"), ["Used"]);
        assert_eq!(refs("/** {@link Outer.Inner} */"), ["Outer"]);
        assert_eq!(refs("/** {@link java.util.List} */"), ["java"]);
        assert_eq!(refs("/** @throws java.io.IOException when bad */"), ["java"]);
    }

    #[test]
    fn member_references_count_their_parameter_types() {
        assert_eq!(refs("/** {@link #foo(Arg)} */"), ["Arg"]);
        assert_eq!(refs("/** {@link Owner#bar(java.lang.String, Second)} */"), ["Owner", "java", "Second"]);
        assert_eq!(refs("/** {@link #foo(List<Generic>)} */"), ["List", "Generic"]);
        assert_eq!(refs("/** {@link #foo(Var...)} and {@link #bar(Arr[])} */"), ["Var", "Arr"]);
        assert_eq!(refs("/** @see Type#get(Object) */"), ["Type", "Object"]);
    }

    #[test]
    fn block_tags_and_multiline_tags() {
        assert_eq!(refs("/**\n * text\n *\n * @see Seen\n * @throws Thrown when bad\n * @exception Excepted\n */"), ["Seen", "Thrown", "Excepted"]);
        assert_eq!(refs("/**\n * multi-line inline {@link\n * Split} here\n */"), ["Split"]);
        assert_eq!(refs("/** @param x the {@link Inline} to use */"), ["Inline"]);
        assert_eq!(refs("/** @return the {@link Value} */"), ["Value"]);
    }
}
