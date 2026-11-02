# Changelog

## Unreleased (v0.1 preview, not published)

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
- Module registry and a suppression stage (no filters implemented yet).

The 1.0 bar was "Google's and Sun's bundled configs run fully with exact output"; it is met on the
two corpora. Known gaps: XPath constructs outside the supported subset are refused when a config
loads, `AvoidEscapedUnicodeCharacters.allowIfAllCharactersEscaped` is only checked on probes
(Checkstyle overflows its stack on the longest corpus string), and a few Checkstyle crashes
(for example `X::new` opening a line under `Indentation`) are reproduced as "no findings for the file".

CI: the workflow is kept at `ci/github-workflow.yml`, not under `.github/workflows/`, because the
token used to push lacked the `workflow` scope. Move it to `.github/workflows/ci.yml` to enable it.
