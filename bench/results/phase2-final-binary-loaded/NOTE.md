# A re-run that was not clean

`bench/phase2-bench.sh` was run again after the ancestor-stack optimisation (see FINDINGS.md,
"Speed with 33 modules"), with the final binary. It overlapped with an unrelated, CPU-heavy test
loop on the same desktop: the load average was 14-18 for the duration and ended at 18. The effect
is visible in the data. Checkstyle's own Spring time went from 58.4 s (the quiet run in
`../phase2`) to 77.1 s with a range of 63-85 s, and javaid's multi-threaded rows slowed by half.

These files are kept so that nothing is hidden, and they should not be quoted. The parity result of
this run is the same as the quiet one (10,296 of 10,296 on Guava, 89,118 of 89,118 on Spring).
The size of the optimisation is reported instead from an interleaved A/B of the two configurations
on the final binary, which a changing load affects far less: `../phase2-ab.txt`.
