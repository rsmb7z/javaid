# Changelog

## 1.0.1

- The README's benchmark chart shows in dark mode on crates.io (it showed only its alt text).
  No change to the linter.

## 1.0.0

- 128 Checkstyle modules (Indentation, the Javadoc family, EmptyLineSeparator, CustomImportOrder, the XPath, SuppressWarnings, comment and text suppression filters, the Google naming modules and more), each accepted only at zero difference from Checkstyle 14.4.0 on probe
  files, two large corpora and mutated copies of them. The newest twenty-two are `EmptyStatement`,
  `TypecastParenPad`, `EmptyForIteratorPad`, `NoWhitespaceAfter`, `AvoidNestedBlocks`,
  `InnerAssignment`, `SimplifyBooleanReturn`, `SimplifyBooleanExpression`, `NewlineAtEndOfFile`,
  `FileLength`, `MethodLength`, `ParameterNumber`, `EmptyBlock`, `RightCurly`, `FinalClass`, `HideUtilityClassConstructor`, `InterfaceIsType`, `FinalParameters`, `RedundantImport`, `IllegalImport`, `IllegalInstantiation` and `MagicNumber`; before them came the whitespace and wrapping
  family (`ParenPad` ... `LeftCurly`).
- Google's bundled configuration (106 of 106 instances) and Sun's (73 of 73) run in full and give
  Checkstyle's findings exactly, in line, column and message, on Guava and Spring Framework:
  1,778,237 of 1,778,237 and 1,696,055 of 1,696,055 (`bench/parity.py --strict`).
- Output now matches Checkstyle in line, column and message, not only in line: columns are tab-expanded (`tabWidth`) and counted in UTF-16 units like Checkstyle's; `<message key=...>` overrides in a configuration are applied (`MessageFormat` quoting included); numbers in messages are grouped (`2,360`). `parity.py --strict` and the `.expected2` probe goldens (`line:col Rule message`) hold the line.
- Probe files are stored byte for byte (`.gitattributes`), so CRLF and lone-CR probes survive a clone.
- A `checkstyle.xml` reader that names every module or property it cannot honour.
- Module registry and a suppression stage: SuppressWarnings, comment, text and XPath filters.
- Licensed under the GNU LGPL, version 2.1 or later, the same terms as Checkstyle. Most of javaid is
  written from Checkstyle's observable behaviour; the modules and syntax-tree emulation translated
  from its source name their source and carry its copyright notice (see NOTICE).
- Release builds unwind on panic, so a tree shape the emulated-AST builder does not handle costs
  that one file its Checkstyle-AST modules (with a message on stderr) instead of ending the run.
- The library can be driven file by file: everything a file's findings depend on, its path
  included, goes through `engine::lint_file`, with no setup call beforehand.
- Code layout: Checkstyle's column arithmetic in one module (`columns`); the configuration in
  `src/config/`, one file per module family, with each registry row naming the function that reads
  its module's properties; `config` does not depend on the rules. The code is formatted with
  `rustfmt` (`rustfmt.toml`).

The 1.0 bar was "Google's and Sun's bundled configs run fully with exact output"; it is met on the
two corpora. Known gaps: XPath constructs outside the supported subset are refused when a config
loads, `AvoidEscapedUnicodeCharacters.allowIfAllCharactersEscaped` is only checked on probes
(Checkstyle overflows its stack on the longest corpus string), and a few Checkstyle crashes
(for example `X::new` opening a line under `Indentation`) are reproduced as "no findings for the file".

CI: `.github/workflows/ci.yml` checks formatting (`cargo fmt --check`), runs `cargo test --locked`, a release build and `cargo package` on every push and pull request. Pushing a `v*` tag publishes that version to crates.io (`.github/workflows/release.yml`).
