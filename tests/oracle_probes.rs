//! Behaviour pinned to Checkstyle 14.4.0.
//!
//! Each probe file under `bench/oracle/probes` contains one distinctively-named violation per
//! construct. The `.expected` fixture next to it is what Checkstyle reported (recorded with
//! `bench/oracle/record-golden.py`). javaid must report exactly the same (line, name) pairs, where
//! the name is what Checkstyle prints in brackets: the module, or its `id` if the config gives one.
//! Unlike a unit test written from memory, a failure here means javaid disagrees with Checkstyle.

use javaid::config::Config;
use javaid::engine::{lint, lint_text, normalize_line_breaks, Diagnostic};
use javaid::syntax::{Grammar, Syntax};
use std::path::Path;

const CASES: &[(&str, &str)] = &[
    ("Probe1.java", "naming"),
    ("Probe2.java", "naming"),
    ("JavadocProbe.java", "unusedimports"),
    ("JavadocProbe2.java", "unusedimports"),
    ("Probe3.java", "naming-options"),
    ("Probe4.java", "typename-tokens"),
    ("Probe5.java", "emptycatch-default"),
    ("Probe5.java", "emptycatch-comment"),
    ("Probe5.java", "emptycatch-name"),
    ("Probe5.java", "emptycatch-both"),
    ("Probe6.java", "emptycatch-anchored"),
    ("Probe7.java", "placeholder-default"),
    // FileTabCharacter
    ("TabProbe.java", "filetab"),
    ("TabProbe.java", "filetab-each"),
    ("MixedProbe.java", "filetab-each"),
    ("CrlfProbe.java", "filetab-each"),
    ("CrProbe.java", "filetab-each"),
    ("FormFeedProbe.java", "filetab-each"),
    // LineLength
    ("LineProbe.java", "linelength"),
    ("LineProbe.java", "linelength-tab4"),
    ("LineProbe.java", "linelength-google"),
    // RegexpSingleline
    ("TrailProbe.java", "regexp-trailing"),
    ("TrailProbe.java", "regexp-max2"),
    ("TrailProbe.java", "regexp-min99"),
    ("TrailProbe.java", "regexp-nocase"),
    ("TrailProbe.java", "regexp-case"),
    ("MixedProbe.java", "regexp-trailing"),
    ("CrlfProbe.java", "regexp-trailing"),
    ("CrProbe.java", "regexp-trailing"),
    ("FormFeedProbe.java", "regexp-trailing"),
    // TodoComment
    ("TodoProbe.java", "todo-default"),
    ("TodoProbe.java", "todo-google"),
    ("TodoProbe.java", "todo-anchored"),
    ("TodoProbe.java", "todo-spaced"),
    ("TodoProbe.java", "todo-endofline"),
    ("TodoProbe.java", "todo-across"),
    ("CrlfProbe.java", "todo-endofline"),
    // Tree rules count lines the way Checkstyle does, whatever ends them.
    ("MixedProbe.java", "typename-never"),
    ("CrlfProbe.java", "typename-never"),
    ("CrProbe.java", "typename-never"),
    // ArrayTypeStyle, UpperEll, AvoidStarImport
    ("ArrayProbe.java", "arraytypestyle"),
    ("EllProbe.java", "upperell"),
    ("StarProbe.java", "avoidstar"),
    ("StarProbe.java", "avoidstar-excludes"),
    ("StarProbe.java", "avoidstar-class"),
    ("StarProbe.java", "avoidstar-static"),
    // MissingSwitchDefault, ModifierOrder, MultipleVariableDeclarations
    ("SwitchProbe.java", "missingswitchdefault"),
    ("ModProbe.java", "modifierorder"),
    ("ModPairs.java", "modifierorder"),
    ("ModProbe2.java", "modifierorder"),
    ("MultiProbe.java", "multivar"),
    ("MultiProbe2.java", "multivar"),
    // OneStatementPerLine
    ("OneProbe.java", "onestatement"),
    ("OneProbe.java", "onestatement-try"),
    ("OneProbe2.java", "onestatement"),
    ("OneProbe2.java", "onestatement-try"),
    ("OneProbe3.java", "onestatement"),
    ("OneProbe4.java", "onestatement"),
    ("OneProbe5.java", "onestatement"),
    ("OneProbe6.java", "onestatement"),
    // FallThrough
    ("FtProbe1.java", "fallthrough"),
    ("FtProbe1.java", "fallthrough-last"),
    ("FtProbe2.java", "fallthrough"),
    ("FtProbe2.java", "fallthrough-last"),
    ("FtProbe3.java", "fallthrough"),
    ("FtProbe3.java", "fallthrough-last"),
    ("FtProbe4.java", "fallthrough"),
    ("FtProbe4.java", "fallthrough-last"),
    ("FtProbe5.java", "fallthrough"),
    ("FtProbe5.java", "fallthrough-last"),
    ("FtInf.java", "fallthrough"),
    ("FtInf.java", "fallthrough-last"),
    ("FtYield.java", "fallthrough"),
    ("FtYield.java", "fallthrough-last"),
    // A Checker-level fileExtensions pulls in files that are not Java.
    ("TabProbe.java", "text-files"),
    ("Props.properties", "text-files"),
    ("Doc.xml", "text-files"),
];

/// The findings as `line name` rows, and whether the grammar left syntax errors in the tree.
/// Probes on which upstream tree-sitter-java parses without an error node but builds the wrong
/// tree, so the agreement test cannot see the problem from the parse alone. Each is a reason the
/// maintained fork is the default.
const UPSTREAM_MISPARSES: &[(&str, &str)] = &[
    ("SwitchProbe.java", "a record pattern in a case label (`case Circle(), Square() ->`) becomes two method calls"),
    ("ModProbe2.java", "`instanceof final @TA String s` loses its modifiers"),
];

fn findings(root: &Path, probe: &str, config: &str, grammar: Grammar) -> Result<(Vec<String>, bool), String> {
    let xml = std::fs::read_to_string(root.join(format!("bench/oracle/{config}.xml"))).unwrap();
    let loaded = Config::from_checkstyle_xml(&xml, None).unwrap();
    if !loaded.skipped.is_empty() {
        return Err(format!("oracle config must be fully supported: {:?}", loaded.skipped));
    }

    let mut src = std::fs::read(root.join("bench/oracle/probes").join(probe)).unwrap();
    let ext = Path::new(probe).extension().and_then(|e| e.to_str()).unwrap();
    let mut out: Vec<Diagnostic> = Vec::new();
    let mut syntax_errors = false;
    if ext == "java" {
        normalize_line_breaks(&mut src);
        let syn = Syntax::new(grammar);
        let tree = syn.new_parser().parse(&src, None).unwrap();
        syntax_errors = tree.root_node().has_error();
        lint(&syn, &loaded.config, &tree, &src, &mut out);
    } else {
        lint_text(&loaded.config, ext, &src, &mut out);
    }
    let mut rows: Vec<(u32, &str)> = out.iter().map(|d| (d.line, d.rule)).collect();
    rows.sort();
    Ok((rows.iter().map(|(l, r)| format!("{l} {r}")).collect(), syntax_errors))
}

fn expected(root: &Path, probe: &str, config: &str) -> Vec<String> {
    let stem = probe.rsplit_once('.').unwrap().0;
    let path = root.join(format!("bench/oracle/probes/{stem}.{config}.expected"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let rows: Vec<String> = text.lines().filter(|l| !l.starts_with('#')).map(String::from).collect();
    // An empty fixture is fine only when the recorder said Checkstyle looked and found nothing;
    // otherwise it is the trace of a broken oracle run, which would prove nothing.
    assert!(
        !rows.is_empty() || text.lines().any(|l| l.starts_with("# no findings")),
        "{}: empty fixture without the 'no findings' marker",
        path.display()
    );
    rows
}

#[test]
fn javaid_reproduces_checkstyle_on_every_probe() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failures = Vec::new();
    for (probe, config) in CASES {
        let want = expected(root, probe, config);
        match findings(root, probe, config, Grammar::Orchard) {
            Err(why) => failures.push(format!("{probe} under {config}.xml: {why}")),
            Ok((got, _)) if got != want => failures.push(format!("{probe} under {config}.xml\n    javaid:     {got:?}\n    Checkstyle: {want:?}")),
            Ok(_) => {}
        }
    }
    assert!(failures.is_empty(), "{} of {} probe cases differ from Checkstyle:\n  {}", failures.len(), CASES.len(), failures.join("\n  "));
}

#[test]
fn the_upstream_grammar_agrees_where_it_can_parse() {
    // The fork is the default because upstream cannot parse annotated varargs, nor an octal literal
    // with an underscore (`0_7l`); on files it parses cleanly, both grammars must give the same findings.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut compared = 0;
    for (probe, config) in CASES {
        if !probe.ends_with(".java") || UPSTREAM_MISPARSES.iter().any(|(p, _)| p == probe) {
            continue;
        }
        let (Ok((fork, _)), Ok((upstream, upstream_errors))) = (findings(root, probe, config, Grammar::Orchard), findings(root, probe, config, Grammar::Upstream)) else {
            continue; // an unsupported config is reported by the other test
        };
        if upstream_errors {
            continue; // upstream could not parse this probe, which is the fork's reason to exist
        }
        assert_eq!(upstream, fork, "{probe}: upstream and fork grammars disagree");
        compared += 1;
    }
    assert!(compared >= 30, "only {compared} probes were comparable: the skip rule has swallowed the test");
}
