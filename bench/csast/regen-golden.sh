#!/usr/bin/env bash
# Regenerates bench/csast/golden/ and golden-comments/ from Checkstyle's own output: one `checkstyle -t`
# (respectively `-T`, with comment nodes) dump per snippet in bench/csast/snippets/ and
# bench/csast/snippets-layout/ (golden/<dir>/<name>.txt). The Rust test tests/csast_golden.rs checks
# that javaid reproduces every file byte for byte.
#
# usage: JAR=checkstyle-14.4.0-all.jar [JAVA_HOME=...] regen-golden.sh
set -euo pipefail
: "${JAR:?set JAR to checkstyle-14.4.0-all.jar}"
here=$(cd "$(dirname "$0")" && pwd)
if [ -n "${JAVA_HOME:-}" ]; then export PATH=$JAVA_HOME/bin:$PATH; fi
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/cls"
javac -cp "$JAR" -d "$work/cls" "$here/BatchDump.java"
for mode in t T; do
  if [ "$mode" = t ]; then gold=golden; else gold=golden-comments; fi
  for dir in snippets snippets-layout; do
    [ -d "$here/$dir" ] || continue
    find "$here/$dir" -name '*.java' | sort > "$work/list"
    rm -rf "$work/out"
    java -cp "$JAR:$work/cls" BatchDump "$mode" "$work/list" "$work/out" 2
    mkdir -p "$here/$gold/$dir"
    rm -f "$here/$gold/$dir"/*.txt
    i=0
    while IFS= read -r f; do
      n=$(printf "%06d" "$i")
      if [ -f "$work/out/$n.txt" ]; then
        cp "$work/out/$n.txt" "$here/$gold/$dir/$(basename "$f").txt"
      else
        echo "checkstyle rejected $f: $(cat "$work/out/$n.err")" >&2
      fi
      i=$((i + 1))
    done < "$work/list"
  done
done
