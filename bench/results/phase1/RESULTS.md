# Benchmark results

```json
{
  "date": "2026-11-01 03:35:40 PKT",
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
| javaid parse only, 1 thread | 2.64 s | 2.62 s – 2.70 s | 17 | 2.64 | 13.3× |  |
| javaid 21 modules, 1 thread | 3.38 s | 3.34 s – 3.46 s | 18 | 3.37 | 10.4× |  |
| javaid 21 modules, 8 threads | 0.48 s | 0.48 s – 0.50 s | 39 | 3.68 | 72.4× |  |
| javaid 21 modules, all threads | 0.44 s | 0.44 s – 0.45 s | 59 | 6.19 | 79.3× |  |
| Checkstyle 21 modules, default JVM | 35.08 s | 35.04 s – 35.92 s | 3257 | 58.30 | 1.0× |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 4.35 s | 4.33 s – 4.40 s | 21 | 4.35 | 13.2× |  |
| javaid 21 modules, 1 thread | 5.62 s | 5.54 s – 5.67 s | 21 | 5.61 | 10.2× |  |
| javaid 21 modules, 8 threads | 0.82 s | 0.80 s – 0.89 s | 42 | 6.13 | 69.8× |  |
| javaid 21 modules, all threads | 0.88 s | 0.73 s – 0.92 s | 57 | 12.12 | 65.2× |  |
| Checkstyle 21 modules, default JVM | 57.23 s | 57.16 s – 58.08 s | 3272 | 89.34 | 1.0× |  |
