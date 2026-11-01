//! parenmut OUTDIR LIST COUNT SEED PERCENT
//! Wraps a random PERCENT of the expressions in redundant parentheses (only where the grammar allows it).
use std::io::Write;
use tree_sitter::{Node, Parser};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

const EXPR: &[&str] = &[
    "binary_expression", "method_invocation", "object_creation_expression", "array_access", "field_access",
    "decimal_integer_literal", "string_literal", "character_literal", "true", "false", "null_literal",
    "parenthesized_expression", "ternary_expression", "unary_expression", "update_expression",
    "array_creation_expression", "lambda_expression", "identifier", "this", "cast_expression",
    "instanceof_expression", "method_reference", "assignment_expression", "decimal_floating_point_literal",
    "hex_integer_literal", "class_literal",
];

fn field_is(parent: Node, field: &str, n: Node) -> bool {
    parent.child_by_field_name(field).map(|c| c.id() == n.id()).unwrap_or(false)
}

fn candidate(n: Node) -> bool {
    if !EXPR.contains(&n.kind()) {
        return false;
    }
    let Some(p) = n.parent() else { return false };
    match p.kind() {
        "argument_list" | "binary_expression" | "parenthesized_expression" | "ternary_expression" | "unary_expression"
        | "return_statement" | "throw_statement" | "array_initializer" | "element_value_array_initializer" => {
            if p.kind() == "binary_expression" || p.kind() == "ternary_expression" {
                true
            } else {
                true
            }
        }
        "variable_declarator" => field_is(p, "value", n),
        "assignment_expression" => field_is(p, "right", n),
        "array_access" => field_is(p, "index", n),
        "cast_expression" => field_is(p, "value", n),
        "lambda_expression" => field_is(p, "body", n),
        "enhanced_for_statement" => field_is(p, "value", n),
        "element_value_pair" => field_is(p, "value", n),
        _ => false,
    }
}

fn walk(n: Node, out: &mut Vec<(usize, usize)>) {
    if candidate(n) {
        out.push((n.start_byte(), n.end_byte()));
    }
    let mut c = n.walk();
    for ch in n.children(&mut c) {
        walk(ch, out);
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (out, list, count, seed, percent) = (&a[1], &a[2], a[3].parse::<usize>().unwrap(), a[4].parse::<u64>().unwrap(), a[5].parse::<u64>().unwrap());
    let files: Vec<String> = std::fs::read_to_string(list).unwrap().lines().filter(|l| !l.is_empty()).map(String::from).collect();
    let mut rng = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
    let mut idx: Vec<usize> = (0..files.len()).collect();
    for i in (1..idx.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        idx.swap(i, j);
    }
    std::fs::create_dir_all(out).unwrap();
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_java_orchard::LANGUAGE.into()).unwrap();
    let mut names = Vec::new();
    let mut done = 0;
    for &i in &idx {
        if done >= count {
            break;
        }
        let Ok(src) = std::fs::read(&files[i]) else { continue };
        let Some(tree) = parser.parse(&src, None) else { continue };
        if tree.root_node().has_error() || src.len() > 200_000 {
            continue;
        }
        let mut cands = Vec::new();
        walk(tree.root_node(), &mut cands);
        // insertions: (position, kind 0 = open / 1 = close, tie-break key)
        let mut ins: Vec<(usize, u8, i64)> = Vec::new();
        for (s, e) in cands {
            if rng.below(100) < percent {
                let times = if rng.below(10) == 0 { 2 } else { 1 };
                for t in 0..times {
                    let len = (e - s) as i64;
                    // opens: larger ranges first; closes: smaller ranges first
                    ins.push((s, 0, -(len * 4 + (1 - t as i64))));
                    ins.push((e, 1, len * 4 + t as i64));
                }
            }
        }
        ins.sort();
        let mut res = Vec::with_capacity(src.len() + ins.len());
        let mut pos = 0;
        for (p, kind, _) in ins {
            res.extend_from_slice(&src[pos..p]);
            res.push(if kind == 0 { b'(' } else { b')' });
            pos = p;
        }
        res.extend_from_slice(&src[pos..]);
        done += 1;
        let path = format!("{out}/p{done:04}.java");
        std::fs::File::create(&path).unwrap().write_all(&res).unwrap();
        names.push(path);
    }
    std::fs::write(format!("{out}/list"), names.join("\n") + "\n").unwrap();
    println!("{} files", names.len());
}
