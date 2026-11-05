# Full bundled configs, javaid vs Checkstyle 14.4.0

Measured 2026-11-05 on an i7-10700 (8 cores, 16 threads), OpenJDK 21.0.12, Linux 7.0. A desktop,
not an isolated lab machine: load average was 2 to 5 from ordinary desktop apps. javaid is run with
the whole configuration (`bench/oracle/google-implemented.xml` is `google_checks.xml` with every
instance javaid implements, which is all 106; `sun-implemented.xml` likewise, 73 of 73). Checkstyle
runs `google_checks.xml` and `sun_checks.xml` themselves, default JVM settings, single-threaded.
Guava at 7cd44b3 (3,277 files), Spring Framework at ed4e0f8 (9,293 files). Wall-clock seconds,
median of 3 runs (javaid) and 2 runs (Checkstyle). Raw data: `full-configs-timing.tsv`; script:
`../full-configs-timing.sh`.

| config / corpus | Checkstyle | javaid 1 thread | javaid 16 threads | 1 thread | 16 threads |
|---|---:|---:|---:|---:|---:|
| Google, Guava | 67.9 s | 18.2 s | 2.88 s | 3.7x | 24x |
| Google, Spring | 153.6 s | 26.8 s | 4.06 s | 5.7x | 38x |
| Sun, Guava | 43.3 s | 12.0 s | 1.94 s | 3.6x | 22x |
| Sun, Spring | 72.4 s | 17.1 s | 2.67 s | 4.2x | 27x |

Peak memory (max resident set): Checkstyle 3.0 to 3.9 GB; javaid 37 to 300 MB on one thread and
190 to 400 MB on 16 threads.

One file (Guava's `Joiner.java`), process start to exit, mean of 100 runs for javaid and median of
10 for Checkstyle: javaid 15 ms (Google config) and 9 ms (Sun config); Checkstyle 1.3 s and 1.0 s.

These are lower multiples than the earlier 42-module figures (6-9x per core): the full
configurations include far heavier modules (Indentation, Javadoc, the emulated Checkstyle AST),
and javaid does that work too. Checkstyle is single-threaded, so the all-threads column compares a
parallel tool with a serial one; the one-thread column is the like-for-like number.
