#!/usr/bin/env bash
# Does the speed advantage survive with all 42 modules (phase3.xml)? Same method as phase1-bench.sh:
# javaid and Checkstyle (default JVM) run the identical config, then their findings are diffed.
# Leave the machine idle while it runs.
# usage: phase3-bench.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TS='--tuned-single=-XX:+UseSerialGC -Xmx512m'
TM='--tuned-multi=-XX:+UseParallelGC'

echo "=== 42 modules: javaid and Checkstyle (default JVM), same config"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase3" \
  --corpus guava --corpus spring-framework --only jv-lint --only jv-parse --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase3.xml" --javaid-config "$HERE/oracle/phase3.xml" --rules-label "42 modules" \
  --slow-runs 3 --fast-runs 10 --skip-realistic --raw-subdir phase3 "$TS" "$TM"

echo "=== one file (the pre-commit / editor / agent-hook case), 42 modules, javaid and Checkstyle"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase3-latency" \
  --corpus latency-one-file --only jv-lint --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase3.xml" --javaid-config "$HERE/oracle/phase3.xml" --rules-label "42 modules" \
  --slow-runs 10 --fast-runs 50 --skip-realistic --raw-subdir phase3-latency "$TS" "$TM"

echo "=== parity: all 42 modules against Checkstyle, same config (the benchmark's own corpora)"
: > "$OUT/parity-phase3.txt"
for c in guava spring-framework; do
  "$BIN" -j 16 --config "$HERE/oracle/phase3.xml" "$SP/corpus/$c" > "$SP/results/phase3/$c/jv-phase3.out" 2>/dev/null || true
  {
    echo "### $c: javaid vs Checkstyle 14.4.0, 42 modules (phase3.xml)"
    python3 -I "$HERE/parity.py" "$SP/results/phase3/$c/cs-spike.out" "$SP/results/phase3/$c/jv-phase3.out" --ignore-reference-message 'has parse error'
    echo
  } | tee -a "$OUT/parity-phase3.txt"
done
echo "PHASE2-BENCH-DONE"
