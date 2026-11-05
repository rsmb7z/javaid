use clap::Parser as ClapParser;
use javaid::config::{Config, module_names};
use javaid::engine::{self, Diagnostic};
use javaid::suppress;
use javaid::syntax::{self, Grammar, Syntax};
use rayon::prelude::*;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

#[derive(ClapParser, Debug)]
#[command(name = "javaid", version, about = "A fast, Checkstyle-compatible Java linter")]
struct Cli {
    /// Files or directories to check
    #[arg(required_unless_present = "list_rules")]
    paths: Vec<PathBuf>,

    /// A Checkstyle configuration (checkstyle.xml). Without it, every implemented module runs at
    /// Checkstyle's default settings. Modules or properties javaid cannot honour are listed on
    /// stderr and do not run.
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Run only these modules (names as in Checkstyle, comma-separated)
    #[arg(long, value_delimiter = ',')]
    rules: Vec<String>,

    /// List the implemented modules and exit
    #[arg(long)]
    list_rules: bool,

    /// Worker threads (default: all logical CPUs)
    #[arg(short = 'j', long)]
    threads: Option<usize>,

    /// Do not follow symlinked directories (Checkstyle does; PMD does not)
    #[arg(long)]
    no_follow_symlinks: bool,

    /// Only parse: no traversal, no rules (the parsing floor)
    #[arg(long)]
    parse_only: bool,

    /// Parse and visit every node but run no rules (traversal overhead)
    #[arg(long)]
    walk_only: bool,

    /// Do not print diagnostics
    #[arg(short, long)]
    quiet: bool,

    /// Print timings and counts to stderr
    #[arg(long)]
    stats: bool,

    /// List files tree-sitter could not parse cleanly (to stderr)
    #[arg(long)]
    list_syntax_errors: bool,

    /// Java grammar to parse with
    #[arg(long, value_enum, default_value_t)]
    grammar: Grammar,

    /// Debug aid: print each file's syntax tree as an S-expression and exit
    #[arg(long, hide = true)]
    dump_tree: bool,

    /// Debug aid: print each file's Checkstyle-style AST (the text of `checkstyle -t`) and exit
    #[arg(long, hide = true)]
    dump_ast: bool,

    /// Debug aid: as `--dump-ast`, with comment nodes (the text of `checkstyle -T`)
    #[arg(long, hide = true)]
    dump_ast_comments: bool,

    /// Debug aid: read file names from this list and write each AST dump to `--dump-ast-out`
    #[arg(long, hide = true, value_name = "LIST")]
    dump_ast_list: Option<std::path::PathBuf>,

    /// Debug aid: as `--dump-ast-list`, with comment nodes
    #[arg(long, hide = true, value_name = "LIST")]
    dump_ast_comments_list: Option<std::path::PathBuf>,

    /// Debug aid: evaluate this XPath query over each file's Checkstyle-style AST (with comment
    /// nodes) and print the selected nodes, one `TYPE -> text [line:col]` per line
    #[arg(long, hide = true, value_name = "QUERY")]
    xpath: Option<String>,

    /// Debug aid: output directory for `--dump-ast-list` (`NNNNNN.txt`, or `.err` when the AST cannot be built)
    #[arg(long, hide = true, value_name = "DIR")]
    dump_ast_out: Option<std::path::PathBuf>,
}

#[derive(Clone, Copy)]
enum Mode<'a> {
    ParseOnly,
    WalkOnly,
    Lint(&'a Config),
}

#[derive(Default)]
struct FileResult {
    bytes: usize,
    nodes: usize,
    syntax_error: bool,
    /// Where tree-sitter first gave up, only filled in when explaining errors.
    syntax_error_at: Option<String>,
    io_error: Option<String>,
    diagnostics: Vec<Diagnostic>,
}

/// `exts` are the file extensions some enabled module reads: `java`, plus whatever a file-level
/// module's `fileExtensions` adds (see `Config::extensions`).
///
/// Returns the files sorted, and in the order the walk met them (which the file-set modules need).
fn discover(paths: &[PathBuf], follow_symlinks: bool, exts: &[String]) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    for p in paths {
        if p.is_file() {
            files.push(p.clone());
            continue;
        }
        // standard_filters(false): do not honour .gitignore, so the file set matches what
        // Checkstyle sees when it is pointed at the same directory. Checkstyle also follows
        // symlinked directories (and reports a file once per path), so that is the default here.
        // A symlink loop yields an error entry, which `flatten` drops.
        let walker = ignore::WalkBuilder::new(p).standard_filters(false).follow_links(follow_symlinks).build();
        for entry in walker.flatten() {
            let is_file = entry.file_type().is_some_and(|t| t.is_file());
            if is_file && entry.path().extension().is_some_and(|e| exts.iter().any(|x| e == x.as_str())) {
                files.push(entry.into_path());
            }
        }
    }
    let walk_order = files.clone();
    files.sort();
    files.dedup();
    (files, walk_order)
}

fn check_file(parser: &mut tree_sitter::Parser, syn: &Syntax, mode: Mode, explain: bool, path: &Path) -> FileResult {
    let mut src = match std::fs::read(path) {
        Ok(s) => s,
        Err(e) => return FileResult { io_error: Some(e.to_string()), ..Default::default() },
    };
    if let Mode::Lint(cfg) = mode {
        if suppress::file_excluded(cfg, path) {
            return FileResult { bytes: src.len(), ..Default::default() };
        }
    }
    let mut result = FileResult { bytes: src.len(), ..Default::default() };
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "java" {
        // A file a Checker-level `fileExtensions` pulled in: only the text modules read it.
        if let Mode::Lint(cfg) = mode {
            engine::lint_text(cfg, ext, &src, &mut result.diagnostics);
            result.diagnostics.sort_by_key(|d| (d.line, d.col));
            suppress::by_file(cfg, path, &src, &mut result.diagnostics);
        }
        return result;
    }
    engine::normalize_line_breaks(&mut src);
    let Some(tree) = parser.parse(&src, None) else {
        result.syntax_error = true;
        return result;
    };
    result.syntax_error = tree.root_node().has_error();
    if explain && result.syntax_error {
        if let Some(n) = syntax::first_syntax_error(&tree) {
            let p = n.start_position();
            let line = src.split(|&b| b == b'\n').nth(p.row).unwrap_or_default();
            let line = String::from_utf8_lossy(line);
            let kind = if n.is_missing() { "missing" } else { "unexpected" };
            result.syntax_error_at = Some(format!("{}:{} {kind}: {}", p.row + 1, p.column + 1, line.trim()));
        }
    }
    match mode {
        Mode::ParseOnly => {}
        Mode::WalkOnly => result.nodes = syntax::walk_only(&tree),
        Mode::Lint(cfg) => {
            let cache = javaid::csast::CsCache::new(&tree, &src);
            engine::lint_file_cached(syn, cfg, &cache, Some(path), &mut result.diagnostics);
            // Checkstyle reports in line order; the walk finds things in document order and the
            // line-based modules afterwards, so put them in order. Stable, so ties keep their order.
            result.diagnostics.sort_by_key(|d| (d.line, d.col));
            suppress::by_file_cached(cfg, path, &cache, &mut result.diagnostics);
        }
    }
    result
}

/// Canonical Checkstyle spelling of the requested module names, or the first unknown one.
fn resolve_modules(requested: &[String]) -> Result<Vec<String>, String> {
    requested.iter().map(|r| module_names().find(|m| m.eq_ignore_ascii_case(r)).map(|m| m.to_string()).ok_or_else(|| r.clone())).collect()
}

fn load_config(cli: &Cli) -> Result<Config, String> {
    let only = if cli.rules.is_empty() { None } else { Some(resolve_modules(&cli.rules).map_err(|bad| format!("unknown module '{bad}' (see --list-rules)"))?) };
    let Some(path) = &cli.config else {
        return Ok(Config::checkstyle_defaults(only.as_deref()));
    };
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let loaded = Config::from_checkstyle_xml(&text, only.as_deref()).map_err(|e| format!("{}: {e}", path.display()))?;
    eprintln!("javaid: {}: running {} module(s), skipped {}", path.display(), loaded.enabled.len(), loaded.skipped.len());
    for (name, why) in &loaded.skipped {
        eprintln!("  skipped {name}: {why}");
    }
    for note in &loaded.notes {
        eprintln!("  note {note}");
    }
    Ok(loaded.config)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if cli.list_rules {
        let mut out = std::io::stdout().lock();
        for m in module_names() {
            if writeln!(out, "{m}").is_err() {
                break; // a closed pipe (`| head`) is not an error
            }
        }
        return ExitCode::SUCCESS;
    }

    if let (Some(list), Some(out)) = (&cli.dump_ast_list, &cli.dump_ast_out) {
        return dump_ast_batch(list, out, cli.grammar, false);
    }
    if let (Some(list), Some(out)) = (&cli.dump_ast_comments_list, &cli.dump_ast_out) {
        return dump_ast_batch(list, out, cli.grammar, true);
    }

    if let Some(query) = &cli.xpath {
        let compiled = match javaid::xpath::XPath::compile(query) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("javaid: cannot compile the XPath query: {e}");
                return ExitCode::from(2);
            }
        };
        for path in discover(&cli.paths, !cli.no_follow_symlinks, &["java".to_string()]).0 {
            let src = std::fs::read(&path).expect("readable file");
            let ast = match javaid::csast::try_build_source(&src, cli.grammar, true) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("javaid: {}: {e}", path.display());
                    return ExitCode::from(2);
                }
            };
            let tree = javaid::xpath::XTree::new(&ast);
            match compiled.select(&tree) {
                Ok(hits) => {
                    for h in hits {
                        println!("{}", javaid::csast::node_line(&ast, h.node));
                    }
                }
                Err(e) => {
                    eprintln!("javaid: {e}");
                    return ExitCode::from(2);
                }
            }
        }
        return ExitCode::SUCCESS;
    }

    if cli.dump_ast || cli.dump_ast_comments {
        for path in discover(&cli.paths, !cli.no_follow_symlinks, &["java".to_string()]).0 {
            let src = std::fs::read(&path).expect("readable file");
            match javaid::csast::try_build_source(&src, cli.grammar, cli.dump_ast_comments) {
                Ok(ast) => print!("{}", javaid::csast::dump(&ast)),
                Err(e) => {
                    eprintln!("javaid: {}: {e}", path.display());
                    return ExitCode::from(2);
                }
            }
        }
        return ExitCode::SUCCESS;
    }

    if cli.dump_tree {
        let syn = Syntax::new(cli.grammar);
        let mut parser = syn.new_parser();
        for path in discover(&cli.paths, !cli.no_follow_symlinks, &["java".to_string()]).0 {
            let src = std::fs::read(&path).expect("readable file");
            let tree = parser.parse(&src, None).expect("parse");
            println!("; {}\n{}\n", path.display(), tree.root_node().to_sexp());
        }
        return ExitCode::SUCCESS;
    }

    let cfg = match load_config(&cli) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("javaid: {e}");
            return ExitCode::from(2);
        }
    };
    let mode = if cli.parse_only {
        Mode::ParseOnly
    } else if cli.walk_only {
        Mode::WalkOnly
    } else {
        Mode::Lint(&cfg)
    };

    // The workers get the big stack the emulated-AST builder recurses on (only the pages it touches
    // are ever committed), so it runs in place rather than on a thread of its own per build.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cli.threads.unwrap_or(0))
        .stack_size(javaid::csast::shared::BIG_STACK)
        .start_handler(|_| javaid::csast::mark_big_stack_thread());
    pool.build_global().expect("thread pool");

    let t0 = Instant::now();
    let (files, walk_order) = discover(&cli.paths, !cli.no_follow_symlinks, &cfg.extensions());
    let t_walk = t0.elapsed();

    let syn = Syntax::new(cli.grammar);
    let t1 = Instant::now();
    let mut results: Vec<FileResult> =
        files.par_iter().map_init(|| syn.new_parser(), |parser, path| check_file(parser, &syn, mode, cli.list_syntax_errors, path)).collect();
    // The modules that look at the set of files, not at one.
    let mut at_end = Vec::new();
    if let Mode::Lint(cfg) = mode {
        let mut sets = javaid::fileset::analyse(cfg, &walk_order);
        for (path, r) in files.iter().zip(results.iter_mut()) {
            if let Some(found) = sets.on_file.remove(path) {
                r.diagnostics.extend(found);
                r.diagnostics.sort_by_key(|d| (d.line, d.col));
            }
        }
        at_end = std::mem::take(&mut sets.at_end);
    }
    let t_lint = t1.elapsed();

    let mut violations = 0usize;
    let out = std::io::stdout();
    let mut out = BufWriter::with_capacity(1 << 16, out.lock());
    for (path, r) in files.iter().zip(&results) {
        if let Some(e) = &r.io_error {
            eprintln!("javaid: {}: {e}", path.display());
        }
        violations += r.diagnostics.len();
        if cli.list_syntax_errors && r.syntax_error {
            eprintln!("syntax error: {}:{}", path.display(), r.syntax_error_at.as_deref().unwrap_or("?"));
        }
        if cli.quiet {
            continue;
        }
        for d in &r.diagnostics {
            if d.col == 0 {
                // Checkstyle prints no column for a finding that has none.
                let _ = writeln!(out, "{}:{}: {}: {}", path.display(), d.line, d.rule, d.message);
            } else {
                let _ = writeln!(out, "{}:{}:{}: {}: {}", path.display(), d.line, d.col, d.rule, d.message);
            }
        }
    }
    for (path, d) in &at_end {
        violations += 1;
        if !cli.quiet {
            let _ = writeln!(out, "{}:{}: {}: {}", path.display(), d.line, d.rule, d.message);
        }
    }
    let _ = out.flush();

    if cli.stats {
        let bytes: usize = results.iter().map(|r| r.bytes).sum();
        let nodes: usize = results.iter().map(|r| r.nodes).sum();
        let syntax_errors = results.iter().filter(|r| r.syntax_error).count();
        eprintln!(
            "javaid: files={} bytes={} nodes={} violations={} syntax_error_files={} threads={} discover={:.1}ms parse+lint={:.1}ms",
            files.len(),
            bytes,
            nodes,
            violations,
            syntax_errors,
            rayon::current_num_threads(),
            t_walk.as_secs_f64() * 1e3,
            t_lint.as_secs_f64() * 1e3,
        );
    }

    if violations > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

/// Dumps the Checkstyle-style AST of every file named in `list` into `out`, in parallel.
fn dump_ast_batch(list: &Path, out: &Path, grammar: Grammar, comments: bool) -> ExitCode {
    let names = std::fs::read_to_string(list).expect("readable list");
    std::fs::create_dir_all(out).expect("output directory");
    let files: Vec<&str> = names.lines().filter(|l| !l.is_empty()).collect();
    files.par_iter().enumerate().for_each(|(i, f)| {
        let src = match std::fs::read(f) {
            Ok(s) => s,
            Err(e) => {
                let _ = std::fs::write(out.join(format!("{i:06}.err")), e.to_string());
                return;
            }
        };
        match javaid::csast::try_build_source(&src, grammar, comments) {
            Ok(ast) => {
                let _ = std::fs::write(out.join(format!("{i:06}.txt")), javaid::csast::dump(&ast));
            }
            Err(e) => {
                let _ = std::fs::write(out.join(format!("{i:06}.err")), e);
            }
        }
    });
    ExitCode::SUCCESS
}
