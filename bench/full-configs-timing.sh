#!/bin/bash
ROOT="$(cd "$(dirname "$0")/.." && pwd)"  # expects ROOT/.scratchpad/{tools,corpus,scripts,lat}
R=$ROOT/.scratchpad; B=$ROOT/target/release/javaid; J=$R/tools/checkstyle/checkstyle-14.4.0-all.jar
OUT=$R/results/timing.tsv; : > $OUT
t() { # label cmd...
  l=$1; shift
  /usr/bin/time -f "$l\t%e\t%U\t%M" -a -o $OUT "$@" >/dev/null 2>/dev/null
}
uptime >> $R/results/timing.load
for c in guava spring-framework; do
 for n in google sun; do
  cfg=$R/scripts/${n}_checks.xml; sub=$ROOT/bench/oracle/$n-implemented.xml
  for i in 1 2 3; do t "$c/$n/javaid-j1" $B -j 1 --config $sub $R/corpus/$c; done
  for i in 1 2 3; do t "$c/$n/javaid-j16" $B -j 16 --config $sub $R/corpus/$c; done
  for i in 1 2; do t "$c/$n/checkstyle" java -jar $J -c $cfg $R/corpus/$c; done
 done
done
for n in google sun; do
  for i in 1 2 3 4 5 6 7 8 9 10; do t "joiner/$n/javaid" $B --config $ROOT/bench/oracle/$n-implemented.xml $R/lat/Joiner.java; done
  for i in 1 2 3 4 5 6 7 8 9 10; do t "joiner/$n/checkstyle" java -jar $J -c $R/scripts/${n}_checks.xml $R/lat/Joiner.java; done
done
uptime >> $R/results/timing.load
touch $R/timing.done
