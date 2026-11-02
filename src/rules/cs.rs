// SPDX-License-Identifier: LGPL-2.1-or-later
//
// Contains helpers translated to Rust from Checkstyle 14.4.0 (https://github.com/checkstyle/checkstyle):
//   src/main/java/com/puppycrawl/tools/checkstyle/utils/CommonUtil.java, utils/TokenUtil.java and utils/CheckUtil.java
// Checkstyle: Copyright (C) 2001-2026 the original author or authors, licensed under the GNU
// Lesser General Public License, version 2.1 or (at your option) any later version.
// Translated, generated and adapted for javaid in November 2026; javaid is licensed under the
// same terms (COPYING.LESSER).

//! Support for modules ported from Checkstyle's own Java source onto the emulated `DetailAST`
//! (`crate::csast`). They run once per file, after the tree-sitter walk, on a tree built from the
//! same parse.

use crate::config::Config;
use crate::csast::{build_checked, CsAst, Tok, N};
use crate::engine::Diagnostic;
use tree_sitter::Tree;

/// What a ported check logs through: the file's lines (to expand tabs) and the output.
pub struct Log<'a> {
    cfg: &'a Config,
    lines: Vec<&'a [u8]>,
    out: &'a mut Vec<Diagnostic>,
    /// Checkstyle keeps its violations in a sorted set, so an identical one is reported once.
    seen: std::collections::HashSet<(&'static str, u32, u32, String)>,
}

impl Log<'_> {
    /// `AbstractCheck.log(DetailAST, ...)`: the node's line and its column with tabs expanded.
    pub fn at(&mut self, rule: &'static str, ast: &CsAst, n: N, message: &str) {
        self.at_pos(rule, ast.line_no(n), ast.column_no(n), message);
    }

    /// `log(int line, int columnNo, ...)`: `column` is the 0-based code-point column.
    pub fn at_pos(&mut self, rule: &'static str, line: i32, column: i32, message: &str) {
        let col = 1 + expanded_len(self.lines.get((line - 1).max(0) as usize).copied().unwrap_or_default(), column.max(0) as usize, self.cfg.tab_width);
        let d = Diagnostic { rule: self.cfg.display(rule), line: line.max(0) as u32, col: col as u32, message: message.to_string() };
        if self.seen.insert((d.rule, d.line, d.col, d.message.clone())) {
            self.out.push(d);
        }
    }

    /// `getLine(index)`: the 0-based line of the file (without its line break).
    pub fn line(&self, index: usize) -> &str {
        match self.lines.get(index) {
            Some(l) => std::str::from_utf8(l).unwrap_or(""),
            None => "",
        }
    }

    /// `log(int line, ...)`: a finding with no column.
    pub fn line_only(&mut self, rule: &'static str, line: i32, message: &str) {
        self.out.push(Diagnostic { rule: self.cfg.display(rule), line: line.max(0) as u32, col: 0, message: message.to_string() });
    }
}

/// `CommonUtil.lengthExpandedTabs`: the width of the first `to` code points, a tab running to the
/// next multiple of the tab width.
fn expanded_len(line: &[u8], to: usize, tab_width: usize) -> usize {
    let text = String::from_utf8_lossy(line);
    let mut width = 0usize;
    for c in text.chars().take(to) {
        if c == '\t' {
            width = (width / tab_width + 1) * tab_width;
        } else {
            width += 1;
        }
    }
    // A column past the end of the line (it cannot normally happen) counts one each.
    width + to.saturating_sub(text.chars().count())
}

/// Java's `Character.isWhitespace(char)`.
pub fn is_java_whitespace(c: char) -> bool {
    match c {
        '\u{a0}' | '\u{2007}' | '\u{202f}' => false,
        '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | '\u{1c}'..='\u{1f}' => true,
        c => {
            let u = c as u32;
            // Zs, Zl and Zp categories.
            matches!(u, 0x20 | 0x1680 | 0x2000..=0x2006 | 0x2008..=0x200a | 0x2028 | 0x2029 | 0x205f | 0x3000)
        }
    }
}

/// Every node in document order (`TreeWalker`'s order: a node, then its children, then its
/// siblings), calling `f` on each.
pub fn preorder(ast: &CsAst, mut f: impl FnMut(&CsAst, N)) {
    if ast.root == crate::csast::NIL {
        return;
    }
    let mut cur = Some(ast.root);
    while let Some(n) = cur {
        f(ast, n);
        cur = if let Some(c) = ast.first_child(n) {
            Some(c)
        } else {
            let mut x = n;
            loop {
                if let Some(s) = ast.next_sibling(x) {
                    break Some(s);
                }
                match ast.parent(x) {
                    Some(p) => x = p,
                    None => break None,
                }
            }
        };
    }
}

/// `CommonUtil.isBlank`.
pub fn is_blank(s: &str) -> bool {
    s.chars().all(is_java_whitespace)
}

/// `TokenUtil.isCommentType`.
pub fn is_comment_type(t: Tok) -> bool {
    matches!(t, Tok::SINGLE_LINE_COMMENT | Tok::BLOCK_COMMENT_BEGIN | Tok::BLOCK_COMMENT_END | Tok::COMMENT_CONTENT)
}

thread_local! {
    static PACKAGE_INFO: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Tells the ported checks which file is being linted (`getFilePath()`), on this thread.
pub fn set_file(path: &std::path::Path) {
    PACKAGE_INFO.with(|p| p.set(path.file_name().is_some_and(|n| n == "package-info.java")));
}

/// `CheckUtil.isPackageInfo(getFilePath())`.
pub fn is_package_info() -> bool {
    PACKAGE_INFO.with(|p| p.get())
}

pub fn same_line(ast: &CsAst, a: N, b: N) -> bool {
    ast.line_no(a) == ast.line_no(b)
}

pub fn ty_is(ast: &CsAst, n: N, t: Tok) -> bool {
    ast.ty(n) == t
}

/// Runs the ported modules the configuration enables.
pub fn run(cfg: &Config, tree: &Tree, src: &[u8], out: &mut Vec<Diagnostic>) {
    // The builder reads the fork grammar's node names (`@interface`, ...); the upstream grammar's
    // trees would make it panic.
    let orchard: tree_sitter::Language = tree_sitter_java_orchard::LANGUAGE.into();
    if *tree.language() != orchard {
        return;
    }
    let plain = cfg.text_block_google;
    let comments = cfg.google_right_curly || cfg.empty_line_separator.is_some();
    if !plain && !comments {
        return;
    }
    let mut log = Log { cfg, lines: src.split(|&b| b == b'\n').collect(), out, seen: Default::default() };
    if plain {
        if let Ok(ast) = build_checked(tree, src, false) {
            if cfg.text_block_google {
                super::textblock::check(&ast, &mut log);
            }
        }
    }
    if comments {
        if let Ok(ast) = build_checked(tree, src, true) {
            if cfg.google_right_curly {
                super::googlerightcurly::check(&ast, &mut log);
            }
            if let Some(opts) = &cfg.empty_line_separator {
                super::emptyline::check(&ast, opts, &mut log);
            }
        }
    }
}
