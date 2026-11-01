# Benchmark results

```json
{
  "date": "2026-11-01 17:14:03 PKT",
  "kernel": "7.0.0-38-generic",
  "cpu": "Intel(R) Core(TM) i7-10700 CPU @ 2.90GHz",
  "logical_cpus": 16,
  "mem": "62Gi",
  "governor": "powersave",
  "java": [
    "openjdk version \"25.0.5\" 2026-10-20 LTS",
    "OpenJDK Runtime Environment Temurin-25.0.5+8 (build 25.0.5+8-LTS)"
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
| javaid parse only, 1 thread | 3.75 s | 3.64 s – 3.96 s | 18 | 3.75 | 10.4× |  |
| javaid 42 modules, 1 thread | 6.41 s | 4.67 s – 8.92 s | 21 | 6.41 | 6.1× |  |
| javaid 42 modules, 8 threads | 0.85 s | 0.79 s – 0.90 s | 43 | 6.37 | 45.9× |  |
| javaid 42 modules, all threads | 0.78 s | 0.70 s – 0.82 s | 61 | 9.97 | 50.0× |  |
| Checkstyle 42 modules, default JVM | 38.88 s | 37.06 s – 40.52 s | 3286 | 66.87 | 1.0× |  |

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid parse only, 1 thread | 4.53 s | 4.45 s – 4.73 s | 21 | 4.53 | 17.3× |  |
| javaid 42 modules, 1 thread | 8.58 s | 7.68 s – 12.91 s | 35 | 8.58 | 9.1× |  |
| javaid 42 modules, 8 threads | 2.36 s | 2.22 s – 2.48 s | 54 | 16.47 | 33.1× |  |
| javaid 42 modules, all threads | 1.54 s | 1.38 s – 2.01 s | 69 | 16.53 | 50.9× |  |
| Checkstyle 42 modules, default JVM | 78.18 s | 78.09 s – 85.22 s | 3269 | 116.03 | 1.0× |  |
