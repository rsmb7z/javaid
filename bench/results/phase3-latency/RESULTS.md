# Benchmark results

```json
{
  "date": "2026-11-01 17:25:35 PKT",
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
    "latency-one-file": {
      "commit": "n/a (not a git checkout)",
      "java_files": 1,
      "bytes": 21513
    }
  }
}
```

## latency-one-file — 1 files, 0.0 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| javaid 42 modules, 1 thread | 5.3 ms | 4.4 ms – 22.3 ms | 14 | 0.01 | 155.6× |  |
| javaid 42 modules, 8 threads | 5.1 ms | 4.8 ms – 6.7 ms | 14 | 0.01 | 163.9× |  |
| javaid 42 modules, all threads | 5.4 ms | 5.0 ms – 7.2 ms | 14 | 0.01 | 153.7× |  |
| Checkstyle 42 modules, default JVM | 0.83 s | 0.73 s – 1.09 s | 195 | 3.07 | 1.0× |  |
