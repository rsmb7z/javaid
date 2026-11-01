#!/usr/bin/env bash
# Run the whole formal benchmark plus the parity checks. Leave the machine idle while it runs.
# usage: TUNED_SINGLE="<jvm flags>" TUNED_MULTI="<jvm flags>" formal.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TUNED_SINGLE=${TUNED_SINGLE:--XX:+UseSerialGC -Xmx512m}
TUNED_MULTI=${TUNED_MULTI:--XX:+UseParallelGC}
mkdir -p "$OUT"

echo "=== single-file latency (the pre-commit / editor / agent-hook case)"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/latency" \
  --corpus latency-one-file --slow-runs 10 --fast-runs 30 --realistic-runs 10 \
  "--tuned-single=$TUNED_SINGLE" "--tuned-multi=$TUNED_MULTI"

echo "=== whole-repository runs"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/large" \
  --corpus guava --corpus spring-framework --slow-runs 3 --fast-runs 10 --realistic-runs 1 --ci-cores 2 \
  "--tuned-single=$TUNED_SINGLE" "--tuned-multi=$TUNED_MULTI"

echo "=== parity against the oracles"
for c in guava spring-framework; do
  F="$SP/results/formal/$c"
  {
    echo "### $c: javaid vs Checkstyle 14.4.0 (oracle)"
    python3 -I "$HERE/parity.py" "$F/cs-spike.out" "$F/jv-lint-jall.out"
    echo
    echo "### $c: javaid vs PMD 7.29.0"
    python3 -I "$HERE/parity.py" "$F/pmd-spike-default.out" "$F/jv-lint-jall.out" --reference-format pmd
  } | tee "$OUT/parity-$c.txt"
done
echo "FORMAL-DONE"
