#!/usr/bin/env bash
# Does the speed advantage survive with all 33 modules (phase2.xml)? Same method as phase1-bench.sh:
# javaid and Checkstyle (default JVM) run the identical config, then their findings are diffed.
# Leave the machine idle while it runs.
# usage: phase2-bench.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TS='--tuned-single=-XX:+UseSerialGC -Xmx512m'
TM='--tuned-multi=-XX:+UseParallelGC'

echo "=== 33 modules: javaid and Checkstyle (default JVM), same config"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase2" \
  --corpus guava --corpus spring-framework --only jv-lint --only jv-parse --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase2.xml" --javaid-config "$HERE/oracle/phase2.xml" --rules-label "33 modules" \
  --slow-runs 3 --fast-runs 10 --skip-realistic --raw-subdir phase2 "$TS" "$TM"

echo "=== one file (the pre-commit / editor / agent-hook case), 33 modules, javaid and Checkstyle"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/phase2-latency" \
  --corpus latency-one-file --only jv-lint --only 'cs-spike$' \
  --cs-config "$HERE/oracle/phase2.xml" --javaid-config "$HERE/oracle/phase2.xml" --rules-label "33 modules" \
  --slow-runs 10 --fast-runs 50 --skip-realistic --raw-subdir phase2-latency "$TS" "$TM"

echo "=== parity: all 33 modules against Checkstyle, same config (the benchmark's own corpora)"
: > "$OUT/parity-phase2.txt"
for c in guava spring-framework; do
  "$BIN" -j 16 --config "$HERE/oracle/phase2.xml" "$SP/corpus/$c" > "$SP/results/phase2/$c/jv-phase2.out" 2>/dev/null || true
  {
    echo "### $c: javaid vs Checkstyle 14.4.0, 33 modules (phase2.xml)"
    python3 -I "$HERE/parity.py" "$SP/results/phase2/$c/cs-spike.out" "$SP/results/phase2/$c/jv-phase2.out" --ignore-reference-message 'has parse error'
    echo
  } | tee -a "$OUT/parity-phase2.txt"
done
echo "PHASE2-BENCH-DONE"
