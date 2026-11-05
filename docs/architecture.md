# How javaid works

- **Parsing.** Every `.java` file is parsed with tree-sitter (the maintained `tree-sitter-java-orchard`
  grammar by default), one reused parser per thread. Files are processed in parallel (`-j N`).
- **One walk.** All rules share a single walk of each syntax tree, dispatched through a node-kind
  lookup table, not one walk per rule. Rules that decide at the end of a file (unused imports, for
  example) finish after the walk.
- **Checkstyle-shaped tree.** Modules that depend on the exact shape of Checkstyle's own syntax tree
  (Indentation, VariableDeclarationUsageDistance, CommentsIndentation, EmptyLineSeparator, the Javadoc
  tree checks, XPath) run on an emulation of it, `src/csast/`, built once per file and shared between
  those modules.
- **Text modules.** Line length, tabs, regular expressions and the like read the file's text and need
  no syntax tree. They also read `.properties` and `.xml` files when a Checker-level `fileExtensions`
  adds them, as Checkstyle does.
- **Findings.** Each finding carries Checkstyle's column convention (tabs expanded, columns counted as
  Checkstyle counts them, all measured in `src/columns.rs`) and its message wording, from
  `src/messages.rs`. Suppression filters
  (`SuppressWarnings`, comment filters, XPath filters, `SuppressionFilter`) run after the rules, over
  the collected findings.
- **Configuration.** `src/config/` reads `checkstyle.xml`, maps each module and property to a rule
  or refuses it by name. The registry in `config/mod.rs` lists every module with the function that
  reads its properties; those functions and the options types they build live in one file per
  family (`config/naming.rs`, `config/whitespace.rs`, ...). Configuration sits below the rules: the
  rules read it, and it does not depend on them.
- **Per-file inputs.** Everything a file's findings depend on reaches the rules through
  `engine::lint_file` (the tree, the source, the path, the configuration), with no setup call
  beforehand, so the library can be driven file by file. The thread-locals that remain hold no
  per-file state: a cache of compiled suppression patterns, and the big-stack helper thread.
- **Failures.** The emulated-AST builder runs under `catch_unwind`, and release builds unwind
  (`panic = "unwind"`), so a tree shape it does not handle costs that file its Checkstyle-shaped
  modules, with a message on stderr, rather than ending the run.
