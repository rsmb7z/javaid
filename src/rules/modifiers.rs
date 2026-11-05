//! `ModifierOrder`.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use tree_sitter::Node;

/// The order the Java Language Specification suggests, which is the order Checkstyle enforces.
/// Verified against Checkstyle for every ordered pair of these 14 (`ModPairs.java`).
const ORDER: [&[u8]; 14] = [
    b"public",
    b"protected",
    b"private",
    b"abstract",
    b"default",
    b"static",
    b"sealed",
    b"non-sealed",
    b"final",
    b"transient",
    b"volatile",
    b"synchronized",
    b"native",
    b"strictfp",
];

/// One finding per declaration, for the first modifier that is out of order. An annotation after a
/// modifier is reported too, unless it can be a type annotation (`public @Nullable String name;`),
/// which is how Checkstyle reads it on a variable, parameter, constructor or a method that does not
/// return `void`; checking then stops for that declaration. All found by probing.
pub fn on_modifiers(cx: &mut Ctx, modifiers: Node) {
    if !cx.cfg.modifier_order {
        return;
    }
    let mut last_rank = 0;
    let mut seen_keyword = false;
    let mut cur = modifiers.walk();
    for m in modifiers.children(&mut cur) {
        if m.is_extra() {
            continue;
        }
        if matches!(m.kind_s(), "marker_annotation" | "annotation") {
            if seen_keyword {
                if !annotation_may_follow_modifiers(cx) {
                    let name =
                        m.child_by_field_id(cx.syn.f.name).map(|n| String::from_utf8_lossy(cx.text(n)).replace(char::is_whitespace, "")).unwrap_or_default();
                    cx.diag("ModifierOrder", m, format!("'@{name}' annotation modifier does not precede non-annotation modifiers."));
                }
                return;
            }
            continue;
        }
        let word = cx.text(m);
        let Some(rank) = ORDER.iter().position(|w| *w == word) else { continue };
        if seen_keyword && rank < last_rank {
            cx.diag("ModifierOrder", m, format!("'{}' modifier out of order with the JLS suggestions.", String::from_utf8_lossy(word)));
            return;
        }
        last_rank = rank;
        seen_keyword = true;
    }
}

/// Could an annotation after the modifiers be one on the declaration's type?
fn annotation_may_follow_modifiers(cx: &Ctx) -> bool {
    let Some(decl) = cx.parent() else { return false };
    match decl.kind_s() {
        "field_declaration"
        | "constant_declaration"
        | "local_variable_declaration"
        | "formal_parameter"
        | "spread_parameter"
        | "catch_formal_parameter"
        | "constructor_declaration"
        | "enhanced_for_statement" => true,
        "method_declaration" => decl.child_by_field_id(cx.syn.f.type_).is_some_and(|t| t.kind_s() != "void_type"),
        _ => false,
    }
}
