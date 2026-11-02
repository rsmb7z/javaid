#!/usr/bin/env bash
# Re-record every probe golden (`.expected` and `.expected2`) with record-golden.py. Each golden is
# `<Stem>.<config>.expected` beside the probe `<Stem>.<ext>`; the config is bench/oracle/<config>.xml.
# usage: rerecord-all-goldens.sh <checkstyle-all.jar> [parallel-jobs]
set -eu
JAR=$1
JOBS=${2:-3}
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/probes"
one() {
  jar=$1; here=$2; exp=$3
  base=$(basename "$exp" .expected)
  stem=${base%%.*}
  config=${base#*.}
  probe=$(ls "$here/probes/$stem".* | grep -v '\.expected' | head -n 1)
  python3 -I "$here/record-golden.py" "$jar" "$here/$config.xml" "$probe" "$exp" > /dev/null
}
export -f one
ls "$P"/*.expected | xargs -P "$JOBS" -I{} bash -c 'one "$0" "$1" "$2"' "$JAR" "$HERE" {}
echo "re-recorded $(ls "$P"/*.expected | wc -l) goldens"
