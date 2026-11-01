# Does a JVM startup fix close the gap?

## A. One 21.5 KB file (what a pre-commit hook, editor save or agent hook pays)

| tool | configuration | wall median (min – max) | peak RSS |
|---|---|---:|---:|
| checkstyle | JDK 21 as shipped | 0.59 s (0.55 s – 0.62 s) | 176 MB |
| checkstyle | JDK 25 | 0.55 s (0.54 s – 0.63 s) | 163 MB |
| checkstyle | JDK 25 + AOT cache | 0.37 s (0.32 s – 0.38 s) | 167 MB |
| checkstyle | JDK 25 + AOT cache + C1-only JIT + SerialGC | 0.30 s (0.30 s – 0.31 s) | 138 MB |
| pmd | JDK 21 as shipped | 1.21 s (1.19 s – 1.26 s) | 216 MB |
| pmd | JDK 25 | 1.25 s (1.22 s – 1.31 s) | 242 MB |
| pmd | JDK 25 + AOT cache | 0.81 s (0.78 s – 0.86 s) | 235 MB |
| pmd | JDK 25 + AOT cache + C1-only JIT + SerialGC | 0.71 s (0.70 s – 0.72 s) | 185 MB |
| javaid | native binary | 3.2 ms (3.0 ms – 3.8 ms) | 14 MB |

## B. Guava, whole repository (3,277 files, 29.5 MB)

| configuration | wall median (min – max) | peak RSS | CPU time |
|---|---:|---:|---:|
| Checkstyle, JDK 21 as shipped | 30.46 s (30.37 s – 30.49 s) | 3259 MB | 50 s |
| Checkstyle, JDK 25 + AOT cache | 29.41 s (29.18 s – 29.43 s) | 2849 MB | 47 s |
| PMD 1 thread, JDK 21 as shipped | 16.26 s (16.09 s – 16.34 s) | 883 MB | 65 s |
| PMD 1 thread, JDK 25 + AOT cache | 14.35 s (14.10 s – 14.55 s) | 987 MB | 57 s |
| PMD 16 threads, JDK 21 as shipped | 7.71 s (7.32 s – 7.85 s) | 1302 MB | 98 s |
| PMD 16 threads, JDK 25 + AOT cache | 6.30 s (6.29 s – 6.31 s) | 1486 MB | 81 s |
| javaid, 1 thread | 3.18 s (3.14 s – 3.26 s) | 17 MB | 3 s |
| javaid, 16 threads | 0.41 s (0.40 s – 0.42 s) | 53 MB | 6 s |

## C. Warm resident JVM (same file, repeated in one process)

```
checkstyle iterations=400 first=293.9ms tenth=12.8ms warm_median=4.00ms warm_min=3.51ms warm_p95=5.93ms peak_rss=733MB
pmd iterations=150 first=1089.7ms tenth=66.4ms warm_median=29.01ms warm_min=24.88ms warm_p95=35.38ms peak_rss=713MB violations_last=0
```
