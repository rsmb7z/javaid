//! The modules ported from Checkstyle's own sources onto the emulated AST (`crate::csast`). They run
//! once per file, after the tree walk, on a tree built from the parse.

use crate::config::Config;
use crate::csast::{with_big_stack, CsCache};
use crate::engine::Diagnostic;

pub fn run(cfg: &Config, cache: &CsCache, out: &mut Vec<Diagnostic>) {
    if cfg.variable_declaration_usage_distance.is_none() && cfg.comments_indentation.is_none() {
        return;
    }
    // A file with syntax errors is Checkstyle's parse error, reported elsewhere.
    if cache.tree().is_some_and(|t| t.root_node().has_error()) {
        return;
    }
    let lines: Vec<&[u8]> = super::text::lines(cache.src()).collect();
    let plain = cfg.variable_declaration_usage_distance.as_ref().and_then(|o| cache.get(false).map(|a| (o, a)));
    let with_comments = cfg.comments_indentation.as_ref().and_then(|o| cache.get(true).map(|a| (o, a)));
    // The checks recurse with the nesting of the code as the builder does; give them room.
    let found = with_big_stack(|| {
        let mut found = Vec::new();
        if let Some((o, a)) = plain {
            super::vdud::run(a, o, &lines, cfg.tab_width, &mut found);
        }
        if let Some((o, a)) = with_comments {
            super::commentsindent::run(a, o, &lines, cfg.tab_width, &mut found);
        }
        found
    });
    if let Ok(mut found) = found {
        // Checkstyle keeps a file's violations in a sorted set: an identical one is stored once
        // (a comment reached twice in the tree, as with `int a, b;`, is reported once).
        let mut seen = std::collections::HashSet::new();
        found.retain(|d| seen.insert((d.rule, d.line, d.col, d.message.clone())));
        for mut d in found {
            d.rule = cfg.display(d.rule);
            out.push(d);
        }
    }
}
