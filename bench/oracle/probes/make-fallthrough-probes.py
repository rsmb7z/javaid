#!/usr/bin/env python3
"""Generate the probe files for FallThrough.

usage: make-fallthrough-probes.py [output-directory]    (default: the directory of this script)

Every case is the body of a `switch` statement, wrapped in its own method inside a labelled loop so
that `continue` and `break label` are legal. `FtInf.java` crosses a set of loop bodies with the
three infinite-loop forms (`while (true)`, `for (;;)`, `do ... while (true)`). Checkstyle's answers
for the generated files are recorded by record-tree-goldens.sh.

NOT_MODELLED lists loop bodies whose answer comes from internals of Checkstyle that javaid does not
reproduce (see the notes in src/rules/fallthrough.rs); they are kept out of the committed probes so
that nothing here claims agreement that does not exist.
"""
import os
import sys

out = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.abspath(__file__))

GROUPS = {
    "FtProbe1": """case 1:
    foo();
case 2:
    foo();
    break;
---
case 1:
    foo();
    break;
case 2:
    foo();
    return;
case 3:
    foo();
    throw new RuntimeException();
case 4:
    foo();
    continue;
case 5:
    foo();
    continue loop;
case 6:
    foo();
    break loop;
default:
    foo();
---
case 1:
case 2:
    foo();
    break;
case 3:
    // just a comment
case 4:
    foo();
---
case 1:
    foo();
    // fall through
case 2:
    foo();
    break;
---
case 1:
    foo(); // falls through
case 2:
    foo();
    break;
---
case 1:
    foo();

    // fall through

case 2:
    foo();
    break;
---
case 1:
    foo();
    /* fall through */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /* fall through */ case 2:
    foo();
    break;
---
case 1:
    foo(); /* fallthrough */
case 2:
    foo();
    break;
---
case 1:
    foo();
    // $FALL-THROUGH$
case 2:
    foo();
    break;
---
case 1:
    foo();
    // Fall Through
case 2:
    foo();
    break;
---
case 1:
    foo();
    // FALL THROUGH
case 2:
    foo();
    break;
---
case 1:
    foo();
    // falls-thru
case 2:
    foo();
    // fallthru
case 3:
    foo();
    // fall-thru
case 4:
    foo();
    // falls thru
case 5:
    foo();
    // fall thru
case 6:
    foo();
    // falls through
case 7:
    foo();
    // fallsthrough
case 8:
    foo();
    // fallsthru
case 9:
    foo();
    break;""",
    "FtProbe2": """case 1:
    { foo(); break; }
case 2:
    foo();
---
case 1:
    if (b) { return; } else { break; }
case 2:
    foo();
---
case 1:
    if (b) { return; }
case 2:
    foo();
---
case 1:
    if (b) return; else break;
case 2:
    foo();
---
case 1:
    if (b) { foo(); } else if (y > 1) { return; } else { return; }
case 2:
    foo();
---
case 1:
    if (b) { foo(); } else if (y > 1) { return; }
case 2:
    foo();
---
case 1:
    try { return; } finally { foo(); }
case 2:
    foo();
---
case 1:
    try { foo(); } finally { return; }
case 2:
    foo();
---
case 1:
    try { return; } catch (RuntimeException e) { return; }
case 2:
    foo();
---
case 1:
    try { return; } catch (RuntimeException e) { foo(); }
case 2:
    foo();
---
case 1:
    try { foo(); } catch (RuntimeException e) { return; }
case 2:
    foo();
---
case 1:
    try { return; } catch (RuntimeException e) { return; } finally { foo(); }
case 2:
    foo();
---
case 1:
    synchronized (this) { return; }
case 2:
    foo();
---
case 1:
    synchronized (this) { foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; default: return; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; default: foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: break; default: return; }
case 2:
    foo();
---
case 1:
    while (true) { return; }
case 2:
    foo();
---
case 1:
    while (b) { return; }
case 2:
    foo();
---
case 1:
    for (;;) { return; }
case 2:
    foo();
---
case 1:
    do { return; } while (b);
case 2:
    foo();
---
case 1:
    do { foo(); } while (true);
case 2:
    foo();
---
case 1:
    for (int i = 0; i < 1; i++) { return; }
case 2:
    foo();
---
case 1:
    for (int i : new int[0]) { return; }
case 2:
    foo();
---
case 1:
    lbl: { break lbl; }
case 2:
    foo();
---
case 1:
    lbl: return;
case 2:
    foo();
---
case 1:
    System.exit(0);
case 2:
    foo();
---
case 1:
    assert b;
case 2:
    foo();
---
case 1:
    ;
case 2:
    foo();
---
case 1:
    foo();
    ;
case 2:
    foo();
---
case 1:
    return;
    foo();
case 2:
    foo();
---
case 1:
    break;
    // fall through
case 2:
    foo();
---
case 1:
    foo();
    break;
    foo();
case 2:
    foo();
---
case 1:
    int a = 1;
case 2:
    foo();
---
case 1:
    class L {}
case 2:
    foo();
---
case 1:
    Runnable r = () -> { return; };
case 2:
    foo();
---
case 1:
    return; // x
case 2:
    foo();""",
    "FtProbe3": """case 1:
    while (b) { foo(); }
case 2:
    foo();
---
case 1:
    while (true) { foo(); }
case 2:
    foo();
---
case 1:
    for (;;) { foo(); }
case 2:
    foo();
---
case 1:
    for (int i = 0; i < 1; i++) { foo(); }
case 2:
    foo();
---
case 1:
    for (int i : new int[0]) { foo(); }
case 2:
    foo();
---
case 1:
    do { foo(); } while (b);
case 2:
    foo();
---
case 1:
    do { foo(); } while (true);
case 2:
    foo();
---
case 1:
    while (b) { break; }
case 2:
    foo();
---
case 1:
    while (b) { continue; }
case 2:
    foo();
---
case 1:
    while (b) foo();
case 2:
    foo();
---
case 1:
    while (b) return;
case 2:
    foo();
---
case 1:
    lbl: foo();
case 2:
    foo();
---
case 1:
    lbl: { foo(); }
case 2:
    foo();
---
case 1:
    lbl: while (b) { foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; case 2: foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { default: return; }
case 2:
    foo();
---
case 1:
    switch (y) { default: foo(); }
case 2:
    foo();
---
case 1:
    switch (y) { }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: continue; default: continue; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: continue; default: break; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; case 2: return; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: case 2: return; }
case 2:
    foo();
---
case 1:
    switch (y) { case 1: return; case 2: case 3: }
case 2:
    foo();
---
case 1:
    switch (y) { case 1 -> { return; } default -> { return; } }
case 2:
    foo();
---
case 1:
    switch (y) { case 1 -> { return; } default -> { foo(); } }
case 2:
    foo();
---
case 1:
    if (b) break; else if (y > 1) break; else foo();
case 2:
    foo();
---
case 1:
    if (b) { return; } else { continue; }
case 2:
    foo();
---
case 1:
    if (b) { throw new RuntimeException(); } else { return; }
case 2:
    foo();
---
case 1:
    try { return; } catch (RuntimeException e) { break; }
case 2:
    foo();
---
case 1:
    try (java.io.StringReader r = null) { return; }
case 2:
    foo();
---
case 1:
    try (java.io.StringReader r = null) { foo(); }
case 2:
    foo();
---
case 1:
    try { foo(); } catch (RuntimeException e) { return; } finally { return; }
case 2:
    foo();
---
case 1:
    try { foo(); } catch (RuntimeException e) { foo(); } finally { return; }
case 2:
    foo();
---
case 1:
    try { return; } finally { }
case 2:
    foo();
---
case 1:
    { { return; } }
case 2:
    foo();
---
case 1:
    { return; foo(); }
case 2:
    foo();
---
case 1:
    {}
case 2:
    foo();
---
case 1:
    foo();
    {}
case 2:
    foo();""",
    "FtProbe4": """case 1:
    while (true) { break; }
case 2:
    foo();
---
case 1:
    while (true) foo();
case 2:
    foo();
---
case 1:
    for (int i = 0; ; i++) { foo(); }
case 2:
    foo();
---
case 1:
    for (;true;) { foo(); }
case 2:
    foo();
---
case 1:
    while (1 < 2) { foo(); }
case 2:
    foo();
---
case 1:
    while ((true)) { foo(); }
case 2:
    foo();
---
case 1:
    while (!false) { foo(); }
case 2:
    foo();
---
case 1:
    do { foo(); } while ((true));
case 2:
    foo();
---
case 1:
    do { foo(); } while (!true);
case 2:
    foo();
---
case 1:
    for (;;) foo();
case 2:
    foo();
---
case 1:
    foo();
case 2:
    foo();
    // fall through
case 3:
    foo();
    break;
---
case 1:
    foo();
    // fall through
    foo();
case 2:
    foo();
    break;
---
case 1:
    foo();
    // fall through
    // another comment
case 2:
    foo();
    break;
---
case 1:
    foo();
    // another comment
    // fall through
case 2:
    foo();
    break;
---
case 1:
    // fall through
    foo();
case 2:
    foo();
    break;
---
case 1:
    foo();
case 2: // fall through
    foo();
    break;
---
case 1:
    foo();
    /*
     * fall through
     */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /** falls through */
case 2:
    foo();
    break;
---
case 1:
    foo();
    // this falls through to the next
case 2:
    foo();
    break;
---
case 1:
    foo();
    // nothing to see
    /* falls thru */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /* falls thru */ // other
case 2:
    foo();
    break;
---
case 1: {
    foo();
    // fall through
}
case 2:
    foo();
    break;
---
case 1: {
    foo();
}
// fall through
case 2:
    foo();
    break;
---
case 1:
    if (b) {
        foo();
        // fall through
    }
case 2:
    foo();
    break;
---
case 1:
    foo();
    // fall through
default:
    foo();
---
case 1:
    foo();
default:
    foo();
---
case 1:
    foo();
case 2: case 3:
    foo();
    break;
---
case 1:
    foo();
case 2:
case 3:
    foo();
    break;
---
case 1:
    foo();
    break;
default:
    foo();
case 2:
    foo();
    break;
---
case 1:
    foo();
    break;
case 2:
    foo();
---
case 1:
    foo();
    break;
default:
    foo();
---
case 1:
    foo();
    break;
case 2:
    foo();
    // fall through
---
case 1:
    foo();
    break;
case 2:
    foo();
    return;""",
    "FtProbe5": """case 1:
    foo();
    /* fall through
       and more */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /*
       fall through */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /*
     * fall through
     * and so on
     */
case 2:
    foo();
    break;
---
case 1:
    foo();
    /*
     * fall through
     */
    // another
case 2:
    foo();
    break;
---
case 1:
    foo();
    // a
    // fall through
    // b
case 2:
    foo();
    break;
---
case 1:
    foo();
    fallthrough();
case 2:
    foo();
    break;
---
case 1:
    foo();
    String s = "falls through";
case 2:
    foo();
    break;
---
case 1:
    foo(); /* falls through */ foo();
case 2:
    foo();
    break;
---
case 1:
    foo();
    // fall through
    ;
case 2:
    foo();
    break;
---
case 1:
    foo();
    // fall through

    // (blank line above)
case 2:
    foo();
    break;
---
case 1:
    foo();
    /* fall through */

case 2:
    foo();
    break;
---
case 1:
    while (true) { if (b) break; foo(); }
case 2:
    foo();
---
case 1:
    while (true) { for (;;) { break; } }
case 2:
    foo();
---
case 1:
    while (true) { switch (y) { case 1: break; } }
case 2:
    foo();
---
case 1:
    while (true) { lbl2: { break lbl2; } }
case 2:
    foo();
---
case 1:
    lbl3: while (true) { while (b) { break lbl3; } }
case 2:
    foo();
---
case 1:
    while (true) { continue; }
case 2:
    foo();
---
case 1:
    while (true) { while (b) { break; } }
case 2:
    foo();
---
case 1:
    for (;;) { if (b) { break; } }
case 2:
    foo();
---
case 1:
    do { if (b) break; } while (true);
case 2:
    foo();
---
case 1:
    while (true) { Runnable r = () -> { }; }
case 2:
    foo();
---
case 1:
    int a = switch (y) { case 1: yield 1; default: yield 2; };
case 2:
    foo();""",
}

INF_BODIES = ['break;', 'foo(); break;', 'break; foo();', 'if (b) break;', 'if (b) { break; }', 'if (b) break; foo();', 'if (b) { break; } foo();', 'foo(); if (b) break;', 'foo(); if (b) { break; }', 'if (b) { foo(); } else { break; }', 'lbl: { break lbl; }', 'lbl: { break lbl; } foo();', '{ break; }', '{ break; } foo();', 'try { break; } finally { }', 'foo(); try { break; } finally { } foo();', 'synchronized (this) { break; }', 'switch (y) { case 1: break; }', 'if (b) return;', 'if (b) break; else foo();', 'if (b) { foo(); break; }', 'foo(); foo();', 'if (b) break; else break;', 'if (b) { if (y > 1) break; }', 'while (b) { break; }', 'foo(); { break; }', 'try { foo(); } catch (RuntimeException e) { break; }', 'if (b) { foo(); } else if (y > 1) { break; }', 'continue;', 'if (b) continue;', 'foo(); continue;', '{ { break; } }', 'foo(); { { break; } }', 'if (b) { { break; } }', 'if (b) ; else break;', 'if (b) foo(); else { break; }', 'try { { break; } } finally { }', 'try { break; } catch (RuntimeException e) { }', 'try { foo(); } finally { break; }', 'try { break; } catch (RuntimeException e) { foo(); } finally { foo(); }', 'lbl: if (b) break;', 'lbl: lbl2: { break; }', 'lbl: { foo(); break lbl; }', 'synchronized (this) { foo(); break; }', 'synchronized (this) { if (b) break; }', 'if (b) { foo(); break; } else { foo(); }', 'if (b) { break; } else if (y > 1) { foo(); }', 'if (b) { foo(); } else { if (y > 1) break; }', 'switch (y) { default: break; }', 'for (int i = 0; i < 1; i++) { break; }', 'do { break; } while (b);', 'break; break;', 'foo(); break; ;', 'return;', 'throw new RuntimeException();', 'if (b) return; else break;', 'if (b) { return; } else { foo(); }', 'if (b) return; else break;', 'if (b) break; else return;', 'if (b) throw new RuntimeException(); else break;', 'if (b) continue; else break;', 'if (b) foo(); else break;', 'if (b) { return; } else { break; }', 'if (b) { break; } else { return; }', 'if (b) return; else foo();', 'if (b) break; else continue;', 'if (b) return; else { foo(); break; }', 'if (b) return; else if (y > 1) break; else foo();', 'if (b) { foo(); return; } else { foo(); break; }', 'if (b) { foo(); } else { foo(); }', 'if (b) { foo(); break; } else { return; }', 'foo(); if (b) return; else break;', '{ if (b) return; else break; }', 'if (b) { if (y > 1) return; else break; }', 'if (b) return; else { break; }', 'if (b) { return; } else break;', 'if (b) foo(); else if (y > 1) break; else break;', 'if (b) foo(); else if (y > 1) return; else break;', 'if (b) return; else if (y > 1) return; else break;', 'if (b) return; else if (y > 1) break; else break;', 'if (b) break; else if (y > 1) return; else foo();', 'if (b) foo(); else if (y > 1) foo(); else break;', 'if (b) foo(); else if (y > 1) break; else foo();', 'if (b) { foo(); } else if (y > 1) { foo(); } else { break; }', 'if (b) { if (y > 1) break; else foo(); }', 'if (b) { if (y > 1) foo(); else break; }', 'if (b) { if (y > 1) return; else foo(); }', '{ if (b) foo(); else break; }', '{ if (b) break; else foo(); }', '{ if (b) break; }', '{ { if (b) foo(); else break; } }', 'foo(); { if (b) return; else break; }', 'if (b) { foo(); } else { if (y > 1) foo(); else break; }', 'if (b) { foo(); } else { if (y > 1) return; else break; }', 'try { if (b) foo(); else break; } finally { }', 'synchronized (this) { if (b) foo(); else break; }']

NOT_MODELLED = ['{ if (b) return; else break; }', 'if (b) { if (y > 1) return; else break; }', 'if (b) foo(); else if (y > 1) break; else break;', 'if (b) foo(); else if (y > 1) return; else break;', 'foo(); { if (b) return; else break; }', 'if (b) { foo(); } else { if (y > 1) return; else break; }']

HEADER = ["package ok.pkg;", "", "public class %s {", "    enum Color { RED, GREEN }", "    void foo() {}", "    void bar() {}", "    int val() { return 0; }"]


def write(name, text):
    with open(os.path.join(out, name + ".java"), "w", encoding="utf-8", newline="") as fh:
        fh.write(text)


def method(i, body_lines):
    return (
        [f"    void m{i}(int x, int y, Color c, boolean b) {{", "        loop: for (;;) {", "        switch (x) {"]
        + ["            " + l if l.strip() else l for l in body_lines]
        + ["        }", "        }", "    }"]
    )


for name, text in GROUPS.items():
    cases = [c.strip("\n") for c in text.split("\n---\n") if c.strip()]
    lines = [h % name if "%s" in h else h for h in HEADER]
    for i, c in enumerate(cases):
        lines += method(i, c.split("\n"))
    write(name, "\n".join(lines + ["}"]) + "\n")

# infinite loops: every body in each of the three forms, followed by a second case group
kinds = ["while (true) { %s }", "for (;;) { %s }", "do { %s } while (true);"]
lines = [h % "FtInf" if "%s" in h else h for h in HEADER]
i = 0
for body in INF_BODIES:
    if body in NOT_MODELLED:
        continue
    for kind in kinds:
        lines += method(i, ["case 1:", "    " + kind % body, "case 2:", "    foo();", "    break;"])
        i += 1
write("FtInf", "\n".join(lines + ["}"]) + "\n")

# yield terminates a group of a switch expression; the rule applies to expressions too
write("FtYield", """package ok.pkg;

public class FtYield {
    void foo() {}
    int f(int x) {
        int r = switch (x) {
            case 1:
                foo();
                yield 1;
            case 2:
                foo();
            case 3:
                yield 3;
            default:
                yield 4;
        };
        switch (x) {
            case 1 -> foo();
            case 2 -> { foo(); }
            default -> foo();
        }
        return r;
    }
}
""")
