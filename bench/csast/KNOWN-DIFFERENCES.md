# Known differences from Checkstyle 14.4.0's AST

Measured against `java -jar checkstyle-14.4.0-all.jar -t` / `-T` (`bench/csast/compare.sh`):

| corpus | files | `-t` identical | `-T` identical |
| --- | ---: | ---: | ---: |
| guava + spring-framework | 12,570 | 12,570 | 12,570 |
| JDK 25.0.5 `lib/src.zip` (all modules) | 15,235 | 15,235 | 15,235 |
| the 38 hand-written snippets + 21 re-laid-out copies (`bench/csast/snippets*`) | 59 | 59 | 59 |
| 300 corpus files, each in 3 layouts (one line / one token per line with random tabs / random spacing and CRLF) | 900 | 900 | 900 |
| 300 JDK files with 70% of the expressions parenthesized, then re-laid-out in the 3 layouts | 900 | 900 | 900 |
| 400 guava/spring files, 20% of the expressions parenthesized | 400 | 400 | 400 |
| 400 JDK files, 70% of the expressions parenthesized | 400 | 400 | 400 |

No file of the real corpora (27,805 files) differs. The differences below were found on
hand-written inputs; each is listed with the file that shows it (`/tmp` is where they were written;
they are small enough to recreate from the quoted source).

## What differs

1. **A UTF-8 byte-order mark at the start of the file.** Checkstyle's lexer reads U+FEFF as an
   identifier character, so `﻿package a; class Bom { }` is the declaration of a variable `a` of
   type `﻿package` and the root is `COMPACT_COMPILATION_UNIT`. tree-sitter skips the mark, javaid
   builds `COMPILATION_UNIT` with the columns of line 1 shifted by one.
   First differing line: 1 (`COMPACT_COMPILATION_UNIT -> COMPACT_COMPILATION_UNIT [1:1]`).

2. **Characters Checkstyle accepts in identifiers and Java does not.** The lexer takes every non-ASCII
   character as a letter; the tree-sitter grammar follows `ID_Start`/`ID_Continue`. `int 😀x = 0;`:
   Checkstyle has `IDENT -> 😀x [1:19]`, javaid `IDENT -> x [1:20]`. (Supplementary letters that Java
   does allow, such as mathematical alphanumerics, are fine.)

3. **An assignment as the false branch of `?:`.** `x = a ? b : c = d;` is not valid Java; ANTLR parses
   it as `(a ? b : c) = d`, tree-sitter as `a ? b : (c = d)`.
   First differing line: the `ASSIGN`/`QUESTION` nesting (line 20 of the dump of
   `class T { void m() { x = a ? b : c = d; } }`).

4. **A switch statement with a trailing `;` that is the body of `if`/`while`/`for`/`else`.**
   `if (a) switch (o) { default -> f(); };`: Checkstyle ends the `if` after the switch and reads the `;`
   as an `EMPTY_STAT` after the `if`; tree-sitter puts the `;` inside the body, and javaid makes the
   `EMPTY_STAT` a sibling of the `LITERAL_SWITCH` inside the `if`. (As a plain statement,
   `switch (o) { ... };`, javaid matches; the JDK has that form.)

5. **Syntax tree-sitter's Java grammar does not have.** The enhanced `for` with a record pattern
   (`for (Point(int a, int b) : list)`, a Java 20 preview feature Checkstyle's grammar still accepts)
   and every file with a syntax error: `try_build_source`/`build_checked` return an error
   ("syntax error at LINE:COL") instead of an AST. `build` itself assumes a tree without `ERROR`
   nodes and panics on shapes only error recovery produces (caught: that file then gets no emulated AST).

6. **Input Checkstyle's grammar accepts but Java does not** in general, where tree-sitter either
   rejects it (5) or reads it as Java does (3).

## Workarounds that stand in for ANTLR's parse (and their limits)

These are not differences in the measured corpora, but they are places where the output depends on
heuristics instead of the grammar, so they are listed:

* **Casts.** tree-sitter sometimes gives a cast the wrong reach inside an arithmetic chain
  (`x * (a) + b * c` reads `(a) + b` as a cast of `+b`; `k * (T) (b) + c * d` gives the cast the
  operand `(b) + c`). The builder detects these casts, flattens the binary operators of the
  surrounding expression and applies the precedences again (`expr.rs`, "casts that tree-sitter
  reads with the wrong reach"). It only crosses binary operators and casts: a misparse that spreads
  over `?:`, `instanceof` or an assignment is not repaired. The first appeared 1 time in 27,805
  files, the second 0 times.
* **Lone carriage returns.** ANTLR ends `//` comments (and lines) at a `\r` that is not followed by
  `\n`; tree-sitter does not. `try_build_source` hands tree-sitter a copy with such `\r` replaced by
  `\n` (same length, so every offset is unchanged). `build(&Tree, &[u8])` takes whatever tree the
  caller made, so a caller that wants this behaviour has to do the same.
* **Things tree-sitter parses as identifiers.** `case null, default ->` (`default` as an identifier),
  `@A Foo this` (a parameter named `this`), `A.super::m` (a type named `A.super`) are mapped back to
  `LITERAL_DEFAULT`, `LITERAL_THIS` and `LITERAL_SUPER`.
* **Deep nesting.** The builder recurses on expression depth (long `a + b + c ...` chains).
  `build_source`/`try_build_source` run it on a thread with a 1 GiB stack; callers of `build` on a
  small stack can overflow it on pathological input. The printer is iterative.

## How to reproduce

```
export JAR=.../checkstyle-14.4.0-all.jar JAVA_HOME=.../jdk-25
cargo build --release
find /path/to/corpus -name '*.java' | sort > list
ORACLE_CACHE=/tmp/cs-cache bench/csast/compare.sh -n 10 list        # -t
ORACLE_CACHE=/tmp/cs-cache bench/csast/compare.sh -T -n 10 list     # -T
```

The JDK corpus is `$JAVA_HOME/lib/src.zip`, extracted into an empty directory.
