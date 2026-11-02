//! `FallThrough`.
//!
//! A case group that has code and does not end in `return`, `throw`, `break`, `continue` or
//! `yield` falls into the next one, unless a comment says it is meant to. Checkstyle's rules for
//! "does not end in" are a hand-written approximation of control flow, and javaid reproduces the
//! approximation, not the Java language: every rule below was found by probing (the `Ft*` fixtures)
//! and not derived from what the code means.

use crate::engine::Ctx;
use tree_sitter::Node;

/// Checkstyle's case group: a run of case labels and the statements after them. Tree-sitter starts
/// a group at every label, so labels with nothing after them are merged into the group that follows.
struct Group<'t> {
    first_label: Node<'t>,
    /// The last statement, an empty statement (`;`) included. `None` if the group has no code,
    /// which a comment does not count as.
    last_statement: Option<Node<'t>>,
}

fn groups<'t>(switch_block: Node<'t>) -> Vec<Group<'t>> {
    let mut out: Vec<Group<'t>> = Vec::new();
    let mut cur = switch_block.walk();
    for g in switch_block.children(&mut cur).filter(|g| g.kind() == "switch_block_statement_group") {
        let mut inner = g.walk();
        let label = g.children(&mut inner).find(|c| c.kind() == "switch_label");
        let Some(label) = label else { continue };
        let mut inner = g.walk();
        // The `:` after a label is an unnamed token and not code; a lone `;` is an empty statement and is.
        let last_statement = g.children(&mut inner).filter(|c| !c.is_extra() && c.kind() != "switch_label" && (c.is_named() || c.kind() == ";")).last();
        match out.last_mut() {
            Some(prev) if prev.last_statement.is_none() => prev.last_statement = last_statement,
            _ => out.push(Group { first_label: label, last_statement }),
        }
    }
    out
}

/// The last statement of a block, where a stray `;` is a statement like any other.
fn last_statement(block: Node) -> Option<Node> {
    let mut i = block.child_count();
    while i > 0 {
        i -= 1;
        let c = block.child(i)?;
        if c.is_extra() || c.kind() == "}" {
            continue;
        }
        return (c.kind() != "{").then_some(c);
    }
    None
}

pub fn on_switch(cx: &mut Ctx, switch: Node) {
    let cfg = cx.cfg;
    let Some(opts) = &cfg.fall_through else { return };
    let Some(body) = switch.child_by_field_id(cx.syn.f.body) else { return };
    let gs = groups(body);
    for (i, g) in gs.iter().enumerate() {
        let Some(last) = g.last_statement else { continue };
        let is_last = i + 1 == gs.len();
        if (is_last && !opts.check_last_case_group) || terminated(cx, last, true, true) {
            continue;
        }
        // The next case, or for the last group the closing brace of the switch.
        let until = if is_last { body.end_byte().saturating_sub(1) } else { gs[i + 1].first_label.start_byte() };
        if has_relief(cx, body, last, until, &opts.relief) {
            continue;
        }
        // Reported where the fall-through lands, which for the last group is its own label.
        let at = if is_last { g.first_label } else { gs[i + 1].first_label };
        let message = if is_last { "Fall through from the last branch of the switch statement." } else { "Fall through from previous branch of the switch statement." };
        cx.diag("FallThrough", at, message.into());
    }
}

/// Can the statement never complete normally? `use_break` and `use_continue` say whether those
/// jumps leave the construct being judged: they do for the case group itself, not for a loop or a
/// nested switch, where they only leave that.
fn terminated(cx: &Ctx, n: Node, use_break: bool, use_continue: bool) -> bool {
    let f = &cx.syn.f;
    match n.kind() {
        "return_statement" | "throw_statement" | "yield_statement" => true,
        "break_statement" => use_break,
        "continue_statement" => use_continue,
        "block" => last_statement(n).is_some_and(|l| terminated(cx, l, use_break, use_continue)),
        "if_statement" => match (n.child_by_field_id(f.consequence), n.child_by_field_id(f.alternative)) {
            (Some(then), Some(otherwise)) => terminated(cx, then, use_break, use_continue) && terminated(cx, otherwise, use_break, use_continue),
            _ => false,
        },
        "try_statement" | "try_with_resources_statement" => {
            let mut body_and_catches = n.child_by_field_id(f.body).is_some_and(|b| terminated(cx, b, use_break, use_continue));
            let mut finally = false;
            let mut cur = n.walk();
            for c in n.named_children(&mut cur) {
                match c.kind() {
                    "catch_clause" => body_and_catches &= c.child_by_field_id(f.body).is_some_and(|b| terminated(cx, b, use_break, use_continue)),
                    "finally_clause" => finally = c.named_child(0).is_some_and(|b| terminated(cx, b, use_break, use_continue)),
                    _ => {}
                }
            }
            body_and_catches || finally
        }
        "synchronized_statement" => n.child_by_field_id(f.body).is_some_and(|b| terminated(cx, b, use_break, use_continue)),
        "labeled_statement" => last_named(n).is_some_and(|inner| terminated(cx, inner, use_break, use_continue)),
        // A nested switch ends the case if every one of its groups does, and no `default` is
        // required. One with arrow rules, or without groups, never does.
        "switch_expression" => {
            let Some(body) = n.child_by_field_id(f.body) else { return false };
            let gs = groups(body);
            !gs.is_empty() && gs.iter().all(|g| g.last_statement.is_some_and(|s| terminated(cx, s, false, use_continue)))
        }
        "while_statement" | "do_statement" | "for_statement" | "enhanced_for_statement" => {
            let body = n.child_by_field_id(f.body);
            body.is_some_and(|b| terminated(cx, b, false, false)) || (is_infinite(cx, n) && !body.is_some_and(exits_by_break))
        }
        _ => false,
    }
}

fn last_named(n: Node) -> Option<Node> {
    let mut i = n.named_child_count();
    while i > 0 {
        i -= 1;
        let c = n.named_child(i as u32)?;
        if !c.is_extra() {
            return Some(c);
        }
    }
    None
}

/// `while (true)`, `do ... while (true)`, `for (;;)`, `for (;true;)`: the condition is the literal
/// and nothing else, so `while ((true))` and `while (!false)` are ordinary loops.
fn is_infinite(cx: &Ctx, n: Node) -> bool {
    match n.kind() {
        "while_statement" | "do_statement" => n.child_by_field_id(cx.syn.f.condition).is_some_and(|c| c.named_child_count() == 1 && c.named_child(0).is_some_and(|l| l.kind() == "true")),
        "for_statement" => n.child_by_field_id(cx.syn.f.condition).is_none_or(|c| c.kind() == "true"),
        _ => false,
    }
}

/// An infinite loop can end only through a `break`, and Checkstyle looks for one only where the
/// body ends: its last statement, looking into a block, an `if` and `else`, a `try` without
/// `catch`, `synchronized` and a labelled block, but not into a nested `if`, loop or switch. An `if`
/// whose other branch returns is left alone. These are the shapes the probes found it to see.
fn exits_by_break(body: Node) -> bool {
    let last = if body.kind() == "block" { last_statement(body) } else { Some(body) };
    last.is_some_and(tail_break)
}

fn tail_break(s: Node) -> bool {
    match s.kind() {
        "break_statement" => true,
        "block" => block_ends_with_break(s),
        "if_statement" => {
            let (then, otherwise) = (s.child_by_field_name("consequence"), s.child_by_field_name("alternative"));
            // `return`/`throw` in one branch excuses a break in the other; `break` and `continue` do not.
            let jumps_out = |b: Option<Node>| b.is_some_and(always_returns);
            (then.is_some_and(branch_breaks) && !jumps_out(otherwise)) || (otherwise.is_some_and(branch_breaks) && !jumps_out(then))
        }
        "try_statement" => {
            let mut cur = s.walk();
            let has_catch = s.named_children(&mut cur).any(|c| c.kind() == "catch_clause");
            let mut cur = s.walk();
            let finally = s.named_children(&mut cur).find(|c| c.kind() == "finally_clause").and_then(|f| f.named_child(0));
            !has_catch && (s.child_by_field_name("body").is_some_and(block_ends_with_break) || finally.is_some_and(block_ends_with_break))
        }
        "synchronized_statement" => s.child_by_field_name("body").is_some_and(block_ends_with_break),
        "labeled_statement" => last_named(s).is_some_and(|inner| match inner.kind() {
            "block" => block_ends_with_break(inner),
            "labeled_statement" => tail_break(inner),
            _ => false,
        }),
        _ => false,
    }
}

fn branch_breaks(b: Node) -> bool {
    b.kind() == "break_statement" || (b.kind() == "block" && block_ends_with_break(b))
}

fn block_ends_with_break(b: Node) -> bool {
    match last_statement(b) {
        Some(l) if l.kind() == "break_statement" => true,
        Some(l) if l.kind() == "block" => block_ends_with_break(l),
        _ => false,
    }
}

/// A branch that cannot continue the loop: it returns or throws, as itself or at the end of a block.
fn always_returns(b: Node) -> bool {
    match b.kind() {
        "return_statement" | "throw_statement" => true,
        "block" => last_statement(b).is_some_and(always_returns),
        _ => false,
    }
}

/// Is there a comment that says the fall-through is intended? It has to come after the group's
/// last statement and end on the last line that has anything on it before the next case (or before
/// the closing brace), which may be that case's own line. Probed: blank lines in between are fine;
/// a comment before more code, or before another comment, is not; a multi-line comment is matched
/// as a whole; and words in code or a string do not count, only a comment does.
fn has_relief(cx: &Ctx, switch_block: Node, last: Node, until: usize, relief: &crate::config::Format) -> bool {
    let from = last.end_byte();
    // Only whitespace and comments lie between the statement and `until`, so the last line with
    // anything on it is the one where the statement or the latest comment ends.
    let comment_between = |c: Node| c.is_extra() && matches!(c.kind(), "line_comment" | "block_comment") && c.start_byte() >= from && c.end_byte() <= until;
    let for_each_candidate = |f: &mut dyn FnMut(Node)| {
        let mut cur = switch_block.walk();
        for child in switch_block.children(&mut cur) {
            if comment_between(child) {
                f(child);
            } else if child.kind() == "switch_block_statement_group" {
                let mut inner = child.walk();
                for c in child.children(&mut inner).filter(|c| comment_between(*c)) {
                    f(c);
                }
            }
        }
    };
    let (mut last_end, mut last_row) = (from, last.end_position().row);
    for_each_candidate(&mut |c| {
        if c.end_byte() > last_end {
            (last_end, last_row) = (c.end_byte(), c.end_position().row);
        }
    });
    let mut found = false;
    for_each_candidate(&mut |c| found |= c.end_position().row == last_row && relief.matches(cx.text(c)));
    found
}
