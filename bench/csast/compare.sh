#!/usr/bin/env bash
# Compares javaid's emulation of Checkstyle's AST with Checkstyle's own `-t` dump.
#
# usage: compare.sh [-t|-T] [-n FAILURES] [-j THREADS] [-k] LIST
#   LIST         a file with one .java path per line
#   -t / -T      AST without (default) / with comment nodes
#   -n N         show the first differing lines of the first N failures (default 10)
#   -j N         Checkstyle JVM threads (default 4; keep it at 6 or fewer on a shared machine)
#   -k           keep the work directory
#
# environment:
#   JAR          checkstyle-14.4.0-all.jar (required)
#   JAVA_HOME    a JDK (or have java/javac on PATH)
#   JAVAID       the javaid binary (default: target/release/javaid next to this script's repo)
#   ORACLE_CACHE directory in which Checkstyle's dumps are cached between runs (keyed by the list's
#                content hash and mode); reuse makes repeated comparisons take seconds
#
# Checkstyle's JVM is started once (BatchDump.java), not once per file.
set -euo pipefail

mode=t; nfail=10; threads=4; keep=0
while getopts "tTn:j:k" opt; do
  case $opt in
    t) mode=t ;;
    T) mode=T ;;
    n) nfail=$OPTARG ;;
    j) threads=$OPTARG ;;
    k) keep=1 ;;
    *) exit 2 ;;
  esac
done
shift $((OPTIND - 1))
list=${1:?usage: compare.sh [-t|-T] [-n N] [-j N] [-k] LIST}
: "${JAR:?set JAR to checkstyle-14.4.0-all.jar}"
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)
javaid=${JAVAID:-$repo/target/release/javaid}
[ -x "$javaid" ] || { echo "no javaid binary at $javaid (set JAVAID)" >&2; exit 2; }
if [ -n "${JAVA_HOME:-}" ]; then export PATH=$JAVA_HOME/bin:$PATH; fi

work=$(mktemp -d)
[ "$keep" = 1 ] || trap 'rm -rf "$work"' EXIT
mkdir -p "$work/cls"
javac -cp "$JAR" -d "$work/cls" "$here/BatchDump.java"

key=$(sha1sum "$list" | cut -c1-12)-$mode
if [ -n "${ORACLE_CACHE:-}" ] && [ -d "$ORACLE_CACHE/$key" ]; then
  oracle=$ORACLE_CACHE/$key
  echo "using cached Checkstyle dumps in $oracle"
else
  oracle=$work/oracle
  echo "running Checkstyle on $(wc -l < "$list") files ($threads threads)..."
  java -Xmx3g -cp "$JAR:$work/cls" BatchDump "$mode" "$list" "$oracle" "$threads"
  if [ -n "${ORACLE_CACHE:-}" ]; then mkdir -p "$ORACLE_CACHE" && cp -r "$oracle" "$ORACLE_CACHE/$key"; fi
fi

mine=$work/mine
echo "running javaid..."
if [ "$mode" = T ]; then flag=--dump-ast-comments-list; else flag=--dump-ast-list; fi
"$javaid" "$flag" "$list" --dump-ast-out "$mine" .   # the path is required by the CLI and ignored

python3 -I - "$list" "$oracle" "$mine" "$nfail" "$mode" <<'PY'
import os, sys
lst, oracle, mine, nfail, mode = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), sys.argv[5]
files = [l for l in open(lst, encoding="utf-8").read().split("\n") if l]
same = 0
fails = []
for i, f in enumerate(files):
    name = "%06d" % i
    o = os.path.join(oracle, name + ".txt")
    oerr = os.path.join(oracle, name + ".err")
    m = os.path.join(mine, name + ".txt")
    merr = os.path.join(mine, name + ".err")
    if os.path.exists(oerr):
        fails.append((f, "checkstyle failed: " + open(oerr, encoding="utf-8", errors="replace").read()[:200], None, None))
        continue
    if not os.path.exists(m):
        msg = open(merr, encoding="utf-8", errors="replace").read()[:300] if os.path.exists(merr) else "no output"
        fails.append((f, "javaid failed: " + msg, None, None))
        continue
    a = open(o, "rb").read()
    b = open(m, "rb").read()
    if a == b:
        same += 1
        continue
    al = a.decode("utf-8", "replace").split("\n")
    bl = b.decode("utf-8", "replace").split("\n")
    n = 0
    while n < len(al) and n < len(bl) and al[n] == bl[n]:
        n += 1
    fails.append((f, "first difference at line %d" % (n + 1), al[max(0, n - 2):n + 3], bl[max(0, n - 2):n + 3]))
print("identical: %d of %d (%.3f%%)" % (same, len(files), 100.0 * same / max(1, len(files))))
print("different: %d" % len(fails))
for f, why, exp, got in fails[:nfail]:
    print("---", f)
    print("   ", why)
    if exp is not None:
        print("    checkstyle:")
        for l in exp: print("      " + l)
        print("    javaid:")
        for l in got: print("      " + l)
PY
