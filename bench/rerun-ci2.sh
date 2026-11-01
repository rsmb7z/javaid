#!/usr/bin/env bash
# Second quiet follow-up: re-measure the 2-core "small CI runner" scenarios, then count how many
# UnusedImports findings Checkstyle reports with its default Javadoc handling.
# usage: rerun-ci2.sh <scratch-dir>
set -eu
SP=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$HERE")"
BIN="$ROOT/target/release/javaid"
OUT="$HERE/results"
TS='--tuned-single=-XX:+UseSerialGC -Xmx512m'
TM='--tuned-multi=-XX:+UseParallelGC'

echo "=== 2-core runner scenarios, quiet machine"
python3 -I "$HERE/run_bench.py" --scratch "$SP" --javaid "$BIN" --out "$OUT/ci2-rerun" \
  --corpus guava --corpus spring-framework --only ci2 --ci-cores 2 --slow-runs 3 --fast-runs 10 --skip-realistic "$TS" "$TM"

echo "=== Checkstyle UnusedImports with default Javadoc handling (finding counts, not timing)"
mkdir -p "$SP/results/javadoc-effect"
for c in guava spring-framework; do
  java -jar "$SP/tools/checkstyle/checkstyle-14.4.0-all.jar" -c "$HERE/checkstyle-unusedimports-default.xml" \
    "$SP/corpus/$c" > "$SP/results/javadoc-effect/$c.out" 2>/dev/null || true
  echo "$c: default (processJavadoc=true) = $(grep -c 'UnusedImports\]' "$SP/results/javadoc-effect/$c.out") findings;" \
       "oracle setting (processJavadoc=false) = $(grep -c 'UnusedImports\]' "$SP/results/formal/$c/cs-spike.out") findings"
done
echo "RERUN2-DONE"
