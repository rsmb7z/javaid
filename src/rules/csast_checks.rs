//! The modules ported from Checkstyle's own sources onto the emulated AST (`crate::csast`). They run
//! once per file, after the tree walk, on a tree built from the parse.

use crate::config::Config;
use crate::csast::build;
use crate::engine::Diagnostic;
use tree_sitter::Tree;

pub fn run(cfg: &Config, tree: &Tree, src: &[u8], out: &mut Vec<Diagnostic>) {
    if cfg.variable_declaration_usage_distance.is_none() && cfg.comments_indentation.is_none() {
        return;
    }
    // A file with syntax errors is Checkstyle's parse error, reported elsewhere.
    if tree.root_node().has_error() {
        return;
    }
    let lines: Vec<&[u8]> = super::text::lines(src).collect();
    // The builder recurses with the nesting of the code; give it room.
    std::thread::scope(|scope| {
        let work = std::thread::Builder::new().stack_size(1 << 30).spawn_scoped(scope, || {
            let mut found = Vec::new();
            if let Some(o) = &cfg.variable_declaration_usage_distance {
                if let Ok(a) = build::build_checked(tree, src, false) {
                    super::vdud::run(&a, o, &lines, cfg.tab_width, &mut found);
                }
            }
            if let Some(o) = &cfg.comments_indentation {
                if let Ok(a) = build::build_checked(tree, src, true) {
                    super::commentsindent::run(&a, o, &lines, cfg.tab_width, &mut found);
                }
            }
            found
        });
        if let Ok(h) = work {
            if let Ok(mut found) = h.join() {
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
    });
}
