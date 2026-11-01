# Benchmark results

```json
{
  "date": "2026-11-01 03:43:44 PKT",
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
  "pmd": "pmd-bin-7.29.0",
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
| javaid 5 modules (spike config), 1 thread | 3.21 s | 3.19 s – 3.25 s | 18 | 3.21 |  |  |
| javaid 5 modules (spike config), 8 threads | 0.47 s | 0.46 s – 0.50 s | 39 | 3.54 |  |  |
| javaid 5 modules (spike config), all threads | 0.43 s | 0.42 s – 0.44 s | 54 | 6.02 |  |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid 5 modules (spike config), 1 thread | 5.31 s | 5.27 s – 5.43 s | 21 | 5.31 |  |  |
| javaid 5 modules (spike config), 8 threads | 0.78 s | 0.77 s – 0.96 s | 39 | 5.84 |  |  |
| javaid 5 modules (spike config), all threads | 0.83 s | 0.71 s – 0.86 s | 52 | 11.61 |  |  |
