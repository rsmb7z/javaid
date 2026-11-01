#!/usr/bin/env bash
# Re-record Checkstyle's answers for the text-level probes (FileTabCharacter, LineLength,
# RegexpSingleline, TodoComment). The list below is the list of cases in tests/oracle_probes.rs.
# usage: record-text-goldens.sh <checkstyle-all.jar>
set -eu
JAR=$1
HERE="$(cd "$(dirname "$0")" && pwd)"
P="$HERE/probes"
rec() { # probe config
  python3 -I "$HERE/record-golden.py" "$JAR" "$HERE/$2.xml" "$P/$1" "$P/${1%.*}.$2.expected" | sed "s#$P/##"
}
python3 -I "$P/make-text-probes.py"
for c in filetab filetab-each; do rec TabProbe.java $c; done
for p in MixedProbe CrlfProbe CrProbe FormFeedProbe; do rec $p.java filetab-each; done
for c in linelength linelength-tab4 linelength-google; do rec LineProbe.java $c; done
for c in regexp-trailing regexp-max2 regexp-min99 regexp-nocase regexp-case; do rec TrailProbe.java $c; done
for p in MixedProbe CrlfProbe CrProbe FormFeedProbe; do rec $p.java regexp-trailing; done
for c in todo-default todo-google todo-anchored todo-spaced todo-endofline todo-across; do rec TodoProbe.java $c; done
rec CrlfProbe.java todo-endofline
rec TabProbe.java text-files
rec Props.properties text-files
rec Doc.xml text-files
# tree rules must count lines the way Checkstyle does, whatever ends them (a form feed ends nothing,
# so FormFeedProbe's line comment would swallow the file: it is a text-rule probe only)
for p in MixedProbe CrlfProbe CrProbe; do rec $p.java typename-never; done
