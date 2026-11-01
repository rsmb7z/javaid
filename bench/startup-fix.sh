#!/usr/bin/env bash
# Third follow-up: "if the JVM tools get the best available startup fix, does javaid's advantage
# survive?" Leave the machine idle while it runs.
# usage: startup-fix.sh <scratch-dir> <jdk25-home>
set -eu
SP=$1
JDK25=$2
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"

echo "=== startup fixes: one file, whole repository, warm resident JVM"
python3 -I "$HERE/startup_fix.py" --scratch "$SP" --javaid "$BIN" --jdk25 "$JDK25" --out "$OUT/startup-fix"

echo "=== 2-core runner: Checkstyle in its default JVM configuration"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/ci2-cs-default" \
  --corpus guava --corpus spring-framework --only ci2-cs-default --ci-cores 2 --slow-runs 3 --skip-realistic \
  "--tuned-single=-XX:+UseSerialGC -Xmx512m" "--tuned-multi=-XX:+UseParallelGC"

echo "STARTUP-FIX-ALL-DONE"
