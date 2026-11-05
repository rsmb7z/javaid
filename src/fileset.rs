//! The modules that look at the set of files rather than one file: `JavadocPackage` and
//! `Translation`. They run once over the list of files javaid is about to read.

use crate::config::Config;
use crate::engine::Diagnostic;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Findings that belong to a file in the run (`JavadocPackage`), and those that Checkstyle reports at
/// the end of the run, on a file or a directory (`Translation`).
#[derive(Default)]
pub struct Findings {
    pub on_file: HashMap<PathBuf, Vec<Diagnostic>>,
    pub at_end: Vec<(PathBuf, Diagnostic)>,
}

/// `discovered` is in the order the directory walk met the files: for `JavadocPackage` the first
/// file of a directory decides, and which one it is depends on that order (Checkstyle uses the file
/// system's own).
pub fn analyse(cfg: &Config, discovered: &[PathBuf]) -> Findings {
    let mut out = Findings::default();
    if cfg.javadoc_package.is_some() {
        javadoc_package(cfg, discovered, &mut out);
    }
    if cfg.translation.is_some() {
        translation(cfg, discovered, &mut out);
    }
    out
}

fn javadoc_package(cfg: &Config, discovered: &[PathBuf], out: &mut Findings) {
    let Some(opts) = &cfg.javadoc_package else { return };
    let mut checked: HashSet<PathBuf> = HashSet::new();
    for path in discovered {
        if path.extension().is_none_or(|e| e != "java") || !cfg.reads(&None, "java") {
            continue;
        }
        // Files kept out by a file filter are not processed; `module-info.java` is skipped by the check.
        if crate::suppress::file_excluded(cfg, path) || path.file_name().is_some_and(|n| n == "module-info.java") {
            continue;
        }
        let dir = path.parent().map(|d| if d.as_os_str().is_empty() { Path::new(".") } else { d }).unwrap_or(Path::new("."));
        let canonical = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
        if !checked.insert(canonical.clone()) {
            continue;
        }
        let info = canonical.join("package-info.java").exists();
        let html = canonical.join("package.html").exists();
        let message = if info && html {
            Some("Legacy package.html file should be removed.")
        } else if !info && !(opts.allow_legacy && html) {
            Some("Missing package-info.java file.")
        } else {
            None
        };
        if let Some(message) = message {
            out.on_file.entry(path.clone()).or_default().push(Diagnostic {
                rule: cfg.display("JavadocPackage"),
                line: 1,
                col: 0,
                message: message.to_string(),
            });
        }
    }
}

/// `^.+\_[a-z]{2}\_[A-Z]{2}\_[A-Za-z]+\..+$` and its shorter relatives: the locale part of a bundle's file name.
fn base_name(file: &str) -> String {
    let dot = file.find('.').unwrap_or(file.len());
    let (stem, ext) = file.split_at(dot);
    let parts: Vec<&str> = stem.split('_').collect();
    let n = parts.len();
    let is_lang = |s: &str| s.len() == 2 && s.bytes().all(|b| b.is_ascii_lowercase());
    let is_country = |s: &str| s.len() == 2 && s.bytes().all(|b| b.is_ascii_uppercase());
    let is_variant = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphabetic());
    let strip = if n >= 4 && is_lang(parts[n - 3]) && is_country(parts[n - 2]) && is_variant(parts[n - 1]) {
        3
    } else if n >= 3 && is_lang(parts[n - 2]) && is_country(parts[n - 1]) {
        2
    } else if n >= 2 && is_lang(parts[n - 1]) {
        1
    } else {
        0
    };
    let _ = ext;
    parts[..n - strip].join("_")
}

fn translation(cfg: &Config, discovered: &[PathBuf], out: &mut Findings) {
    let Some(opts) = &cfg.translation else { return };
    // (directory, base name, extension) -> files
    let mut bundles: BTreeMap<(PathBuf, String, String), BTreeSet<PathBuf>> = BTreeMap::new();
    for path in discovered {
        if path.extension().is_none_or(|e| e != "properties") || !cfg.reads(&opts.extensions, "properties") || crate::suppress::file_excluded(cfg, path) {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        let base = base_name(name);
        if !opts.base_name.matches(base.as_bytes()) {
            continue;
        }
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        bundles.entry((dir, base, "properties".to_string())).or_default().insert(path.clone());
    }
    for ((dir, base, ext), files) in bundles {
        let report_dir = |name: String, out: &mut Findings| {
            out.at_end.push((
                dir.clone(),
                Diagnostic { rule: cfg.display("Translation"), line: 1, col: 0, message: format!("Properties file '{name}' is missing.") },
            ));
        };
        let default_name = format!("{base}.{ext}");
        if !files.iter().any(|f| f.file_name().is_some_and(|n| *n == *default_name)) {
            report_dir(default_name, out);
        }
        for code in &opts.required {
            let has = files.iter().any(|f| {
                let n = f.file_name().and_then(|n| n.to_str()).unwrap_or_default();
                let rest = n.strip_prefix(&format!("{base}_{code}")).unwrap_or("");
                n.starts_with(&format!("{base}_{code}"))
                    && (rest == format!(".{ext}")
                        || (rest.len() == ext.len() + 4
                            && rest.starts_with('_')
                            && rest.ends_with(&format!(".{ext}"))
                            && rest[1..3].bytes().all(|b| b.is_ascii_uppercase()))
                        || (rest.starts_with('_') && rest.ends_with(&format!(".{ext}"))))
            });
            if !has {
                report_dir(format!("{base}_{code}.{ext}"), out);
            }
        }
        let keys: BTreeMap<&PathBuf, BTreeSet<String>> = files.iter().map(|f| (f, std::fs::read(f).map(|b| property_keys(&b)).unwrap_or_default())).collect();
        let union: BTreeSet<&String> = keys.values().flatten().collect();
        for (file, have) in &keys {
            for key in union.iter().filter(|k| !have.contains(**k)) {
                let shown = key.replace('\n', "\\n");
                out.at_end
                    .push(((*file).clone(), Diagnostic { rule: cfg.display("Translation"), line: 1, col: 0, message: format!("Key '{shown}' missing.") }));
            }
        }
    }
}

/// The keys of a `.properties` file, read the way `java.util.Properties.load` reads a byte stream.
pub fn property_keys(bytes: &[u8]) -> BTreeSet<String> {
    // ISO-8859-1: each byte is a character.
    let text: String = bytes.iter().map(|&b| b as char).collect();
    let mut keys = BTreeSet::new();
    let mut lines = text.split(['\n', '\r']).peekable();
    while let Some(first) = lines.next() {
        let trimmed = first.trim_start_matches([' ', '\t', '\x0c']);
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
            continue;
        }
        // Join continuation lines: an odd number of trailing backslashes.
        let mut logical = trimmed.to_string();
        while logical.chars().rev().take_while(|c| *c == '\\').count() % 2 == 1 {
            logical.pop();
            match lines.next() {
                Some(next) => logical.push_str(next.trim_start_matches([' ', '\t', '\x0c'])),
                None => break,
            }
        }
        let mut key = String::new();
        let mut chars = logical.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '=' | ':' | ' ' | '\t' | '\x0c' => break,
                '\\' => match chars.next() {
                    Some('u') => {
                        let hex: String = chars.by_ref().take(4).collect();
                        if let Some(ch) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                            key.push(ch);
                        }
                    }
                    Some('t') => key.push('\t'),
                    Some('n') => key.push('\n'),
                    Some('r') => key.push('\r'),
                    Some('f') => key.push('\x0c'),
                    Some(other) => key.push(other),
                    None => {}
                },
                other => key.push(other),
            }
        }
        keys.insert(key);
    }
    keys
}
