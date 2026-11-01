//! A reader for Javadoc comments that builds the tree Checkstyle's `AbstractJavadocCheck` hands to
//! its subclasses: `JAVADOC_CONTENT` with `NEWLINE`, `LEADING_ASTERISK`, `TEXT`, `HTML_ELEMENT`,
//! `JAVADOC_INLINE_TAG`, `JAVADOC_BLOCK_TAG` and the rest of `JavadocCommentsTokenTypes`.
//!
//! The shape was learned from Checkstyle's own tree dump (`checkstyle -j`) and is checked against it
//! comment by comment over the Guava and Spring sources (`bench/oracle/javadoc-tree/compare.sh`).
//! What it models:
//!
//! * Lines: `LEADING_ASTERISK(S)` is the first `*` run of a line after white space, which is hidden;
//!   a line without one keeps its white space as part of the `TEXT`.
//! * Block tags start a line (after the asterisk and white space). Their `DESCRIPTION` runs to the
//!   next block tag. A group (`DESCRIPTION`, `HTML_CONTENT`) never starts or ends with `NEWLINE`,
//!   `LEADING_ASTERISK` or a blank `TEXT`: those sit beside it.
//! * HTML: an element is *tight* when a matching end tag follows (nested elements may be loose
//!   inside it); otherwise it is *loose*, a start tag with the plain text up to the next element,
//!   comment, end tag or block tag. Void elements (`br`, `hr`, `img`...) and `<x/>` are
//!   `VOID_ELEMENT`s.
//! * Inline tags with their own productions: `code`, `literal`, `link`, `linkplain`, `value`,
//!   `summary`, `return`, `inheritDoc`, `index`, `systemProperty`, `snippet`, other names custom.
//!
//! Malformed comments (a stray end tag, an unclosed inline tag) are rejected before this runs, by
//! `javadoc_tags::parse`; here they are read as leniently as possible.

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Ty {
    JavadocContent,
    LeadingAsterisk,
    LeadingAsterisks,
    Newline,
    Text,
    JavadocBlockTag,
    AtSign,
    TagName,
    AuthorBlockTag,
    DeprecatedBlockTag,
    ExceptionBlockTag,
    HiddenBlockTag,
    ParamBlockTag,
    ProvidesBlockTag,
    ReturnBlockTag,
    SeeBlockTag,
    SerialBlockTag,
    SerialDataBlockTag,
    SerialFieldBlockTag,
    SinceBlockTag,
    ThrowsBlockTag,
    UsesBlockTag,
    VersionBlockTag,
    CustomBlockTag,
    JavadocInlineTag,
    JavadocInlineTagStart,
    JavadocInlineTagEnd,
    CodeInlineTag,
    LinkInlineTag,
    LinkplainInlineTag,
    ValueInlineTag,
    SummaryInlineTag,
    InheritDocInlineTag,
    SystemPropertyInlineTag,
    LiteralInlineTag,
    ReturnInlineTag,
    IndexInlineTag,
    SnippetInlineTag,
    CustomInlineTag,
    Identifier,
    Hash,
    Question,
    Lparen,
    Rparen,
    Comma,
    Reference,
    MemberReference,
    ParameterTypeList,
    ParameterType,
    TypeArguments,
    TypeArgument,
    Lt,
    Gt,
    Description,
    ParameterName,
    StringLiteral,
    FieldType,
    IndexTerm,
    Colon,
    SnippetBody,
    SnippetAttributes,
    SnippetAttribute,
    SnippetAttrName,
    HtmlElement,
    VoidElement,
    HtmlContent,
    HtmlAttribute,
    HtmlAttributes,
    HtmlTagStart,
    HtmlTagEnd,
    TagOpen,
    TagClose,
    TagSlashClose,
    TagSlash,
    TagAttrName,
    Equals,
    AttributeValue,
    HtmlComment,
    HtmlCommentStart,
    HtmlCommentEnd,
    HtmlCommentContent,
}

impl Ty {
    /// Checkstyle's name for the token type, as `checkstyle -j` prints it.
    pub fn name(self) -> &'static str {
        use Ty::*;
        match self {
            JavadocContent => "JAVADOC_CONTENT",
            LeadingAsterisk => "LEADING_ASTERISK",
            LeadingAsterisks => "LEADING_ASTERISKS",
            Newline => "NEWLINE",
            Text => "TEXT",
            JavadocBlockTag => "JAVADOC_BLOCK_TAG",
            AtSign => "AT_SIGN",
            TagName => "TAG_NAME",
            AuthorBlockTag => "AUTHOR_BLOCK_TAG",
            DeprecatedBlockTag => "DEPRECATED_BLOCK_TAG",
            ExceptionBlockTag => "EXCEPTION_BLOCK_TAG",
            HiddenBlockTag => "HIDDEN_BLOCK_TAG",
            ParamBlockTag => "PARAM_BLOCK_TAG",
            ProvidesBlockTag => "PROVIDES_BLOCK_TAG",
            ReturnBlockTag => "RETURN_BLOCK_TAG",
            SeeBlockTag => "SEE_BLOCK_TAG",
            SerialBlockTag => "SERIAL_BLOCK_TAG",
            SerialDataBlockTag => "SERIAL_DATA_BLOCK_TAG",
            SerialFieldBlockTag => "SERIAL_FIELD_BLOCK_TAG",
            SinceBlockTag => "SINCE_BLOCK_TAG",
            ThrowsBlockTag => "THROWS_BLOCK_TAG",
            UsesBlockTag => "USES_BLOCK_TAG",
            VersionBlockTag => "VERSION_BLOCK_TAG",
            CustomBlockTag => "CUSTOM_BLOCK_TAG",
            JavadocInlineTag => "JAVADOC_INLINE_TAG",
            JavadocInlineTagStart => "JAVADOC_INLINE_TAG_START",
            JavadocInlineTagEnd => "JAVADOC_INLINE_TAG_END",
            CodeInlineTag => "CODE_INLINE_TAG",
            LinkInlineTag => "LINK_INLINE_TAG",
            LinkplainInlineTag => "LINKPLAIN_INLINE_TAG",
            ValueInlineTag => "VALUE_INLINE_TAG",
            SummaryInlineTag => "SUMMARY_INLINE_TAG",
            InheritDocInlineTag => "INHERIT_DOC_INLINE_TAG",
            SystemPropertyInlineTag => "SYSTEM_PROPERTY_INLINE_TAG",
            LiteralInlineTag => "LITERAL_INLINE_TAG",
            ReturnInlineTag => "RETURN_INLINE_TAG",
            IndexInlineTag => "INDEX_INLINE_TAG",
            SnippetInlineTag => "SNIPPET_INLINE_TAG",
            CustomInlineTag => "CUSTOM_INLINE_TAG",
            Identifier => "IDENTIFIER",
            Hash => "HASH",
            Question => "QUESTION",
            Lparen => "LPAREN",
            Rparen => "RPAREN",
            Comma => "COMMA",
            Reference => "REFERENCE",
            MemberReference => "MEMBER_REFERENCE",
            ParameterTypeList => "PARAMETER_TYPE_LIST",
            ParameterType => "PARAMETER_TYPE",
            TypeArguments => "TYPE_ARGUMENTS",
            TypeArgument => "TYPE_ARGUMENT",
            Lt => "LT",
            Gt => "GT",
            Description => "DESCRIPTION",
            ParameterName => "PARAMETER_NAME",
            StringLiteral => "STRING_LITERAL",
            FieldType => "FIELD_TYPE",
            IndexTerm => "INDEX_TERM",
            Colon => "COLON",
            SnippetBody => "SNIPPET_BODY",
            SnippetAttributes => "SNIPPET_ATTRIBUTES",
            SnippetAttribute => "SNIPPET_ATTRIBUTE",
            SnippetAttrName => "SNIPPET_ATTR_NAME",
            HtmlElement => "HTML_ELEMENT",
            VoidElement => "VOID_ELEMENT",
            HtmlContent => "HTML_CONTENT",
            HtmlAttribute => "HTML_ATTRIBUTE",
            HtmlAttributes => "HTML_ATTRIBUTES",
            HtmlTagStart => "HTML_TAG_START",
            HtmlTagEnd => "HTML_TAG_END",
            TagOpen => "TAG_OPEN",
            TagClose => "TAG_CLOSE",
            TagSlashClose => "TAG_SLASH_CLOSE",
            TagSlash => "TAG_SLASH",
            TagAttrName => "TAG_ATTR_NAME",
            Equals => "EQUALS",
            AttributeValue => "ATTRIBUTE_VALUE",
            HtmlComment => "HTML_COMMENT",
            HtmlCommentStart => "HTML_COMMENT_START",
            HtmlCommentEnd => "HTML_COMMENT_END",
            HtmlCommentContent => "HTML_COMMENT_CONTENT",
        }
    }
}

pub type Id = usize;

#[derive(Debug, Clone)]
pub struct Node {
    pub ty: Ty,
    /// The token's text; for a node with children, its type name (as `DetailNode.getText()` gives).
    pub text: String,
    /// 1-based line in the file.
    pub line: u32,
    /// 0-based character column in that line of the file.
    pub col: u32,
    pub parent: Option<Id>,
    pub children: Vec<Id>,
    /// Index among the parent's children.
    pub index: usize,
    /// Character span in the comment content (for backtracking).
    start: usize,
    end: usize,
}

#[derive(Debug, Default)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub const ROOT: Id = 0;

    pub fn node(&self, id: Id) -> &Node {
        &self.nodes[id]
    }
    pub fn ty(&self, id: Id) -> Ty {
        self.nodes[id].ty
    }
    pub fn text(&self, id: Id) -> &str {
        &self.nodes[id].text
    }
    pub fn parent(&self, id: Id) -> Option<Id> {
        self.nodes[id].parent
    }
    pub fn first_child(&self, id: Id) -> Option<Id> {
        self.nodes[id].children.first().copied()
    }
    pub fn next_sibling(&self, id: Id) -> Option<Id> {
        let n = &self.nodes[id];
        n.parent.and_then(|p| self.nodes[p].children.get(n.index + 1).copied())
    }
    pub fn prev_sibling(&self, id: Id) -> Option<Id> {
        let n = &self.nodes[id];
        match (n.parent, n.index) {
            (Some(p), i) if i > 0 => self.nodes[p].children.get(i - 1).copied(),
            _ => None,
        }
    }
    /// `JavadocUtil.findFirstToken`: the first direct child of this type.
    pub fn find_first(&self, id: Id, ty: Ty) -> Option<Id> {
        self.nodes[id].children.iter().copied().find(|c| self.nodes[*c].ty == ty)
    }
    /// `JavadocUtil.isTag`: an element whose start tag is named `tag` (case-insensitively).
    pub fn is_tag(&self, id: Id, tag: &str) -> bool {
        self.find_first(id, Ty::HtmlTagStart).and_then(|s| self.find_first(s, Ty::TagName)).is_some_and(|n| self.nodes[n].text.eq_ignore_ascii_case(tag))
    }
    /// All node ids in document (pre-)order.
    pub fn preorder(&self) -> Vec<Id> {
        let mut out = Vec::with_capacity(self.nodes.len());
        let mut stack = vec![Tree::ROOT];
        while let Some(n) = stack.pop() {
            out.push(n);
            for c in self.nodes[n].children.iter().rev() {
                stack.push(*c);
            }
        }
        out
    }

    /// One line per node, `depth<TAB>TYPE<TAB>text<TAB>line`, the line counted from `first_line` as
    /// Checkstyle's dump does; text of a node with children is its type name.
    pub fn dump(&self, first_line: u32) -> String {
        let mut out = String::new();
        self.dump_node(Tree::ROOT, 0, first_line, &mut out);
        out
    }

    fn dump_node(&self, id: Id, depth: usize, first_line: u32, out: &mut String) {
        let n = &self.nodes[id];
        let text = if n.children.is_empty() && n.ty != Ty::JavadocContent { n.text.replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t") } else { n.ty.name().to_string() };
        out.push_str(&format!("{}\t{}\t{}\t{}\n", depth, n.ty.name(), text, n.line - first_line));
        for c in &n.children {
            self.dump_node(*c, depth + 1, first_line, out);
        }
    }
}

/// Java's `Character.isWhitespace`.
pub fn is_java_ws(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r' | '\u{1C}'..='\u{1F}') || (c.is_whitespace() && !matches!(c, '\u{00A0}' | '\u{2007}' | '\u{202F}'))
}

/// `CommonUtil.isBlank`.
pub fn is_blank(s: &str) -> bool {
    s.chars().all(is_java_ws)
}

/// Parse a Javadoc comment's content (the text between `/**` and `*/`). `line` and `col` are the
/// 1-based line and 0-based column in the file of the first character of `content`.
pub fn parse(content: &str, line: u32, col: u32) -> Tree {
    let s: Vec<char> = content.chars().collect();
    let mut line_starts = vec![0usize];
    for (i, c) in s.iter().enumerate() {
        if *c == '\n' {
            line_starts.push(i + 1);
        }
    }
    let mut p = P { s, pos: 0, nodes: Vec::new(), line_starts, base_line: line, base_col: col, tight_fail: HashSet::new(), inline_depth: 0 };
    // Node 0 is the root, filled in last.
    p.nodes.push(Node { ty: Ty::JavadocContent, text: Ty::JavadocContent.name().to_string(), line, col, parent: None, children: Vec::new(), index: 0, start: 0, end: 0 });
    let items = p.parse_seq(&Mode::Top);
    for (i, k) in items.iter().enumerate() {
        p.nodes[*k].parent = Some(0);
        p.nodes[*k].index = i;
    }
    if let Some(first) = items.first() {
        p.nodes[0].line = p.nodes[*first].line;
        p.nodes[0].col = p.nodes[*first].col;
    }
    p.nodes[0].children = items;
    Tree { nodes: p.nodes }
}

#[derive(Clone, Debug, PartialEq)]
enum Mode {
    Top,
    /// A block tag's description: ends at the next block tag.
    Desc,
    /// Inside an element that needs `</name>`.
    Tight(String),
    /// Inside an element without an end tag.
    Loose,
    /// Inside `{@...}`: ends at the closing brace.
    Brace,
}

struct P {
    s: Vec<char>,
    pos: usize,
    nodes: Vec<Node>,
    line_starts: Vec<usize>,
    base_line: u32,
    base_col: u32,
    tight_fail: HashSet<usize>,
    /// Depth of inline tags being read: no block tags inside.
    inline_depth: u32,
}

const VOID: [&str; 13] = ["br", "hr", "img", "input", "col", "area", "base", "link", "meta", "param", "frame", "basefont", "isindex"];

impl P {
    fn at(&self, i: usize) -> Option<char> {
        self.s.get(i).copied()
    }

    fn line_col(&self, p: usize) -> (u32, u32) {
        let li = self.line_starts.partition_point(|&st| st <= p) - 1;
        (self.base_line + li as u32, (p - self.line_starts[li]) as u32 + if li == 0 { self.base_col } else { 0 })
    }

    fn leaf(&mut self, ty: Ty, from: usize, to: usize) -> Id {
        let (line, col) = self.line_col(from);
        let text: String = self.s[from..to].iter().collect();
        self.nodes.push(Node { ty, text, line, col, parent: None, children: Vec::new(), index: 0, start: from, end: to });
        self.nodes.len() - 1
    }

    fn group(&mut self, ty: Ty, kids: Vec<Id>) -> Id {
        let (line, col, start) = match kids.first() {
            Some(k) => (self.nodes[*k].line, self.nodes[*k].col, self.nodes[*k].start),
            None => {
                let (l, c) = self.line_col(self.pos);
                (l, c, self.pos)
            }
        };
        let end = kids.last().map_or(start, |k| self.nodes[*k].end);
        let id = self.nodes.len();
        for (i, k) in kids.iter().enumerate() {
            self.nodes[*k].parent = Some(id);
            self.nodes[*k].index = i;
        }
        self.nodes.push(Node { ty, text: ty.name().to_string(), line, col, parent: None, children: kids, index: 0, start, end });
        id
    }

    /// Whether only white space and asterisks precede `p` on its line.
    fn fresh_at(&self, p: usize) -> bool {
        let mut i = p;
        while i > 0 {
            let c = self.s[i - 1];
            if c == '\n' {
                return true;
            }
            if c != ' ' && c != '\t' && c != '*' {
                return false;
            }
            i -= 1;
        }
        true
    }

    /// At `p` (a fresh position) is there white space and then a block tag's `@name`? Returns the
    /// position of the `@`.
    fn block_tag_at(&self, p: usize) -> Option<usize> {
        let mut i = p;
        while matches!(self.at(i), Some(' ' | '\t')) {
            i += 1;
        }
        (self.at(i) == Some('@') && self.at(i + 1).is_some_and(|c| c.is_alphanumeric())).then_some(i)
    }

    fn blank_node(&self, id: Id) -> bool {
        let n = &self.nodes[id];
        matches!(n.ty, Ty::Newline | Ty::LeadingAsterisk | Ty::LeadingAsterisks) || (n.ty == Ty::Text && is_blank(&n.text))
    }

    /// Split items into leading blanks, the middle, and trailing blanks. With only blanks they are all
    /// leading.
    fn split_blanks(&self, items: &[Id]) -> (usize, usize) {
        let lead = items.iter().take_while(|i| self.blank_node(**i)).count();
        if lead == items.len() {
            return (lead, lead);
        }
        let mut end = items.len();
        while end > lead && self.blank_node(items[end - 1]) {
            end -= 1;
        }
        (lead, end)
    }

    /// Give back items from `from` on: rewind to where the first one started.
    fn give_back(&mut self, items: &[Id]) {
        if let Some(first) = items.first() {
            self.pos = self.nodes[*first].start;
        }
    }

    // ---- sequences -------------------------------------------------------------------------

    fn parse_seq(&mut self, mode: &Mode) -> Vec<Id> {
        let mut items: Vec<Id> = Vec::new();
        let mut depth = 0usize; // plain braces in text, for Brace mode
        let n = self.s.len();
        'outer: while self.pos < n {
            let c = self.s[self.pos];
            let fresh = self.fresh_at(self.pos);
            // End of this sequence?
            if fresh && self.inline_depth == 0 {
                if let Some(at) = self.block_tag_at(self.pos) {
                    if *mode == Mode::Top {
                        if at > self.pos {
                            let t = self.leaf(Ty::Text, self.pos, at);
                            items.push(t);
                            self.pos = at;
                        }
                        let b = self.parse_block_tag();
                        items.push(b);
                        continue;
                    }
                    break;
                }
            }
            if c == '}' && self.inline_depth > 0 && depth == 0 {
                break;
            }
            if c == '<' {
                if self.at(self.pos + 1) == Some('/') && self.at(self.pos + 2).is_some_and(|x| x.is_ascii_alphabetic()) {
                    if let Some(tag_end) = self.end_tag_at(self.pos) {
                        if matches!(mode, Mode::Tight(_) | Mode::Loose) {
                            break;
                        }
                        // A stray end tag where nothing waits for it: text.
                        let t = self.leaf(Ty::Text, self.pos, tag_end);
                        items.push(t);
                        self.pos = tag_end;
                        continue 'outer;
                    }
                } else if self.starts_with("<!--") {
                    if *mode == Mode::Loose {
                        break;
                    }
                    if let Some(cm) = self.parse_comment() {
                        items.push(cm);
                        continue;
                    }
                } else if self.at(self.pos + 1).is_some_and(|x| x.is_ascii_alphabetic()) && self.tag_start_ok(self.pos) {
                    if *mode == Mode::Loose {
                        break;
                    }
                    let e = self.parse_element();
                    items.push(e);
                    continue;
                }
            }
            if c == '{' && self.at(self.pos + 1) == Some('@') {
                if let Some(t) = self.parse_inline_tag() {
                    items.push(t);
                    continue;
                }
            }
            if c == '\n' || (c == '\r' && self.at(self.pos + 1) == Some('\n')) {
                let len = if c == '\r' { 2 } else { 1 };
                let nl = self.leaf(Ty::Newline, self.pos, self.pos + len);
                self.pos += len;
                items.push(nl);
                continue;
            }
            // A line's leading asterisks, after white space.
            if fresh && self.line_start_prefix_ok() {
                let mut i = self.pos;
                while matches!(self.at(i), Some(' ' | '\t')) {
                    i += 1;
                }
                if self.at(i) == Some('*') {
                    let mut j = i;
                    while self.at(j) == Some('*') {
                        j += 1;
                    }
                    let ty = if j - i > 1 { Ty::LeadingAsterisks } else { Ty::LeadingAsterisk };
                    let la = self.leaf(ty, i, j);
                    self.pos = j;
                    items.push(la);
                    continue;
                }
            }
            // Text up to the next special character.
            let end = self.text_end(mode, &mut depth);
            if end == self.pos {
                // A '}' that closes nothing here, or a lone special: take it as text.
                let end = self.pos + 1;
                let t = self.leaf(Ty::Text, self.pos, end);
                self.pos = end;
                items.push(t);
                continue;
            }
            let t = self.leaf(Ty::Text, self.pos, end);
            self.pos = end;
            items.push(t);
        }
        items
    }

    /// Whether the line so far (before `self.pos`) holds only white space (no asterisk yet): a line
    /// has at most one run of leading asterisks.
    fn line_start_prefix_ok(&self) -> bool {
        let mut i = self.pos;
        while i > 0 {
            let c = self.s[i - 1];
            if c == '\n' {
                return true;
            }
            if c != ' ' && c != '\t' {
                return false;
            }
            i -= 1;
        }
        true
    }

    fn starts_with(&self, pat: &str) -> bool {
        pat.chars().enumerate().all(|(k, c)| self.at(self.pos + k) == Some(c))
    }

    /// End of the text run starting at `self.pos` (at least one character unless a terminator is first).
    fn text_end(&self, _mode: &Mode, depth: &mut usize) -> usize {
        let mut i = self.pos;
        let n = self.s.len();
        // Leading white space on a fresh line stays in the run, unless a block tag or markup follows.
        while i < n {
            let c = self.s[i];
            match c {
                '\n' => break,
                '\r' if self.at(i + 1) == Some('\n') => break,
                '<' if i > self.pos || true => {
                    let next = self.at(i + 1);
                    let markup = match next {
                        Some('/') => self.at(i + 2).is_some_and(|x| x.is_ascii_alphabetic()) && self.end_tag_at(i).is_some(),
                        Some('!') => self.s[i..].starts_with(&['<', '!', '-', '-']),
                        Some(x) if x.is_ascii_alphabetic() => self.tag_start_ok(i),
                        _ => false,
                    };
                    if markup {
                        break;
                    }
                }
                '{' => {
                    if self.at(i + 1) == Some('@') {
                        break;
                    }
                    if self.inline_depth > 0 {
                        *depth += 1;
                    }
                }
                '}' if self.inline_depth > 0 => {
                    if *depth == 0 {
                        break;
                    }
                    *depth -= 1;
                }
                _ => {}
            }
            // White space then a block tag on a fresh line: the white space is its own run.
            if i == self.pos && matches!(c, ' ' | '\t') && self.inline_depth == 0 && self.fresh_at(i) {
                if let Some(at) = self.block_tag_at(i) {
                    return at;
                }
            }
            i += 1;
        }
        i
    }

    // ---- HTML ------------------------------------------------------------------------------

    /// If a start tag begins at `p`, where it ends (after the `>`).
    fn tag_start_ok(&self, p: usize) -> bool {
        self.scan_tag_start(p).is_some()
    }

    /// Scan a start tag `<name attrs>` or `<name attrs/>`: the end position and whether it is self-closing.
    fn scan_tag_start(&self, p: usize) -> Option<(usize, bool)> {
        let mut i = p + 1;
        if !self.at(i)?.is_ascii_alphabetic() {
            return None;
        }
        while self.at(i).is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':' || c == '_') {
            i += 1;
        }
        // Attributes: anything up to '>' that is not another '<' outside a quoted value.
        while let Some(c) = self.at(i) {
            match c {
                '"' | '\'' => {
                    i = self.quoted_end(i);
                    continue;
                }
                '>' => {
                    let selfclose = self.at(i - 1) == Some('/') && i - 1 > p + 1;
                    return Some((i + 1, selfclose));
                }
                '<' => return None,
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// End (exclusive) of the quoted attribute value that opens at `i`: through the closing quote
    /// on the same line, else up to the next `>` or the end of the line.
    fn quoted_end(&self, i: usize) -> usize {
        let q = self.s[i];
        let mut j = i + 1;
        // The closing quote may be lines away, but not past an end tag.
        while self.at(j).is_some_and(|c| c != q) {
            if self.at(j) == Some('<') && self.at(j + 1) == Some('/') {
                break;
            }
            j += 1;
        }
        if self.at(j) == Some(q) {
            return j + 1;
        }
        let mut j = i + 1;
        while self.at(j).is_some_and(|c| c != '>' && c != '\n') {
            j += 1;
        }
        j
    }

    /// `</name>` at `p`: its end.
    fn end_tag_at(&self, p: usize) -> Option<usize> {
        let mut i = p + 2;
        while self.at(i).is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':' || c == '_') {
            i += 1;
        }
        if i == p + 2 {
            return None;
        }
        while matches!(self.at(i), Some(' ' | '\t')) {
            i += 1;
        }
        (self.at(i) == Some('>')).then_some(i + 1)
    }

    fn end_tag_name(&self, p: usize) -> String {
        let mut i = p + 2;
        let mut name = String::new();
        while let Some(c) = self.at(i).filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == ':' || *c == '_') {
            name.push(c.to_ascii_lowercase());
            i += 1;
        }
        name
    }

    fn parse_comment(&mut self) -> Option<Id> {
        let start = self.pos;
        // Find the end.
        let mut i = start + 4;
        let end = loop {
            if i + 2 >= self.s.len() {
                return None;
            }
            if self.s[i] == '-' && self.s[i + 1] == '-' && self.s[i + 2] == '>' {
                break i;
            }
            i += 1;
        };
        let open = self.leaf(Ty::HtmlCommentStart, start, start + 4);
        self.pos = start + 4;
        // The content is read as it stands, asterisks and all: text per line, and the line breaks.
        let mut items: Vec<Id> = Vec::new();
        while self.pos < end {
            if self.s[self.pos] == '\n' {
                let nl = self.leaf(Ty::Newline, self.pos, self.pos + 1);
                self.pos += 1;
                items.push(nl);
            } else {
                let mut j = self.pos;
                while j < end && self.s[j] != '\n' {
                    j += 1;
                }
                let t = self.leaf(Ty::Text, self.pos, j);
                self.pos = j;
                items.push(t);
            }
        }
        // Line breaks around the content belong to the comment itself.
        let is_nl = |p: &P, i: Id| p.nodes[i].ty == Ty::Newline;
        let lead = items.iter().take_while(|i| is_nl(self, **i)).count();
        let mut stop = items.len();
        while stop > lead && is_nl(self, items[stop - 1]) {
            stop -= 1;
        }
        let mut all = vec![open];
        all.extend_from_slice(&items[..lead]);
        if lead < stop {
            let content: Vec<Id> = items[lead..stop].to_vec();
            all.push(self.group(Ty::HtmlCommentContent, content));
        }
        all.extend_from_slice(&items[stop.max(lead)..]);
        let close = self.leaf(Ty::HtmlCommentEnd, end, end + 3);
        self.pos = end + 3;
        all.push(close);
        Some(self.group(Ty::HtmlComment, all))
    }

    /// Text, newlines and leading asterisks between `self.pos` and `end` (exclusive), as raw tokens.
    fn raw_lines(&mut self, end: usize) -> Vec<Id> {
        let mut kids = Vec::new();
        while self.pos < end {
            let c = self.s[self.pos];
            if c == '\n' {
                let nl = self.leaf(Ty::Newline, self.pos, self.pos + 1);
                self.pos += 1;
                kids.push(nl);
                continue;
            }
            if self.fresh_at(self.pos) && self.line_start_prefix_ok() {
                let mut i = self.pos;
                while matches!(self.at(i), Some(' ' | '\t')) {
                    i += 1;
                }
                if i < end && self.at(i) == Some('*') {
                    let mut j = i;
                    while j < end && self.at(j) == Some('*') {
                        j += 1;
                    }
                    let ty = if j - i > 1 { Ty::LeadingAsterisks } else { Ty::LeadingAsterisk };
                    let la = self.leaf(ty, i, j);
                    self.pos = j;
                    kids.push(la);
                    continue;
                }
            }
            let mut j = self.pos;
            while j < end && self.s[j] != '\n' {
                j += 1;
            }
            let t = self.leaf(Ty::Text, self.pos, j);
            self.pos = j;
            kids.push(t);
        }
        kids
    }

    /// An element starting at `self.pos`, which is a valid start tag.
    fn parse_element(&mut self) -> Id {
        let start = self.pos;
        let name = self.start_tag_name(start);
        let saved_len = self.nodes.len();
        // Read the start tag once to learn whether it closes itself.
        let (kids, selfclose) = self.parse_start_tag();
        if selfclose || VOID.contains(&name.as_str()) {
            let ve = if selfclose {
                self.group(Ty::VoidElement, kids)
            } else {
                let st = self.group(Ty::HtmlTagStart, kids);
                self.group(Ty::VoidElement, vec![st])
            };
            return self.group(Ty::HtmlElement, vec![ve]);
        }
        self.nodes.truncate(saved_len);
        self.pos = start;
        if !self.tight_fail.contains(&start) {
            if let Some(el) = self.try_tight(&name) {
                return el;
            }
            self.tight_fail.insert(start);
            self.nodes.truncate(saved_len);
            self.pos = start;
        }
        // Loose.
        let (kids, _) = self.parse_start_tag();
        let st = self.group(Ty::HtmlTagStart, kids);
        let items = self.parse_seq(&Mode::Loose);
        let (lead, end) = self.split_blanks(&items);
        let mut el = vec![st];
        el.extend_from_slice(&items[..lead]);
        if lead < end {
            let content: Vec<Id> = items[lead..end].to_vec();
            el.push(self.group(Ty::HtmlContent, content));
            self.give_back(&items[end..]);
        } else {
            // All blank: nothing belongs to the element.
            el.truncate(1);
            self.give_back(&items);
        }
        self.group(Ty::HtmlElement, el)
    }

    fn start_tag_name(&self, p: usize) -> String {
        let mut i = p + 1;
        let mut name = String::new();
        while let Some(c) = self.at(i).filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == ':' || *c == '_') {
            name.push(c.to_ascii_lowercase());
            i += 1;
        }
        name
    }

    fn try_tight(&mut self, name: &str) -> Option<Id> {
        let (kids, _) = self.parse_start_tag();
        let st = self.group(Ty::HtmlTagStart, kids);
        let items = self.parse_seq(&Mode::Tight(name.to_string()));
        // Must stand at the matching end tag.
        if self.at(self.pos) != Some('<') || self.at(self.pos + 1) != Some('/') {
            return None;
        }
        let end = self.end_tag_at(self.pos)?;
        if self.end_tag_name(self.pos) != name {
            return None;
        }
        let (lead, mid_end) = self.split_blanks(&items);
        let mut el = vec![st];
        el.extend_from_slice(&items[..lead]);
        if lead < mid_end {
            let content: Vec<Id> = items[lead..mid_end].to_vec();
            el.push(self.group(Ty::HtmlContent, content));
            el.extend_from_slice(&items[mid_end..]);
        }
        let es = self.pos;
        let open = self.leaf(Ty::TagOpen, es, es + 1);
        let slash = self.leaf(Ty::TagSlash, es + 1, es + 2);
        let mut i = es + 2;
        while self.at(i).is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':' || c == '_') {
            i += 1;
        }
        let nm = self.leaf(Ty::TagName, es + 2, i);
        let mut kids = vec![open, slash, nm];
        if i < end - 1 {
            let t = self.leaf(Ty::Text, i, end - 1);
            kids.push(t);
        }
        let close = self.leaf(Ty::TagClose, end - 1, end);
        kids.push(close);
        let te = self.group(Ty::HtmlTagEnd, kids);
        el.push(te);
        self.pos = end;
        Some(self.group(Ty::HtmlElement, el))
    }

    /// Parse `<name attrs>` or `<name attrs/>` at `self.pos`: the child tokens.
    fn parse_start_tag(&mut self) -> (Vec<Id>, bool) {
        let p = self.pos;
        let open = self.leaf(Ty::TagOpen, p, p + 1);
        let mut i = p + 1;
        while self.at(i).is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':' || c == '_') {
            i += 1;
        }
        let nm = self.leaf(Ty::TagName, p + 1, i);
        let mut kids = vec![open, nm];
        self.pos = i;
        let mut attrs: Vec<Id> = Vec::new();
        // Attributes.
        loop {
            let mut pend: Vec<Id> = Vec::new();
            let save = self.pos;
            // Optional newline + asterisk, then white space.
            self.skip_attr_gap(&mut pend);
            match self.at(self.pos) {
                Some('>') => {
                    self.pos = save;
                    self.drop_nodes_from_pend(&pend);
                    break;
                }
                Some('/') if self.at(self.pos + 1) == Some('>') => {
                    self.pos = save;
                    self.drop_nodes_from_pend(&pend);
                    break;
                }
                Some(c) if c.is_alphabetic() || c == '_' || c == ':' || c == '-' || c == '@' => {
                    let a0 = self.pos;
                    let mut j = a0;
                    while self.at(j).is_some_and(|c| !c.is_whitespace() && c != '=' && c != '>' && c != '/' && c != '<') {
                        j += 1;
                    }
                    if j == a0 {
                        j += 1;
                    }
                    let an = self.leaf(Ty::TagAttrName, a0, j);
                    pend.push(an);
                    self.pos = j;
                    // '=' and the value, perhaps after a gap.
                    let save2 = self.pos;
                    let mut gap: Vec<Id> = Vec::new();
                    self.skip_attr_gap(&mut gap);
                    if self.at(self.pos) == Some('=') {
                        let eq = self.leaf(Ty::Equals, self.pos, self.pos + 1);
                        pend.extend(gap);
                        pend.push(eq);
                        self.pos += 1;
                        // After a line break the value keeps its leading white space.
                        let mut gap2: Vec<Id> = Vec::new();
                        if self.at(self.pos) == Some('\n') {
                            self.skip_newline_only(&mut gap2);
                        }
                        pend.extend(gap2);
                        let v0 = self.pos;
                        let mut vs = v0;
                        while matches!(self.at(vs), Some(' ' | '\t')) {
                            vs += 1;
                        }
                        let mut j = vs;
                        match self.at(vs) {
                            Some('"' | '\'') => {
                                j = self.quoted_end(vs);
                            }
                            _ => {
                                while self.at(j).is_some_and(|c| !c.is_whitespace() && c != '>') {
                                    j += 1;
                                }
                                // The white space after an unquoted value belongs to it.
                                if self.at(j).is_some_and(|c| c == ' ' || c == '\t') {
                                    j += 1;
                                }
                            }
                        }
                        let v = self.leaf(Ty::AttributeValue, v0, j);
                        pend.push(v);
                        self.pos = j;
                    } else {
                        self.nodes.truncate(self.nodes.len() - gap.len());
                        self.pos = save2;
                    }
                    let at = self.group(Ty::HtmlAttribute, pend);
                    attrs.push(at);
                }
                _ => {
                    self.pos = save;
                    self.drop_nodes_from_pend(&pend);
                    break;
                }
            }
        }
        if !attrs.is_empty() {
            let a = self.group(Ty::HtmlAttributes, attrs);
            kids.push(a);
        }
        // Trailing gap before '>'; before '/>' it is not a token at all.
        let mark = self.nodes.len();
        let mut gap: Vec<Id> = Vec::new();
        self.skip_attr_gap(&mut gap);
        let selfclose = self.at(self.pos) == Some('/') && self.at(self.pos + 1) == Some('>');
        if selfclose {
            self.nodes.truncate(mark);
        } else {
            kids.extend(gap);
        }
        if selfclose {
            let c = self.leaf(Ty::TagSlashClose, self.pos, self.pos + 2);
            self.pos += 2;
            kids.push(c);
        } else {
            // Skip to '>' defensively.
            while self.at(self.pos).is_some_and(|c| c != '>') {
                self.pos += 1;
            }
            let c = self.leaf(Ty::TagClose, self.pos, self.pos + 1);
            self.pos += 1;
            kids.push(c);
        }
        (kids, selfclose)
    }

    fn drop_nodes_from_pend(&mut self, pend: &[Id]) {
        // Pending nodes were allocated last; remove them.
        if let Some(first) = pend.iter().min() {
            self.nodes.truncate(*first);
        }
    }

    /// A line break and the asterisk that starts the next line, nothing more.
    fn skip_newline_only(&mut self, out: &mut Vec<Id>) {
        let nl = self.leaf(Ty::Newline, self.pos, self.pos + 1);
        self.pos += 1;
        out.push(nl);
        let mut i = self.pos;
        while matches!(self.at(i), Some(' ' | '\t')) {
            i += 1;
        }
        if self.at(i) == Some('*') {
            let mut j = i;
            while self.at(j) == Some('*') {
                j += 1;
            }
            let ty = if j - i > 1 { Ty::LeadingAsterisks } else { Ty::LeadingAsterisk };
            let la = self.leaf(ty, i, j);
            self.pos = j;
            out.push(la);
        }
    }

    /// White space, possibly across a line break with its asterisk: NEWLINE, LEADING_ASTERISK, TEXT.
    fn skip_attr_gap(&mut self, out: &mut Vec<Id>) {
        loop {
            match self.at(self.pos) {
                Some('\n') => {
                    let nl = self.leaf(Ty::Newline, self.pos, self.pos + 1);
                    self.pos += 1;
                    out.push(nl);
                    // The asterisk of the next line.
                    let mut i = self.pos;
                    while matches!(self.at(i), Some(' ' | '\t')) {
                        i += 1;
                    }
                    if self.at(i) == Some('*') {
                        let mut j = i;
                        while self.at(j) == Some('*') {
                            j += 1;
                        }
                        let ty = if j - i > 1 { Ty::LeadingAsterisks } else { Ty::LeadingAsterisk };
                        let la = self.leaf(ty, i, j);
                        self.pos = j;
                        out.push(la);
                    }
                }
                Some(' ' | '\t') => {
                    let mut j = self.pos;
                    while matches!(self.at(j), Some(' ' | '\t')) {
                        j += 1;
                    }
                    let t = self.leaf(Ty::Text, self.pos, j);
                    self.pos = j;
                    out.push(t);
                }
                _ => break,
            }
        }
    }

    // ---- block tags ------------------------------------------------------------------------

    fn parse_block_tag(&mut self) -> Id {
        let at = self.pos;
        let a = self.leaf(Ty::AtSign, at, at + 1);
        let mut j = at + 1;
        while self.at(j).is_some_and(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '$')) {
            j += 1;
        }
        let nm = self.leaf(Ty::TagName, at + 1, j);
        let name: String = self.s[at + 1..j].iter().collect();
        self.pos = j;
        let ty = match name.as_str() {
            "author" => Ty::AuthorBlockTag,
            "deprecated" => Ty::DeprecatedBlockTag,
            "exception" => Ty::ExceptionBlockTag,
            "hidden" => Ty::HiddenBlockTag,
            "param" => Ty::ParamBlockTag,
            "provides" => Ty::ProvidesBlockTag,
            "return" => Ty::ReturnBlockTag,
            "see" => Ty::SeeBlockTag,
            "serial" => Ty::SerialBlockTag,
            "serialData" => Ty::SerialDataBlockTag,
            "serialField" => Ty::SerialFieldBlockTag,
            "since" => Ty::SinceBlockTag,
            "throws" => Ty::ThrowsBlockTag,
            "uses" => Ty::UsesBlockTag,
            "version" => Ty::VersionBlockTag,
            _ => Ty::CustomBlockTag,
        };
        let mut kids = vec![a, nm];
        let mark = (self.pos, self.nodes.len());
        match ty {
            Ty::ParamBlockTag => {
                self.take_gap(&mut kids);
                if self.at(self.pos).is_some() && !matches!(self.at(self.pos), Some('\n')) {
                    let p0 = self.pos;
                    let mut e = p0;
                    while self.at(e).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '$' | '<' | '>')) {
                        e += 1;
                    }
                    if e > p0 {
                        let pn = self.leaf(Ty::ParameterName, p0, e);
                        kids.push(pn);
                        self.pos = e;
                    }
                }
            }
            Ty::ThrowsBlockTag | Ty::ExceptionBlockTag | Ty::ProvidesBlockTag | Ty::UsesBlockTag => {
                self.take_gap(&mut kids);
                let p0 = self.pos;
                let mut e = p0;
                while self.at(e).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '$' | '.')) {
                    e += 1;
                }
                if e > p0 {
                    let id = self.leaf(Ty::Identifier, p0, e);
                    kids.push(id);
                    self.pos = e;
                }
            }
            Ty::SerialFieldBlockTag => {
                self.take_gap(&mut kids);
                let p0 = self.pos;
                let mut e = p0;
                while self.at(e).is_some_and(|c| !c.is_whitespace()) {
                    e += 1;
                }
                if e > p0 {
                    let id = self.leaf(Ty::Identifier, p0, e);
                    kids.push(id);
                    self.pos = e;
                    self.take_gap(&mut kids);
                    let p1 = self.pos;
                    let mut e = p1;
                    while self.at(e).is_some_and(|c| !c.is_whitespace()) {
                        e += 1;
                    }
                    if e > p1 {
                        let id = self.leaf(Ty::FieldType, p1, e);
                        kids.push(id);
                        self.pos = e;
                    }
                }
            }
            Ty::SeeBlockTag => {
                self.take_gap(&mut kids);
                match self.at(self.pos) {
                    Some('"') => {
                        let p0 = self.pos;
                        let mut e = p0 + 1;
                        while self.at(e).is_some_and(|c| c != '"' && c != '\n') {
                            e += 1;
                        }
                        if self.at(e) == Some('"') {
                            e += 1;
                        }
                        let sl = self.leaf(Ty::StringLiteral, p0, e);
                        kids.push(sl);
                        self.pos = e;
                    }
                    Some('<') if self.at(self.pos + 1).is_some_and(|c| c.is_ascii_alphabetic()) && self.tag_start_ok(self.pos) => {
                        let el = self.parse_element();
                        kids.push(el);
                    }
                    Some(c) if c != '\n' => {
                        if let Some(r) = self.parse_reference(false) {
                            kids.push(r);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        // A tag that wants an argument and found none leaves its white space to the description.
        let takes_arg = matches!(ty, Ty::ParamBlockTag | Ty::ThrowsBlockTag | Ty::ExceptionBlockTag | Ty::ProvidesBlockTag | Ty::UsesBlockTag | Ty::SerialFieldBlockTag | Ty::SeeBlockTag);
        let has_arg = kids.iter().any(|k| !matches!(self.nodes[*k].ty, Ty::AtSign | Ty::TagName | Ty::Text | Ty::Newline | Ty::LeadingAsterisk | Ty::LeadingAsterisks));
        if takes_arg && !has_arg {
            kids.truncate(2);
            self.nodes.truncate(mark.1);
            self.pos = mark.0;
        }
        // The rest is the description.
        let items = self.parse_seq(&Mode::Desc);
        let (lead, end) = self.split_blanks(&items);
        kids.extend_from_slice(&items[..lead]);
        if lead < end {
            let d: Vec<Id> = items[lead..end].to_vec();
            kids.push(self.group(Ty::Description, d));
            self.give_back(&items[end..]);
        } else {
            kids.truncate(kids.len() - lead);
            self.give_back(&items);
        }
        let tag = self.group(ty, kids);
        self.group(Ty::JavadocBlockTag, vec![tag])
    }

    /// Blank tokens (white space, a line break and its asterisk) after a tag name, into `kids`. Stops
    /// before a block tag or the end.
    fn take_gap(&mut self, kids: &mut Vec<Id>) {
        let save = self.pos;
        let save_len = self.nodes.len();
        let mut gap = Vec::new();
        loop {
            match self.at(self.pos) {
                Some(' ' | '\t') => {
                    let mut j = self.pos;
                    while matches!(self.at(j), Some(' ' | '\t')) {
                        j += 1;
                    }
                    let t = self.leaf(Ty::Text, self.pos, j);
                    self.pos = j;
                    gap.push(t);
                }
                Some('\n') => {
                    // Only continue onto the next line if it is not a block tag.
                    let nl = self.leaf(Ty::Newline, self.pos, self.pos + 1);
                    self.pos += 1;
                    gap.push(nl);
                    let mut i = self.pos;
                    while matches!(self.at(i), Some(' ' | '\t')) {
                        i += 1;
                    }
                    if self.at(i) == Some('*') {
                        let mut j = i;
                        while self.at(j) == Some('*') {
                            j += 1;
                        }
                        let ty = if j - i > 1 { Ty::LeadingAsterisks } else { Ty::LeadingAsterisk };
                        let la = self.leaf(ty, i, j);
                        self.pos = j;
                        gap.push(la);
                    }
                    if self.block_tag_at(self.pos).is_some() {
                        self.nodes.truncate(save_len);
                        self.pos = save;
                        return;
                    }
                }
                _ => break,
            }
        }
        kids.extend(gap);
    }

    // ---- inline tags -----------------------------------------------------------------------

    fn parse_inline_tag(&mut self) -> Option<Id> {
        let at = self.pos;
        let mut j = at + 2;
        while self.at(j).is_some_and(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '$')) {
            j += 1;
        }
        let name: String = self.s[at + 2..j].iter().collect();
        if name.is_empty() {
            return None;
        }
        // Must have a closing brace somewhere.
        let save_len = self.nodes.len();
        let start = self.leaf(Ty::JavadocInlineTagStart, at, at + 2);
        self.pos = at + 2;
        self.inline_depth += 1;
        let kids = self.inline_body(&name, start, j);
        self.inline_depth -= 1;
        match kids {
            Some((ty, kids)) => {
                let t = self.group(ty, kids);
                Some(self.group(Ty::JavadocInlineTag, vec![t]))
            }
            None => {
                self.nodes.truncate(save_len);
                self.pos = at;
                None
            }
        }
    }

    fn close_brace(&mut self, kids: &mut Vec<Id>) -> bool {
        if self.at(self.pos) == Some('}') {
            let e = self.leaf(Ty::JavadocInlineTagEnd, self.pos, self.pos + 1);
            self.pos += 1;
            kids.push(e);
            true
        } else {
            false
        }
    }

    /// Everything after `{@` up to and including `}`.
    fn inline_body(&mut self, name: &str, start: Id, name_end: usize) -> Option<(Ty, Vec<Id>)> {
        let nm = self.leaf(Ty::TagName, self.pos, name_end);
        self.pos = name_end;
        let mut kids = vec![start];
        if name != "snippet" {
            kids.push(nm);
        }
        let ty = match name {
            "code" | "literal" => {
                let ty = if name == "code" { Ty::CodeInlineTag } else { Ty::LiteralInlineTag };
                let end = self.brace_end()?;
                let raw = self.raw_lines(end);
                kids.extend(raw);
                ty
            }
            "snippet" => {
                // `{@snippet attrs :` body `}`.
                let end = self.brace_end()?;
                let mut attrs: Vec<Id> = Vec::new();
                loop {
                    let save = self.pos;
                    let save_len = self.nodes.len();
                    let mut pend: Vec<Id> = Vec::new();
                    self.skip_attr_gap(&mut pend);
                    let a0 = self.pos;
                    let mut j = a0;
                    while self.at(j).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '-')) {
                        j += 1;
                    }
                    if j == a0 {
                        self.nodes.truncate(save_len);
                        self.pos = save;
                        break;
                    }
                    let an = self.leaf(Ty::SnippetAttrName, a0, j);
                    pend.push(an);
                    self.pos = j;
                    if self.at(j) == Some('=') {
                        let eq = self.leaf(Ty::Equals, j, j + 1);
                        pend.push(eq);
                        let v0 = j + 1;
                        let mut e = v0;
                        if matches!(self.at(v0), Some('"' | '\'')) {
                            e = self.quoted_end(v0);
                        } else {
                            while self.at(e).is_some_and(|c| !c.is_whitespace() && c != ':' && c != '}') {
                                e += 1;
                            }
                            if matches!(self.at(e), Some(' ' | '\t')) {
                                e += 1;
                            }
                        }
                        let v = self.leaf(Ty::AttributeValue, v0, e);
                        pend.push(v);
                        self.pos = e;
                    }
                    attrs.push(self.group(Ty::SnippetAttribute, pend));
                }
                if !attrs.is_empty() {
                    kids.push(self.group(Ty::SnippetAttributes, attrs));
                }
                let mut i = self.pos;
                while matches!(self.at(i), Some(' ' | '\t')) {
                    i += 1;
                }
                if self.at(i) == Some(':') {
                    let colon = self.leaf(Ty::Colon, i, i + 1);
                    self.pos = i + 1;
                    kids.push(colon);
                    let body = self.raw_lines(end);
                    if !body.is_empty() {
                        let b = self.group(Ty::SnippetBody, body);
                        kids.push(b);
                    }
                } else {
                    let raw = self.raw_lines(end);
                    kids.extend(raw);
                }
                Ty::SnippetInlineTag
            }
            "link" | "linkplain" => {
                let ty = if name == "link" { Ty::LinkInlineTag } else { Ty::LinkplainInlineTag };
                self.take_gap(&mut kids);
                if let Some(r) = self.parse_reference(true) {
                    kids.push(r);
                }
                self.inline_description(&mut kids);
                ty
            }
            "value" => {
                self.take_gap(&mut kids);
                if self.at(self.pos) != Some('}') {
                    if let Some(r) = self.parse_reference(true) {
                        kids.push(r);
                    }
                }
                self.take_gap(&mut kids);
                Ty::ValueInlineTag
            }
            "inheritDoc" => {
                self.take_gap(&mut kids);
                if self.at(self.pos) != Some('}') {
                    if let Some(r) = self.parse_reference(true) {
                        kids.push(r);
                    }
                }
                self.take_gap(&mut kids);
                Ty::InheritDocInlineTag
            }
            "index" => {
                self.take_gap(&mut kids);
                let p0 = self.pos;
                let mut e = p0;
                if self.at(p0) == Some('"') {
                    e += 1;
                    while self.at(e).is_some_and(|c| c != '"' && c != '\n' && c != '}') {
                        e += 1;
                    }
                    if self.at(e) == Some('"') {
                        e += 1;
                    }
                } else {
                    while self.at(e).is_some_and(|c| !c.is_whitespace() && c != '}') {
                        e += 1;
                    }
                }
                if e > p0 {
                    let t = self.leaf(Ty::IndexTerm, p0, e);
                    kids.push(t);
                    self.pos = e;
                }
                self.inline_description(&mut kids);
                Ty::IndexInlineTag
            }
            "systemProperty" => {
                self.take_gap(&mut kids);
                let p0 = self.pos;
                let mut e = p0;
                while self.at(e).is_some_and(|c| !c.is_whitespace() && c != '}') {
                    e += 1;
                }
                if e > p0 {
                    let t = self.leaf(Ty::Identifier, p0, e);
                    kids.push(t);
                    self.pos = e;
                }
                self.take_gap(&mut kids);
                Ty::SystemPropertyInlineTag
            }
            _ => {
                let ty = match name {
                    "summary" => Ty::SummaryInlineTag,
                    "return" => Ty::ReturnInlineTag,
                    _ => Ty::CustomInlineTag,
                };
                self.inline_description(&mut kids);
                ty
            }
        };
        if !self.close_brace(&mut kids) {
            return None;
        }
        Some((ty, kids))
    }

    /// The position of the `}` that closes the inline tag whose body starts at `self.pos`.
    fn brace_end(&self) -> Option<usize> {
        let mut depth = 0usize;
        let mut i = self.pos;
        while let Some(c) = self.at(i) {
            match c {
                '{' => depth += 1,
                '}' => {
                    if depth == 0 {
                        return Some(i);
                    }
                    depth -= 1;
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Items up to the closing brace: line breaks and asterisks around them are direct children; the
    /// rest, white space included, is the DESCRIPTION.
    fn inline_description(&mut self, kids: &mut Vec<Id>) {
        let items = self.parse_seq(&Mode::Brace);
        let brk = |p: &P, i: Id| matches!(p.nodes[i].ty, Ty::Newline | Ty::LeadingAsterisk | Ty::LeadingAsterisks);
        let lead = items.iter().take_while(|i| brk(self, **i)).count();
        let mut end = items.len();
        while end > lead && brk(self, items[end - 1]) {
            end -= 1;
        }
        kids.extend_from_slice(&items[..lead]);
        if lead < end {
            let d: Vec<Id> = items[lead..end].to_vec();
            kids.push(self.group(Ty::Description, d));
        }
        kids.extend_from_slice(&items[end.max(lead)..]);
    }

    /// `Class#member(Type, Type)`, `#member`, `pkg.Class`, `member(Type)`. In an inline tag it ends at
    /// white space or `}`.
    fn parse_reference(&mut self, _inline: bool) -> Option<Id> {
        let p0 = self.pos;
        let mut kids: Vec<Id> = Vec::new();
        let ident = |c: char| c.is_alphanumeric() || c == '_' || c == '$' || c == '.' || c == '/';
        let mut i = p0;
        while self.at(i).is_some_and(ident) {
            i += 1;
        }
        if i > p0 && self.at(i) == Some('(') {
            // A member without a class: `getIndex(Class[])`.
            let mut mk = Vec::new();
            let id = self.leaf(Ty::Identifier, p0, i);
            mk.push(id);
            self.pos = i;
            self.parse_member_args(&mut mk);
            let m = self.group(Ty::MemberReference, mk);
            kids.push(m);
            return Some(self.group(Ty::Reference, kids));
        }
        if i > p0 {
            let id = self.leaf(Ty::Identifier, p0, i);
            kids.push(id);
            self.pos = i;
        }
        if self.at(self.pos) == Some('<') && !kids.is_empty() {
            if let Some(ta) = self.parse_type_arguments() {
                kids.push(ta);
                let i0 = self.pos;
                let mut i1 = i0;
                while self.at(i1).is_some_and(ident) {
                    i1 += 1;
                }
                if i1 > i0 {
                    let id = self.leaf(Ty::Identifier, i0, i1);
                    kids.push(id);
                    self.pos = i1;
                }
            }
        }
        if self.at(self.pos) == Some('#') {
            let h = self.leaf(Ty::Hash, self.pos, self.pos + 1);
            kids.push(h);
            self.pos += 1;
            // `Class##anchor`: a plain identifier after the hashes, no member reference.
            if self.at(self.pos) == Some('#') {
                while self.at(self.pos) == Some('#') {
                    let h = self.leaf(Ty::Hash, self.pos, self.pos + 1);
                    kids.push(h);
                    self.pos += 1;
                }
                let a0 = self.pos;
                let mut j = a0;
                while self.at(j).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '$' | '-' | '.' | ':')) {
                    j += 1;
                }
                if j > a0 {
                    let id = self.leaf(Ty::Identifier, a0, j);
                    kids.push(id);
                    self.pos = j;
                }
                return Some(self.group(Ty::Reference, kids));
            }
            let m0 = self.pos;
            let mut j = m0;
            while self.at(j).is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$') {
                j += 1;
            }
            let mut mk = Vec::new();
            if j > m0 {
                let id = self.leaf(Ty::Identifier, m0, j);
                mk.push(id);
                self.pos = j;
            }
            if self.at(self.pos) == Some('(') {
                self.parse_member_args(&mut mk);
            }
            if !mk.is_empty() {
                let m = self.group(Ty::MemberReference, mk);
                kids.push(m);
            }
        }
        if kids.is_empty() {
            return None;
        }
        Some(self.group(Ty::Reference, kids))
    }

    /// `<A, B>` after a class name.
    fn parse_type_arguments(&mut self) -> Option<Id> {
        let save_len = self.nodes.len();
        let save = self.pos;
        let mut kids = Vec::new();
        let lt = self.leaf(Ty::Lt, self.pos, self.pos + 1);
        kids.push(lt);
        self.pos += 1;
        loop {
            let a0 = self.pos;
            let mut j = a0;
            if self.at(a0) == Some('?') {
                j += 1;
            } else {
                while self.at(j).is_some_and(|c| c.is_alphanumeric() || matches!(c, '_' | '$' | '.')) {
                    j += 1;
                }
            }
            if j == a0 {
                break;
            }
            let id = self.leaf(if self.at(a0) == Some('?') { Ty::Question } else { Ty::Identifier }, a0, j);
            self.pos = j;
            let arg = self.group(Ty::TypeArgument, vec![id]);
            kids.push(arg);
            if self.at(self.pos) == Some(',') {
                let c = self.leaf(Ty::Comma, self.pos, self.pos + 1);
                self.pos += 1;
                kids.push(c);
                self.skip_attr_gap(&mut kids);
                continue;
            }
            break;
        }
        if self.at(self.pos) == Some('>') {
            let gt = self.leaf(Ty::Gt, self.pos, self.pos + 1);
            self.pos += 1;
            kids.push(gt);
            return Some(self.group(Ty::TypeArguments, kids));
        }
        self.nodes.truncate(save_len);
        self.pos = save;
        None
    }

    /// `(Type, Type)` of a member reference, from the opening parenthesis.
    fn parse_member_args(&mut self, mk: &mut Vec<Id>) {
        let lp = self.leaf(Ty::Lparen, self.pos, self.pos + 1);
        mk.push(lp);
        self.pos += 1;
        self.skip_attr_gap(mk);
        let mut list: Vec<Id> = Vec::new();
        let mut trailing: Vec<Id> = Vec::new();
        loop {
            let t0 = self.pos;
            let mut e = t0;
            while self.at(e).is_some_and(|c| !c.is_whitespace() && !matches!(c, ',' | ')' | '}')) {
                e += 1;
            }
            if e == t0 {
                break;
            }
            let pt = self.leaf(Ty::ParameterType, t0, e);
            list.push(pt);
            self.pos = e;
            let mut gap = Vec::new();
            self.skip_attr_gap(&mut gap);
            if self.at(self.pos) == Some(',') {
                list.extend(gap);
                let c = self.leaf(Ty::Comma, self.pos, self.pos + 1);
                self.pos += 1;
                list.push(c);
                let mut g2 = Vec::new();
                self.skip_attr_gap(&mut g2);
                list.extend(g2);
                continue;
            }
            trailing = gap;
            break;
        }
        if !list.is_empty() {
            let l = self.group(Ty::ParameterTypeList, list);
            mk.push(l);
        }
        mk.extend(trailing);
        if self.at(self.pos) == Some(')') {
            let rp = self.leaf(Ty::Rparen, self.pos, self.pos + 1);
            self.pos += 1;
            mk.push(rp);
        }
    }
}
