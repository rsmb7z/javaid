#!/usr/bin/env python3
"""Collect every `/** ... */` comment under the given directories into one record file.

usage: extract.py <records-out> <meta-out> <dir>...

A record is the comment without its `/**` and `*/`; records are separated by U+0001. `meta-out`
gets one `file:line` per record. The comments are read by Checkstyle's Javadoc parser through
`Dump.java` and by javaid through `tests/javadoc_tree_corpus.rs`; `compare.py` diffs the trees.
"""
import os
import re
import sys

out, meta, *roots = sys.argv[1:]
comment = re.compile(r"/\*\*(.*?)\*/", re.S)
records, where = [], []
for root in roots:
    for directory, _, files in os.walk(root):
        for name in sorted(files):
            if not name.endswith(".java"):
                continue
            path = os.path.join(directory, name)
            with open(path, encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            for m in comment.finditer(text):
                body = m.group(1)
                if "\x01" in body or "\r" in body:
                    continue
                records.append(body)
                where.append(f"{path}:{text.count(chr(10), 0, m.start()) + 1}")
with open(out, "w", encoding="utf-8") as fh:
    fh.write("\x01".join(records))
with open(meta, "w") as fh:
    fh.write("\n".join(where))
print(f"{len(records)} comments")
