# Benchmark results

```json
{
  "date": "2026-11-01 05:28:56 PKT",
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
| javaid parse only, 1 thread | 3.27 s | 3.18 s – 3.33 s | 17 | 3.27 | 12.0× |  |
| javaid 33 modules, 1 thread | 4.07 s | 3.35 s – 4.49 s | 19 | 4.07 | 9.7× |  |
| javaid 33 modules, 8 threads | 0.57 s | 0.48 s – 0.66 s | 41 | 4.32 | 69.2× |  |
| javaid 33 modules, all threads | 0.57 s | 0.54 s – 0.62 s | 63 | 7.25 | 69.5× |  |
| Checkstyle 33 modules, default JVM | 39.35 s | 39.14 s – 45.49 s | 3392 | 68.50 | 1.0× |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 4.56 s | 4.52 s – 4.71 s | 21 | 4.56 | 16.9× |  |
| javaid 33 modules, 1 thread | 5.97 s | 5.84 s – 7.11 s | 31 | 5.97 | 12.9× |  |
| javaid 33 modules, 8 threads | 1.59 s | 1.54 s – 1.65 s | 51 | 11.43 | 48.5× |  |
| javaid 33 modules, all threads | 1.49 s | 1.46 s – 1.68 s | 67 | 12.29 | 51.6× |  |
| Checkstyle 33 modules, default JVM | 77.13 s | 63.19 s – 84.84 s | 3779 | 116.15 | 1.0× |  |
