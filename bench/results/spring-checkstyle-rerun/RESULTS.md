# Benchmark results

```json
{
  "date": "2026-11-01 02:27:17 PKT",
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
    "spring-framework": {
      "commit": "ed4e0f8",
      "java_files": 9293,
      "bytes": 53674435
    }
  }
}
```

## spring-framework — 9293 files, 53.7 MB

| scenario | wall median | min – max | peak RSS (MB) | CPU (s) | vs Checkstyle default | vs PMD default |
|---|---:|---:|---:|---:|---:|---:|
| Checkstyle 5 rules, default JVM | 47.33 s | 47.18 s – 48.18 s | 3221 | 71.76 | 1.0× |  |
| Checkstyle 5 rules, tuned JVM | 67.57 s | 62.50 s – 68.44 s | 712 | 79.91 | 0.7× |  |
