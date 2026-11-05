# Contributing to javaid

javaid has one goal: for every module it implements, print exactly what Checkstyle 14.4.0 prints,
line, column and message, faster. The known exceptions are listed in the README's
[Limits](README.md#limits) and in [docs/compatibility.md](docs/compatibility.md). Everything below
follows from that goal.

## The rule: Checkstyle is the oracle

A finding is right when Checkstyle reports it, at the same line and column, with the same message.
It is not right because the Java language, the module's documentation or common sense says so.
Checkstyle has quirks, and javaid reproduces them; the source comments name the probe or
Checkstyle class each one comes from. When you are not sure what Checkstyle does, run it on a probe
file and record the answer (see below). Do not guess.

**Never skip silently.** A module javaid does not implement, or a property value it cannot honour,
is refused by name when the configuration loads, and that module does not run. Do not accept a
property and ignore it.

## Build and test

```bash
cargo fmt                    # settings in rustfmt.toml
cargo test --locked
cargo build --release --locked
```

CI runs `cargo fmt --check`, `cargo test --locked` and the release build; a pull request is ready
when all three pass. A full `cargo test` takes two to three minutes including compilation; most of
the run is `tests/oracle_probes.rs`, which replays 1,242 probe cases against Checkstyle's recorded
output (and again under the upstream grammar) in about 85 seconds in a debug build. The unit tests
in `src/rules/tests.rs` run every case under both tree-sitter grammars and require the same result.

A change to a rule should also leave javaid's output on real code unchanged except where intended.
Check that directly: run the release binary before and after the change on the same large codebase
and compare the two outputs byte for byte. To compare against Checkstyle itself (needs the
Checkstyle jar and the corpora), `bench/oracle/make-reference.sh` produces Checkstyle's output and
`python3 bench/parity.py --strict reference.out javaid.out` compares it module by module; see
[bench/README.md](bench/README.md).

## Recording what Checkstyle does

Needs Checkstyle 14.4.0's `-all.jar` and a JDK 21 with `java` on the `PATH`. Keep the jar (and any
corpora) under `.scratchpad/`, which git ignores.

1. Write a probe file in `bench/oracle/probes/` with one distinctively named case per construct, and
   a configuration in `bench/oracle/<name>.xml`. The configuration may only use modules and
   properties javaid models: a module javaid would skip fails the test.
2. Record Checkstyle's answer:
   `python3 bench/oracle/record-golden.py <checkstyle-all.jar> bench/oracle/<name>.xml bench/oracle/probes/<Probe>.java bench/oracle/probes/<Probe>.<name>.expected`
   It writes `.expected` and `.expected2` and refuses a run in which Checkstyle did not finish.
3. Add `("<Probe>.java", "<name>")` to `CASES` in `tests/oracle_probes.rs`, watch it fail, then
   make javaid pass it. The same cases also run under the upstream grammar; a probe that grammar
   cannot parse, or one for a module run from `src/rules/cs.rs`, goes in `UPSTREAM_MISPARSES` with
   its reason.

Golden files are Checkstyle's output: generate them with the recorder, never edit them by hand.
Existing goldens are re-recorded only when moving to a new Checkstyle version
(`bench/oracle/rerecord-all-goldens.sh <checkstyle-all.jar>`), and the diff is reviewed. The same
holds for the emulated Checkstyle AST: `tests/csast_golden.rs` compares it with Checkstyle's own
dumps in `bench/csast/golden*/`, which `bench/csast/regen-golden.sh` regenerates.

## Adding a module

1. **Registry and options.** Add a row to `REGISTRY` in `src/config/mod.rs`: the module's `name`,
   its `family` label, `multi` (may a configuration list it more than once), `own_name` (does each
   instance print its own `id`) and `apply`, the function that reads its properties. That function
   goes in the family's file (`src/config/coding.rs`, `naming.rs`, `whitespace.rs`, ...), with a
   field on `Config`; the options type usually sits beside it (the small structural modules keep
   theirs in `src/config/structural.rs`; follow the neighbouring modules). The reader takes each
   property it understands from `Props`, uses `expect_default` for one it supports only at its
   default, and ends with `p.finish()?`, which refuses whatever is left. Without that call an
   unknown property is silently dropped.
2. **The rule.** Most modules are hooks on the single tree walk: `rules::visit` in
   `src/rules/mod.rs` dispatches on node kind through the table in `src/syntax.rs`. Modules that
   depend on the exact shape of Checkstyle's syntax tree are ported onto the emulated tree in
   `src/csast/` instead and run after the walk (`src/rules/csast_checks.rs`, `src/rules/cs.rs`).
   The modules run from `src/rules/cs.rs` report nothing under the upstream grammar (their probes
   go in `UPSTREAM_MISPARSES` with the reason `"CsAst-based module"`). Line-based modules go in
   `src/rules/text.rs`.
3. **Columns.** Report a node through `Ctx::diag` (or a byte position through `Ctx::diag_at`),
   which converts to the column Checkstyle prints. When you measure a column yourself, use
   `src/columns.rs` (Checkstyle counts columns in more than one way, and the functions there are
   named for which) and report it with `Ctx::diag_exact`, so that it is not converted twice.
4. **Probes** as above. Update `docs/modules.md` (the output of `javaid --list-rules`) and the
   module count in `README.md`, `docs/modules.md` and `docs/compatibility.md`.

## Style

- Format with `cargo fmt`; CI checks it.
- Comments say why, and cite the probe or Checkstyle class a behaviour comes from.
- Commit messages: a type prefix (`feat:`, `fix:`, `perf:`, `refactor:`, `docs:`, `test:`,
  `bench:`, `chore:`) and a subject in the imperative; the body explains the reason and how the
  change was checked.

## Releasing

1. Set the new `version` in `Cargo.toml`, and move the changelog's notes under that version.
2. Commit, then tag the commit `v<version>` (for example `v1.0.1`) and push the tag.
3. `.github/workflows/release.yml` waits for a maintainer to approve the run (Actions tab, the
   `release` environment). Then it checks that the tag matches `Cargo.toml` and that its commit is
   on `main`, runs the checks CI runs, and publishes to crates.io through trusted publishing. A
   published version cannot be replaced, only yanked, so reject the run if the tag was a mistake.

## License

javaid is licensed under the GNU LGPL, version 2.1 or later (see the [README](README.md#license)),
and by contributing you agree that your contribution is licensed under the same terms.

If your change translates or copies anything from Checkstyle's source, keep Checkstyle's copyright
notice in the file, name the source class in its header (the existing translated files show the
form), and list the file in [NOTICE](NOTICE). Code written from Checkstyle's observable behaviour,
recorded on probes, needs none of this.
