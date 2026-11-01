#!/usr/bin/env bash
# Check javaid's Javadoc tree (src/rules/javadoc_tree.rs) against Checkstyle's own, over every
# Javadoc comment of the Guava and Spring corpora. Needs `java` and `javac` on the PATH.
# usage: compare.sh <scratch-dir>      (the directory make-reference.sh uses)
set -eu
SP=$1
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
JAR="$SP/tools/checkstyle/checkstyle-14.4.0-all.jar"
WORK="$SP/results/javadoc-tree"
mkdir -p "$WORK/classes"
javac -cp "$JAR" -d "$WORK/classes" "$HERE/Dump.java"
python3 -I "$HERE/extract.py" "$WORK/records.bin" "$WORK/records.meta" "$SP/corpus/guava" "$SP/corpus/spring-framework"
java -cp "$JAR:$WORK/classes" Dump "$WORK/records.bin" "$WORK/checkstyle.out"
(cd "$ROOT" && JT_IN="$WORK/records.bin" JT_OUT="$WORK/javaid.out" cargo test --test javadoc_tree_corpus -- --ignored)
python3 -I "$HERE/compare.py" "$WORK/checkstyle.out" "$WORK/javaid.out" 5
