# Benchmark results

```json
{
  "date": "2026-11-01 05:03:29 PKT",
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
| javaid parse only, 1 thread | 2.70 s | 2.60 s – 2.80 s | 18 | 2.70 | 13.3× |  |
| javaid 33 modules, 1 thread | 4.07 s | 3.84 s – 4.29 s | 19 | 4.07 | 8.8× |  |
| javaid 33 modules, 8 threads | 0.68 s | 0.54 s – 0.83 s | 42 | 5.15 | 52.8× |  |
| javaid 33 modules, all threads | 0.60 s | 0.57 s – 0.71 s | 57 | 8.18 | 59.6× |  |
| Checkstyle 33 modules, default JVM | 35.86 s | 34.72 s – 36.69 s | 3330 | 60.64 | 1.0× |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 4.54 s | 4.37 s – 4.66 s | 21 | 4.53 | 12.9× |  |
| javaid 33 modules, 1 thread | 6.38 s | 6.21 s – 6.72 s | 31 | 6.38 | 9.1× |  |
| javaid 33 modules, 8 threads | 0.98 s | 0.95 s – 1.10 s | 53 | 7.10 | 59.4× |  |
| javaid 33 modules, all threads | 0.89 s | 0.86 s – 1.04 s | 67 | 11.80 | 65.8× |  |
| Checkstyle 33 modules, default JVM | 58.37 s | 57.05 s – 58.96 s | 3269 | 90.09 | 1.0× |  |
