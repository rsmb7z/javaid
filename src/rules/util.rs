//! Small helpers shared by rule families.

use crate::engine::Ctx;
use crate::syntax::KindExt;
use tree_sitter::Node;

/// Does the declaration carry this modifier keyword? Compares token *text*, because upstream
/// tree-sitter-java emits anonymous `public`/`static` tokens while the orchard fork emits named
/// `visibility`/`modifier` nodes. The text is the same in both.
pub fn has_modifier(cx: &Ctx, decl: Node, word: &[u8]) -> bool {
    let mut cur = decl.walk();
    for c in decl.children(&mut cur) {
        if !c.is_named() || c.is_extra() {
            continue; // `for (`, comments
        }
        if c.kind_s() != "modifiers" {
            return false; // modifiers, when present, are the first real child
        }
        let mut inner = c.walk();
        return c.children(&mut inner).any(|m| cx.text(m) == word);
    }
    false
}

/// `@Override` (or `@java.lang.Override`) among the declaration's annotations.
pub fn has_override_annotation(cx: &Ctx, decl: Node) -> bool {
    let mut cur = decl.walk();
    let Some(mods) = decl.children(&mut cur).find(|c| c.kind_s() == "modifiers") else { return false };
    let mut inner = mods.walk();
    mods.children(&mut inner).any(|c| {
        c.kind_s() == "marker_annotation"
            && c.child_by_field_id(cx.syn.f.name).is_some_and(|n| matches!(cx.text(n), b"Override" | b"java.lang.Override"))
    })
}

/// The name of the type whose body directly contains the member being visited, if that type has a
/// name (anonymous classes and enum-constant bodies do not).
pub fn enclosing_type_name<'a>(cx: &Ctx<'a>) -> Option<&'a [u8]> {
    // Above an enum's members there is one more level, the `enum_body` that also holds the constants.
    let up = if cx.ancestor(0)?.kind_s() == "enum_body_declarations" { 1 } else { 0 };
    let decl = cx.ancestor(up + 1)?;
    match decl.kind_s() {
        "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
            decl.child_by_field_id(cx.syn.f.name).map(|n| cx.text(n))
        }
        _ => None,
    }
}
