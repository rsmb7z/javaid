# javaid: feasibility findings

Date: 2026-11-01. Everything here was measured or looked up during the spike unless it is
explicitly marked as judgement.

## TL;DR

**Verdict: technically proven, strategically uncertain.** Go for a time-boxed Phase 1 (2-3 weeks)
with the kill criteria in section 6. Expect hundreds to low thousands of stars, not Ruff-scale
(section 4.2). Treat the follower payoff as a bonus, not the plan.

What the spike established, on the same files with the same five rules:

| | Checkstyle 14.4 | PMD 7.29 | **javaid** |
|---|---:|---:|---:|
| Guava (3,277 files, 29 MB), 1 thread | 30.5 s † | 14.7 s | **3.05 s** † |
| Guava, all 16 threads | single-threaded | 7.4 s | **0.40 s** † |
| Spring (9,293 files, 54 MB), 1 thread | 47 s † | 18.5 s | **5.0 s** † |
| Spring, all 16 threads | single-threaded | 11.4 s | **0.81 s** † |
| 2-core CI runner, Guava / Spring | 34 s / 52 s † | 17 s / 20 s † | **1.6 s / 2.6 s** † |
| Peak memory (either corpus) | 0.7-3.3 GB | 0.5-2.5 GB | **17-53 MB** |
| One file (pre-commit, editor save, agent hook) | 0.59 s † | 1.21 s † | **3.2 ms** † |

Each baseline is shown in its fastest configuration (medians; ranges in section 2). † re-measured
with nothing else of mine running.

- **It finds the same things.** For the five spike rules it reproduces **10,460 of 10,460**
  Checkstyle findings (Guava 8,650; Spring 1,810), same file, line and rule, with zero extras
  (section 3). 1,440 of those were `UnusedImports` findings under a *non-default* Checkstyle
  setting (`processJavadoc=false`). **Phase 1 closed that gap** (section 8): `UnusedImports` now
  also matches at Checkstyle's defaults, 17 naming modules match on 10,771 of 10,771 findings,
  twelve more modules bring it to 33 and the nine whitespace and wrapping modules to 42 (round 3,
  section 8). Run on the instances javaid implements from Checkstyle's own Google and Sun configs,
  it matches **1,025,538 of 1,025,538** and **345,358 of 345,358** findings, and on mutated copies
  of both corpora (edited so the rare rules fire) **43,410 of 43,410** for the 33 and more than
  **6 million** findings for the whitespace family (round 3).
- **Speedup, honestly stated.** About 10x single-threaded and 58-76x with all threads against
  Checkstyle; 3.7-4.8x single-threaded and 14-18x multi-threaded against PMD. Against PMD on one
  thread the win is modest. The big wins are one-file latency (about 185x and 380x as shipped),
  memory (10-190x less) and CPU-seconds on small CI runners (8-15x fewer).
- **A JVM startup fix does not close the gap** (section 2.5). The best available one, JDK 25's AOT
  cache, roughly halves one-file latency (Checkstyle 0.59 to 0.30 s, PMD 1.21 to 0.71 s) but trims
  a whole-repository run by only 3.5% (Checkstyle) to 12-18% (PMD). Only a *warm resident JVM*
  matches javaid on one file (Checkstyle 4.0 ms against 3.2 ms), and it holds 733 MB doing so.
  **Speed is a head start, not a moat** (section 6).
- **What it does not show.** The spike measured five cheap syntactic rules; Phase 1 has raised
  that to 42 modules (section 8) but not to indentation or Javadoc-style rules. The speed numbers
  below were taken with 33 modules; a 42-module run on a loaded machine is in section 8 (6-9x per
  core, about 50x on all threads, same findings). Checkstyle's own `google_checks.xml` names 88 distinct modules and javaid runs 44 of
  its 106 instances; `sun_checks.xml` has 73 and javaid runs 33. What javaid runs from those two
  configs matches Checkstyle exactly, but about 58% and 55% of their instances do not run at all. Cold
  runs only: Checkstyle and PMD have incremental caches that were not used, and javaid has none yet.
- **Cheapest extra win found:** Checkstyle 14.4.0 needs **Java 21** to run; PMD 7.29's CLI needs
  Java 8. A native binary needs no JDK at all (section 4.4).

## 1. Question and method

**Question.** Can a native, parallel, syntax-only linter reproduce Checkstyle's findings at a
fraction of the time and memory?

**Subjects.** javaid (this repo; Rust 1.96, release build with LTO), Checkstyle 14.4.0 (`-all.jar`)
and PMD 7.29.0 (binary distribution). Both downloads were verified against the publisher's SHA-256
digests.

**Rules.** The five rules that have a twin in both tools and are purely syntactic:
`EmptyCatchBlock`, `NeedBraces`, `EqualsHashCode`, `UnusedImports`, `MethodName`. Properties were
set to equivalent behaviour (comments allowed in empty catch blocks, `processJavadoc=false`, the
same method-name regex, and so on). The configs are in `bench/`.

**Corpora.** Guava at 7cd44b3 (3,277 files, 29.5 MB), Spring Framework at ed4e0f8 (9,293 files,
53.7 MB), and one file, Guava's `Joiner.java` (21.5 KB), for latency. Shallow clones, `.java`
files only, no ignore-file filtering, so all three tools see the same file set.

**Machine.** Intel i7-10700 (8 cores, 16 threads), 62 GB RAM, Linux 7.0 on Ubuntu 24.04, OpenJDK
21.0.12, CPU governor `powersave`. An ordinary desktop, not an isolated lab machine.

**Measurement.** Wall time, peak RSS and CPU time come from the kernel's accounting for the child
process (`wait4`), so they include JVM startup, which is what users pay. Every tool writes its
findings to a file. The corpus is read into the page cache first. javaid runs get one discarded
warm-up and then 10 samples; JVM runs get 3 samples and no warm-up (they keep no state between
runs). Medians are reported with min-max.

**Baselines get their best fair configuration.** A JVM-flag sweep on Guava (one run per cell)
showed wall time barely moves with flags but memory does (Checkstyle from 0.7 to 6 GB). So each
JVM tool is shown with default flags and a memory-lean configuration, and PMD both with its default
one thread per core and with one thread. Restricting the JIT to C1 made Checkstyle slower (52 s)
and was dropped. Checkstyle's CLI documents no thread option, so it is single-threaded as shipped.

**Quiet re-runs.** Some scenarios in the formal run showed wide variance: Checkstyle on Spring
ranged from 67 s to 159 s, and one javaid sample took 24.5 s. I do not know the cause; this is an
ordinary desktop whose background load I did not control. I re-measured Checkstyle on Spring, the
final javaid binary on both corpora, and the 2-core scenarios with nothing else of mine running
(marked †). Where a number was re-measured, the re-measured value is used. The Guava JVM baselines
agree with independent earlier runs (the flag sweep and the first baseline) and are kept. PMD on
Spring was run only once, in the formal run, with the ranges shown; read its absolute seconds with
the same caution.

## 2. Results

### 2.1 Whole repository, same five rules

**Guava**, 3,277 files, 29.5 MB. Median (min-max).

| tool and configuration | wall | peak RSS | CPU time |
|---|---:|---:|---:|
| Checkstyle, default JVM † | 30.5 s (30.4-30.5) | 3.3 GB | 50 s |
| Checkstyle, 512 MB heap | 35.7 s (34.3-38.8) | 0.70 GB | 46 s |
| PMD, 1 thread, 512 MB heap | 14.7 s (14.5-14.9) | 0.53 GB | 58 s |
| PMD, 16 threads, default JVM | 7.9 s (7.5-8.9) | 1.3 GB | 100 s |
| PMD, 16 threads, low-memory flags | 7.4 s (7.3-7.7) | 0.76 GB | 79 s |
| **javaid, 1 thread** † | **3.05 s** (2.98-3.16) | **17 MB** | 3.0 s |
| **javaid, 8 threads** † | **0.46 s** (0.43-0.54) | 39 MB | 3.4 s |
| **javaid, 16 threads** † | **0.40 s** (0.39-0.46) | 53 MB | 5.8 s |

**Spring Framework**, 9,293 files, 53.7 MB.

| tool and configuration | wall | peak RSS | CPU time |
|---|---:|---:|---:|
| Checkstyle, default JVM † | 47.3 s (47.2-48.2) | 3.2 GB | 72 s |
| Checkstyle, 512 MB heap † | 67.6 s (62.5-68.4) | 0.71 GB | 80 s |
| PMD, 1 thread, 512 MB heap | 18.5 s (18.2-24.2) | 0.48 GB | 58 s |
| PMD, 16 threads, default JVM | 11.4 s (9.8-12.3) | 1.5 GB | 113 s |
| PMD, 16 threads, low-memory flags | 11.5 s (9.6-13.8) | 0.75 GB | 104 s |
| **javaid, 1 thread** † | **5.00 s** (4.92-5.20) | **20 MB** | 5.0 s |
| **javaid, 8 threads** † | **0.77 s** (0.72-0.94) | 39 MB | 5.7 s |
| **javaid, 16 threads** † | **0.81 s** (0.69-0.87) | 52 MB | 11.0 s |

The 512 MB-heap Checkstyle trades speed for memory on the bigger corpus (68 s against 47 s); its
default JVM trades memory for speed. javaid needs neither trade.

### 2.2 A small CI runner (pinned to 2 physical cores)

All three tools pinned with `taskset` to two physical cores, to approximate a small hosted CI
runner. All rows are quiet re-runs (†).

| corpus | tool | wall | peak RSS | CPU time |
|---|---|---:|---:|---:|
| Guava | Checkstyle, 512 MB heap (Gradle's default heap size) | 34.9 s (34.3-35.9) | 638 MB | 43 s |
| Guava | Checkstyle, default JVM | 33.7 s (33.1-34.7) | 7.3 GB | 44 s |
| Guava | PMD, 2 threads | 17.0 s (16.6-17.1) | 1.8 GB | 34 s |
| Guava | **javaid** | **1.59 s** (1.54-1.61) | **27 MB** | 3.1 s |
| Spring | Checkstyle, 512 MB heap (Gradle's default heap size) | 64.3 s (63.5-66.4) | 646 MB | 73 s |
| Spring | Checkstyle, default JVM | 52.1 s (51.7-52.1) | 6.4 GB | 65 s |
| Spring | PMD, 2 threads | 20.2 s (20.1-21.2) | 1.2 GB | 40 s |
| Spring | **javaid** | **2.60 s** (2.57-2.72) | **25 MB** | 5.1 s |

javaid is 20-25x faster than Checkstyle (21x and 20x against its faster default-JVM configuration),
8-11x faster than PMD, and uses 8-15x fewer CPU-seconds. **Caveat:** `taskset` limits CPUs but not
RAM, so the default-JVM Checkstyle rows had a 16 GB heap ceiling and used up to 7 GB of it. A real
2-core runner has about 7 GB in total, so the 512 MB-heap rows are the realistic baseline there.

### 2.3 One file (what a pre-commit hook, an editor save or an AI-agent hook pays)

`Joiner.java`, 21.5 KB.

| tool | wall | peak RSS |
|---|---:|---:|
| **javaid**, 5 rules † | **3.2 ms** (3.0-3.8) | **14 MB** |
| Checkstyle, 5 rules † | 0.59 s (0.55-0.62) | 176 MB |
| PMD, 5 rules † | 1.21 s (1.19-1.26) | 216 MB |
| Checkstyle with `google_checks.xml` | 1.63 s (1.21-2.02) | 264 MB |
| PMD with its `quickstart` ruleset | 2.90 s (2.65-3.27) | 724 MB |

The † rows are from the quiet startup experiment (section 2.5). The formal run measured higher
figures for the same commands (0.73-0.77 s and 1.28-1.31 s), another sign of the noise described in
section 1; the last two rows are from the formal run and were not re-measured.

That is about 185x faster than Checkstyle and 380x faster than PMD for the same rules, as shipped.
It is what each call pays in a process-per-invocation workflow: a CI step, a pre-commit hook, an
AI-agent hook. Where a JVM is already resident (an IDE) the picture differs; see section 2.5.

### 2.4 Observations

- **The cost is startup plus parsing, not rules.** PMD with one rule takes 15.0 s on Guava against
  16.6 s with five. javaid spends 83% (Guava) and 84% (Spring) of its single-thread time just parsing.
  So rule work is a small share of every tool's time here. Heavier rule families (Javadoc,
  whitespace, indentation) are untested and could move the ratios either way.
- **Scaling flattens at the physical cores.** javaid goes from 3.05 s to 0.40 s (7.6x) from 1 to 16
  threads on 8 cores; the 8-to-16 step adds little (Guava 0.46 to 0.40 s, Spring 0.77 to 0.81 s).
- **PMD is far better than Checkstyle.** Even on one thread it is about twice as fast, and 16
  threads gain it roughly another 2x. Quoting only the Checkstyle ratio would overstate the case,
  which is why both are shown.
- **What teams really run costs far more.** Checkstyle with `google_checks.xml` took 83 s (Guava)
  and 189 s (Spring); PMD's `quickstart` ruleset took 54 s and 21 s. One noisy sample each, so
  indicative only. javaid cannot run those configs yet, so **no speedup is claimed for them.**

### 2.5 Does a JVM startup fix close the gap?

The fair objection to the latency numbers is that JVM startup is a known, fixable cost. So I gave
the JVM tools the best fixes available today and re-measured on a quiet machine, using Temurin
25.0.5 (the current LTS; SHA-256 verified) as the second JDK. The tools' own distributions ship no
native image and no AOT cache; I found no native-image build, issue or pull request for either.

**A. One 21.5 KB file.** The AOT cache (Project Leyden: JEP 483 class loading and linking, plus
JEP 515 method profiles) is trained on a *different* directory (26 Guava files), then used. It is a
24 MB (Checkstyle) or 39 MB (PMD) file built by a 2-4 s training run, tied to the exact JDK build
and classpath. Medians; min-max in parentheses.

| tool | configuration | wall | peak RSS |
|---|---|---:|---:|
| Checkstyle | JDK 21 as shipped | 0.59 s (0.55-0.62) | 176 MB |
| Checkstyle | JDK 25 | 0.55 s (0.54-0.63) | 163 MB |
| Checkstyle | JDK 25 + AOT cache | 0.37 s (0.32-0.38) | 167 MB |
| Checkstyle | + C1-only JIT, SerialGC | 0.30 s (0.30-0.31) | 138 MB |
| PMD | JDK 21 as shipped | 1.21 s (1.19-1.26) | 216 MB |
| PMD | JDK 25 | 1.25 s (1.22-1.31) | 242 MB |
| PMD | JDK 25 + AOT cache | 0.81 s (0.78-0.86) | 235 MB |
| PMD | + C1-only JIT, SerialGC | 0.71 s (0.70-0.72) | 185 MB |
| **javaid** | native binary | **3.2 ms** (3.0-3.8) | **14 MB** |

The cache roughly halves startup, in line with the 42% the [JEP](https://openjdk.org/jeps/483)
reports. It leaves javaid about 95x (Checkstyle) and 220x (PMD) faster.

**B. Guava, whole repository** (3,277 files, 29.5 MB), same session. The same fixes barely move a
run dominated by parsing and analysis.

| configuration | wall | peak RSS | CPU time |
|---|---:|---:|---:|
| Checkstyle, JDK 21 as shipped | 30.46 s (30.37-30.49) | 3,259 MB | 50 s |
| Checkstyle, JDK 25 + AOT cache | 29.41 s (29.18-29.43) | 2,849 MB | 47 s |
| PMD 1 thread, JDK 21 as shipped | 16.26 s (16.09-16.34) | 883 MB | 65 s |
| PMD 1 thread, JDK 25 + AOT cache | 14.35 s (14.10-14.55) | 987 MB | 57 s |
| PMD 16 threads, JDK 21 as shipped | 7.71 s (7.32-7.85) | 1,302 MB | 98 s |
| PMD 16 threads, JDK 25 + AOT cache | 6.30 s (6.29-6.31) | 1,486 MB | 81 s |
| **javaid, 1 thread** | **3.18 s** (3.14-3.26) | **17 MB** | 3 s |
| **javaid, 16 threads** | **0.41 s** (0.40-0.42) | **53 MB** | 6 s |

The AOT cache saves 3.5% for Checkstyle, 12% for PMD on one thread and 18% on 16 threads.

**C. A warm resident JVM** (what an IDE plugin, a Gradle daemon or a lint server gives you): the
same file linted repeatedly inside one process.

| tool | first call | warm median (min, p95) | peak RSS |
|---|---:|---:|---:|
| Checkstyle (400 calls) | 294 ms | **4.0 ms** (3.5, 5.9) | 733 MB |
| PMD (150 calls) | 1,090 ms | 29 ms (25, 35) | 713 MB |

PMD's figure reloads its ruleset on every call (that is how the `PmdAnalysis` API is used), so a
purpose-built server would be faster. The warm-JVM source is `bench/WarmCheckstyle.java` and
`bench/WarmPmd.java`. Each runs on its own classpath, because the two distributions bundle Saxon
with different jar signatures and cannot share one JVM; my first attempt did, PMD's rules silently
failed to load, and I discarded that result.

**What this means.**

- Startup is a small share of any big run (Checkstyle's fixed cost is 0.6 s of 30 s on Guava), so
  fixing it cannot close the whole-repository gap. The per-core throughput gap (about 10x against
  Checkstyle, 3.7-4.8x against PMD on one thread) is untouched.
- On one file the gap is real for **process-per-invocation workflows** (CI steps, pre-commit hooks,
  agent hooks). Where a JVM is already resident (an IDE), latency is roughly a tie (4.0 ms against
  3.2 ms) and the difference is memory: 14 MB against 733 MB.
- **Not measured:** a GraalVM native-image build of Checkstyle or PMD (none exists that I found).
  The warm numbers above are roughly what such a build could hope to approach on one file.

## 3. Output parity: does it find the same things?

Findings are matched on (file, line, rule) in the tables below, which were taken before columns
were compared. Since then `parity.py --strict` matches on the column too and counts message
differences, and javaid prints Checkstyle's column and message: the AST column is counted in code
points and tab-expanded to `tabWidth` (the expansion walks UTF-16 units, so a tab after an emoji
expands from one column too few), the text modules print what Checkstyle prints (no column for
`LineLength`, `FileLength`, `RegexpSingleline` and `NewlineAtEndOfFile`; a UTF-16 index for
`FileTabCharacter`), a declaration without modifiers is reported at the last `.` of a qualified
type, and the `<message>` overrides of a configuration are applied.

| rule | Checkstyle | javaid | identical |
|---|---:|---:|---:|
| `EmptyCatchBlock` | 158 + 89 | 158 + 89 | 100% |
| `NeedBraces` | 60 + 48 | 60 + 48 | 100% |
| `EqualsHashCode` | 68 + 4 | 68 + 4 | 100% |
| `MethodName` | 8,004 + 589 | 8,004 + 589 | 100% |
| `UnusedImports` | 360 + 1,080 | 360 + 1,080 | 100% |
| **total** | **8,650 + 1,810** | **8,650 + 1,810** | **10,460 of 10,460** |

(Guava + Spring.) Zero findings that Checkstyle does not report.

**How it got there.** Diffing against Checkstyle exposed six real semantic differences in my first
versions, each now fixed with a regression test that runs under both grammars: (1) `EqualsHashCode`
anchors on the method's first annotation; (2) an abstract `hashCode()` does not count as an
override; (3) `@Override` methods are exempt from `MethodName`; (4) annotations on the package
declaration (`package-info.java`) count as uses of imports; (5) a member type shadows an import
only inside the body that declares it, not file-wide; (6) the right-hand side of a dot, a method's
own name and the target of `::` are not uses of an imported type (`Type.URI` is an enum constant,
not `java.net.URI`). This is the case for the oracle method: I found each of these by diffing real
output, not by reading documentation.

**Caveats that matter.**

- **`processJavadoc=false` is not Checkstyle's default**, and the difference is nearly total. With
  the default, Checkstyle treats Javadoc references as uses, so almost all of the 360 and 1,080
  findings above (imports used only in Javadoc) disappear. At the default, Checkstyle's output on
  the two corpora has 5 lines: **one** real unused import and **four Javadoc-parser diagnostics**
  that Checkstyle prints under the module's name. I first misread all five as unused imports; the
  comparison tool silently dropped the four, which is the kind of thing it must never do, and has
  since been fixed to name what it ignores. During the spike javaid did not match the default;
  since Phase 1 it does (section 8).
- **Parity against PMD is only partial, as expected.** PMD's twins are different rules. On Guava and
  Spring respectively: `EmptyCatchBlock` 100%; `MethodName` 100% (PMD reports 2 extra on Guava);
  `NeedBraces` 87.5% and 68.4%; `EqualsHashCode` 0% because PMD reports it on a different line; and
  `UnnecessaryImport` flags 32 and 0 imports against Checkstyle's 360 and 1,080 (likely because it
  counts Javadoc uses and does not flag imports it cannot resolve without a classpath; I did not
  verify the cause). Checkstyle, being purely syntactic, is the right oracle.

### Threats to validity

1. **One machine, one OS, a desktop whose background load I did not control.** Re-measured where
   variance was wide, but absolute numbers will differ elsewhere. Ratios are the claim, not seconds.
2. **Five cheap syntactic rules.** Heavier families may shift the ratios in either direction.
3. **Cold-process runs only.** Checkstyle (`-Dcheckstyle.cache.file`) and PMD (`--cache`) can skip
   unchanged files on repeat runs; javaid has no cache yet. Warm-run comparison is future work.
4. **Two mainstream, well-formatted corpora.** Messier code may expose more rule differences.
5. **Parity is for these rules under the oracle settings stated.** The spike's `UnusedImports` used
   `processJavadoc=false`; Phase 1 added the default (section 8).
6. **Environment metadata.** The generated result files from before the fix show PMD's ASCII banner
   instead of its version, and a git error for the non-git latency corpus; the versions are PMD 7.29.0
   and the corpus is a single copied file. The harness is fixed for future runs.
7. **The 2-core emulation limits CPUs, not RAM** (section 2.2), and the AOT-cache experiment used
   one second JDK (Temurin 25.0.5) on one machine.


## 4. What the research says

### 4.1 Prior art: the slot is not empty, but nobody holds the position

Searched on 2026-11-01 with the GitHub search API (six queries: Rust/Java/linter/formatter/
tree-sitter/checkstyle combinations), the crates.io search API, and web search.

| project | what it is | stars | notes |
|---|---|---:|---|
| [mihaidaniel34/grounds](https://github.com/mihaidaniel34/grounds) | Rust static analyzer for Java, aimed at AI coding-agent hooks | 9 | Created 2026-10-10. README claims 314 rules, type resolution, dataflow, a warm daemon, and ~27k lines of Rust "mostly written by Claude over roughly a day". Benchmarks itself on 21 projects; no head-to-head against Checkstyle; compares to PMD by sampling. |
| [speakeasy-api/dprint-plugin-java](https://github.com/speakeasy-api/dprint-plugin-java) | Rust+WASM Java formatter "inspired by palantir-java-format" | 5 | No performance claims, no exact-match claim. |
| ryckakas/bonsai-lint | multi-language cognitive-complexity linter | 13 | not a Checkstyle/PMD substitute |
| rmuir/pegon | fast Java language server in Rust | 3 | different problem |
| [ast-grep/ast-grep](https://github.com/ast-grep/ast-grep) | tree-sitter structural search and lint, general purpose | 16.4k | not Java-specific; no Checkstyle compatibility |

Nobody offers **Checkstyle-compatible output, verified against Checkstyle as an oracle, with a
published head-to-head benchmark**. That is the unclaimed position. It is also a position
that a well-funded or fast-moving competitor could take within weeks: `grounds` was written in
about a day. The moat is not code. It is rigour (the oracle), distribution, and the story.

### 4.2 Star ceiling in this niche

Star counts fetched from the GitHub API on 2026-11-01.

| Java incumbents | stars | | Rust tooling elsewhere | stars |
|---|---:|---|---|---:|
| Checkstyle | 9.6k | | uv | 91.4k |
| Error Prone | 7.3k | | Ruff | 50.4k |
| google-java-format | 6.2k | | Biome | 26.0k |
| Spotless | 5.7k | | Oxc | 23.3k |
| PMD | 5.5k | | ast-grep | 16.4k |
| SpotBugs | 4.0k | | | |
| palantir-java-format | 0.9k | | | |

The whole Java lint-and-format incumbent set lives between roughly 1k and 10k stars. Ruff's
50k reflects Python's much larger audience for this kind of tool (and a funded full-time team);
it is not a target for a Java linter. **Judgement, not a measurement:** a competent launch of a
credible tool plausibly lands in the hundreds to low thousands; a very successful one could
reach a few thousand; passing Checkstyle's 9.6k would be a breakout. Converting stars into
profile followers is something I found no data on.

### 4.3 Demand evidence and its limits

What supports demand:
- **Measured here:** Checkstyle takes about 30-40 s and 1-3 GB of RSS to lint Guava (3,277
  files, 29 MB) with five rules. A single trivial rule costs the same, so the cost is
  startup plus parsing, not rule work.
- **Anecdotes:** a Checker Framework issue reports its formatting pre-commit hook taking
  "more than 10 seconds" on an M1 (cut from ~9.2 s to ~2 s by splitting the work); Grammarly's
  engineering write-up calls the JVM + Gradle + Spotless combination "quite slow indeed".
- **Startup is a known, partly-fixed problem for a *formatter*:** google-java-format's current
  release (v2.0.0, 2026-10-30) ships GraalVM native-image binaries for Linux, macOS and Windows
  (about 33 MB each). I did not verify since when, or any startup figure: the v1.21.0 release notes
  only mention a native-image version-string bug, and an earlier claim of mine that they advertised
  faster startup came from a secondary summary and is withdrawn. Native builds carry their own bug
  surface (a closed issue is titled "Native version fails to format record patterns in switches…").
  I found no native-image build, issue or pull request for Checkstyle or PMD (title searches of
  both trackers for "graalvm", "native image", "native-image" and "startup" returned nothing
  relevant), so for those two tools the fix has not been applied.
- **AI coding agents** need sub-second feedback after every edit; `grounds` was built for exactly
  that and says a Gradle build after each edit takes 2-5 minutes on its motivating projects.

What does not: I found no developer survey on linter speed, mvnrepository blocks automated
access so I have no install-base figure beyond one "used by" count for the PMD Maven plugin
(2,121 artifacts), and the Java Rust-tool projects that exist today have 3-13 stars. Speed
alone has not yet produced pull in this niche.

### 4.4 Technical findings that shape the plan

- **Checkstyle is purely syntactic.** Its own docs say you cannot determine the type of an
  expression from its syntax tree. So the *entire* Checkstyle rule library (250 `*Check` classes
  in 14.4.0) is portable to a parse-tree-only tool, with no type resolution. PMD 7.29.0 has 327
  Java rules, 149 of them XPath over the AST; some of the rest need symbol resolution.
- **Coverage is the long road.** Checkstyle's bundled `google_checks.xml` uses 88 distinct modules
  and `sun_checks.xml` 74 (129 in the union). The spike implements 5. (Phase 1 recounted without
  the `Checker` and `TreeWalker` wrappers: 88, 73 and 128. The classification that follows was made
  on the earlier 129 and is a judgement, not a measurement.) By my own classification of
  those 129: 20 naming rules, 44 single-node or small-subtree syntax rules and 11 text or comment
  scans (75 straightforward); 21 whitespace, wrapping and brace-layout rules; 17 Javadoc rules; 8
  suppression filters; 4 XPath-based; 4 needing global analysis. The hard long tail is
  Javadoc (38 check classes), whitespace/indentation/wrapping, and XPath-based modules
  (`MatchXpath`, `Suppression*Xpath*`), which cannot port directly because Checkstyle's AST
  token tree differs from tree-sitter's.
- **Grammar risk is real.** tree-sitter-java 0.23.5 (last released 2024-12-21) cannot parse a
  type-use annotation on varargs (`Object @Nullable ... args`). That is JSpecify style, now
  standard in Spring Framework 7 / Boot 4 (GA November 2025), JUnit 6 and Guava. It broke 13 of
  Guava's and 58 of Spring's files. Upstream PR #206 has been open since June 2025. The
  maintained fork `tree-sitter-java-orchard` (MIT, updated 2026-10-29) fixes it.
- **A JDK requirement the native tool does not have.** Checkstyle 14.4.0's class files are
  version 65, so it needs **Java 21** to run; PMD 7.29.0's CLI is version 52 (Java 8). Teams
  whose project still builds on Java 8 or 11 (31% and 17% in JetBrains' 2025 survey; multi-select,
  so not additive) must install a second JDK just to run current Checkstyle. A native binary
  needs none.
- **Licensing.** Checkstyle is LGPL-2.1; GitHub reports PMD's licence as "Other" (BSD-style).
  Reimplement from documented behaviour and black-box comparison, not from their source.
  Differential testing against the real tool as an oracle does not copy code.

### 4.5 Is Java staying, and how much is it used?

Looked up on 2026-11-01. Each row measures something different, so do not compare across rows.

| source | Java | what it measures |
|---|---|---|
| [SlashData](https://www.slashdata.co/post/global-developer-population-trends-2025-how-many-developers-are-there) (early 2025) | about 23M developers, roughly tied with Python, behind JavaScript | modelled estimate of active developers; communities overlap |
| [Stack Overflow 2025](https://survey.stackoverflow.co/2025/technology) | 29.6% of professional developers (6th; C# 29.9%) | past-year use, self-selected survey |
| [TIOBE, Oct 2026](https://www.tiobe.com/tiobe-index/) | #4 at 8.41%, +0.06 points vs Oct 2025; #1 in 2006, 2011, 2016; #3 in 2021 | search-engine popularity, not usage |
| [RedMonk, Jan 2026](https://redmonk.com/sogrady/2026/04/14/language-rankings-1-26/) | #3 | GitHub and Stack Overflow discussion |
| [GitHub Octoverse 2025](https://github.blog/news-insights/octoverse/octoverse-a-new-developer-joins-github-every-second-as-ai-leads-typescript-to-1/) | +174,705 contributors (+20.7%) year over year, 4th-largest gain | Aug 2025 vs Aug 2024 |
| [JetBrains State of Java 2025](https://lp.jetbrains.com/the-state-of-java-2025) | backend 78%, Spring 65%, Maven 67%; Java 21 40%, 17 39%, Java 8 31% (83% in 2019) | 5,000+ Java developers; 37% from China, 14% from India |
| [Azul 2026](https://secure.businesswire.com/news/home/20260210972138/en/Azul-2026-State-of-Java-Survey-Report-62-of-Enterprises-Now-Leverage-Java-to-Power-AI-Functionality-41-Rely-on-High-Performance-Java-Platforms-to-Reduce-Cloud-Compute-Costs) | 62% of organisations use Java to build AI functionality (50% a year earlier) | 2,000+ Java professionals; vendor-run survey |

**Judgement:** Java is a mature incumbent. Top five on every measure, flat to growing in absolute
terms, not a growth language. Expect it to remain a top-five language through the 2030s; do not
expect it to regain share from Python and TypeScript.

Why it stays: (1) the installed base decays slowly, since Java 8 (2014) is still used by 31% in
2025; (2) funded runway, since Oracle's
[roadmap](https://www.oracle.com/java/technologies/java-se-support-roadmap.html) lists Java 25
(LTS, September 2025) with Premier Support to September 2030 and Extended to September 2033, and
the next LTS, Java 29, planned for September 2027 (dates "subject to change"); (3) the platform
is fixing its weak spots, namely virtual threads (Loom), an AOT cache that cuts startup (Leyden),
and value classes (Valhalla, JEP 401, targeted at JDK 28 preview; see the
[Inside Java newscast](https://nipafx.dev/inside-java-newscast-104/)); (4) ecosystem gravity,
with Spring and Maven used by two thirds of surveyed Java developers.

What erodes it, in relative share and not existence: TypeScript took #1 on GitHub by
contributors; Python leads TIOBE at 19.23%; JetBrains says Java developers who switch
[tend to move to Python or TypeScript](https://gigazine.net/gsc_news/en/20260820-jetbrains-programming-language-migration/)
(no percentages given); Kotlin is the preferred language for new Android work. I found no survey
showing an enterprise exodus. Most "migration" figures concern Oracle JDK to other OpenJDK
builds (Azul: 81% moving some estate), which is a vendor change and not a language change.

Corrections to claims I met while researching: I could **not** confirm a report that TIOBE put
Java at an all-time low in March 2026. TIOBE's own table shows Java up slightly year over year.

**What this means for javaid:** the audience is large and durable (about 23M developers and a
long legacy tail), so building for it is safe. But audience size is not what caps stars: Java and
Python communities are about the same size, yet Java's biggest tools sit near 10k stars and
Ruff has 50k. **Hypothesis, not measured:** engagement culture, not headcount, sets the ceiling,
so the launch should also target the cross-ecosystem "fast Rust tooling" audience and not Java
developers alone.

## 5. Risks

1. **Star pull is unproven** in the Java niche (4.3) and the ceiling is low (4.2).
2. **Fast followers.** Another tool could claim the position first (4.1).
3. **Grammar dependency.** One grammar underpins everything. Upstream tree-sitter-java has not
   released since 2024-12-21. The fork that fixes the gap, `tree-sitter-java-orchard`, is active
   (508 commits, latest 2026-10-29, 25 releases, MIT) but has a single visible maintainer, is
   hosted on Codeberg, and has 5 stars. Mitigation: pin it, run the canary test that documents the
   upstream gap, and treat a purpose-built parser as the longer-term lever (it is also the main
   speed lever, since parsing is ~80% of the time).
4. **Compat is a treadmill.** Each Checkstyle release changes behaviour; the oracle suite must
   run in CI against pinned and latest versions.
5. **Silent wrongness.** A fast linter that disagrees with Checkstyle is worse than slow. Rules
   must ship with measured agreement numbers and say so when they do not match.

## 6. Roadmap and kill criteria

Phase 1 (2-3 weeks, public), in this order:

1. **Javadoc-reference scan for `UnusedImports`**, so it matches Checkstyle's *default* settings,
   not only `processJavadoc=false`. **Done** (section 8).
2. **The naming family first** (20 of the 129 configured modules are one regex on one identifier),
   then imports, braces, modifiers, empty blocks and simple size rules: about 25 rules in total.
   Every port is accepted only at zero diff against Checkstyle on the oracle corpora.
   **Naming done** (17 modules); the remaining families are next.
3. A `checkstyle.xml` reader that runs the supported modules and **lists the unsupported ones
   explicitly**, never skipping silently. **Done for the 42 modules** (section 8).
4. A content-hash cache, so repeat runs are incremental and the comparison with Checkstyle's and
   PMD's caches becomes fair.
5. SARIF and GitHub annotations, release binaries, a pre-commit hook and a GitHub Action.
6. Publish the benchmark and the oracle harness with the first release.

Phase 2: Javadoc and whitespace families, `--fix` for safe rules, Maven/Gradle wrappers that
fetch the native binary, an own parser if grammar upkeep or parse time becomes the bottleneck.

Kill / pivot criteria: stop or pivot if (a) the Phase 1 launch does not clear a few hundred stars
within two weeks of a Show HN, (b) oracle agreement on a rule family cannot be raised above ~99%
without type information, or (c) a competitor ships the same position with traction first.

### Where it runs: CI first, inner loop second

javaid is a command-line linter, not a CI product: no server, no dashboard, no service. It runs
wherever a linter runs, and the evidence points to two places.

- **CI (whole repository, fresh runner): the best-evidenced payoff.** On a 2-core runner it takes
  1.6 s (Guava) and 2.6 s (Spring), where Checkstyle takes 35 s and 64 s and PMD 17 s and 20 s, in
  25-27 MB instead of 0.6-1.8 GB. Startup is not the story here. Most Java teams run Checkstyle
  through Maven, where the plugin's
  [`cacheFile`](https://maven.apache.org/plugins/maven-checkstyle-plugin/check-mojo.html) defaults to
  `target/checkstyle-cachefile` (so `mvn clean` removes it) and there is no thread setting, or
  through Gradle, which runs it
  ["in a separate process"](https://docs.gradle.org/current/userguide/checkstyle_plugin.html) with a
  512 MB default heap. The saving is throughput, CPU-seconds and memory, which no startup fix
  touches, and a fresh runner has no warm daemon or restored cache to fall back on.
- **Inner loop (pre-commit, editor save, AI-agent hook): about 3 ms per file.** Real, but it is the
  one place where JVM startup fixes and warm daemons narrow the gap (section 2.5).

What stands between it and a CI step today (updated after round 3, section 8): it implements 42
modules, and about 57% of the instances in Checkstyle's two bundled configs belong to modules it
lacks; and there is no GitHub Action, SARIF output or cache yet. The `checkstyle.xml` reader exists
and `UnusedImports` matches Checkstyle's defaults. Until the rest is there it is a fast first-pass
gate that complements Checkstyle and does not replace it.

### What is, and is not, a moat

Speed is the wedge, not the moat. Which advantages survive a determined response?

| advantage | measured | can an incumbent close it? |
|---|---|---|
| One-file latency | 3.2 ms against 0.59 s (Checkstyle) and 1.21 s (PMD) | Partly. JDK 25's AOT cache halves it (0.30 s, 0.71 s), and a warm resident JVM reaches 4.0 ms (Checkstyle) at 733 MB. A native-image build would remove the rest; none exists. |
| Throughput per core, whole repository | about 10x against Checkstyle, 3.7-4.8x against PMD on one thread | Not with startup fixes (3.5% and 12-18%). Checkstyle is single-threaded as shipped. PMD already scales across threads, by about 2x on 16. |
| Memory | 17-53 MB against 0.5-3.3 GB | Tunable to about 0.5-0.7 GB at some speed cost; not to tens of MB. |
| CPU time on small runners | 8-15x fewer CPU-seconds | Follows throughput. |
| No JDK to install | Checkstyle 14.4 needs Java 21 | Yes, if they ship a native build. |
| **Verified compatibility** | 10,460 of 10,460 findings (see the `UnusedImports` caveat); harness public | Not closable by them: it is earned rule by rule, and the incumbents define the behaviour. The method can be copied, at real cost per rule. |
| **Distribution and integrations** | none yet | Not their game: a Java project will not ship a Rust binary wrapper. My read of why Ruff stuck is that it became part of other people's configs and workflows. |
| Brand | "Not Java. Javaid." | Cannot be copied. |

What erodes the wedge: native builds or AOT caches shipped by default, a Checkstyle server mode, or
a Rust clone that reaches the same parity first (`grounds` was written in about a day). What builds
a moat: (1) publish the oracle harness and a conformance table with every release, so the trust is
checkable; (2) integrations, namely pre-commit, GitHub Action, SARIF, editor and Maven/Gradle
wrappers, each a place others copy config snippets from; (3) visible velocity, rules ported per
week; (4) a cache, so repeat CI runs are incremental; (5) a contributor funnel built on the oracle
("port one check; acceptance is zero diff"). None of this is a guarantee. A funded team or a faster
clone can take the position, and the only protection is being first with a story anyone can check.

### Launch plan: what would make the first week count

Speed alone is not enough in this niche (4.3), so the launch has to make three things obvious in
the first screen of the README:

1. **One chart.** Log-scale wall time and peak memory for Guava and Spring against Checkstyle and
   PMD, with the single-file latency number next to it. Every bar reproducible with one command.
2. **One trust claim nobody else makes.** "Reproduces Checkstyle's findings exactly on Guava
   (8,650 of 8,650, same file and line), checked by a harness you can run." The oracle harness is
   the credibility; publish it as a first-class part of the repo.
3. **One honest limit.** State the coverage plainly (rules ported, rules not) and that unsupported
   Checkstyle modules in your config are *listed*, never silently skipped.

Distribution, in order: prebuilt binaries and `cargo install`, a `pre-commit` hook, a GitHub
Action, Homebrew, then Maven and Gradle wrappers that fetch the binary (that is where Java
developers live). Contributor funnel: one "port a Checkstyle check" issue template whose
acceptance test is "zero diff against Checkstyle on the oracle corpora". It makes contribution
cheap and each merged port is a small public win.

The original idea survives as a flag, not the product: `--roast` rewrites findings in a savage,
deterministic voice (no LLM, no network). The tool earns trust; the flag earns screenshots.

## 8. Phase 1 progress

Started 2026-11-01 from the list in section 6. The rule for every module: accepted only at zero
difference from Checkstyle 14.4.0 on both corpora **and** on probe files that exercise every
construct. The corpora alone are not enough: they contain few pattern variables or record
components, and none of the odd Javadoc layouts.

| task | state | evidence |
|---|---|---|
| 1. Javadoc-reference scan for `UnusedImports` | done | At Checkstyle's defaults: 1 of 1 real findings on both corpora. Two probe files with 39 Javadoc forms are identical. |
| 2. Naming family, 17 modules | done | **10,771 of 10,771** findings, zero extras, at default settings. Five probe files (143 construct-level findings) identical, including three non-default options. |
| 2b. Twelve more modules (round 2) | done | `FileTabCharacter`, `LineLength`, `RegexpSingleline`, `TodoComment`, `ArrayTypeStyle`, `UpperEll`, `AvoidStarImport`, `MissingSwitchDefault`, `ModifierOrder`, `MultipleVariableDeclarations`, `OneStatementPerLine`, `FallThrough`. 65 more probe-and-config pairs identical, and **43,410 of 43,410** findings on mutated copies of both corpora (section "Mutation testing" below). |
| 2c. Nine whitespace and wrapping modules (round 3) | done | `ParenPad`, `MethodParamPad`, `NoWhitespaceBefore`, `WhitespaceAfter`, `WhitespaceAround`, `GenericWhitespace`, `OperatorWrap`, `SeparatorWrap`, `LeftCurly`. 452 probe-and-config pairs now identical (375 of them new, the 375 in the table below), and zero extras and zero misses on every corpus and mutated-corpus run listed under "Round 3" below. |
| 3. `checkstyle.xml` reader | done for the 42 modules | Every property that appears on an implemented module in Checkstyle's bundled Google and Sun configs is honoured; anything else is listed by name with the reason and does not run. Run on those configs' implemented instances, Checkstyle and javaid agree on **1,025,538 of 1,025,538** (Google, 44 instances) and **345,358 of 345,358** (Sun, 33 instances) findings. |
| 4. Content-hash cache | not started | |
| 5. SARIF, release binaries, pre-commit hook, GitHub Action | not started | |
| 6. Publish the benchmark and harness | not done | The harness is in `bench/`; there is no release yet. |

42 modules are implemented in total: the 21 of round 1 (the 4 spike modules and the 17 naming
modules), the 12 of round 2, which takes the roadmap past its "about 25", and the 9 of round 3.
`cargo test` runs 37 unit tests and an integration test that replays 452 probe-and-config pairs
recorded from Checkstyle (`bench/oracle/probes`), under both grammars.

### What the oracle taught this time

Each of these was a wrong or missing assumption that probing Checkstyle corrected.

- **A default I had misremembered.** `MethodName`'s default format in 14.4 is `^[a-z][a-zA-Z0-9]*$`;
  I had believed it was stricter. The defaults now come from the metadata inside the Checkstyle
  jar, not from memory.
- **A second message.** A method named like its class gets its own finding ("must not equal the
  enclosing class name"), which can fire alongside the format finding on the same method.
- **Who owns which declaration.** Catch parameters belong to `CatchParameterName` and not
  `LocalVariableName` (despite the module's description); a `final` catch parameter is also a
  final local. Try-with-resources variables are final locals without the keyword. Constructor type
  parameters, enum constants and annotation elements are checked by nobody. Pattern variables,
  `final` or not, belong to `PatternVariableName` alone.
- **Which Javadoc counts.** Only comments that Checkstyle can attach to a member-level declaration
  (or a local type, or the package statement) count; a Javadoc before a local variable, an
  initializer block or a record component does not. Association is line-based: blank lines and
  comment-only lines between the Javadoc and the declaration are skipped, including a multi-line
  ordinary comment. A Javadoc that ends the line before a member documents that member even if it
  trails a previous declaration. Of two stacked Javadocs only the lower counts.
- **Four findings that are not findings.** At default settings Checkstyle's `UnusedImports` prints
  "Javadoc comment ... has parse error" for four comments in Guava. They come from Checkstyle's
  Javadoc parser, not from imports. javaid does not imitate them, and the comparison tool now says
  so out loud instead of dropping them.
- **Symlinks.** Checkstyle follows symlinked directories and reports a file once per path; PMD
  does not. Spring has one symlink (`framework-docs/modules/ROOT/examples/docs-src`) that exposes
  222 more Java files (310 KB, 0.6% of the bytes). In the section 2 benchmarks Checkstyle therefore
  processed 9,515 files on Spring while PMD and javaid processed 9,293. The effect on the ratios is
  well under 1%, but it is a difference in work done. javaid now follows symlinks by default, as
  Checkstyle does (`--no-follow-symlinks` turns it off).
- **What `EmptyCatchBlock` options really test.** `commentFormat` is applied to the *first*
  comment in the block only, and to exactly the text between its delimiters, untrimmed: with
  `^ignored$`, `//ignored` and `/*ignored*/` pass while `// ignored` and `//ignored ` (trailing
  space) do not. `exceptionVariableName` is matched against the variable's name, and either option
  alone is enough to allow the block. A block holding only a stray `;` is not empty (Probes 5 and 6;
  my first version got the `;` case wrong and the probe caught it).
- **Property defaults.** `value="${x}" default="y"` with `x` unset becomes `y`, replacing the
  *whole* value even when the placeholder was only a part of it; a default is ignored when nothing
  is unset. With no default, Checkstyle refuses to load the configuration at all, whereas javaid
  skips just that module and says why (Probe 7). I had believed any `${...}` meant "skip".

### Round 2: twelve more modules

Chosen from the 25 modules that both of Checkstyle's bundled configs use and javaid lacked
(section "What a real config looks like today", below): the ones that look at one declaration, one
line or one comment. The method was the same, and stricter in one respect: for each module the
probe files and Checkstyle's answers for them were recorded **first**, the tests were watched to
fail, and only then was the module written.

| module | what it taught me | probe-and-config pairs (all identical) | on the real corpora |
|---|---|---:|---|
| `FileTabCharacter` | first tab only unless `eachLine`; a tab counts in a string, a comment, a text block | 6 | Google settings: 934,099 of 934,099 |
| `LineLength` | counts code points, tabs run to the next multiple of `tabWidth`, trailing spaces count | 3 | 79,029 of 79,029 (max 100) |
| `RegexpSingleline` | `\s` is ASCII; `maximum` tolerates the first N matches; a `minimum` shortfall is one finding on line 1 | 9 | 113 of 113 |
| `TodoComment` | the pattern sees each line of a comment, a finding is per comment | 7 | 240 of 240; Google's lookahead instance 1,224 of 1,224 (its backreference one finds nothing here) |
| `ArrayTypeStyle` | every `[` pair counts alone; "C-style" means same line as the token before it and right of the name | 1 | 5 of 5 |
| `UpperEll` | every integer literal ending in `l` | 1 | no findings in either corpus |
| `AvoidStarImport` | `excludes` is trimmed and accepts `.*`; `allowClassImports` allows every non-static star | 4 | no findings in either corpus |
| `MissingSwitchDefault` | statements only, never switch expressions; a pattern or `null` label exempts the switch | 1 | 142 of 142 |
| `ModifierOrder` | the 14-word order verified on every ordered pair (182); annotations after a modifier are tolerated on variables, parameters, constructors and non-`void` methods | 3 | 58 of 58 |
| `MultipleVariableDeclarations` | two different messages; the report is at the first token, annotations included | 2 | 42 of 42 |
| `OneStatementPerLine` | tracks the line of the last `;` that ends a statement, with exceptions for `for` headers, resources, empty statements and `do ... while` | 8 | 2 of 2; 3 of 3 with try resources counted |
| `FallThrough` | what ends a case group, what an infinite loop is, where a "falls through" comment may sit | 14 | 1 of 1; 26 of 26 with `checkLastCaseGroup` |

What the oracle corrected, beyond the table:

- **Line numbers.** Checkstyle ends a line at `\n`, `\r\n` or a lone `\r`, and at nothing else
  (form feed, vertical tab, NEL, U+2028 and U+2029 do not, and Checkstyle cannot even parse a file
  with NEL, U+2028 or U+2029 outside a literal). Tree-sitter counts only `\n`, so a file with old-Mac line
  ends would have had every finding on line 1. javaid rewrites a lone `\r` to `\n` before parsing;
  the length does not change, so no offset moves.
- **A module's `id` is its name in the output.** Google's `noTrailingWhitespace`, `TodoFormat`
  and `BoxComments` appear in brackets where `RegexpSingleline` and `TodoComment` would. javaid
  prints the id too, so its output can be diffed against Checkstyle's as it is.
- **Where a file-level module looks.** A Checker-level `fileExtensions` is a ceiling and a module's
  own list narrows it; Google's lists `java, properties, xml`, so `FileTabCharacter` and
  `RegexpSingleline` report on XML and properties files, and `LineLength` (limited to `java`) does
  not. javaid reads those files for the modules that ask. Without any list Checkstyle reads every
  file under the paths and javaid reads `.java` only, and says so.
- **Three upstream-grammar defects, found because the fork disagreed.** tree-sitter-java 0.23.5
  mis-parses `0_7l` (an `ERROR` node) and turns `case Circle(), Square() ->` into two method calls
  and `instanceof final @TA String s` loses its modifiers. The fork gets all three right. The
  grammar-agreement test names these probes instead of loosening itself.
- **Behaviour that is not the language.** `FallThrough` treats `yield` as an ending but not
  `System.exit()`; a `while (b)` loop whose body returns ends the case; a nested switch ends it when
  every group does, with no `default` needed, but not when it uses arrow rules; and an infinite loop
  ends it unless a `break` sits where Checkstyle looks for one. That last rule is a quirk of where it
  looks (the end of the body, through blocks, `if`/`else`, a `try` without `catch`,
  `synchronized` and labelled blocks, but not through a nested `if`), found by crossing 90 loop
  bodies with three loop forms. Six shapes of it are not reproduced (an `if` that pairs a `return`
  with a `break`, nested in the loop) and are listed in `make-fallthrough-probes.py` instead of
  being left in a fixture. `OneStatementPerLine` has two quirks of the same kind about members that
  share a line.
- **Refused by name, not guessed:** `ArrayTypeStyle javaStyle=false` (it flags methods in both
  modes and generics in one), `ModifierOrder modifiersOrder`, `AvoidStarImport maxAllowedStarImports`
  and a `FallThrough reliefPattern` other than the default.

### Mutation testing

The first corpus runs said little about half of these modules: at their defaults Guava and Spring
contain 3 findings for `FallThrough` and `OneStatementPerLine` together, none for `UpperEll` or
`AvoidStarImport`. Agreeing on nothing is not evidence. So `bench/oracle/mutate_corpus.py` makes a
copy of each corpus and edits it in small, valid ways (a regular-expression edit of one or two
lines, seeded): `10L` becomes `10l`, `import a.b.C;` becomes `import a.b.*;`, `public static`
becomes `static public`, `int[] xs` becomes `int xs[]`, two one-line statements are joined onto one
line, a `break;` line is deleted, a `default:` becomes `case 0:`. Checkstyle and javaid then read the
same mutated tree. 2,414 of Guava's 3,277 files and 5,983 of Spring's 9,293 were changed.

| module | Checkstyle's findings in the mutated trees | javaid |
|---|---:|---|
| `OneStatementPerLine` (try resources counted) | 22,317 | all |
| `AvoidStarImport` | 13,156 | all |
| `MultipleVariableDeclarations` | 3,407 | all |
| `ModifierOrder` | 3,086 | all |
| `UpperEll` | 878 | all |
| `ArrayTypeStyle` | 287 | all |
| `MissingSwitchDefault` | 172 | all |
| `FallThrough` (last group checked) | 107 | all |
| **total** | **43,410** | **43,410 of 43,410, no extras** |

It paid for itself at once. The first run had **one extra** `OneStatementPerLine` finding in 43,411:
`do { ... } while (cond); return x;` on one line, produced when the mutator joined a loop's closing
`while (...);` to the `return` after it. Checkstyle does not count the `;` that ends a `do`-`while`
as the end of a statement; my model did; the hand-written probes had never put a statement after a
`do`-`while`. That case is now probe `OneProbe6`. The mutated trees are not committed (they are
derived from Guava and Spring, and take seconds to regenerate); the numbers are in
`bench/results/parity-mutated.txt`. A second seed and a higher rate would find more. This is the
cheapest way I have found to test a lint rule: it needs no understanding of Java.

### What a real config looks like today

Running the config reader on Checkstyle's own bundled configs shows how far javaid is from a
drop-in, and why:

| config | module instances | run | skipped: module not implemented | skipped: implemented but not honoured |
|---|---:|---:|---:|---:|
| `google_checks.xml` | 106 | 44 | 62 | 0 |
| `sun_checks.xml` | 73 | 33 | 40 | 0 |

It was 10 of the Google instances at the start of Phase 1, with 8 more that javaid implemented but
could not honour. That list was the work of round 1: `tokens` on `NeedBraces` and on `TypeName`
(Google scopes two `TypeName` instances to different type kinds), a lookahead in the
`LocalVariableName` and `LocalFinalVariableName` formats that Rust's `regex` crate cannot compile
(javaid now falls back to a slower engine for those patterns only), `commentFormat` on
`EmptyCatchBlock`, and suppression filters whose `${...}` placeholder has a default. Round 2 added
the file-level modules, which brought `fileExtensions`, ids, two instances of `TodoComment` and
patterns with a backreference. Each is handled, and each was checked against Checkstyle instead of
assumed. After round 3, about 58% of the Google instances and 55% of the Sun ones still belong to
modules javaid does not implement. Two limits remain on the ones that do run:

- **Custom messages are not applied.** `google_checks.xml` words 14 of its naming messages and both
  `TodoComment`s its own way. javaid prints its own wording and lists each one as a `note` on
  stderr. The text differs; whether and where a finding is reported does not, and the comparison
  keys on file, line and rule (or the id).
- **One instance per module**, except `TypeName` (one per type kind), `RegexpSingleline`,
  `TodoComment`, `OperatorWrap`, `SeparatorWrap` and `LeftCurly`, whose findings carry the instance's
  `id`. A second instance of any other module is skipped by name. Neither bundled config needs one.

"Run" is only worth something if it means "agrees". `bench/oracle/config-parity.sh` cuts a bundled
config down to the instances javaid implements without editing any of them (`make-subset.py`), and
Checkstyle and javaid then run the identical file over both corpora, files of every extension the
config names included:

| subset of | instances | Checkstyle's findings | javaid |
|---|---:|---:|---|
| `google_checks.xml` | 44 | 1,025,538 | **1,025,538 of 1,025,538**, no extras |
| `sun_checks.xml` | 33 | 345,358 | **345,358 of 345,358**, no extras |

After round 1 these were 16 instances with 1,188 findings and 12 with 11,658. The Google subset is
dominated by `FileTabCharacter` (934,099: Google asks for every line, in Java, XML and properties
files), then `LineLength` at 100 columns (79,029); the rest is where the variety is: 479
`LocalVariableName` findings through a lookahead format, 247 `EmptyCatchBlock` with a
`commentFormat`, 1,224 `TodoFormat` and 113 `noTrailingWhitespace`, the last two printed under the
ids Google gives them. The Sun subset adds `UnusedImports` with its one non-default setting,
`processJavadoc=false` (1,440), and `LineLength` at the default 80 (306,738). `CatchParameterName`,
`RecordTypeParameterName` and Google's `BoxComments` report nothing on either corpus, so only the
probe fixtures vouch for them. Raw tables: `bench/results/parity-google-subset.txt` and
`bench/results/parity-sun-subset.txt`.

**Where the other 77 and 49 instances are**, counted by the Checkstyle package of each module (read
from the metadata in the jar):

| family | `google_checks.xml` | `sun_checks.xml` |
|---|---:|---:|
| whitespace and wrapping | 19 | 11 |
| Javadoc | 14 | 6 |
| coding | 9 | 8 |
| miscellaneous (the top-level package) | 4 | 4 |
| suppression filters | 15 | 3 |
| everything else (annotation, imports, blocks, naming, indentation, sizes, design, modifier, regexp, header) | 16 | 17 |

Written before round 3, which implemented the nine whitespace modules in this list: thirteen rule modules that javaid lacked then appeared in *both* bundled configs, which makes them the
likeliest to matter to a team: `GenericWhitespace`, `MethodParamPad`, `NoWhitespaceBefore`,
`OperatorWrap`, `ParenPad`, `SeparatorWrap`, `WhitespaceAfter`, `WhitespaceAround` and `LeftCurly`
(whitespace and wrapping, nine), `JavadocMethod`, `MissingJavadocMethod` and
`InvalidJavadocPosition` (Javadoc, three) and `Indentation`. The two suppression filters
`SuppressionFilter` and `SuppressionXpathFilter` are the only other thing both configs share.

Judgement, not measured. The nine whitespace modules are token-adjacency rules. Tree-sitter keeps
every token with its position, but generics, lambdas, annotations, array initialisers and text blocks
are where zero diff gets expensive, as the `ArrayTypeStyle` column rule showed in miniature. The
Javadoc modules need a real Javadoc parser and tag semantics, of which the `UnusedImports` scan is
only the first thin slice. `Indentation` re-implements a layout engine and is the likeliest never to
reach zero diff.

### Round 3: the whitespace and wrapping family

The judgement above said these nine modules were token-adjacency rules whose cost would sit in
generics, lambdas, annotations and array initialisers. That held: `WhitespaceAround` took more
probing than the other eight together, and the two things that decided it were which token a
brace or a colon is. The method did not change: a probe file written before any code, one oracle
config per token (and per option), the golden findings recorded from Checkstyle 14.4.0, the test
watched failing, the code written, then the corpora and mutated copies of them. Each module is its
own commit, so any of them can be lifted out alone.

| module | probe-and-config pairs | corpus and mutation run |
|---|---:|---|
| `ParenPad` | 33 | identical on every config |
| `MethodParamPad` | 13 | 100% on four configs; 118,073 findings at the defaults and 1,846,582 with `option=space` |
| `NoWhitespaceBefore` | 18 | 100% on 11 configs, 21,458 findings at the defaults |
| `WhitespaceAfter` | 28 | 100% on 21 configs (9,779 at the defaults); 13,241 of 13,241 on Google's selection, with blanks taken out of comments and annotations |
| `WhitespaceAround` | 67 | 100% on the configs rerun after the last fix |
| `GenericWhitespace` | 1 (183 findings) | 55 of 55 on the corpora; 7,591 of 7,591 on a copy with blanks put round angle brackets |
| `OperatorWrap` | 76 (a probe of 269 findings under all tokens) | 55,988 of 55,988 across four instances, operators moved across line breaks |
| `SeparatorWrap` | 28 (all nine tokens, both options, a probe of every kind of parenthesis) | 1,735,000 of 1,735,000 across six instances, separators moved across line breaks |
| `LeftCurly` | 111 (all four options; 1,323 constructs) | 1,093,513 of 1,093,513 across seven instances, braces moved to and from lines of their own |

What the oracle taught this time:

- **Reading the check beat probing it, for three of the modules.** `OperatorWrap` looked like a text
  rule until `x = new` at the end of a line was flagged and `x = new Foo` was not. The bytecode of
  the check (`javap -c` on the jar) showed it compares the lines of nodes in Checkstyle's tree, and
  that the tree puts a prefix operator, `new` and a cast's `(` above the operand they introduce.
  `LeftCurly` was read from Checkstyle's published source, then confirmed by 59 configs;
  `SeparatorWrap` turned out to be the text rule `OperatorWrap` is not. Both source and bytecode
  were used as a reference for behaviour; the code in javaid is its own.
- **Defaults that differ between siblings.** `OperatorWrap`'s option defaults to `nl`,
  `SeparatorWrap`'s and `LeftCurly`'s to `eol`. A config generator that assumed one default wrote a
  config that was, by accident, the other, and the probe showed it.
- **Checkstyle reports a finding once.** A brace reached by two of an instance's tokens (the block
  of `case 1 -> {` belongs to both the switch rule and the `case`) is one finding. javaid now
  does the same per instance.
- **Tokens the first version of a module did not know.** Google's `WhitespaceAfter` names
  `ANNOTATIONS` and `SINGLE_LINE_COMMENT`, which 14.4 accepts; without them the module was skipped
  from the very config it was written for. Both were probed. The corpus run then found 146 extra
  findings (Spring's `@Caching(cacheable = { @Cacheable("a"), ... })`: an annotation that is the
  value of another is not in a list of annotations), now a probe line.
- **A mutation can invalidate itself.** The first comment-and-annotation pass turned `@Foo Bar`
  into `@FooBar`, Checkstyle aborted on those files and javaid reported 10,775 findings it had no
  twin for. The numbers above are from the corrected pass; a run that reports a large "extra" count
  with a non-empty Checkstyle stderr should be read as a broken mutation first.
- **Still approximate.** `LeftCurly` places the first statement after a brace by Checkstyle's rule
  for expression statements (its root operator, a call's `(`); other nodes are placed at their
  first token. A brace followed on its own line by a statement that begins before and ends after a
  line break is the only shape known to differ, and none appears in either corpus.

The scripts: `bench/oracle/make-*-configs.py` write the oracle configs, `bench/oracle/mutate_corpus.py`
has one pass per module (`--generics`, `--wrap`, `--separators`, `--curly`, `--comments`, `--around`,
`--before-punct`, `--after-punct`, `--before-paren`, `--parens`), and `bench/oracle/config-parity.sh`
runs a bundled config through both tools.

### Speed with 21 modules

Same machine and method as section 1. Checkstyle and javaid ran the identical 21-module config
(`bench/oracle/phase1.xml`, Checkstyle's defaults), so both did the same work, which the parity
tables above and `bench/results/parity-phase1.txt` check finding by finding (Guava 9,265 of 9,265,
Spring 1,934 of 1,934; the four Checkstyle Javadoc-parser diagnostics are excluded, and the tool
says so). Checkstyle ran with default JVM flags, its faster configuration in section 2.1. PMD is
absent because it has no twin for most of these modules. Samples: javaid 10, Checkstyle 3 (one
file: 50 and 10).

| corpus | tool | wall | peak RSS | CPU time |
|---|---|---:|---:|---:|
| Guava | Checkstyle, 21 modules | 35.1 s (35.0-35.9) | 3.3 GB | 58 s |
| Guava | **javaid, 1 thread** | **3.38 s** (3.34-3.46) | **18 MB** | 3.4 s |
| Guava | **javaid, 8 threads** | **0.48 s** (0.48-0.50) | 39 MB | 3.7 s |
| Guava | **javaid, 16 threads** | **0.44 s** (0.44-0.45) | 59 MB | 6.2 s |
| Spring | Checkstyle, 21 modules | 57.2 s (57.2-58.1) | 3.3 GB | 89 s |
| Spring | **javaid, 1 thread** | **5.62 s** (5.54-5.67) | **21 MB** | 5.6 s |
| Spring | **javaid, 8 threads** | **0.82 s** (0.80-0.89) | 42 MB | 6.1 s |
| Spring | **javaid, 16 threads** | **0.88 s** (0.73-0.92) | 57 MB | 12.1 s |
| one file | Checkstyle, 21 modules | 0.67 s (0.66-0.73) | 182 MB | 2.9 s |
| one file | **javaid** | **3.3 ms** (3.1-4.2) | **14 MB** | 0.00 s |

Against Checkstyle: 10.4x (Guava) and 10.2x (Spring) on one thread, 79x and 65x with all threads,
about 200x on one file, and 17x and 16x fewer CPU-seconds on one thread.

- **The ratio did not move while the work quadrupled.** The spike's five modules took javaid 3.05 s
  and 5.00 s; 21 modules take 3.38 s and 5.62 s (+11% and +12%). About half of that is the engine,
  not the rules: restructuring it so every module shares one walk, dispatched through a node-kind
  table, made the *same five modules* run in 3.21 s and 5.31 s (+5% and +6%,
  `bench/results/phase1-spike-config`; that comparison spans two sessions, so read it as
  indicative). Adding 16 modules, including the Javadoc scan, cost about another 5% in the same
  session. Checkstyle went from 30.5 s to 35.1 s (+15%) on Guava and from 47.3 s to 57.2 s (+21%)
  on Spring over the same step, which also switches `UnusedImports` from `processJavadoc=false` to
  its default of parsing Javadoc.
- **One file stays at 3 ms.** Loading 16 more modules, with the naming modules' regular expressions
  compiled at startup, adds about 0.1 ms (3.2 ms to 3.3 ms).
- **Spring's file set changed.** Both tools now follow the symlink and read 9,515 files, 222 more
  than the spike gave javaid. The header of `bench/results/phase1/RESULTS.md` still shows the
  harness's own count of 9,293.
- **16 threads buy nothing over 8** (Spring 0.82 s against 0.88 s): the machine has 8 physical cores.
- **Checkstyle's 3.3 GB is the default JVM's choice, not a need.** A 512 MB heap runs it in 0.7 GB
  at a cost of about 40% in time on Spring (section 2.1).
- **Noise.** These are single-session runs, not the † re-runs of section 1. Their ranges are tight
  (Checkstyle's three samples within 3% of each other), so there is no sign of the noise the spike's
  formal run had, but I did not rule it out; the load average was about 1.7 when the one-file run
  started.
- **Still not measured at 21 modules:** PMD, the 2-core CI runner and a warm resident JVM.

### Speed with 33 modules

The same method and the same machine as the 21-module run: `bench/phase2-bench.sh`, javaid and
Checkstyle (default JVM) on the identical `bench/oracle/phase2.xml`, which has all 33 modules. The
21 of round 1 are at Checkstyle's defaults and the 12 added are at Google's settings where the
defaults would print a million findings (`FileTabCharacter` first tab only, `LineLength` at 100), so
that the timing measures analysis and not printing. Afterwards the two outputs are diffed:
**10,296 of 10,296** findings on Guava and **89,118 of 89,118** on Spring, none extra
(`bench/results/parity-phase2.txt`). This is the quiet run, with the binary as it was before the
optimisation described below.

| corpus | tool | wall | peak RSS | CPU time |
|---|---|---:|---:|---:|
| Guava | Checkstyle, 33 modules | 35.9 s (34.7-36.7) | 3.3 GB | 61 s |
| Guava | **javaid, 1 thread** | **4.07 s** (3.84-4.29) | **19 MB** | 4.1 s |
| Guava | **javaid, 8 threads** | **0.68 s** (0.54-0.83) | 42 MB | 5.2 s |
| Guava | **javaid, 16 threads** | **0.60 s** (0.57-0.71) | 57 MB | 8.2 s |
| Spring | Checkstyle, 33 modules | 58.4 s (57.1-59.0) | 3.3 GB | 90 s |
| Spring | **javaid, 1 thread** | **6.38 s** (6.21-6.72) | **31 MB** | 6.4 s |
| Spring | **javaid, 8 threads** | **0.98 s** (0.95-1.10) | 53 MB | 7.1 s |
| Spring | **javaid, 16 threads** | **0.89 s** (0.86-1.04) | 67 MB | 11.8 s |
| one file | Checkstyle, 33 modules | 0.81 s (0.68-0.96) | 193 MB | 3.6 s |
| one file | **javaid** | **3.9 ms** (3.6-9.6) | **14 MB** | 0.00 s |

Against Checkstyle: 8.8x (Guava) and 9.1x (Spring) on one thread, 60x and 66x with all threads,
about 210x on one file, and 15x and 14x fewer CPU-seconds on one thread.

- **The twelve new modules were not free, and then they were cheaper.** In this run, 21 to 33
  modules took javaid from 3.38 s to 4.07 s on Guava (+20%) and from 5.62 s to 6.38 s on Spring
  (+14%), and Checkstyle from 35.1 s to 35.9 s and from 57.2 s to 58.4 s (+2% each), so the
  per-core ratio slipped from about 10x to about 9x. Timing each module alone against a bare parse
  and walk (`bench/results/phase2-ab.txt`) found `OneStatementPerLine` costing five times what a
  trivial module does, with `ModifierOrder`, `MultipleVariableDeclarations`, `FallThrough` and
  `MissingSwitchDefault` next. The cause was not in the rules. Tree-sitter nodes know no parent:
  `Node::parent` searches from the root and `prev_sibling`/`next_sibling` search the parent's
  children, and these rules asked for one of each per statement. I could not profile, since `perf`
  is closed to unprivileged users here (`perf_event_paranoid` is 4, and loosening it is a system
  setting I did not change), so this was reasoned from how tree-sitter works and then measured.
  The walk now keeps the stack of ancestors and the previous sibling at each level, and the rules
  ask that. Interleaved A/B on the final binary, Guava, one thread: **33 modules cost +0.104 s
  (+3.2%) over the 21, down from +0.345 s (+10.5%)**. A debug-build assertion checks the stack
  against tree-sitter's own answer at every node of every probe file, and I confirmed it fails when
  I break the stack on purpose.
- **So the table understates the final binary, and I could not re-measure it cleanly.** A re-run of
  the whole benchmark with the final binary overlapped with an unrelated test loop on the same
  desktop (load average 14-18; Checkstyle's own Spring time went from 58 s to 77 s). Those files
  are in `bench/results/phase2-final-binary-loaded/` with a note and should not be quoted. The A/B
  is the evidence for the optimisation; on a quiet machine the 33-module one-thread run should land
  near 3.4 s on Guava, which would be about 10x.
- **One file stays under 4 ms.** 3.3 ms at 21 modules, 3.9 ms at 33: the extra setup is compiling
  the new modules' regular expressions and opening more of the config.
- **Memory is still 19-67 MB** against Checkstyle's 3.3 GB at its default JVM size (0.7 GB with a
  512 MB heap, slower on Spring, section 2.1).
- **Not measured at 33 modules:** PMD, the 2-core CI runner and a warm resident JVM.

### Speed with 42 modules

`bench/phase3-bench.sh`, the same method as the 33-module run, on `bench/oracle/phase3.xml` (the 33
plus the nine whitespace and wrapping modules at their defaults). **This run was not quiet**: the
machine is shared and its load average stayed between 8 and 10 for the whole run (the script waited
ten minutes for it to fall below 2.5 and went ahead), so every absolute time is inflated and the
spread is wide. javaid's parse-only baseline was 3.75 s on Guava against 2.70 s in the quiet run
and Checkstyle took 38.9 s against 35.9 s. Read the ratios, not the seconds.

| | Guava | Spring |
|---|---:|---:|
| javaid, 42 modules, 1 thread (median; fastest) | 6.41 s (4.67 s) | 8.58 s (7.68 s) |
| javaid, 42 modules, all threads | 0.78 s | 1.54 s |
| Checkstyle, 42 modules, default JVM | 38.9 s | 78.2 s |
| per core | 6.1x | 9.1x |
| all threads | 50x | 51x |
| memory | 21-61 MB against 3.3 GB | 35-69 MB against 3.3 GB |
| one file | 5.3 ms against 0.83 s (155x) | |

Both corpora are identical to Checkstyle at 42 modules: **28,534 of 28,534** (Guava) and **118,215
of 118,215** (Spring) findings, none extra (`bench/results/parity-phase3.txt`).

What the nine modules cost: timed alone, one thread, fastest of five interleaved runs on Guava,
against a 2.9-3.1 s floor that includes parsing: `WhitespaceAround` +0.2 to +0.4 s,
`OperatorWrap` +0.2, `SeparatorWrap` +0.1 to +0.2, `LeftCurly` +0.06 to +0.25, and
`ParenPad`, `MethodParamPad`, `NoWhitespaceBefore`, `WhitespaceAfter` and `GenericWhitespace` none
that I could measure. The cost is mostly that a module which looks at tokens makes the walker
hand it every operator and punctuation token. I tried to cut it (no allocation until a finding,
the right-hand side read only for `nl`) and an interleaved A/B showed no difference, so I dropped
the change. Per core the ratio against Checkstyle moved from 8.8x and 9.1x at 33 modules to
6.1x and 9.1x at 42 in this loaded run, so the Guava drop may be partly noise; a quiet re-run
would settle it and I did not get one.

## 7. Reproduce

See `README.md` and `bench/`. Each result folder in `bench/results/` holds a `RESULTS.md` and, where
the harness produced one, a `results.json` with every sample.

| script | what it produced |
|---|---|
| `bench/jvm-flags.sh` | the JVM-flag sweep that chose the baseline configurations |
| `bench/formal.sh` | `results/latency`, `results/large`, `parity-*.txt` (the formal run) |
| `bench/rerun-quiet.sh` | `results/final-binary`, `results/spring-checkstyle-rerun` |
| `bench/rerun-ci2.sh` | `results/ci2-rerun` and the Javadoc-effect counts |
| `bench/startup-fix.sh` | `results/startup-fix` (section 2.5) and `results/ci2-cs-default` |
| `bench/phase1-bench.sh` | `results/phase1`, `results/phase1-spike-config`, `results/phase1-latency`, `parity-phase1.txt` (section 8) |
| `bench/phase2-bench.sh` | `results/phase2`, `results/phase2-latency`, `parity-phase2.txt` (33 modules, section 8); `results/phase2-ab.txt` is the A/B by hand |
| `bench/oracle/mutate_corpus.py` | edited copies of the corpora, on which the rare rules fire (section 8, "Mutation testing") |
| `bench/oracle/config-parity.sh` | `oracle/<name>-implemented.xml` and `parity-<name>-subset.txt`: a bundled config cut to what javaid implements (`make-subset.py`), run by both tools |
| `bench/oracle/make-reference.sh` | Checkstyle's reference output for an oracle config over both corpora |
| `bench/oracle/record-golden.py` | one probe fixture in `bench/oracle/probes`, replayed by `cargo test` |
| `bench/parity.py` | the oracle diff against Checkstyle (or PMD) output |

Tools and corpora are downloaded outside the repo (Checkstyle 14.4.0 and PMD 7.29.0 release
artifacts, Temurin 25.0.5, shallow clones of Guava and Spring Framework); each script takes the
directory as its first argument.
