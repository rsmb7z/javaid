#!/usr/bin/env python3
"""Record Checkstyle's findings for a probe file as a golden fixture.

usage: record-golden.py <checkstyle.jar> <config.xml> <probe.java> <out.expected>

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
if "Audit done." not in res.stdout or "Exception" in res.stdout + res.stderr:
    sys.exit(f"Checkstyle did not complete the audit of {probe} with {config}:\n{res.stdout}\n{res.stderr}")
rx = re.compile(r"^\[(?:WARN|ERROR|INFO)\] .*?:(\d+)(?::\d+)?: .* \[(\w+)\]$")
rows = []
for line in res.stdout.splitlines():
    if line.startswith("[") and not rx.match(line):
        sys.exit(f"unparsed finding: {line}")
    m = rx.match(line)
    if m:
        rows.append((int(m.group(1)), m.group(2)))
rows.sort()
with open(out, "w") as fh:
    if not rows:
        fh.write("# no findings; Checkstyle completed the audit\n")
    for ln, rule in rows:
        fh.write(f"{ln} {rule}\n")
print(f"{probe}: {len(rows)} findings -> {out}")
