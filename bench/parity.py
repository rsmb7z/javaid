#!/usr/bin/env python3
"""Compare Checkstyle (or PMD) output with javaid output, rule by rule.

usage: parity.py <reference.out> <javaid.out> [--reference-format checkstyle|pmd]
                 [--detail RULE] [--limit N] [--ignore-reference-message REGEX]

Violations are matched on (file, line, rule). Columns are ignored: Checkstyle expands tabs and
anchors some reports on a different token, which is not a semantic difference.

Nothing is dropped silently. A reference line that looks like a finding but does not parse is
counted and reported; a line is ignored only if --ignore-reference-message says so, and the
number ignored is printed.
"""
import argparse
import collections
import os
import re
import sys

# Checkstyle plain format. The column is optional: some messages (Javadoc parse errors) have none.
CS = re.compile(r"^\[(?:WARN|ERROR|INFO)\] (?P<file>.*?):(?P<line>\d+)(?::\d+)?: (?P<msg>.*) \[(?P<rule>\w+)\]$")
JV = re.compile(r"^(?P<file>.*?):(?P<line>\d+)(?::\d+)?: (?P<rule>\w+): (?P<msg>.*)$")
# PMD text format: <file>:<line>:<TAB><Rule>:<TAB><message>  (no column)
PMD = re.compile(r"^(?P<file>.*?):(?P<line>\d+):\t(?P<rule>\w+):\t(?P<msg>.*)$")
PMD_TO_JAVAID = {
    "EmptyCatchBlock": "EmptyCatchBlock",
    "ControlStatementBraces": "NeedBraces",
    "OverrideBothEqualsAndHashcode": "EqualsHashCode",
    "UnnecessaryImport": "UnusedImports",
    "MethodNamingConventions": "MethodName",
}


def load(path, rx, rename=None, ignore=None, looks_like_finding=None):
    out = collections.defaultdict(collections.Counter)
    ignored, unparsed = 0, []
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            line = line.rstrip("\n")
            m = rx.match(line)
            if not m:
                if looks_like_finding and looks_like_finding(line):
                    unparsed.append(line)
                continue
            if ignore and ignore.search(m.group("msg")):
                ignored += 1
                continue
            rule = m.group("rule")
            if rename:
                rule = rename.get(rule, rule)
            out[rule][(m.group("file"), int(m.group("line")))] += 1
    return out, ignored, unparsed


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("reference", help="Checkstyle or PMD output")
    ap.add_argument("javaid")
    ap.add_argument("--reference-format", choices=["checkstyle", "pmd"], default="checkstyle")
    ap.add_argument("--detail", help="print example mismatches for this rule")
    ap.add_argument("--limit", type=int, default=8)
    ap.add_argument("--ignore-reference-message", help="drop reference findings whose message matches this regex (counted and reported)")
    args = ap.parse_args()
    ignore = re.compile(args.ignore_reference_message) if args.ignore_reference_message else None

    if args.reference_format == "pmd":
        cs, ignored, unparsed = load(args.reference, PMD, PMD_TO_JAVAID, ignore)
    else:
        cs, ignored, unparsed = load(args.reference, CS, None, ignore, lambda l: l.startswith("[WARN]") or l.startswith("[ERROR]"))
    jv, _, _ = load(args.javaid, JV)

    rules = sorted(set(cs) | set(jv))
    print(f"{'rule':<28}{'reference':>10}{'javaid':>9}{'agree':>8}{'only-ref':>10}{'only-jv':>9}{'agreement':>11}")
    t_ref = t_jv = t_agree = t_a = t_b = 0
    for r in rules:
        a, b = cs[r], jv[r]
        common = sum((a & b).values())
        only_a = sum((a - b).values())
        only_b = sum((b - a).values())
        union = common + only_a + only_b
        print(f"{r:<28}{sum(a.values()):>10}{sum(b.values()):>9}{common:>8}{only_a:>10}{only_b:>9}{(common / union if union else 1):>11.1%}")
        t_ref, t_jv, t_agree, t_a, t_b = t_ref + sum(a.values()), t_jv + sum(b.values()), t_agree + common, t_a + only_a, t_b + only_b
    union = t_agree + t_a + t_b
    print(f"{'TOTAL':<28}{t_ref:>10}{t_jv:>9}{t_agree:>8}{t_a:>10}{t_b:>9}{(t_agree / union if union else 1):>11.1%}")
    if ignored:
        print(f"ignored {ignored} reference finding(s) matching /{args.ignore_reference_message}/")
    if unparsed:
        print(f"WARNING: {len(unparsed)} reference line(s) look like findings but were not understood, for example:")
        for l in unparsed[:3]:
            print("   ", l[:160])

    if args.detail:
        a, b = cs[args.detail], jv[args.detail]
        files = [f for f, _ in list(a) + list(b)]
        prefix = os.path.commonpath(files) if files else ""
        for title, diff in (("only in reference", a - b), ("only in javaid", b - a)):
            print(f"\n--- {args.detail}: {title} ({sum(diff.values())})")
            for (f, ln), n in sorted(diff.items())[: args.limit]:
                print(f"  {os.path.relpath(f, prefix)}:{ln}" + (f" x{n}" if n > 1 else ""))


if __name__ == "__main__":
    sys.exit(main())
