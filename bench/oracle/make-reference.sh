#!/usr/bin/env bash
# Produce Checkstyle's reference output for an oracle config over both corpora. Slow (about a
# minute or two), so run it once per config and reuse the output while developing rules.
# usage: make-reference.sh <scratch-dir> <config.xml> <name>
set -eu
SP=$1
CFG=$2
NAME=$3
OUT="$SP/results/oracle"
mkdir -p "$OUT"
java -jar "$SP/tools/checkstyle/checkstyle-14.4.0-all.jar" -c "$CFG" \
  "$SP/corpus/guava" "$SP/corpus/spring-framework" > "$OUT/$NAME.cs.out" 2> "$OUT/$NAME.cs.err" || true
echo "$NAME: $(grep -c -E '^\[(WARN|ERROR)\]' "$OUT/$NAME.cs.out") Checkstyle findings, $(wc -l < "$OUT/$NAME.cs.err") stderr lines"
