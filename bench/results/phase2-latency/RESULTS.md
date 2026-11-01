# Benchmark results

```json
{
  "date": "2026-11-01 05:12:08 PKT",
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
| javaid 33 modules, 1 thread | 3.9 ms | 3.6 ms – 9.6 ms | 14 | 0.00 | 209.0× |  |
| javaid 33 modules, 8 threads | 4.2 ms | 4.0 ms – 4.8 ms | 14 | 0.00 | 191.6× |  |
| javaid 33 modules, all threads | 4.6 ms | 4.4 ms – 6.2 ms | 14 | 0.01 | 174.3× |  |
| Checkstyle 33 modules, default JVM | 0.81 s | 0.68 s – 0.96 s | 193 | 3.59 | 1.0× |  |
