//! The Checkstyle AST emulation against Checkstyle's own output.
//!
//! `bench/csast/golden/` holds the text `checkstyle -t` printed (Checkstyle 14.4.0) for every
//! snippet in `bench/csast/snippets/` (one per language area) and `bench/csast/snippets-layout/`
//! (the same sources re-laid-out: one line, one token per line, random tabs and line breaks).
//! `golden-comments/` holds the `checkstyle -T` dumps (with comment nodes). `bench/csast/regen-golden.sh`
//! regenerates them. The emulation must print them byte for byte.

use std::path::Path;

use javaid::csast::{dump, try_build_source};
use javaid::syntax::Grammar;

fn check_dir(dir: &str, comments: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("bench/csast");
    let mut failures = Vec::new();
    let mut count = 0;
    let mut entries: Vec<_> = std::fs::read_dir(root.join(if comments { "golden-comments" } else { "golden" }).join(dir)).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for golden in entries {
        let name = golden.file_name().unwrap().to_str().unwrap().trim_end_matches(".txt").to_string();
        let src = std::fs::read(root.join(dir).join(&name)).unwrap();
        let expected = std::fs::read_to_string(&golden).unwrap();
        let got = dump(&try_build_source(&src, Grammar::default(), comments).unwrap());
        count += 1;
        if got != expected {
            let line = got.lines().zip(expected.lines()).position(|(a, b)| a != b).map_or(got.lines().count().min(expected.lines().count()), |i| i) + 1;
            failures.push(format!("{dir}/{name}: first difference at line {line}"));
        }
    }
    assert!(count > 0, "no golden files in {dir}");
    assert!(failures.is_empty(), "{} of {count} dumps differ from Checkstyle's:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn snippets_match_checkstyle() {
    check_dir("snippets", false);
}

#[test]
fn snippets_with_comments_match_checkstyle() {
    check_dir("snippets", true);
}

#[test]
fn relaid_out_snippets_match_checkstyle() {
    check_dir("snippets-layout", false);
}

#[test]
fn relaid_out_snippets_with_comments_match_checkstyle() {
    check_dir("snippets-layout", true);
}
