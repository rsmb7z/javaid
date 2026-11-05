# Instructions for coding agents

Read [CONTRIBUTING.md](CONTRIBUTING.md) first: the oracle rule, how to test, how to record
Checkstyle's behaviour and how to add a module all apply to you. This file adds what an agent
tends to get wrong.

## Hard rules

- **Do not guess Checkstyle's behaviour.** If a finding's line, column or wording is not already
  pinned by a probe golden, say so and ask for it to be recorded with Checkstyle (or record it, if
  you can run Checkstyle). An answer that looks more correct than Checkstyle's is wrong.
- **Never edit golden files (`.expected`, `.expected2`, `bench/csast/golden*/`), or remove or
  exempt a case in `tests/oracle_probes.rs`, to make a test pass.** A failing probe means javaid
  disagrees with Checkstyle. An `UPSTREAM_MISPARSES` entry is only for a real grammar limitation,
  with its reason.
- **Never accept a configuration property and ignore it.** Refuse it by name (see `Props::finish`
  and the `expect_default` calls in `src/config/`).
- **Keep the provenance of translated code.** Files translated from Checkstyle's source carry its
  copyright notice and name their source; never remove that header or the "ported from" comments.
  If you translate more, follow CONTRIBUTING.md's License section.
- **Do not commit local material:** corpora, Checkstyle jars and benchmark scratch go in the
  ignored `.scratchpad/`; no absolute paths, user names or credentials in files or messages.

## Working notes

- Run `cargo fmt` before committing; CI runs `cargo fmt --check` with the settings in
  `rustfmt.toml`.
- A full `cargo test` takes two to three minutes, about 85 seconds of it the probe replay. Run it
  in the background and wait for it rather than cutting it short; a killed run proves nothing.
- A refactor should change no output. Check that directly: run the release binary on the same
  inputs before and after and compare the output byte for byte, not only the test counts.
- Both grammars matter: `src/rules/tests.rs` runs every case under the upstream and the fork
  grammar and requires the same result. The emulated Checkstyle tree (`src/csast/`) is built from
  the fork grammar only, and the modules run from `src/rules/cs.rs` report nothing under the
  upstream grammar, so their unit tests cannot go through the two-grammar helpers.
- Where things are: `src/engine.rs` (the per-file pipeline and the single walk),
  `src/rules/mod.rs` (dispatch from node kind to rule hooks), `src/config/` (registry, options,
  the `checkstyle.xml` reader), `src/csast/` (the Checkstyle AST emulation), `src/suppress.rs`,
  `src/rules/commentfilter.rs` and `src/rules/xpathfilter.rs` (filters), `src/columns.rs`
  (column arithmetic), `docs/architecture.md` (the overview).
