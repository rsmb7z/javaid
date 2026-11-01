#!/usr/bin/env bash
# Sweep JVM flags for the Java baselines on one corpus, so the formal benchmark gives them
# their best fair configuration instead of a straw man. One run per cell.
# usage: jvm-flags.sh <scratch-dir> <corpus-name>
set -u
SP=$1
C=$2
BENCH="$(cd "$(dirname "$0")" && pwd)"
CS="$SP/tools/checkstyle/checkstyle-14.4.0-all.jar"
PMD="$SP/tools/pmd/pmd-bin-7.29.0/bin/pmd"
DIR="$SP/corpus/$C"
OUT="$SP/results/jvm-flags"
mkdir -p "$OUT"

# Prints one row. PMD exits 4 when it finds violations and Checkstyle may exit non-zero on
# errors, so the exit status is recorded rather than assumed; a JVM crash shows as FAILED.
measure() {
  local label=$1
  shift
  /usr/bin/time -f "%e %M %U %S %x" -o "$OUT/t" "$@" >/dev/null 2>"$OUT/err"
  local wall rss user sys rc
  read -r wall rss user sys rc < <(tail -1 "$OUT/t")
  local status="ok"
  if grep -q -E 'OutOfMemoryError|StackOverflowError' "$OUT/err"; then
    status="FAILED: $(grep -m1 -oE 'OutOfMemoryError[^"]*|StackOverflowError' "$OUT/err")"
  elif [ "$rc" != "0" ] && [ "$rc" != "4" ]; then
    status="FAILED: exit $rc"
  fi
  printf '%-34s wall=%6ss rss=%6s MB cpu=%5ss  %s\n' "$label" "$wall" "$((rss / 1024))" \
    "$(echo "$user $sys" | awk '{printf "%.1f", $1 + $2}')" "$status"
}

declare -A FLAGS=(
  [default]=""
  [serial]="-XX:+UseSerialGC"
  [serial+xmx512m]="-XX:+UseSerialGC -Xmx512m"
  [serial+xmx1g]="-XX:+UseSerialGC -Xmx1g"
  [parallel]="-XX:+UseParallelGC"
)

echo "### Checkstyle 14.4.0, 5 rules, $C"
for k in default serial serial+xmx512m serial+xmx1g parallel; do
  # shellcheck disable=SC2086
  measure "checkstyle [$k]" java ${FLAGS[$k]} -jar "$CS" -c "$BENCH/checkstyle-spike.xml" "$DIR"
done

echo "### PMD 7.29.0, 5 rules, 1 thread, $C"
for k in default serial serial+xmx512m serial+xmx1g; do
  measure "pmd -t1 [$k]" env PMD_JAVA_OPTS="${FLAGS[$k]}" "$PMD" check -d "$DIR" -R "$BENCH/pmd-spike.xml" -f text --no-cache --no-progress -t 1
done

echo "### PMD 7.29.0, 5 rules, 16 threads, $C"
for k in default parallel serial; do
  measure "pmd -t16 [$k]" env PMD_JAVA_OPTS="${FLAGS[$k]}" "$PMD" check -d "$DIR" -R "$BENCH/pmd-spike.xml" -f text --no-cache --no-progress -t 16
done
echo "JVM-FLAGS-DONE"
