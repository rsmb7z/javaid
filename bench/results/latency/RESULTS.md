# Benchmark results

```json
{
  "date": "2026-11-01 01:30:40 PKT",
  "kernel": "7.0.0-38-generic",
  "cpu": "Intel(R) Core(TM) i7-10700 CPU @ 2.90GHz",
  "logical_cpus": 16,
  "mem": "62Gi",
  "governor": "powersave",
  "java": [
    "openjdk version \"21.0.12.1\" 2026-08-18",
    "OpenJDK Runtime Environment (build 21.0.12.1+1-1-24.04.4-Ubuntu)"
  ],
  "checkstyle": "checkstyle-14.4.0-all.jar",
  "pmd": [
    "\u2588\u2588\u2588\u2588                            \u2588\u2588\u2588\u2588"
  ],
  "tuned_single": "-XX:+UseSerialGC -Xmx512m",
  "tuned_multi": "-XX:+UseParallelGC",
  "lowmem_multi": "-XX:+UseSerialGC",
  "ci_cores": 0,
  "corpora": {
    "latency-one-file": {
      "commit": "fatal: not a git repository (or any of the parent directories): .git",
      "java_files": 1,
      "bytes": 21513
    }
  }
}
```

## latency-one-file — 1 files, 0.0 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 2.7 ms | 2.6 ms – 3.0 ms | 14 | 0.00 | 282.5× | 483.8× |
| javaid 5 rules, 1 thread | 3.0 ms | 2.9 ms – 4.3 ms | 14 | 0.00 | 259.3× | 444.2× |
| javaid 5 rules, 8 threads | 3.2 ms | 3.1 ms – 4.8 ms | 14 | 0.00 | 236.6× | 405.1× |
| javaid 5 rules, all threads | 3.6 ms | 3.4 ms – 5.0 ms | 14 | 0.00 | 213.8× | 366.1× |
| Checkstyle 1 trivial rule | 0.75 s | 0.72 s – 0.83 s | 175 | 3.15 | 1.0× | 1.7× |
| Checkstyle 5 rules, default JVM | 0.77 s | 0.75 s – 0.85 s | 176 | 3.25 | 1.0× | 1.7× |
| Checkstyle 5 rules, tuned JVM | 0.73 s | 0.70 s – 0.83 s | 183 | 2.82 | 1.1× | 1.8× |
| PMD 1 rule, 1 thread | 1.63 s | 1.27 s – 1.77 s | 205 | 4.98 | 0.5× | 0.8× |
| PMD 5 rules, 1 thread, default JVM | 1.30 s | 1.23 s – 2.18 s | 218 | 4.34 | 0.6× | 1.0× |
| PMD 5 rules, 1 thread, tuned JVM | 1.28 s | 1.25 s – 1.39 s | 291 | 4.23 | 0.6× | 1.0× |
| PMD 5 rules, default threads, default JVM | 1.31 s | 1.24 s – 1.40 s | 218 | 4.43 | 0.6× | 1.0× |
| PMD 5 rules, default threads, fastest JVM flags | 1.31 s | 1.27 s – 1.54 s | 276 | 3.99 | 0.6× | 1.0× |
| PMD 5 rules, default threads, low-memory JVM flags | 1.45 s | 1.35 s – 2.10 s | 261 | 4.29 | 0.5× | 0.9× |
| Checkstyle google_checks.xml (what teams run) | 1.63 s | 1.21 s – 2.02 s | 264 | 6.14 | 0.5× | 0.8× |
| PMD quickstart ruleset (what teams run) | 2.90 s | 2.65 s – 3.27 s | 724 | 8.70 | 0.3× | 0.5× |
