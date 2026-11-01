# javaid

A fast Java linter written in Rust, built to **reproduce Checkstyle's findings exactly**. It is a
v0.1 preview (see [CHANGELOG.md](CHANGELOG.md)): 64 modules, a `checkstyle.xml` reader, no cache, no
fixes. The goal is a drop-in for Checkstyle configurations; today it runs the modules it has and
lists, by name, every one it skips. The project began with one question, answered with evidence:

> Can a native, parallel, syntax-only linter reproduce Checkstyle's findings at a
> fraction of the time and memory?

The answer, the method and the caveats are in [FINDINGS.md](FINDINGS.md).

## What it does

Parses every `.java` file with [tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java),
walks each syntax tree **once** (all rules share the walk, dispatched through a node-kind lookup
table), and processes files in parallel with one reused parser per thread.

| family | modules (Checkstyle's names) |
|---|---|
| blocks and coding | `EmptyCatchBlock`, `NeedBraces`, `EqualsHashCode`, `MissingSwitchDefault`, `FallThrough`, `MultipleVariableDeclarations`, `OneStatementPerLine` |
| imports | `UnusedImports`, including Javadoc references (`processJavadoc`); `AvoidStarImport` |
| modifiers and small checks | `ModifierOrder`, `ArrayTypeStyle`, `UpperEll` |
| text, per line or per comment | `FileTabCharacter`, `LineLength`, `RegexpSingleline`, `TodoComment` |
| whitespace and wrapping | `ParenPad`, `MethodParamPad`, `NoWhitespaceBefore`, `WhitespaceAfter`, `WhitespaceAround`, `GenericWhitespace`, `OperatorWrap`, `SeparatorWrap`, `LeftCurly` |
| naming | `TypeName`, `MethodName`, `MemberName`, `ConstantName`, `StaticVariableName`, `ParameterName`, `LocalVariableName`, `LocalFinalVariableName`, `PackageName`, `ClassTypeParameterName`, `InterfaceTypeParameterName`, `MethodTypeParameterName`, `RecordTypeParameterName`, `CatchParameterName`, `LambdaParameterName`, `PatternVariableName`, `RecordComponentName` |

The four text modules need no syntax tree. They also read the files a Checker-level `fileExtensions`
adds (`.properties`, `.xml`), as Checkstyle does. A module's `id` is what a finding is labelled with,
as in Checkstyle.

Checkstyle is the **oracle**. A module is accepted only at zero difference from Checkstyle 14.4.0
on two large codebases (Guava, Spring Framework) and on probe files that exercise every construct,
which are pinned as fixtures in `bench/oracle/probes` and checked by `cargo test`.

## Use

```bash
cargo build --release
target/release/javaid path/to/src                       # every module, at Checkstyle's defaults
target/release/javaid --config checkstyle.xml src/      # your Checkstyle configuration
target/release/javaid --rules NeedBraces,MethodName src/
target/release/javaid --list-rules
target/release/javaid -j 1 --stats src/                 # one thread, timings on stderr
```

Output is `path:line:col: Module: message`. Exit status is 1 if anything was reported.

**It never skips silently.** With `--config`, a module javaid does not implement, or a property it
cannot honour, is listed on stderr with the reason, and that module does not run:

```
javaid: google_checks.xml: running 44 module(s), skipped 62
  skipped SuppressWarningsFilter: not implemented
  skipped SuppressionFilter: not implemented
  ...
  skipped Indentation: not implemented
  note TypeName: the custom message for 'name.invalidPattern' is not applied; javaid prints its own wording
```

A property it cannot honour reads `skipped NeedBraces: unsupported property: allowSingleLineStatement`.
`${property}` placeholders take their `default` as Checkstyle does; one with no default skips just
that module. The reader is checked, not assumed: the 44 instances javaid runs from Checkstyle's
`google_checks.xml` and the 33 from `sun_checks.xml` give exactly Checkstyle's findings on Guava and
Spring Framework, XML and properties files included (1,025,538 of 1,025,538 and 345,358 of 345,358).

By default javaid follows symlinked directories, as Checkstyle does (`--no-follow-symlinks` turns
that off).

## Reproduce the benchmark and the oracle

Needs Checkstyle's `-all.jar`, a PMD distribution, and two corpora (Guava and Spring Framework,
shallow clones). The scripts in `bench/` take the directory that holds them as their first argument.

```bash
cargo test                                   # unit tests and the Checkstyle-derived probe fixtures
python3 bench/run_bench.py --help            # the timing harness
python3 bench/parity.py reference.out javaid.out    # per-module diff against Checkstyle (or PMD) output
bench/oracle/make-reference.sh <dir> bench/oracle/naming.xml naming    # Checkstyle's reference output
bench/oracle/config-parity.sh <dir> <dir>/google_checks.xml google     # a bundled config, cut to what javaid runs
python3 bench/oracle/mutate_corpus.py <src> <dst>                      # a copy of a corpus edited so the rare rules fire
bench/phase2-bench.sh <dir>                                            # speed with all 33 modules, then the same diff
```

## Results at a glance

The spike's numbers: five rules, same files, on an i7-10700 desktop. Medians; each baseline in its
fastest configuration. Method, ranges, 2-core CI-runner numbers and threats to validity are in
[FINDINGS.md](FINDINGS.md). Section 8 there has the Phase 1 numbers for 21 and 33 modules.

| | Checkstyle 14.4 | PMD 7.29 | javaid |
|---|---:|---:|---:|
| Guava (3,277 files), 1 thread | 30.5 s | 14.7 s | **3.05 s** |
| Guava, 16 threads | single-threaded | 7.4 s | **0.40 s** |
| Spring Framework (9,293 files), 1 thread | 47 s | 18.5 s | **5.0 s** |
| Spring Framework, 16 threads | single-threaded | 11.4 s | **0.81 s** |
| 2-core CI runner (Guava / Spring) | 34 s / 52 s | 17 s / 20 s | **1.6 s / 2.6 s** |
| Peak memory | 0.7-3.3 GB | 0.5-2.5 GB | **17-53 MB** |
| One file (hook / editor / agent) | 0.59 s | 1.21 s | **3.2 ms** |

With **all 33 modules** javaid and Checkstyle do the same work, checked finding by finding (99,414
identical findings on the two corpora, none extra), and the ratio holds. Checkstyle's defaults for
the first 21 modules, Google's settings for the file-level ones:

| | Checkstyle 14.4 | javaid |
|---|---:|---:|
| Guava (3,277 files), 1 thread | 35.9 s | **4.07 s** (8.8x) |
| Guava, 16 threads | single-threaded | **0.60 s** (60x) |
| Spring Framework (9,515 files with the symlink), 1 thread | 58.4 s | **6.38 s** (9.1x) |
| Spring Framework, 16 threads | single-threaded | **0.89 s** (66x) |
| Peak memory | 3.3 GB | **19-67 MB** |
| One file (hook / editor / agent) | 0.81 s | **3.9 ms** (209x) |

These numbers are from before a later optimisation of the tree walk that cut what the twelve newest
modules cost from about 10% to about 3% of the run; a clean re-run was not possible on a shared
desktop. See [FINDINGS.md](FINDINGS.md) section 8 for ranges, the A/B measurement and caveats.

A JVM startup fix does not erase the gap. JDK 25's AOT cache roughly halves the JVM tools' one-file
latency and changes a whole-repository run by 3.5-18%. Only a *warm resident* JVM matches javaid on
one file (Checkstyle 4.0 ms), at 733 MB. See [FINDINGS.md](FINDINGS.md) section 2.5.

## Known limits

- **64 modules.** Checkstyle 14.4 has 243 rule modules; its two bundled configs name 128 distinct
  modules, filters included. javaid runs every instance of the modules it implements, which is
  44 of the 106 in `google_checks.xml` and 55 of the 73 in `sun_checks.xml`, and
  says which it skips and why. Most of what is missing is Javadoc style, `Indentation`, the
  suppression filters and the rest of the whitespace family (`SingleSpaceSeparator`,
  `EmptyLineSeparator`, `GoogleRightCurly` and a few more), so javaid is **not yet a drop-in for those
  configs**. The speed figures below were measured with the first 33 modules. A
  42-module run on a loaded machine gave 6-9x per core and about 50x on all threads against
  Checkstyle, with identical findings (FINDINGS.md section 8); a quiet re-run is still to do.
- **Syntax-only.** No type resolution, so nothing that needs symbol or type information. This
  matches Checkstyle, which is also purely syntactic.
- **Grammar.** The default is the maintained fork `tree-sitter-java-orchard`. Upstream
  `tree-sitter-java` 0.23.5 cannot parse annotated varargs (`Object @Nullable ... args`), which
  Spring 7, JUnit 6 and Guava use; `--grammar upstream` switches back. A test documents the gap.
- **Modelled by probing, not by reading Checkstyle.** A few modules approximate control flow or
  layout in ways that are not obvious, and javaid reproduces the approximation. `FallThrough` decides
  whether a `break` can leave an infinite loop by looking at a few positions only; six odd shapes of
  that (an `if` pairing a `return` with a `break`, nested inside the loop) are not reproduced, and
  `OneStatementPerLine` follows a handful of quirks about members that share a line. Both are listed
  in `bench/oracle/probes/make-fallthrough-probes.py` and `FINDINGS.md`.
- **Checkstyle's Javadoc-parser diagnostics are not imitated.** At default settings Checkstyle
  prints "Javadoc comment ... has parse error" for odd Javadoc under the `UnusedImports` name; those
  are not unused-import findings and javaid does not produce them.
- **Properties not honoured yet** include a second instance of a module other than `TypeName`,
  `RegexpSingleline`, `TodoComment`, `OperatorWrap`, `SeparatorWrap` and `LeftCurly`, and custom
  `<message>` templates: those are
  listed as notes and javaid prints its own wording, which changes the text of a finding and not
  whether or where it is reported. Specifically refused, by name: `ArrayTypeStyle javaStyle=false`,
  `ModifierOrder modifiersOrder`, `FallThrough reliefPattern` other than the default, and
  `AvoidStarImport maxAllowedStarImports`. Regular expressions that need lookahead work, through a
  slower engine used only for them. All of this is reported by name, not ignored.
- **File types.** Without any `fileExtensions`, Checkstyle hands the file-level modules every file
  under the paths; javaid reads `.java` only, and prints a note saying so. A file with old-Mac
  (`\r`-only) line endings gets Checkstyle's line numbers. The one known difference there is
  `TodoComment` on a multi-line block comment: Checkstyle splits a comment's text only at `\n`, so it
  sees one line, while javaid has turned each `\r` into `\n` to parse the file and sees several.
- No cache, no `--fix`, no output format other than plain text.
- No licence has been chosen yet.
