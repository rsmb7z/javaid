//! An emulation of Checkstyle 14.4.0's AST (`DetailAST`) built on top of the tree-sitter parse.
//!
//! Checkstyle's own parser is ANTLR; its `JavaAstVisitor` turns the parse tree into the `DetailAST`
//! that every Checkstyle check and the XPath/suppression machinery work on. This module rebuilds
//! those trees (same node types, text, positions and sibling/child structure) from javaid's
//! tree-sitter parse, and prints them exactly as `checkstyle -t` does.

pub mod ast;
pub mod build;
pub mod print;
pub mod shared;
pub mod tok;

pub use shared::{CsCache, mark_big_stack_thread, with_big_stack};

pub use ast::{CsAst, N, NIL};
pub use build::{build, build_checked, build_source, build_with_comments, try_build_source};
pub use print::{dump, node_line};
pub use tok::Tok;
