//! `CustomImportOrder`: imports in the groups and order a configuration spells out.
//!
//! Every import is collected as the walk meets it and judged at the end of the file, like
//! Checkstyle's `finishTree`.

use crate::config::CustomImportOrder;
use crate::engine::Ctx;
use tree_sitter::Node;

const NON_GROUP: &str = "NOT_ASSIGNED_TO_ANY_GROUP";

struct Import {
    path: String,
    group: String,
    is_static: bool,
    line: u32,
    col: u32,
    end_line: u32,
}

#[derive(Default)]
pub struct State {
    imports: Vec<Import>,
    same_package: String,
}

/// The dotted name, as Checkstyle's `FullIdent` spells it: identifiers and dots only.
fn full_ident(cx: &Ctx, name: Node) -> String {
    let mut out = String::new();
    let mut cur = name.walk();
    let mut stack = vec![name];
    while let Some(n) = stack.pop() {
        if n.child_count() == 0 {
            if !matches!(n.kind(), "line_comment" | "block_comment") {
                out.push_str(&String::from_utf8_lossy(cx.text(n)));
            }
        } else {
            for c in n.children(&mut cur).collect::<Vec<_>>().into_iter().rev() {
                stack.push(c);
            }
        }
    }
    out
}

fn first_domains(count: usize, path: &str) -> String {
    path.split('.').take(count).collect()
}

pub fn on_package(cx: &Ctx, st: &mut State, node: Node) {
    let Some(opts) = &cx.cfg.custom_import_order else { return };
    let mut cur = node.walk();
    let name = node.children(&mut cur).find(|c| matches!(c.kind(), "scoped_identifier" | "identifier"));
    st.same_package = name.map_or_else(String::new, |n| first_domains(opts.same_package_depth, &full_ident(cx, n)));
}

pub fn on_import(cx: &Ctx, st: &mut State, node: Node) {
    let Some(opts) = &cx.cfg.custom_import_order else { return };
    let mut cur = node.walk();
    let mut path = String::new();
    let mut is_static = false;
    let mut star = false;
    let mut end_line = node.end_position().row as u32 + 1;
    for c in node.children(&mut cur) {
        match c.kind() {
            "static" => is_static = true,
            "scoped_identifier" | "identifier" => path = full_ident(cx, c),
            "asterisk" => star = true,
            ";" => end_line = c.start_position().row as u32 + 1,
            _ => {}
        }
    }
    if star {
        path.push_str(".*");
    }
    let group = group_of(opts, &st.same_package, is_static, &path);
    let p = node.start_position();
    st.imports.push(Import { path, group, is_static, line: p.row as u32 + 1, col: p.column as u32 + 1, end_line });
}

fn group_of(opts: &CustomImportOrder, same_package: &str, is_static: bool, path: &str) -> String {
    let mut best: (String, usize, usize) = (NON_GROUP.to_string(), 0, 0);
    if is_static && opts.rules.iter().any(|r| r == "STATIC") {
        best = ("STATIC".to_string(), path.len(), 0);
    } else if opts.rules.iter().any(|r| r == "SAME_PACKAGE") && same_package == first_domains(opts.same_package_depth, path) {
        best = ("SAME_PACKAGE".to_string(), path.len(), 0);
    }
    for group in &opts.rules {
        let re = match group.as_str() {
            "STANDARD_JAVA_PACKAGE" => &opts.standard_package,
            "SPECIAL_IMPORTS" => &opts.special_imports,
            _ => continue,
        };
        for (start, len) in re.find_all(path) {
            if len > best.1 || (len == best.1 && start < best.2) {
                best = (group.clone(), len, start);
            }
        }
    }
    if best.0 == NON_GROUP && opts.rules.iter().any(|r| r == "THIRD_PARTY_PACKAGE") && opts.third_party.matches(path.as_bytes()) {
        best.0 = "THIRD_PARTY_PACKAGE".to_string();
    }
    best.0
}

fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let at: Vec<&str> = a.split('.').collect();
    let bt: Vec<&str> = b.split('.').collect();
    for (x, y) in at.iter().zip(bt.iter()) {
        // Java's String.compareTo compares UTF-16 code units.
        let o = x.encode_utf16().cmp(y.encode_utf16());
        if o != std::cmp::Ordering::Equal {
            return o;
        }
    }
    at.len().cmp(&bt.len())
}

fn blank_lines_between(src: &[u8], from: u32, to: u32) -> usize {
    let lines: Vec<&[u8]> = src.split(|b| *b == b'\n').collect();
    (from + 1..to).filter(|i| lines.get(*i as usize - 1).is_some_and(|l| std::str::from_utf8(l).map_or_else(|_| l.iter().all(|b| b.is_ascii_whitespace()), |s| s.chars().all(char::is_whitespace)))).count()
}

pub fn finish(cx: &mut Ctx, st: &mut State) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.custom_import_order else { return };
    if st.imports.is_empty() {
        return;
    }
    let rules = &opts.rules_with_fallback;
    let mut current = st.imports[0].group.clone();
    let mut current_number = rules.iter().rposition(|r| *r == current).map_or(-1, |i| i as i64);
    let mut prev_obj: Option<usize> = None;
    let mut prev_path: Option<String> = None;
    let has_group = |g: &str| st.imports.iter().any(|i| i.group == g);
    let mut findings: Vec<(usize, String)> = Vec::new();
    for idx in 0..st.imports.len() {
        let (group, path) = (st.imports[idx].group.clone(), st.imports[idx].path.clone());
        if group == current {
            if let Some(p) = prev_obj {
                if blank_lines_between(cx.src, st.imports[p].end_line, st.imports[idx].line) > 0 {
                    findings.push((idx, format!("Extra separation in import group before '{path}'")));
                }
            }
            if opts.sort_alphabetically && prev_path.as_ref().is_some_and(|pp| compare(&path, pp) == std::cmp::Ordering::Less) {
                findings.push((idx, format!("Wrong lexicographical order for '{path}' import. Should be before '{}'.", prev_path.clone().unwrap())));
            } else {
                prev_path = Some(path.clone());
            }
            prev_obj = Some(idx);
        } else if rules.len() as i64 > current_number + 1 {
            // The next group that has any import in it.
            let mut next_number = (current_number + 1) as usize;
            while rules.len() > next_number + 1 {
                if has_group(&rules[next_number]) {
                    break;
                }
                next_number += 1;
            }
            let next = rules[next_number].clone();
            if group == next {
                if let Some(p) = prev_obj {
                    if opts.separate_line_between_groups && blank_lines_between(cx.src, st.imports[p].end_line, st.imports[idx].line) != 1 {
                        findings.push((idx, format!("'{path}' should be separated from previous import group by one line.")));
                    }
                }
                current = next.clone();
                current_number = rules.iter().rposition(|r| *r == next).map_or(-1, |i| i as i64);
                prev_path = Some(path.clone());
            } else {
                findings.push((idx, wrong_order(&group, &next, &path)));
            }
            prev_obj = Some(idx);
        } else {
            findings.push((idx, wrong_order(&group, &current, &path)));
        }
    }
    for (idx, msg) in findings {
        let imp = &st.imports[idx];
        let _ = imp.is_static;
        cx.diag_at("CustomImportOrder", imp.line, imp.col, msg);
    }
}

fn wrong_order(group: &str, current: &str, path: &str) -> String {
    if group == NON_GROUP {
        format!("Imports without groups should be placed at the end of the import list: '{path}'.")
    } else if current == NON_GROUP {
        format!("Import statement for '{path}' is in the wrong order. Should be in the '{group}' group, expecting not assigned imports on this line.")
    } else {
        format!("Import statement for '{path}' is in the wrong order. Should be in the '{group}' group, expecting group '{current}' on this line.")
    }
}
