# Changelog

## Unreleased (v0.1 preview, not published)

- 33 Checkstyle modules, each accepted only at zero difference from Checkstyle 14.4.0 on probe
  files, two large corpora and mutated copies of them.
- A `checkstyle.xml` reader that names every module or property it cannot honour.
- Module registry and a suppression stage (no filters implemented yet).

Roadmap to 1.0, defined as "Google's and Sun's bundled configs run fully with exact output":
whitespace and wrapping, suppression filters, Javadoc, then `Indentation`. See FINDINGS.md.

CI: the workflow is kept at `ci/github-workflow.yml`, not under `.github/workflows/`, because the
token used to push lacked the `workflow` scope. Move it to `.github/workflows/ci.yml` to enable it.
