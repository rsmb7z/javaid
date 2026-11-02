#!/usr/bin/env python3
"""Record Checkstyle's findings for a probe file as a golden fixture.

usage: record-golden.py <checkstyle.jar> <config.xml> <probe.java> <out.expected>

Next to `<out>.expected` it writes `<out>.expected2`, the richer form the probe test compares:
one `<line>:<col> <Name> <message>` row per finding (`<line>:0` when Checkstyle prints no column),
sorted by line, column and name. The column and message are exactly what Checkstyle printed.

Each line of the fixture is `<line> <Name>`, sorted, where Name is what Checkstyle prints in
brackets: the module name, or the module's `id` if the configuration gives it one. The fixture is
what the integration test `tests/oracle_probes.rs` requires javaid to reproduce. Re-record only
when upgrading the pinned Checkstyle version, and review the diff: a changed line is a behaviour
change in Checkstyle.

A run that did not complete the audit (a broken configuration prints an exception and no findings)
is refused, so an empty fixture always means "Checkstyle looked and found nothing", and says so.
"""
import re
import subprocess
import sys

jar, config, probe, out = sys.argv[1:5]
res = subprocess.run(["java", "-jar", jar, "-c", config, probe], capture_output=True, text=True)
# A finding's message may name an exception class; only a line that is not a finding counts.
broken = [l for l in (res.stdout + res.stderr).splitlines() if "Exception" in l and not l.startswith(("[WARN]", "[ERROR]", "[INFO]"))]
if "Audit done." not in res.stdout or broken:
    sys.exit(f"Checkstyle did not complete the audit of {probe} with {config}:\n{res.stdout}\n{res.stderr}")
rx = re.compile(r"^\[(?:WARN|ERROR|INFO)\] .*?:(\d+)(?::(\d+))?: (.*) \[(\w+)\]$")
rows = []
for line in res.stdout.splitlines():
    if line.startswith("[") and not rx.match(line):
        sys.exit(f"unparsed finding: {line}")
    m = rx.match(line)
    if m:
        rows.append((int(m.group(1)), m.group(4), int(m.group(2) or 0), m.group(3)))
rich = sorted((l, c, r, msg) for l, r, c, msg in rows)
rows = sorted((l, r) for l, r, _c, _m in rows)
with open(out, "w") as fh:
    if not rows:
        fh.write("# no findings; Checkstyle completed the audit\n")
    for ln, rule in rows:
        fh.write(f"{ln} {rule}\n")
with open(out + "2", "w") as fh:
    if not rich:
        fh.write("# no findings; Checkstyle completed the audit\n")
    for ln, col, rule, msg in rich:
        fh.write(f"{ln}:{col} {rule} {msg}\n")
print(f"{probe}: {len(rows)} findings -> {out}")
