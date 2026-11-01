#!/usr/bin/env python3
"""Compare javaid's Javadoc trees with Checkstyle's, node by node (type, text, depth, line).

usage: compare.py <checkstyle-dump> <javaid-dump> [examples]

`checkstyle-dump` comes from `Dump.java`, `javaid-dump` from `tests/javadoc_tree_corpus.rs`. Comments
that Checkstyle cannot parse (it prints ERR) are skipped: javaid rejects those before building a tree.
"""
import collections
import re
import sys

cs_path, rs_path = sys.argv[1:3]
examples = int(sys.argv[3]) if len(sys.argv) > 3 else 5
cs = open(cs_path, encoding="utf-8").read().split("\x01")[1:]
rs = open(rs_path, encoding="utf-8").read().split("\x01")[1:]
assert len(cs) == len(rs), (len(cs), len(rs))
line = re.compile(r"^((?:\|   |    )*)(\|--|`--)?(\w+) -> (.*) \[(\d+):\d+\]$")


def checkstyle(record):
    rows = []
    for text in record.split("\n"):
        m = line.match(text)
        if m:
            indent, branch, ty, tx, ln = m.group(1), m.group(2), m.group(3), m.group(4), m.group(5)
            rows.append((len(indent) // 4 + (1 if branch else 0), ty, tx, int(ln)))
    return rows


def javaid(record):
    rows = []
    for text in record.split("\n"):
        if text:
            depth, ty, tx, ln = text.split("\t")
            rows.append((int(depth), ty, tx, int(ln)))
    return rows


total = bad = shown = 0
kinds = collections.Counter()
for i, (a, b) in enumerate(zip(cs, rs)):
    if a.startswith("ERR") or a.startswith("EMPTY"):
        continue
    total += 1
    x, y = checkstyle(a), javaid(b)
    if x == y:
        continue
    bad += 1
    k = next((j for j in range(min(len(x), len(y))) if x[j] != y[j]), min(len(x), len(y)))
    kinds[(x[k][:2] if k < len(x) else None, y[k][:2] if k < len(y) else None)] += 1
    if shown < examples:
        shown += 1
        print(f"--- record {i}: first difference at node {k}")
        for j in range(max(0, k - 3), min(k + 3, max(len(x), len(y)))):
            cx = x[j] if j < len(x) else None
            cy = y[j] if j < len(y) else None
            print(("  " if cx == cy else "!!"), cx, "|", cy)
print(f"compared {total} comments, {bad} differ")
for kind, n in kinds.most_common(20):
    print(n, kind)
sys.exit(1 if bad else 0)
