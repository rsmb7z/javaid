#!/usr/bin/env bash
# Phase 1 check: does the speed advantage survive with 21 modules (including Javadoc processing)
# instead of 5? Also re-measures javaid on the spike's 5-module config, to show the engine
# restructuring did not slow it down. Leave the machine idle while it runs.
# usage: phase1-bench.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TS='--tuned-single=-XX:+UseSerialGC -Xmx512m'
TM='--tuned-multi=-XX:+UseParallelGC'

echo "=== 21 modules: javaid and Checkstyle (default JVM), same config"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase1" \
  --corpus guava --corpus spring-framework --only jv-lint --only jv-parse --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase1.xml" --javaid-config "$HERE/oracle/phase1.xml" --rules-label "21 modules" \
  --slow-runs 3 --fast-runs 10 --skip-realistic "$TS" "$TM"

echo "=== javaid on the spike's 5-module config (engine restructuring should not slow it)"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase1-spike-config" \
  --corpus guava --corpus spring-framework --only jv-lint \
  --javaid-config "$HERE/checkstyle-spike.xml" --rules-label "5 modules (spike config)" \
  --fast-runs 10 --skip-realistic --raw-subdir phase1-spike "$TS" "$TM"

echo "=== one file (the pre-commit / editor / agent-hook case), 21 modules, javaid and Checkstyle"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase1-latency" \
  --corpus latency-one-file --only jv-lint --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase1.xml" --javaid-config "$HERE/oracle/phase1.xml" --rules-label "21 modules" \
  --slow-runs 10 --fast-runs 50 --skip-realistic --raw-subdir phase1-latency "$TS" "$TM"

echo "=== parity: all 21 modules against Checkstyle, same config (the benchmark's own corpora)"
: > "$OUT/parity-phase1.txt"
for c in guava spring-framework; do
  "$BIN" -j 16 --config "$HERE/oracle/phase1.xml" "$SP/corpus/$c" > "$SP/results/formal/$c/jv-phase1.out" 2>/dev/null || true
  {
    echo "### $c: javaid vs Checkstyle 14.4.0, 21 modules at default settings"
    python3 -I "$HERE/parity.py" "$SP/results/formal/$c/cs-spike.out" "$SP/results/formal/$c/jv-phase1.out" --ignore-reference-message 'has parse error'
    echo
  } | tee -a "$OUT/parity-phase1.txt"
done
echo "PHASE1-BENCH-DONE"
