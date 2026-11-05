//! javaid: a fast, Checkstyle-compatible Java linter.
//!
//! Files are parsed with tree-sitter and walked once; rules are dispatched by node kind. The
//! library exists so that integration tests (and, later, editor integrations) can drive the same
//! engine as the command-line binary.

pub mod columns;
pub mod config;
pub mod csast;
pub mod engine;
pub mod fileset;
pub mod javadoc;
pub mod messages;
mod messages_table;
pub mod rules;
pub mod suppress;
pub mod syntax;
pub mod tokens;
pub mod xpath;
