//! The stage between the rules and the output: filters that remove findings.
//!
//! Checkstyle's filters (`SuppressWarningsFilter`, `SuppressionFilter`, the nearby-comment and
//! `// CHECKSTYLE:OFF` filters, the XPath ones) do not find anything; they decide which findings
//! stand. Google's config has 15 such instances. They get their own stage because they need things
//! rules do not: the whole file's findings at once, and the comments and annotations around them.
//!
//! No filter is implemented yet, so `apply` keeps everything. The point of the stage existing is
//! that each filter can now be added, probed against Checkstyle and tested on its own, without
//! touching the rules.

use crate::config::Config;
use crate::engine::Diagnostic;

/// Remove the findings the configuration's filters suppress.
pub fn apply(cfg: &Config, _src: &[u8], findings: &mut Vec<Diagnostic>) {
    if cfg.filters.is_empty() {
        return;
    }
    // Filters are applied in order; none exists yet, see the module comment.
    let _ = findings;
}
