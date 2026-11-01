#!/usr/bin/env python3
"""Make a mutated copy of a Java source tree, to stress modules the original code never violates.

usage: mutate_corpus.py <source-dir> <destination-dir> [--seed N] [--rate 0.05]

Guava and Spring are tidy: at Checkstyle's defaults they contain one fall-through, two statements
sharing a line, no star imports and no lowercase `l` literal. A module that never fires on a
corpus has not been tested by it. So this copies every .java file and, at a fraction of the places
where it is possible, makes a small edit that stays valid Java:

  * UpperEll                       `10L`            -> `10l`
  * AvoidStarImport                `import a.b.C;`  -> `import a.b.*;`
  * ModifierOrder                  `public static`  -> `static public`, and the like
  * ArrayTypeStyle                 `int[] xs`       -> `int xs[]`
  * OneStatementPerLine,           two simple one-line statements are joined onto one line
    MultipleVariableDeclarations
  * FallThrough                    a `break;` on a line of its own is deleted
  * MissingSwitchDefault           a `default:` line is turned into a `case 0:` line

Checkstyle and javaid are then run on the *same* mutated tree, so whatever they disagree about is a
real difference in how the two read code, found where it happens in real programs. Nothing here
needs to understand Java: a mutation is a regular-expression edit of one or two lines, deterministic
for a given seed, and a file Checkstyle cannot parse afterwards simply produces no output from it.
"""
import argparse
import os
import random
import re
import sys

MODIFIER_SWAPS = [
    (re.compile(r"\bpublic static\b"), "static public"),
    (re.compile(r"\bprivate static\b"), "static private"),
    (re.compile(r"\bprotected static\b"), "static protected"),
    (re.compile(r"\bpublic final\b"), "final public"),
    (re.compile(r"\bprivate final\b"), "final private"),
    (re.compile(r"\bpublic abstract\b"), "abstract public"),
    (re.compile(r"\bstatic final\b"), "final static"),
    (re.compile(r"\bpublic synchronized\b"), "synchronized public"),
]
ELL = re.compile(r"\b((?:0[xX][0-9a-fA-F_]+|\d[\d_]*))L\b")
STAR = re.compile(r"^(import (?!static )[\w.]+)\.[A-Z]\w*;\s*$")
ARRAY = re.compile(r"^(\s*(?:(?:final|private|public|protected|static|transient|volatile)\s+)*[A-Z]\w*|\s*(?:int|long|byte|char|short|double|float|boolean))\[\] (\w+)(\s*(?:=|;))")
SIMPLE_STATEMENT = re.compile(r"^\s+[\w.\[\]()<>\", +*/!&|?:=-]+;\s*$")
BREAK = re.compile(r"^\s*break;\s*$")
DEFAULT = re.compile(r"^(\s*)default:\s*$")


def mutate(text, rng, rate):
    lines = text.split("\n")
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()
        in_comment = stripped.startswith(("*", "//", "/*"))
        if not in_comment:
            if ELL.search(line) and rng.random() < rate * 4:
                line = ELL.sub(lambda m: m.group(1) + "l", line)
            m = STAR.match(line)
            if m and rng.random() < rate * 2:
                line = m.group(1) + ".*;"
            for pattern, replacement in MODIFIER_SWAPS:
                if pattern.search(line) and rng.random() < rate:
                    line = pattern.sub(replacement, line, count=1)
                    break
            m = ARRAY.match(line)
            if m and rng.random() < rate:
                line = ARRAY.sub(lambda m: f"{m.group(1)} {m.group(2)}[]{m.group(3)}", line, count=1)
            if BREAK.match(line) and rng.random() < rate * 3:
                i += 1
                continue
            m = DEFAULT.match(line)
            if m and rng.random() < rate * 3:
                line = f"{m.group(1)}case 0:"
            if SIMPLE_STATEMENT.match(line) and i + 1 < len(lines) and SIMPLE_STATEMENT.match(lines[i + 1]) and rng.random() < rate * 2:
                line = line.rstrip() + " " + lines[i + 1].strip()
                i += 1
        out.append(line)
        i += 1
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("source")
    ap.add_argument("destination")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--rate", type=float, default=0.05)
    a = ap.parse_args()
    rng = random.Random(a.seed)
    files = changed = 0
    for root, _, names in os.walk(a.source):
        for name in sorted(names):
            if not name.endswith(".java"):
                continue
            src = os.path.join(root, name)
            dst = os.path.join(a.destination, os.path.relpath(src, a.source))
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            with open(src, encoding="utf-8", errors="surrogateescape", newline="") as fh:
                text = fh.read()
            new = mutate(text, rng, a.rate)
            with open(dst, "w", encoding="utf-8", errors="surrogateescape", newline="") as fh:
                fh.write(new)
            files += 1
            changed += new != text
    print(f"{files} files copied to {a.destination}, {changed} of them mutated", file=sys.stderr)


if __name__ == "__main__":
    main()
