# Benchmark results

```json
{
  "date": "2026-11-01 02:24:06 PKT",
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
| javaid parse only, 1 thread | 2.53 s | 2.50 s – 2.59 s | 17 | 2.53 |  |  |
| javaid 5 rules, 1 thread | 3.05 s | 2.98 s – 3.16 s | 17 | 3.04 |  |  |
| javaid 5 rules, 8 threads | 0.46 s | 0.43 s – 0.54 s | 39 | 3.40 |  |  |
| javaid 5 rules, all threads | 0.40 s | 0.39 s – 0.46 s | 53 | 5.75 |  |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 4.21 s | 4.19 s – 4.29 s | 20 | 4.21 |  |  |
| javaid 5 rules, 1 thread | 5.00 s | 4.92 s – 5.20 s | 20 | 5.00 |  |  |
| javaid 5 rules, 8 threads | 0.77 s | 0.72 s – 0.94 s | 39 | 5.71 |  |  |
| javaid 5 rules, all threads | 0.81 s | 0.69 s – 0.87 s | 52 | 10.98 |  |  |
