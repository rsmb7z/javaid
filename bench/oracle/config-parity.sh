#!/usr/bin/env bash
# Does javaid agree with Checkstyle on a real configuration? Cuts the config down to the module
# instances javaid implements (make-subset.py edits none of them), runs both tools over both
# corpora, and diffs the findings. Slow: Checkstyle alone takes a few minutes.
# usage: config-parity.sh <scratch-dir> <checkstyle-config.xml> <name>
#   e.g. config-parity.sh "$SP" "$SP/scripts/google_checks.xml" google
# Writes bench/oracle/<name>-implemented.xml and bench/results/parity-<name>-subset.txt.
set -eu
SP=$1
CFG=$2
NAME=$3
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$(dirname "$HERE")")"
BIN="$ROOT/target/release/javaid"
SUBSET="$HERE/$NAME-implemented.xml"
OUT="$SP/results/oracle"

JAVAID="$BIN" python3 -I "$HERE/make-subset.py" "$CFG" "$SUBSET"
"$HERE/make-reference.sh" "$SP" "$SUBSET" "$NAME-subset"
"$BIN" -j 16 --config "$SUBSET" "$SP/corpus/guava" "$SP/corpus/spring-framework" \
  > "$OUT/$NAME-subset.jv.out" 2> "$OUT/$NAME-subset.jv.err" || true
{
  echo "### javaid vs Checkstyle 14.4.0 on the instances of $NAME's configuration that javaid implements (Guava and Spring Framework)"
  python3 -I "$ROOT/bench/parity.py" "$OUT/$NAME-subset.cs.out" "$OUT/$NAME-subset.jv.out" --ignore-reference-message 'has parse error'
} | tee "$ROOT/bench/results/parity-$NAME-subset.txt"
