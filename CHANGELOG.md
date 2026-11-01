# Changelog

## Unreleased (v0.1 preview, not published)

- 81 Checkstyle modules, each accepted only at zero difference from Checkstyle 14.4.0 on probe
  files, two large corpora and mutated copies of them. The newest twenty-two are `EmptyStatement`,
  `TypecastParenPad`, `EmptyForIteratorPad`, `NoWhitespaceAfter`, `AvoidNestedBlocks`,
  `InnerAssignment`, `SimplifyBooleanReturn`, `SimplifyBooleanExpression`, `NewlineAtEndOfFile`,
  `FileLength`, `MethodLength`, `ParameterNumber`, `EmptyBlock`, `RightCurly`, `FinalClass`, `HideUtilityClassConstructor`, `InterfaceIsType`, `FinalParameters`, `RedundantImport`, `IllegalImport`, `IllegalInstantiation` and `MagicNumber`; before them came the whitespace and wrapping
  family (`ParenPad` ... `LeftCurly`).
- On the instances javaid implements, Google's configuration (49 of 106) and Sun's (72 of 73)
  give Checkstyle's findings exactly: 1,046,328 of 1,046,328 and 697,951 of 697,951.
- Probe files are stored byte for byte (`.gitattributes`), so CRLF and lone-CR probes survive a clone.
- A `checkstyle.xml` reader that names every module or property it cannot honour.
- Module registry and a suppression stage (no filters implemented yet).

Roadmap to 1.0, defined as "Google's and Sun's bundled configs run fully with exact output":
the rest of the whitespace family, suppression filters, Javadoc, then `Indentation`. See FINDINGS.md.

CI: the workflow is kept at `ci/github-workflow.yml`, not under `.github/workflows/`, because the
token used to push lacked the `workflow` scope. Move it to `.github/workflows/ci.yml` to enable it.
