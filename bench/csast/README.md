# csast: Checkstyle's AST on top of tree-sitter

`src/csast/` rebuilds the `DetailAST` that Checkstyle 14.4.0 (`JavaAstVisitor`) produces, from javaid's
tree-sitter parse, and prints it like `java -jar checkstyle-14.4.0-all.jar -t FILE` (`-T` adds comment
nodes). Everything here checks that against Checkstyle itself.

## Commands

```
# print one file's AST (hidden flags)
javaid --dump-ast File.java
javaid --dump-ast-comments File.java          # the text of `-T`

# compare with Checkstyle over a list of files (one path per line)
export JAR=/path/to/checkstyle-14.4.0-all.jar JAVA_HOME=/path/to/jdk
cargo build --release
bench/csast/compare.sh -n 10 LIST             # -T for comment nodes, -k keeps the work dir
ORACLE_CACHE=/tmp/cs-cache bench/csast/compare.sh LIST   # reuse Checkstyle's dumps between runs
```

`compare.sh` starts Checkstyle's JVM once (`BatchDump.java`, which prints the dump of every listed file),
runs `javaid --dump-ast-list`, and reports how many dumps are byte-identical and the first differing lines
of the first failures.

## Tests

`tests/csast_golden.rs` (part of `cargo test`) checks `golden/` (the `-t` dumps) and `golden-comments/`
(the `-T` dumps), which Checkstyle printed for

* `snippets/`: one file per language area (records, enums, interfaces, annotations including type
  annotations, expressions and ambiguities, statements and switch/patterns, module declarations, compact
  source files, comments, Unicode, tabs, CRLF, lone CR, form feed, empty files);
* `snippets-layout/`: the same sources with the token stream unchanged and the layout rewritten
  (everything on one line; one token per line with random tab/space indentation; random spacing and
  CRLF line breaks), which exercises the position rules.

## Mutated variants

`tools/mutate.py OUTDIR LIST COUNT SEED` re-lays-out COUNT random files of LIST three ways (token stream
unchanged), and `tools/parenmut` (`cargo build --release --offline` inside it; it is not part of the
javaid crate) wraps a random PERCENT of the expressions in redundant parentheses:
`parenmut OUTDIR LIST COUNT SEED PERCENT`. Both write `OUTDIR/list`, ready for `compare.sh`; the two
compose (parenthesize, then re-lay-out). All variants tried so far are identical to Checkstyle's dumps
(see `KNOWN-DIFFERENCES.md`).

`regen-golden.sh` regenerates the golden files from Checkstyle (needs `JAR`). The snippets are
sources only Checkstyle's grammar has to accept, not compilable programs.

See `KNOWN-DIFFERENCES.md` for what does not match.
