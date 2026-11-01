//! Unit tests for single rules. Every test runs under both grammars and requires identical results.
//! Behaviour that came from Checkstyle is additionally pinned by `tests/oracle_probes.rs`.

use crate::config::Config;
use crate::engine::lint;
use crate::syntax::{Grammar, Syntax};

fn run_with(src: &str, cfg: &Config) -> Vec<(&'static str, u32)> {
    let mut per_grammar = Vec::new();
    for g in [Grammar::Upstream, Grammar::Orchard] {
        let syn = Syntax::new(g);
        let tree = syn.new_parser().parse(src, None).unwrap();
        let mut out = Vec::new();
        lint(&syn, cfg, &tree, src.as_bytes(), &mut out);
        per_grammar.push(out.iter().map(|d| (d.rule, d.line)).collect::<Vec<_>>());
    }
    assert_eq!(per_grammar[0], per_grammar[1], "the two grammars disagree on:\n{src}");
    per_grammar.remove(0)
}

/// Run the named modules at Checkstyle's default settings.
fn run(src: &str, modules: &[&str]) -> Vec<(&'static str, u32)> {
    let only: Vec<String> = modules.iter().map(|m| m.to_string()).collect();
    run_with(src, &Config::checkstyle_defaults(Some(&only)))
}

/// Run a one-module Checkstyle XML config.
fn run_xml(src: &str, module_xml: &str) -> Vec<(&'static str, u32)> {
    let xml = format!(r#"<module name="Checker"><module name="TreeWalker">{module_xml}</module></module>"#);
    let loaded = Config::from_checkstyle_xml(&xml, None).unwrap();
    assert!(loaded.skipped.is_empty(), "unexpectedly skipped: {:?}", loaded.skipped);
    run_with(src, &loaded.config)
}

#[test]
fn empty_catch() {
    let src = "class A { void f() {\n try { g(); } catch (Exception e) {}\n try { g(); } catch (Exception e) { /* why */ }\n try { g(); } catch (Exception e) { h(); }\n} }";
    assert_eq!(run(src, &["EmptyCatchBlock"]), vec![("EmptyCatchBlock", 2)]);
}

#[test]
fn need_braces() {
    let src = "class A { void f(boolean b) {\n if (b) g();\n if (b) { g(); } else h();\n if (b) { g(); } else if (b) { g(); } else { h(); }\n while (b) g();\n for (;;) g();\n for (int i : xs) g();\n do g(); while (b);\n} }";
    assert_eq!(
        run(src, &["NeedBraces"]),
        vec![("NeedBraces", 2), ("NeedBraces", 3), ("NeedBraces", 5), ("NeedBraces", 6), ("NeedBraces", 7), ("NeedBraces", 8)]
    );
}

#[test]
fn equals_hash_code() {
    let only_equals = "class A { public boolean equals(Object o) { return true; } }";
    let only_hash = "class A { public int hashCode() { return 1; } }";
    let both = "class A { public boolean equals(Object o) { return true; } public int hashCode() { return 1; } }";
    let other_signature = "class A { public boolean equals(A o) { return true; } }";
    let anonymous = "class A { Object o = new Object() { public boolean equals(Object x) { return true; } }; }";
    assert_eq!(run(only_equals, &["EqualsHashCode"]).len(), 1);
    assert_eq!(run(only_hash, &["EqualsHashCode"]).len(), 1);
    assert!(run(both, &["EqualsHashCode"]).is_empty());
    assert!(run(other_signature, &["EqualsHashCode"]).is_empty());
    assert_eq!(run(anonymous, &["EqualsHashCode"]).len(), 1);
}

#[test]
fn equals_hash_code_is_reported_on_the_first_annotation_line() {
    // Checkstyle anchors a method-level finding on the declaration's first token, which is the
    // annotation, not the name two lines further down.
    let src = "class A {\n  @SuppressWarnings(\"x\")\n  @Override\n  public boolean equals(Object o) { return true; }\n}";
    assert_eq!(run(src, &["EqualsHashCode"]), vec![("EqualsHashCode", 2)]);
}

#[test]
fn abstract_hash_code_is_not_an_override() {
    let src = "abstract class A { @Override public boolean equals(Object o) { return true; } @Override public abstract int hashCode(); }";
    assert_eq!(run(src, &["EqualsHashCode"]).len(), 1);
}

#[test]
fn unused_imports() {
    let src = "package p;\nimport java.util.List;\nimport java.util.Map;\nimport java.io.*;\nimport static java.lang.Math.max;\nclass A { List<String> l; int f() { return max(1, 2); } }";
    assert_eq!(run(src, &["UnusedImports"]), vec![("UnusedImports", 3)]);
}

#[test]
fn import_shadowed_by_own_nested_type() {
    let src = "package p;\nimport p.A.Inner;\nclass A { class Inner {} class B extends Inner {} }";
    assert_eq!(run(src, &["UnusedImports"]), vec![("UnusedImports", 2)]);
}

#[test]
fn nested_type_does_not_shadow_the_import_in_the_class_header() {
    // `extends Base<Inner>` is outside A's body, where Inner is not in scope: the import is needed.
    let src = "package p;\nimport p.A.Inner;\nclass A extends Base<Inner> { static class Inner {} }";
    assert!(run(src, &["UnusedImports"]).is_empty());
}

#[test]
fn type_nested_two_levels_down_is_not_in_scope_of_the_outer_body() {
    let src = "package p;\nimport p.A.B.C;\nclass A { class B { class C {} } C field; }";
    assert!(run(src, &["UnusedImports"]).is_empty());
}

#[test]
fn qualified_member_names_are_not_uses_of_the_import() {
    // `Type.URI` selects an enum constant; it is not the imported java.net.URI.
    let enum_constant = "package p;\nimport java.net.URI;\nclass A { Object x = Type.URI; }";
    assert_eq!(run(enum_constant, &["UnusedImports"]), vec![("UnusedImports", 2)]);
    let method_name = "package p;\nimport java.util.List;\nclass A { void f() { x.List(); } }";
    assert_eq!(run(method_name, &["UnusedImports"]), vec![("UnusedImports", 2)]);
    let own_method = "package p;\nimport java.util.List;\nclass A { void List() {} }";
    assert_eq!(run(own_method, &["UnusedImports"]), vec![("UnusedImports", 2)]);
    let fully_qualified = "package p;\nimport java.util.List;\nclass A { java.util.List<String> l; }";
    assert_eq!(run(fully_qualified, &["UnusedImports"]), vec![("UnusedImports", 2)]);
    let method_ref_target = "package p;\nimport java.net.URI;\nclass A { Object f = this::URI; }";
    assert_eq!(run(method_ref_target, &["UnusedImports"]), vec![("UnusedImports", 2)]);
}

#[test]
fn leftmost_names_and_unqualified_calls_are_uses() {
    let static_call = "package p;\nimport java.net.URI;\nclass A { Object x = URI.create(\"a\"); }";
    assert!(run(static_call, &["UnusedImports"]).is_empty());
    let method_ref = "package p;\nimport java.util.Objects;\nclass A { Object f = Objects::isNull; }";
    assert!(run(method_ref, &["UnusedImports"]).is_empty());
    let static_import = "package p;\nimport static java.lang.Math.max;\nclass A { int f() { return max(1, 2); } }";
    assert!(run(static_import, &["UnusedImports"]).is_empty());
    let generic = "package p;\nimport java.util.List;\nclass A { Map<String, List<String>> m; }";
    assert!(run(generic, &["UnusedImports"]).is_empty());
}

#[test]
fn package_annotation_uses_import() {
    let src = "@Nonnull\npackage p;\nimport javax.annotation.Nonnull;\n";
    assert!(run(src, &["UnusedImports"]).is_empty());
}

#[test]
fn override_is_exempt_from_method_name() {
    let src = "class A { @Override void Bad() {} void Worse() {} }";
    assert_eq!(run(src, &["MethodName"]).len(), 1);
}

#[test]
fn method_names() {
    let src = "class A { void ok() {} void Bad() {} void bad_name() {} void aB1() {} }";
    assert_eq!(run(src, &["MethodName"]), vec![("MethodName", 1), ("MethodName", 1)]);
}

#[test]
fn a_method_named_like_its_class_is_reported_unless_allowed() {
    let src = "class lower {\n void lower() {}\n}";
    assert_eq!(run(src, &["MethodName"]), vec![("MethodName", 2)]);
    assert!(run_xml(src, r#"<module name="MethodName"><property name="allowClassName" value="true"/></module>"#).is_empty());
}

#[test]
fn a_custom_format_is_honoured() {
    let src = "class A { void ab() {} void aB() {} }";
    assert_eq!(run_xml(src, r#"<module name="MethodName"><property name="format" value="^[a-z]+$"/></module>"#), vec![("MethodName", 1)]);
}

#[test]
fn annotated_varargs_parse_only_with_the_fork() {
    // Canary: documents the upstream gap. If this starts failing, upstream fixed it.
    let src = "class A { void f(String @Nullable ... xs) {} void g(@Nullable Object @Nullable ... ys) {} }";
    let has_error = |g| Syntax::new(g).new_parser().parse(src, None).unwrap().root_node().has_error();
    assert!(has_error(Grammar::Upstream), "upstream tree-sitter-java now parses annotated varargs");
    assert!(!has_error(Grammar::Orchard));
}
