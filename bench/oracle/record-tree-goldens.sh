#!/usr/bin/env bash
# Re-record Checkstyle's answers for the probes of the tree-based modules added after the naming
# family. The list below is the list of cases in tests/oracle_probes.rs.
# usage: record-tree-goldens.sh <checkstyle-all.jar>
set -eu
JAR=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/probes"
rec() { # probe config
  python3 -I "$HERE/record-golden.py" "$JAR" "$HERE/$2.xml" "$P/$1" "$P/${1%.*}.$2.expected" | sed "s#$P/##"
}
rec ArrayProbe.java arraytypestyle
rec EllProbe.java upperell
for c in avoidstar avoidstar-excludes avoidstar-class avoidstar-static; do rec StarProbe.java $c; done
rec SwitchProbe.java missingswitchdefault
rec ModProbe.java modifierorder
rec ModPairs.java modifierorder
rec MultiProbe.java multivar
rec MultiProbe2.java multivar
rec ModProbe2.java modifierorder
for p in OneProbe OneProbe2; do rec $p.java onestatement; rec $p.java onestatement-try; done
for p in OneProbe3 OneProbe4 OneProbe5 OneProbe6; do rec $p.java onestatement; done
python3 -I "$P/make-fallthrough-probes.py"
for p in FtProbe1 FtProbe2 FtProbe3 FtProbe4 FtProbe5 FtInf FtYield; do rec $p.java fallthrough; rec $p.java fallthrough-last; done
python3 -I "$HERE/make-parenpad-configs.py" >/dev/null
for c in parenpad parenpad-space $(cd "$HERE" && ls parenpad-*.xml | sed 's/\.xml$//' | grep -v '^parenpad-space$'); do rec PadProbe.java $c; done
rec PadProbe3.java parenpad
rec PadProbe3.java parenpad-space
for c in parenpad parenpad-expr parenpad-question parenpad-dot; do rec PadProbe2.java $c; done
rec PadProbe4.java parenpad
rec PadProbe4.java parenpad-space
python3 -I "$HERE/make-methodparampad-configs.py" >/dev/null
for c in $(python3 -I "$HERE/make-methodparampad-configs.py"); do rec MppProbe.java $c; done
for c in $(python3 -I "$HERE/make-nowhitespacebefore-configs.py"); do rec NwbProbe.java $c; done
for c in $(python3 -I "$HERE/make-whitespaceafter-configs.py"); do rec WaProbe.java $c; done
for c in whitespaceafter whitespaceafter-annotations whitespaceafter-singlelinecomment whitespaceafter-google; do rec Wa2Probe.java $c; done
for c in $(python3 -I "$HERE/make-whitespacearound-configs.py"); do rec WarProbe.java $c; done
rec GwProbe.java genericwhitespace
for c in $(python3 -I "$HERE/make-operatorwrap-configs.py"); do rec OwProbe.java $c; done
for c in $(python3 -I "$HERE/make-separatorwrap-configs.py"); do rec SwProbe.java $c; done
for c in $(python3 -I "$HERE/make-leftcurly-configs.py"); do rec LcProbe.java $c; done
for c in separatorwrap separatorwrap-nl separatorwrap-eol separatorwrap-lparen-nl separatorwrap-lparen-eol separatorwrap-rparen-nl separatorwrap-rparen-eol; do rec SpProbe.java $c; done
