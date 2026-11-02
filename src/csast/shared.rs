//! One emulated AST per file and variant, shared by every module that reads it.
//!
//! Building the Checkstyle AST costs about as much as parsing the file, and half a dozen modules
//! (`Indentation`, `VariableDeclarationUsageDistance`, `CommentsIndentation`, the Google
//! `RightCurly` / `EmptyLineSeparator` / text-block ports, the XPath filters and `MatchXpath`)
//! want one. A [`CsCache`] builds each variant (with the comment nodes, or without) the first
//! time a module asks and hands the same tree to everyone after that, so a file whose
//! configuration needs none of them never pays, and one that needs several pays once.
//!
//! The builder recurses with the nesting of the code, so it runs on a thread with a big stack:
//! the worker threads of the CLI are started with one (see [`mark_big_stack_thread`]); any other
//! thread borrows a helper thread that is spawned once and reused.

use std::cell::{Cell, OnceCell, RefCell};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::mpsc;
use tree_sitter::Tree;

use super::ast::CsAst;

/// The stack the builder's recursion gets. Only the pages a file touches are ever committed.
pub const BIG_STACK: usize = 1 << 30;

thread_local! {
    static BIG: Cell<bool> = const { Cell::new(false) };
    static HELPER: RefCell<Option<mpsc::Sender<Job>>> = const { RefCell::new(None) };
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Declares that the current thread was started with [`BIG_STACK`], so [`with_big_stack`] runs
/// work on it directly.
pub fn mark_big_stack_thread() {
    BIG.with(|b| b.set(true));
}

fn spawn_helper() -> Option<mpsc::Sender<Job>> {
    let (tx, rx) = mpsc::channel::<Job>();
    std::thread::Builder::new()
        .stack_size(BIG_STACK)
        .spawn(move || {
            BIG.with(|b| b.set(true));
            // Ends when the owning thread exits and drops its sender.
            while let Ok(job) = rx.recv() {
                job();
            }
        })
        .ok()?;
    Some(tx)
}

/// Hands `job` to this thread's helper, spawning it if need be; gives the job back if there is none.
fn send_to_helper(mut job: Job) -> Result<(), Job> {
    HELPER.with(|h| {
        let mut h = h.borrow_mut();
        for _ in 0..2 {
            if h.is_none() {
                *h = spawn_helper();
            }
            let Some(tx) = h.as_ref() else { break };
            match tx.send(job) {
                Ok(()) => return Ok(()),
                Err(mpsc::SendError(back)) => {
                    job = back;
                    *h = None;
                }
            }
        }
        Err(job)
    })
}

/// Runs `f` on a thread with a big stack and returns its result (`Err` if it panicked).
///
/// On a thread that already has one this is a plain call. Elsewhere the work goes to a helper
/// thread owned by the calling thread, created on first use and kept for the next call.
pub fn with_big_stack<R: Send>(f: impl FnOnce() -> R + Send) -> std::thread::Result<R> {
    if BIG.with(|b| b.get()) {
        return catch_unwind(AssertUnwindSafe(f));
    }
    let (rtx, rrx) = mpsc::channel();
    let job: Box<dyn FnOnce() + Send + '_> = Box::new(move || {
        let _ = rtx.send(catch_unwind(AssertUnwindSafe(f)));
    });
    // SAFETY: the job borrows from the caller's frame, and this function does not return before
    // the job is finished with: it blocks on `rrx` until the job has sent its result, or until the
    // job was dropped unrun (the sender is gone then, and so is every borrow).
    let job: Job = unsafe { std::mem::transmute::<Box<dyn FnOnce() + Send + '_>, Job>(job) };
    if let Err(job) = send_to_helper(job) {
        // No helper thread could be started: do it here, on whatever stack this thread has.
        job();
    }
    rrx.recv().unwrap_or_else(|_| Err(Box::new("the helper thread is gone")))
}

/// A file's parse and the emulated ASTs built from it, on demand.
pub struct CsCache<'a> {
    tree: Option<&'a Tree>,
    src: &'a [u8],
    slots: [OnceCell<Option<CsAst>>; 2],
    /// Found once for the parse, whichever variant is built first (see `build::misparsed_cast_roots`).
    roots: OnceCell<std::collections::HashSet<usize>>,
}

impl<'a> CsCache<'a> {
    /// `tree` is the parse of `src` that the rest of the linter walks.
    pub fn new(tree: &'a Tree, src: &'a [u8]) -> Self {
        CsCache { tree: Some(tree), src, slots: Default::default(), roots: OnceCell::new() }
    }

    /// For a caller with no parse at hand: the first request parses `src` itself.
    pub fn detached(src: &'a [u8]) -> Self {
        CsCache { tree: None, src, slots: Default::default(), roots: OnceCell::new() }
    }

    pub fn src(&self) -> &'a [u8] {
        self.src
    }

    pub fn tree(&self) -> Option<&'a Tree> {
        self.tree
    }

    /// The tree of the file with (`comments`) or without the comment nodes, or `None` for a file
    /// the builder declines (syntax errors, or a construct it cannot handle).
    pub fn get(&self, comments: bool) -> Option<&CsAst> {
        self.slots[comments as usize].get_or_init(|| self.build(comments)).as_ref()
    }

    fn build(&self, comments: bool) -> Option<CsAst> {
        let (tree, src) = (self.tree, self.src);
        let reusable = tree.filter(|t| is_orchard(t) && !has_lone_cr(src));
        let roots = reusable.map(|t| self.roots.get_or_init(|| if t.root_node().has_error() { Default::default() } else { super::build::misparsed_cast_roots(t) }));
        with_big_stack(move || {
            // The builder reads the fork grammar's node names (`@interface`, ...) and ends a line
            // at a lone `\r`, as ANTLR does. The walk's tree is reusable when it came from that
            // grammar and the buffer has no lone `\r` left (the CLI replaces them before parsing).
            match reusable.zip(roots) {
                Some((t, roots)) => super::build::build_checked_with(t, src, comments, roots).ok(),
                None => super::build::parse_and_build(src, crate::syntax::Grammar::Orchard, comments).ok(),
            }
        })
        .ok()
        .flatten()
    }
}

/// Is `tree` from the grammar the builder was written for?
pub fn is_orchard(tree: &Tree) -> bool {
    let orchard: tree_sitter::Language = tree_sitter_java_orchard::LANGUAGE.into();
    *tree.language() == orchard
}

fn has_lone_cr(src: &[u8]) -> bool {
    memchr::memchr_iter(b'\r', src).any(|i| src.get(i + 1) != Some(&b'\n'))
}
