# Compatibility notes

javaid aims to print exactly what Checkstyle 14.4.0 prints. This page lists where it does not, and
how it behaves when a configuration asks for something it cannot do. The short version is in the
README's [Limits](../README.md#limits).

## What is verified

For `google_checks.xml` (106 module instances) and `sun_checks.xml` (73), javaid's output matches
Checkstyle 14.4.0 on Guava and Spring Framework on file, line, column, rule and message. The
comparison leaves out Checkstyle's own "Javadoc comment ... has parse error" lines (36 for the
Google config, 4 for Sun). At default settings Checkstyle prints them for odd Javadoc, under the
module's name (for example `UnusedImports`); they are not findings of that module and javaid does
not imitate Checkstyle's Javadoc parser diagnostics.

## What javaid does with a config it cannot fully run

It never skips silently. With `--config`, a module javaid does not implement, or a property it
cannot honour, is listed on stderr with the reason, and that module does not run:

```
javaid: custom.xml: running 17 module(s), skipped 1
  skipped Foo: not implemented
```

- `${property}` placeholders take their `default`, as Checkstyle does; one with no default skips
  just that module.
- Specifically refused, by name: `ArrayTypeStyle javaStyle=false`, `ModifierOrder modifiersOrder`,
  `FallThrough reliefPattern` other than the default, `AvoidStarImport maxAllowedStarImports`.
- XPath constructs outside the subset listed in `src/xpath.rs` (`MatchXpath`, the XPath suppression
  filters) are refused the same way.
- Custom `<message>` templates are applied for the modules in `src/messages_table.rs`; for any other
  the property is noted on stderr and javaid prints its own wording. That changes the text of a
  finding, not whether or where it is reported.
- Several instances of the same module in one configuration run for `IllegalTokenText`,
  `AnnotationLocation`, `TypeName`, `RegexpSingleline`, `TodoComment`, `OperatorWrap`,
  `SeparatorWrap`, `LeftCurly`, `DescendantToken`, `MatchXpath` and the suppression filters. For any
  other module a second instance is skipped and listed on stderr ("a second instance of the same
  module is not supported").
- Regular expressions that need lookahead work, through a slower engine used only for them.

## Syntax only

javaid has no type resolution, so nothing that needs symbol or type information. Checkstyle is also
purely syntactic, so this loses nothing against it.

## Grammar

The default is the maintained fork `tree-sitter-java-orchard`. Upstream `tree-sitter-java` 0.23.5
cannot parse annotated varargs (`Object @Nullable ... args`), which Spring 7, JUnit 6 and Guava use.
`--grammar upstream` switches back. With it, the modules built on the emulated Checkstyle syntax tree
(Indentation, VariableDeclarationUsageDistance, CommentsIndentation, the Google ports and the XPath
filters) re-parse with the Orchard grammar internally, and the rest use upstream. A test documents the
gap.

## Modelled by probing, not by reading Checkstyle

A few modules approximate control flow or layout in ways that are not obvious, and javaid reproduces
the approximation. `FallThrough` decides whether a `break` can leave an infinite loop by looking at a
few positions only; six odd shapes of that (an `if` pairing a `return` with a `break`, nested inside
the loop) are not reproduced, and `OneStatementPerLine` follows a handful of quirks about members that
share a line. Both are listed in `bench/oracle/probes/make-fallthrough-probes.py` and `FINDINGS.md`.

## File types and line endings

Without any `fileExtensions`, Checkstyle hands the file-level modules every file under the paths;
javaid reads `.java` only, and prints a note saying so. The four text modules also read the files a
Checker-level `fileExtensions` adds (`.properties`, `.xml`), as Checkstyle does.

A file with old-Mac (`\r`-only) line endings gets Checkstyle's line numbers. The one known difference
is `TodoComment` on a multi-line block comment: Checkstyle splits a comment's text only at `\n`, so it
sees one line, while javaid has turned each `\r` into `\n` to parse the file and sees several.

By default javaid follows symlinked directories, as Checkstyle does (`--no-follow-symlinks` turns that
off).

## Not there yet

No cache, no `--fix`, no output format other than plain text. Checkstyle 14.4.0's jar has roughly 250
check classes; javaid implements 128 modules (see [modules.md](modules.md)).
