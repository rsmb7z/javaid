# Benchmarks and the Checkstyle oracle

Needs Checkstyle 14.4.0's `-all.jar`, a JDK 21, and two corpora (Guava at 7cd44b3 and Spring Framework
at ed4e0f8, shallow clones). The scripts take the directory that holds them as their first argument.

```bash
cargo test                                   # unit tests and the Checkstyle-derived probe fixtures
bench/full-configs-timing.sh                 # the full Google and Sun timings in results/full-configs.md
python3 bench/parity.py reference.out javaid.out            # per-module diff against Checkstyle output
python3 bench/parity.py --strict reference.out javaid.out   # also match on the column; count message differences
bench/oracle/make-reference.sh <dir> bench/oracle/naming.xml naming    # Checkstyle's reference output
bench/oracle/config-parity.sh <dir> <dir>/google_checks.xml google     # a bundled config against Checkstyle
python3 bench/oracle/mutate_corpus.py <src> <dst>                      # a corpus edited so rare rules fire
```

`bench/results/full-configs.md` has the current speed table and its method. `FINDINGS.md` has the
original experiment (five rules, with PMD as a second baseline) and its caveats.
