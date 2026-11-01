#!/usr/bin/env bash
# Follow-up to formal.sh: some JVM scenarios showed wide run-to-run variance (the machine was
# not idle the whole time), and javaid gained a rule fix after the formal run. This re-measures
# (1) the final javaid binary on both corpora and (2) Checkstyle on Spring, with nothing else
# running. usage: rerun-quiet.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TS='--tuned-single=-XX:+UseSerialGC -Xmx512m'
TM='--tuned-multi=-XX:+UseParallelGC'

echo "=== final javaid binary, both corpora"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/final-binary" \
  --corpus guava --corpus spring-framework --only jv-parse --only jv-lint --fast-runs 10 --skip-realistic "$TS" "$TM"

echo "=== Checkstyle on Spring, quiet machine"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/spring-checkstyle-rerun" \
  --corpus spring-framework --only cs-spike --slow-runs 3 --skip-realistic "$TS" "$TM"

echo "RERUN-DONE"
