//! The library entry points on their own, the way an embedder (an editor integration, a test) calls
//! them: everything a file's findings depend on must come in through the arguments, with no setup
//! call beforehand and nothing carried over from the file linted before on the same thread.

use javaid::config::Config;
use javaid::engine::{lint_file, normalize_line_breaks};
use javaid::syntax::{Grammar, Syntax};
use std::path::Path;

fn lint(config: &str, path: &str, text: &str) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let xml = std::fs::read_to_string(root.join("bench/oracle").join(config)).unwrap();
    let cfg = Config::from_checkstyle_xml(&xml, None).unwrap().config;
    let syn = Syntax::new(Grammar::Orchard);
    let mut src = text.as_bytes().to_vec();
    normalize_line_breaks(&mut src);
    let tree = syn.new_parser().parse(&src, None).unwrap();
    let mut out = Vec::new();
    lint_file(&syn, &cfg, &tree, &src, Some(Path::new(path)), &mut out);
    out.iter().map(|d| format!("{}:{} {}", d.line, d.col, d.rule)).collect()
}

/// `EmptyLineSeparator` lets an annotated package declaration in `package-info.java` follow a comment
/// directly (`bench/oracle/probes/pi1`), and nowhere else. Which file it is comes from the path given
/// to `lint_file`, in either order on one thread.
#[test]
fn the_file_name_reaches_the_ported_checks_through_lint_file() {
    let text = "/*\n * header\n */\n@Deprecated\npackage p;\n";
    for _ in 0..2 {
        assert_eq!(lint("els-default.xml", "p/A.java", text), ["5:1 EmptyLineSeparator"]);
        assert_eq!(lint("els-default.xml", "p/package-info.java", text), Vec::<String>::new());
    }
}
