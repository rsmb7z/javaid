//! `RedundantImport`, `IllegalImport` and `IllegalInstantiation`.

use crate::engine::Ctx;
use rustc_hash::FxHashMap;
use tree_sitter::Node;

#[derive(Default)]
pub struct State {
    package: String,
    /// Import text, and whether it is static, to the line it first appeared on.
    seen: FxHashMap<(bool, String), Vec<u32>>,
    /// `IllegalInstantiation`: what the file declares, imports and instantiates.
    inst_package: Option<String>,
    inst_imports: Vec<String>,
    inst_classes: rustc_hash::FxHashSet<String>,
    inst_new: Vec<(u32, u32, String)>,
}

fn compact(cx: &Ctx, n: Node) -> String {
    String::from_utf8_lossy(cx.text(n)).chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn on_package(cx: &Ctx, st: &mut State, node: Node) {
    if cx.cfg.redundant_import || cx.cfg.illegal_instantiation.is_some() {
        let mut cur = node.walk();
        let name = node.children(&mut cur).find(|c| matches!(c.kind(), "scoped_identifier" | "identifier"));
        st.package = name.map(|n| compact(cx, n)).unwrap_or_default();
        st.inst_package = Some(st.package.clone());
    }
}

/// A class or record declaration: `new X()` is the file's own `X`, not `java.lang.X`, from then on.
pub fn on_class_decl(cx: &Ctx, st: &mut State, node: Node) {
    if cx.cfg.illegal_instantiation.is_some() && matches!(node.kind(), "class_declaration" | "record_declaration") {
        if let Some(name) = node.child_by_field_id(cx.syn.f.name) {
            st.inst_classes.insert(compact(cx, name));
        }
    }
}

pub fn on_new(cx: &Ctx, st: &mut State, node: Node) {
    if cx.cfg.illegal_instantiation.is_none() {
        return;
    }
    let Some(ty) = node.child_by_field_name("type") else { return };
    let base = if ty.kind() == "generic_type" { ty.child(0).unwrap_or(ty) } else { ty };
    let p = node.start_position();
    st.inst_new.push((p.row as u32 + 1, p.column as u32 + 1, compact(cx, base)));
}

/// Resolve every `new` once the whole file is known, as Checkstyle does at the end of the tree.
pub fn finish(cx: &mut Ctx, st: &mut State) {
    let Some(classes) = &cx.cfg.illegal_instantiation else { return };
    let base_name = |s: &str| s.rsplit('.').next().unwrap_or(s).to_string();
    for (line, col, name) in std::mem::take(&mut st.inst_new) {
        let found = if classes.contains(&name) {
            Some(name.clone())
        } else {
            let via_import = st.inst_imports.iter().find_map(|imp| {
                let arg = match imp.strip_suffix(".*") {
                    Some(prefix) => format!("{prefix}.{name}"),
                    None => imp.clone(),
                };
                (base_name(&arg) == name && classes.contains(&arg)).then_some(arg)
            });
            via_import.or_else(|| {
                classes
                    .iter()
                    .find(|illegal| {
                        let same_package = st.inst_package.as_deref().is_some_and(|pkg| format!("{pkg}.{name}") == **illegal);
                        let standard = **illegal == format!("java.lang.{name}") && !st.inst_classes.contains(&name);
                        same_package || standard
                    })
                    .cloned()
            })
        };
        if let Some(class) = found {
            cx.diag_at("IllegalInstantiation", line, col, format!("Instantiation of {class} should be avoided."));
        }
    }
}

pub fn on_import(cx: &mut Ctx, st: &mut State, node: Node) {
    if !cx.cfg.redundant_import && cx.cfg.illegal_import.is_none() && cx.cfg.illegal_instantiation.is_none() {
        return;
    }
    let (mut name, mut star, mut is_static) = (None, false, false);
    let mut cur = node.walk();
    for c in node.children(&mut cur) {
        match c.kind() {
            "identifier" | "scoped_identifier" => name = Some(c),
            "asterisk" => star = true,
            "static" => is_static = true,
            _ => {}
        }
    }
    let Some(name) = name else { return };
    let mut text = compact(cx, name);
    if star {
        text.push_str(".*");
    }
    let line = node.start_position().row as u32 + 1;
    if cx.cfg.illegal_instantiation.is_some() && !is_static {
        st.inst_imports.push(text.clone());
    }
    if cx.cfg.redundant_import {
        let package = text.rsplit_once('.').map(|(p, _)| p.to_string());
        // Three separate checks: Checkstyle reports a duplicate once for every earlier copy, and a
        // `java.lang` or same-package import that is also a duplicate gets both.
        if !is_static && package.as_deref() == Some("java.lang") {
            cx.diag("RedundantImport", node, format!("Redundant import from the java.lang package - {text}."));
        }
        if !is_static && package.as_deref().is_some_and(|p| p == st.package) {
            cx.diag("RedundantImport", node, format!("Redundant import from the same package - {text}."));
        }
        let earlier = st.seen.entry((is_static, text.clone())).or_default();
        for first in earlier.clone() {
            cx.diag("RedundantImport", node, format!("Duplicate import to line {first} - {text}."));
        }
        earlier.push(line);
    }
    if let Some(opts) = &cx.cfg.illegal_import {
        let pkg = text.rsplit_once('.').map_or("", |(p, _)| p);
        let illegal = if opts.regexp {
            opts.pkg_formats.iter().any(|f| f.matches(pkg.as_bytes())) || opts.class_formats.iter().any(|f| f.matches(text.as_bytes()))
        } else {
            opts.pkgs.iter().any(|p| text.starts_with(&format!("{p}."))) || opts.classes.iter().any(|c| *c == text)
        };
        if illegal {
            cx.diag("IllegalImport", node, format!("Illegal import - {text}."));
        }
    }
}
