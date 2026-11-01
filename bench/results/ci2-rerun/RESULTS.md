# Benchmark results

```json
{
  "date": "2026-11-01 02:33:45 PKT",
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
| [2-core runner] javaid | 1.59 s | 1.54 s – 1.61 s | 27 | 3.07 |  |  |
| [2-core runner] Checkstyle, tuned JVM | 34.89 s | 34.33 s – 35.87 s | 638 | 43.03 |  |  |
| [2-core runner] PMD, default threads, tuned JVM | 17.00 s | 16.58 s – 17.05 s | 1838 | 33.53 |  |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| [2-core runner] javaid | 2.60 s | 2.57 s – 2.72 s | 25 | 5.05 |  |  |
| [2-core runner] Checkstyle, tuned JVM | 64.30 s | 63.52 s – 66.36 s | 646 | 73.43 |  |  |
| [2-core runner] PMD, default threads, tuned JVM | 20.22 s | 20.12 s – 21.17 s | 1248 | 39.89 |  |  |
