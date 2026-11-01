#!/usr/bin/env bash
# One-shot sanity run of the Java baselines (not the formal benchmark).
# usage: quick-baseline.sh <scratch-dir> <corpus-name>
set -u
SP=$1
C=$2
BENCH="$(cd "$(dirname "$0")" && pwd)"
CS="$SP/tools/checkstyle/checkstyle-14.4.0-all.jar"
PMDLIB="$SP/tools/pmd/pmd-bin-7.29.0/lib/*"
OUT="$SP/results/quick"
mkdir -p "$OUT"

run() {
  local name=$1
  shift
  /usr/bin/time -f "wall=%es rss=%MKB user=%Us sys=%Ss cpu=%P" -o "$OUT/$name.time" \
    "$@" >"$OUT/$name.out" 2>"$OUT/$name.err"
  local rc=$?
  echo "$name exit=$rc $(tail -1 "$OUT/$name.time") out_lines=$(wc -l <"$OUT/$name.out")"
}

pmd() { java -cp "$PMDLIB" net.sourceforge.pmd.cli.PmdCli check -d "$SP/corpus/$C" -f text --no-cache --no-progress "$@"; }

run "cs-floor-$C"      java -jar "$CS" -c "$BENCH/checkstyle-floor.xml" "$SP/corpus/$C"
run "cs-spike-$C"      java -jar "$CS" -c "$BENCH/checkstyle-spike.xml" "$SP/corpus/$C"
run "pmd-floor-t1-$C"  pmd -R "$BENCH/pmd-floor.xml" -t 1
run "pmd-spike-t1-$C"  pmd -R "$BENCH/pmd-spike.xml" -t 1
run "pmd-spike-t1C-$C" pmd -R "$BENCH/pmd-spike.xml" -t 1C
echo "QUICK-BASELINE-DONE"
