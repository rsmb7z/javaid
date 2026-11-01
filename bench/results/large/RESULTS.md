# Benchmark results

```json
{
  "date": "2026-11-01 01:33:20 PKT",
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
  "ci_cores": 2,
  "corpora": {
    "guava": {
      "commit": "7cd44b3",
      "java_files": 3277,
      "bytes": 29474527
    },
    "spring-framework": {
      "commit": "ed4e0f8",
      "java_files": 9293,
      "bytes": 53674435
    }
  }
}
```

## guava — 3277 files, 29.5 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 2.59 s | 2.53 s – 2.69 s | 16 | 2.59 | 12.4× | 3.0× |
| javaid 5 rules, 1 thread | 3.00 s | 2.91 s – 24.50 s | 17 | 2.96 | 10.7× | 2.6× |
| javaid 5 rules, 8 threads | 0.48 s | 0.44 s – 0.53 s | 38 | 3.60 | 67.3× | 16.6× |
| javaid 5 rules, all threads | 0.44 s | 0.41 s – 0.64 s | 52 | 5.82 | 73.2× | 18.1× |
| Checkstyle 1 trivial rule | 42.56 s | 37.20 s – 45.10 s | 2988 | 63.77 | 0.8× | 0.2× |
| Checkstyle 5 rules, default JVM | 32.01 s | 29.44 s – 36.68 s | 3272 | 54.17 | 1.0× | 0.2× |
| Checkstyle 5 rules, tuned JVM | 35.66 s | 34.29 s – 38.84 s | 697 | 46.25 | 0.9× | 0.2× |
| PMD 1 rule, 1 thread | 15.00 s | 14.95 s – 15.24 s | 876 | 65.47 | 2.1× | 0.5× |
| PMD 5 rules, 1 thread, default JVM | 16.58 s | 16.36 s – 16.63 s | 901 | 68.79 | 1.9× | 0.5× |
| PMD 5 rules, 1 thread, tuned JVM | 14.67 s | 14.54 s – 14.92 s | 527 | 58.32 | 2.2× | 0.5× |
| PMD 5 rules, default threads, default JVM | 7.90 s | 7.53 s – 8.94 s | 1298 | 99.63 | 4.1× | 1.0× |
| PMD 5 rules, default threads, fastest JVM flags | 7.43 s | 6.93 s – 7.92 s | 2461 | 82.62 | 4.3× | 1.1× |
| PMD 5 rules, default threads, low-memory JVM flags | 7.36 s | 7.34 s – 7.68 s | 761 | 79.36 | 4.3× | 1.1× |
| [2-core runner] javaid | 1.68 s | 1.53 s – 1.88 s | 27 | 3.31 | 19.1× | 4.7× |
| [2-core runner] Checkstyle, tuned JVM | 37.63 s | 37.31 s – 39.55 s | 643 | 47.66 | 0.9× | 0.2× |
| [2-core runner] PMD, default threads, tuned JVM | 18.81 s | 18.73 s – 24.20 s | 1827 | 36.85 | 1.7× | 0.4× |
| Checkstyle google_checks.xml (what teams run) | 82.96 s | 82.96 s – 82.96 s | 3456 | 133.39 | 0.4× | 0.1× |
| PMD quickstart ruleset (what teams run) | 53.70 s | 53.70 s – 53.70 s | 4779 | 218.09 | 0.6× | 0.1× |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 5.03 s | 4.29 s – 5.71 s | 20 | 5.03 | 28.1× | 2.3× |
| javaid 5 rules, 1 thread | 6.32 s | 5.43 s – 10.67 s | 20 | 6.31 | 22.3× | 1.8× |
| javaid 5 rules, 8 threads | 1.23 s | 1.10 s – 1.51 s | 40 | 9.13 | 115.3× | 9.3× |
| javaid 5 rules, all threads | 1.06 s | 1.02 s – 1.13 s | 52 | 10.73 | 133.6× | 10.7× |
| Checkstyle 1 trivial rule | 53.31 s | 52.61 s – 60.30 s | 3286 | 77.77 | 2.7× | 0.2× |
| Checkstyle 5 rules, default JVM | 141.35 s | 67.39 s – 158.63 s | 5740 | 133.90 | 1.0× | 0.1× |
| Checkstyle 5 rules, tuned JVM | 71.63 s | 64.35 s – 136.03 s | 694 | 77.29 | 2.0× | 0.2× |
| PMD 1 rule, 1 thread | 20.42 s | 19.68 s – 38.28 s | 908 | 69.67 | 6.9× | 0.6× |
| PMD 5 rules, 1 thread, default JVM | 20.81 s | 20.57 s – 22.63 s | 890 | 72.80 | 6.8× | 0.5× |
| PMD 5 rules, 1 thread, tuned JVM | 18.47 s | 18.19 s – 24.22 s | 480 | 58.42 | 7.7× | 0.6× |
| PMD 5 rules, default threads, default JVM | 11.37 s | 9.81 s – 12.28 s | 1505 | 112.97 | 12.4× | 1.0× |
| PMD 5 rules, default threads, fastest JVM flags | 12.36 s | 9.95 s – 16.37 s | 2469 | 102.22 | 11.4× | 0.9× |
| PMD 5 rules, default threads, low-memory JVM flags | 11.53 s | 9.60 s – 13.78 s | 745 | 103.88 | 12.3× | 1.0× |
| [2-core runner] javaid | 3.23 s | 2.74 s – 4.01 s | 25 | 6.37 | 43.7× | 3.5× |
| [2-core runner] Checkstyle, tuned JVM | 83.48 s | 64.80 s – 96.00 s | 645 | 98.84 | 1.7× | 0.1× |
| [2-core runner] PMD, default threads, tuned JVM | 22.00 s | 20.63 s – 35.65 s | 1225 | 43.30 | 6.4× | 0.5× |
| Checkstyle google_checks.xml (what teams run) | 188.56 s | 188.56 s – 188.56 s | 3669 | 263.55 | 0.7× | 0.1× |
| PMD quickstart ruleset (what teams run) | 21.33 s | 21.33 s – 21.33 s | 4645 | 272.20 | 6.6× | 0.5× |
