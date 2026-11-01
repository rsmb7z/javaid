#!/usr/bin/env python3
"""Reduce a Checkstyle configuration to the module instances javaid implements.

usage: make-subset.py <checkstyle-config.xml> <output.xml>

Nothing about the kept instances is edited: their properties, their `tokens`, their custom
<message> children and the Checker-level properties stay exactly as the original has them. Only
the instances of modules that javaid does not implement (and the filters, which are not rules) are
dropped. Running Checkstyle and javaid on the result therefore tests the configuration reader
against a real, unmodified configuration, which a hand-written probe config cannot do.

Run `javaid --list-rules` for the implemented names; this script asks the binary, so the two cannot
drift apart. Set JAVAID to point at a binary other than target/release/javaid.
"""
import os
import subprocess
import sys
import xml.etree.ElementTree as ET

HEADER = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC
    "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN"
    "https://checkstyle.org/dtds/configuration_1_3.dtd">
"""


def implemented():
    binary = os.environ.get("JAVAID", "target/release/javaid")
    out = subprocess.run([binary, "--list-rules"], capture_output=True, text=True, check=True).stdout
    return {line.strip() for line in out.splitlines() if line.strip()}


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    src, dst = sys.argv[1], sys.argv[2]
    keep = implemented()
    root = ET.parse(src).getroot()
    if root.get("name") != "Checker":
        sys.exit(f"{src}: the root module is {root.get('name')!r}, expected Checker")
    kept, dropped = [], []
    for walker in list(root):
        if walker.tag != "module":
            continue
        if walker.get("name") != "TreeWalker":
            # A module that sits directly under Checker: a file-level rule (FileTabCharacter,
            # LineLength, RegexpSingleline) or a filter. Kept when javaid implements it.
            name = walker.get("name")
            (kept if name in keep else dropped).append(name)
            if name not in keep:
                root.remove(walker)
            continue
        for child in list(walker):
            if child.tag != "module":
                continue
            name = child.get("name")
            (kept if name in keep else dropped).append(name)
            if name not in keep:
                walker.remove(child)
    ET.indent(root, space="  ")
    with open(dst, "w", encoding="utf-8") as fh:
        fh.write(HEADER + ET.tostring(root, encoding="unicode") + "\n")
    print(f"{dst}: kept {len(kept)} instances, dropped {len(dropped)}")


if __name__ == "__main__":
    main()
