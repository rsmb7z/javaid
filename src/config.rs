//! What to check and how: Checkstyle's own defaults, or a `checkstyle.xml`.
//!
//! The rule is *never skip silently*. A module javaid does not implement, or a property it cannot
//! honour, is reported by name with the reason, and that module does not run.

use regex::bytes::{Regex, RegexBuilder};

/// One implemented module. This table is the single source of truth for which modules exist and
/// how a configuration may use them; adding a module means adding a row, an arm in `Config::apply`
/// and its rule. A test checks that the table and `apply` agree.
#[derive(Clone, Copy, Debug)]
pub struct Module {
    /// Checkstyle's spelling.
    pub name: &'static str,
    pub family: &'static str,
    /// A configuration may list it more than once (`TypeName` once per type kind).
    pub multi: bool,
    /// Each instance is labelled with its own `id`, instead of one id for the module.
    pub own_name: bool,
}

pub const REGISTRY: &[Module] = &[
    Module { name: "EmptyCatchBlock", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "NeedBraces", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "EqualsHashCode", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "UnusedImports", family: "imports", multi: false, own_name: false },
    Module { name: "TypeName", family: "naming", multi: true, own_name: false },
    Module { name: "MethodName", family: "naming", multi: false, own_name: false },
    Module { name: "MemberName", family: "naming", multi: false, own_name: false },
    Module { name: "ConstantName", family: "naming", multi: false, own_name: false },
    Module { name: "StaticVariableName", family: "naming", multi: false, own_name: false },
    Module { name: "ParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "LocalVariableName", family: "naming", multi: false, own_name: false },
    Module { name: "LocalFinalVariableName", family: "naming", multi: false, own_name: false },
    Module { name: "PackageName", family: "naming", multi: false, own_name: false },
    Module { name: "ClassTypeParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "InterfaceTypeParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "MethodTypeParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "RecordTypeParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "CatchParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "LambdaParameterName", family: "naming", multi: false, own_name: false },
    Module { name: "PatternVariableName", family: "naming", multi: false, own_name: false },
    Module { name: "RecordComponentName", family: "naming", multi: false, own_name: false },
    Module { name: "FileTabCharacter", family: "text", multi: false, own_name: false },
    Module { name: "LineLength", family: "text", multi: false, own_name: false },
    Module { name: "RegexpSingleline", family: "text", multi: true, own_name: true },
    Module { name: "TodoComment", family: "text", multi: true, own_name: true },
    Module { name: "ArrayTypeStyle", family: "modifiers and small checks", multi: false, own_name: false },
    Module { name: "UpperEll", family: "modifiers and small checks", multi: false, own_name: false },
    Module { name: "AvoidStarImport", family: "imports", multi: false, own_name: false },
    Module { name: "MissingSwitchDefault", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "ModifierOrder", family: "modifiers and small checks", multi: false, own_name: false },
    Module { name: "MultipleVariableDeclarations", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "OneStatementPerLine", family: "coding and blocks", multi: false, own_name: false },
    Module { name: "FallThrough", family: "coding and blocks", multi: false, own_name: false },
];

/// Every module name, in registry order.
pub fn module_names() -> impl Iterator<Item = &'static str> {
    REGISTRY.iter().map(|m| m.name)
}

/// An `id` from the configuration, kept for the life of the process. Configurations are read once
/// and have a handful of ids, so leaking them is the simplest way to hand out `&'static str`.
fn leak(id: &str) -> &'static str {
    Box::leak(id.to_string().into_boxed_str())
}

pub fn find_module(name: &str) -> Option<&'static Module> {
    REGISTRY.iter().find(|m| m.name == name)
}

/// A naming pattern. Java and Rust regexes differ on shorthand classes (`\w` is ASCII-only in
/// Java and Unicode in Rust), so patterns are translated before compiling. Names are matched with
/// `find` semantics, exactly as Checkstyle matches them.
///
/// Most patterns compile with the fast `regex` crate and match raw bytes. Patterns that need
/// lookahead or backreferences (Google's naming formats use lookahead) fall back to `fancy-regex`.
#[derive(Clone, Debug)]
pub struct Format {
    pub source: String,
    engine: Engine,
}

#[derive(Clone, Debug)]
enum Engine {
    Fast(Regex),
    Fancy(fancy_regex::Regex),
}

impl Format {
    pub fn new(source: &str) -> Result<Format, String> {
        Format::with_case(source, false)
    }

    /// `ignore_case` is Java's `CASE_INSENSITIVE` flag (`RegexpSingleline.ignoreCase`).
    pub fn with_case(source: &str, ignore_case: bool) -> Result<Format, String> {
        let translated = java_regex_to_rust(source);
        let engine = match RegexBuilder::new(&translated).case_insensitive(ignore_case).build() {
            Ok(re) => Engine::Fast(re),
            Err(_) => {
                let pattern = if ignore_case { format!("(?i){translated}") } else { translated };
                match fancy_regex::Regex::new(&pattern) {
                    Ok(re) => Engine::Fancy(re),
                    Err(e) => return Err(format!("regular expression '{source}' is not supported: {e}")),
                }
            }
        };
        Ok(Format { source: source.to_string(), engine })
    }

    pub fn matches(&self, name: &[u8]) -> bool {
        match &self.engine {
            Engine::Fast(re) => re.is_match(name),
            // A backtracking error (pathological pattern) counts as no match, never a panic.
            Engine::Fancy(re) => std::str::from_utf8(name).is_ok_and(|s| re.is_match(s).unwrap_or(false)),
        }
    }
}

/// Rewrite Java's ASCII shorthand classes into explicit ASCII classes.
fn java_regex_to_rust(java: &str) -> String {
    let mut out = String::with_capacity(java.len() + 16);
    let mut class_depth = 0usize;
    let mut chars = java.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('w') => out.push_str(if class_depth > 0 { "a-zA-Z0-9_" } else { "[a-zA-Z0-9_]" }),
                Some('d') => out.push_str(if class_depth > 0 { "0-9" } else { "[0-9]" }),
                Some('s') => out.push_str(if class_depth > 0 { r" \t\n\x0B\f\r" } else { r"[ \t\n\x0B\f\r]" }),
                Some('W') if class_depth == 0 => out.push_str("[^a-zA-Z0-9_]"),
                Some('D') if class_depth == 0 => out.push_str("[^0-9]"),
                Some('S') if class_depth == 0 => out.push_str(r"[^ \t\n\x0B\f\r]"),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            },
            '[' => {
                class_depth += 1;
                out.push('[');
            }
            ']' => {
                class_depth = class_depth.saturating_sub(1);
                out.push(']');
            }
            _ => out.push(c),
        }
    }
    out
}

#[derive(Clone, Debug)]
pub struct UnusedImports {
    pub process_javadoc: bool,
}

#[derive(Clone, Debug)]
pub struct MethodName {
    pub format: Format,
    pub allow_class_name: bool,
}

#[derive(Clone, Debug)]
pub struct ParameterName {
    pub format: Format,
    pub ignore_overridden: bool,
}

#[derive(Clone, Debug)]
pub struct LocalVariableName {
    pub format: Format,
    pub allow_one_char_var_in_for_loop: bool,
}

/// `TypeName` can be configured once per type kind (Google's config uses two instances, one for
/// classes and one for the rest), so each kind has its own optional format.
#[derive(Clone, Debug, Default)]
pub struct TypeNames {
    pub class: Option<Format>,
    pub interface: Option<Format>,
    pub enumeration: Option<Format>,
    pub annotation: Option<Format>,
    pub record: Option<Format>,
}

/// The naming family. `None` means the module is not enabled.
#[derive(Clone, Debug, Default)]
pub struct Naming {
    pub type_name: TypeNames,
    pub method_name: Option<MethodName>,
    pub member_name: Option<Format>,
    pub constant_name: Option<Format>,
    pub static_variable_name: Option<Format>,
    pub parameter_name: Option<ParameterName>,
    pub local_variable_name: Option<LocalVariableName>,
    pub local_final_variable_name: Option<Format>,
    pub package_name: Option<Format>,
    pub class_type_parameter_name: Option<Format>,
    pub interface_type_parameter_name: Option<Format>,
    pub method_type_parameter_name: Option<Format>,
    pub record_type_parameter_name: Option<Format>,
    pub catch_parameter_name: Option<Format>,
    pub lambda_parameter_name: Option<Format>,
    pub pattern_variable_name: Option<Format>,
    pub record_component_name: Option<Format>,
}

/// `None` means Checkstyle's default for that property (`^$` and `.*`), which needs no work at all.
#[derive(Clone, Debug, Default)]
pub struct EmptyCatchBlock {
    /// A block is allowed if the catch variable's name matches this.
    pub exception_variable_name: Option<Format>,
    /// A block is allowed if its *first* comment's text matches this.
    pub comment_format: Option<Format>,
}

/// The extra file extensions a file-level module reads, from its own `fileExtensions` property.
/// `None` means the property is not set.
pub type Extensions = Option<Vec<String>>;

#[derive(Clone, Debug)]
pub struct FileTabCharacter {
    /// Report every line that has a tab, instead of only the first one in the file.
    pub each_line: bool,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct LineLength {
    pub max: usize,
    /// A line this matches (with `find` semantics) is never reported. The default is
    /// `^(package|import) .*`. An empty pattern matches every line, so nothing is reported.
    pub ignore_pattern: Format,
    pub extensions: Extensions,
}

#[derive(Clone, Debug)]
pub struct RegexpSingleline {
    /// What Checkstyle prints in brackets: the module's `id` if it has one, else its name.
    pub name: &'static str,
    pub format: Format,
    /// The `message` property, which replaces the default wording.
    pub message: Option<String>,
    pub minimum: usize,
    pub maximum: usize,
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Default)]
pub struct AvoidStarImport {
    pub allow_class_imports: bool,
    pub allow_static_member_imports: bool,
    /// Imported names (`java.io`, or `java.lang.Math` for a static import) whose star import is fine.
    pub excludes: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct OneStatementPerLine {
    /// Two resources of one `try (...)` on a line count as two statements.
    pub treat_try_resources_as_statement: bool,
}

#[derive(Clone, Debug)]
pub struct FallThrough {
    /// Hold the last case group to the same rule; it falls out of the switch, not into a case.
    pub check_last_case_group: bool,
    /// A comment matching this on the line before the next case excuses a group. Only the
    /// default, `falls?[ -]?thr(u|ough)`, is supported.
    pub relief: Format,
}

#[derive(Clone, Debug)]
pub struct TodoComment {
    pub name: &'static str,
    pub format: Format,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub empty_catch_block: Option<EmptyCatchBlock>,
    pub need_braces: bool,
    pub equals_hash_code: bool,
    pub unused_imports: Option<UnusedImports>,
    pub naming: Naming,
    pub file_tab_character: Option<FileTabCharacter>,
    pub line_length: Option<LineLength>,
    pub regexp_singleline: Vec<RegexpSingleline>,
    pub todo_comments: Vec<TodoComment>,
    pub array_type_style: bool,
    pub upper_ell: bool,
    pub avoid_star_import: Option<AvoidStarImport>,
    pub missing_switch_default: bool,
    pub modifier_order: bool,
    pub multiple_variable_declarations: bool,
    pub one_statement_per_line: Option<OneStatementPerLine>,
    pub fall_through: Option<FallThrough>,
    /// Filters that remove findings (see `suppress`). None are implemented yet.
    pub filters: Vec<String>,
    /// The Checker's own `fileExtensions`, if the configuration sets one.
    pub checker_extensions: Option<Vec<String>>,
    /// The Checker's `tabWidth`: how far a tab advances when a line is measured. Default 8.
    pub tab_width: usize,
    /// `id`s that single-instance modules were given. Checkstyle prints the id in brackets where it
    /// would print the module name, so javaid does too (see `display`).
    pub ids: Vec<(&'static str, &'static str)>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            empty_catch_block: None,
            need_braces: false,
            equals_hash_code: false,
            unused_imports: None,
            naming: Naming::default(),
            file_tab_character: None,
            line_length: None,
            regexp_singleline: Vec::new(),
            todo_comments: Vec::new(),
            array_type_style: false,
            upper_ell: false,
            avoid_star_import: None,
            missing_switch_default: false,
            modifier_order: false,
            multiple_variable_declarations: false,
            one_statement_per_line: None,
            fall_through: None,
            filters: Vec::new(),
            checker_extensions: None,
            tab_width: 8,
            ids: Vec::new(),
        }
    }
}

/// Checkstyle spells extensions with or without the dot; the comparison here is on the bare name.
fn parse_extensions(list: &str) -> Vec<String> {
    list.split(',').map(|e| e.trim().trim_start_matches('.').to_string()).filter(|e| !e.is_empty()).collect()
}

impl Config {
    /// What Checkstyle prints in brackets for a module's findings: its `id` if it has one.
    pub fn display(&self, module: &'static str) -> &'static str {
        self.ids.iter().find(|(m, _)| *m == module).map_or(module, |(_, id)| id)
    }

    /// Does a file-level module with these own extensions read a file named `*.ext`? Checkstyle
    /// requires the file to pass the Checker's `fileExtensions` and then the module's own, and reads
    /// every file when neither is set. javaid reads only `.java` then, rather than every file type.
    pub fn reads(&self, own: &Extensions, ext: &str) -> bool {
        let named = |set: &Vec<String>| set.iter().any(|e| e == ext);
        let checker_ok = self.checker_extensions.as_ref().is_none_or(named);
        let own_ok = own.as_ref().is_none_or(named);
        let unconstrained = self.checker_extensions.is_none() && own.is_none();
        checker_ok && own_ok && (!unconstrained || ext == "java")
    }

    /// Every file extension some enabled module reads: what the file search must collect.
    pub fn extensions(&self) -> Vec<String> {
        let mut found = vec!["java".to_string()];
        let mut add = |own: &Extensions| {
            for e in own.iter().chain(self.checker_extensions.iter()).flatten() {
                if !found.contains(e) && self.reads(own, e) {
                    found.push(e.clone());
                }
            }
        };
        if let Some(m) = &self.file_tab_character {
            add(&m.extensions);
        }
        if let Some(m) = &self.line_length {
            add(&m.extensions);
        }
        for m in &self.regexp_singleline {
            add(&m.extensions);
        }
        found
    }
}

/// The outcome of reading a `checkstyle.xml`.
pub struct Loaded {
    pub config: Config,
    pub enabled: Vec<String>,
    /// Module name and the reason it did not run.
    pub skipped: Vec<(String, String)>,
    /// Things that do not stop a module running but change what the user sees, e.g. a custom
    /// message template that javaid does not apply.
    pub notes: Vec<String>,
}

/// One `<module>` as written: its name, its properties and its `<message>` overrides.
struct RawModule {
    name: String,
    props: Vec<(String, String)>,
    messages: Vec<String>,
}

/// Properties of one `<module>`, consumed as they are understood; anything left over is a
/// property javaid cannot honour.
struct Props {
    items: Vec<(String, String)>,
}

/// A comma-separated list compared as a set: order, spacing and case do not matter.
fn normalize(s: &str) -> String {
    let mut parts: Vec<String> = s.split(',').map(|p| p.trim().to_ascii_lowercase()).filter(|p| !p.is_empty()).collect();
    parts.sort();
    parts.join(",")
}

impl Props {
    fn take(&mut self, name: &str) -> Option<String> {
        let i = self.items.iter().position(|(k, _)| k == name)?;
        Some(self.items.remove(i).1)
    }

    /// The property may be present, but only at its default value.
    fn expect_default(&mut self, name: &str, default: &str) -> Result<(), String> {
        match self.take(name) {
            Some(v) if normalize(&v) != normalize(default) => Err(format!("property {name}={v} is not supported yet")),
            _ => Ok(()),
        }
    }

    fn bool(&mut self, name: &str, default: bool) -> Result<bool, String> {
        match self.take(name) {
            None => Ok(default),
            Some(v) => match v.trim() {
                "true" => Ok(true),
                "false" => Ok(false),
                other => Err(format!("property {name}: expected true or false, got '{other}'")),
            },
        }
    }

    fn usize(&mut self, name: &str, default: usize) -> Result<usize, String> {
        match self.take(name) {
            None => Ok(default),
            Some(v) => v.trim().parse().map_err(|_| format!("property {name}: expected a non-negative integer, got '{}'", v.trim())),
        }
    }

    fn extensions(&mut self) -> Extensions {
        self.take("fileExtensions").map(|v| parse_extensions(&v))
    }

    fn format(&mut self, default: &str) -> Result<Format, String> {
        let src = self.take("format").unwrap_or_else(|| default.to_string());
        Format::new(&src)
    }

    fn finish(self) -> Result<(), String> {
        if self.items.is_empty() {
            Ok(())
        } else {
            let names: Vec<&str> = self.items.iter().map(|(k, _)| k.as_str()).collect();
            Err(format!("unsupported property: {}", names.join(", ")))
        }
    }
}

const SIMPLE_LOWER: &str = r"^[a-z][a-zA-Z0-9]*$";
const LOWER_OR_UNDERSCORE: &str = r"^([a-z][a-zA-Z0-9]*|_)$";
const TYPE_PARAMETER: &str = r"^[A-Z]$";

impl Config {
    /// Every implemented module at Checkstyle's default settings.
    pub fn checkstyle_defaults(only: Option<&[String]>) -> Config {
        let mut cfg = Config::default();
        for m in module_names() {
            if allowed(only, m) {
                cfg.apply(m, Props { items: Vec::new() }, None).expect("default settings are always valid");
            }
        }
        cfg
    }

    /// Read a Checkstyle configuration. Modules and properties javaid cannot honour are listed in
    /// `skipped` with the reason, and do not run.
    pub fn from_checkstyle_xml(text: &str, only: Option<&[String]>) -> Result<Loaded, String> {
        let opts = roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() };
        let doc = roxmltree::Document::parse_with_options(text, opts).map_err(|e| format!("cannot parse the configuration: {e}"))?;

        let mut raw = Vec::new();
        collect_modules(doc.root_element(), &mut raw);

        let mut cfg = Config::default();
        let mut enabled: Vec<String> = Vec::new();
        let mut skipped = Vec::new();
        let mut notes = Vec::new();
        for RawModule { name: full_name, props: items, messages } in raw {
            let name = short_name(&full_name).to_string();
            let mut props = Props { items };
            if name == "Checker" {
                // The Checker's own settings that decide which files are read and how lines are measured.
                if let Some(e) = props.extensions() {
                    cfg.checker_extensions = Some(e);
                }
                match props.usize("tabWidth", 8) {
                    Ok(w) if w > 0 => cfg.tab_width = w,
                    _ => notes.push("Checker: tabWidth is not a positive integer; javaid measures tabs as 8 columns".to_string()),
                }
                continue;
            }
            if name == "TreeWalker" || !allowed(only, &name) {
                continue;
            }
            let id = props.take("id");
            if props.take("severity").is_some_and(|s| s.trim() == "ignore") {
                continue;
            }
            if props.items.iter().any(|(_, v)| v.contains("${")) {
                skipped.push((name, "uses an unresolved ${...} property".to_string()));
                continue;
            }
            if !find_module(&name).is_some_and(|m| m.multi) && enabled.contains(&name) {
                skipped.push((name, "a second instance of the same module is not supported".to_string()));
                continue;
            }
            match cfg.apply(&name, props, id.as_deref()) {
                Ok(()) => {
                    for key in messages {
                        notes.push(format!("{name}: the custom message for '{key}' is not applied; javaid prints its own wording"));
                    }
                    // Modules that keep one name per instance record their own id (see `apply`).
                    if let (Some(id), Some(module)) = (&id, find_module(&name)) {
                        if !module.own_name {
                            let module = module.name;
                            match cfg.ids.iter().find(|(m, _)| *m == module) {
                                Some((_, first)) if first != id => notes.push(format!("{name}: its instances have different ids; javaid prints '{first}' for all")),
                                Some(_) => {}
                                None => cfg.ids.push((module, leak(id))),
                            }
                        }
                    }
                    enabled.push(name);
                }
                Err(why) => skipped.push((name, why)),
            }
        }
        // Checkstyle hands a file-level module every file under the paths when no `fileExtensions`
        // narrows it. javaid reads Java files only, and says so rather than skipping silently.
        let unconstrained = |own: &Extensions| own.is_none();
        let file_level_without_extensions = cfg.file_tab_character.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.line_length.as_ref().is_some_and(|m| unconstrained(&m.extensions))
            || cfg.regexp_singleline.iter().any(|m| unconstrained(&m.extensions));
        if cfg.checker_extensions.is_none() && file_level_without_extensions {
            notes.push("no fileExtensions are set: Checkstyle would read every file under the paths for FileTabCharacter, LineLength and RegexpSingleline, javaid reads .java files only".to_string());
        }
        Ok(Loaded { config: cfg, enabled, skipped, notes })
    }

    fn apply(&mut self, module: &str, mut p: Props, id: Option<&str>) -> Result<(), String> {
        let access = ["applyToPublic", "applyToProtected", "applyToPackage", "applyToPrivate"];
        match module {
            "FileTabCharacter" => {
                let each_line = p.bool("eachLine", false)?;
                let extensions = p.extensions();
                p.finish()?;
                self.file_tab_character = Some(FileTabCharacter { each_line, extensions });
            }
            "LineLength" => {
                let max = p.usize("max", 80)?;
                let ignore_pattern = Format::new(&p.take("ignorePattern").unwrap_or_else(|| "^(package|import) .*".to_string()))?;
                let extensions = p.extensions();
                p.finish()?;
                self.line_length = Some(LineLength { max, ignore_pattern, extensions });
            }
            "RegexpSingleline" => {
                let ignore_case = p.bool("ignoreCase", false)?;
                // The default can never match: `$` is the end of the line, and a character must follow.
                let format = Format::with_case(&p.take("format").unwrap_or_else(|| "$.".to_string()), ignore_case)?;
                let message = p.take("message").filter(|m| !m.is_empty());
                let minimum = p.usize("minimum", 0)?;
                let maximum = p.usize("maximum", 0)?;
                let extensions = p.extensions();
                p.finish()?;
                let name = id.map_or("RegexpSingleline", leak);
                self.regexp_singleline.push(RegexpSingleline { name, format, message, minimum, maximum, extensions });
            }
            "ArrayTypeStyle" => {
                p.expect_default("javaStyle", "true")?; // C style (false) has quirks that were not modelled
                p.finish()?;
                self.array_type_style = true;
            }
            "UpperEll" => {
                p.finish()?;
                self.upper_ell = true;
            }
            "MissingSwitchDefault" => {
                p.finish()?;
                self.missing_switch_default = true;
            }
            "ModifierOrder" => {
                // A custom order of modifiers is a different check; only the JLS order is modelled.
                if p.take("modifiersOrder").is_some() {
                    return Err("property modifiersOrder is not supported yet".to_string());
                }
                p.finish()?;
                self.modifier_order = true;
            }
            "MultipleVariableDeclarations" => {
                p.finish()?;
                self.multiple_variable_declarations = true;
            }
            "FallThrough" => {
                let check_last_case_group = p.bool("checkLastCaseGroup", false)?;
                const DEFAULT_RELIEF: &str = "falls?[ -]?thr(u|ough)";
                if p.take("reliefPattern").is_some_and(|v| v != DEFAULT_RELIEF) {
                    return Err("property reliefPattern is not supported yet".to_string());
                }
                p.finish()?;
                self.fall_through = Some(FallThrough { check_last_case_group, relief: Format::new(DEFAULT_RELIEF)? });
            }
            "OneStatementPerLine" => {
                let treat_try_resources_as_statement = p.bool("treatTryResourcesAsStatement", false)?;
                p.finish()?;
                self.one_statement_per_line = Some(OneStatementPerLine { treat_try_resources_as_statement });
            }
            "AvoidStarImport" => {
                let allow_class_imports = p.bool("allowClassImports", false)?;
                let allow_static_member_imports = p.bool("allowStaticMemberImports", false)?;
                p.expect_default("maxAllowedStarImports", "0")?;
                // Entries are trimmed, and `java.io.*` means the same as `java.io`.
                let excludes = p.take("excludes").map_or_else(Vec::new, |v| {
                    v.split(',').map(|e| e.trim().trim_end_matches(".*").to_string()).filter(|e| !e.is_empty()).collect()
                });
                p.finish()?;
                self.avoid_star_import = Some(AvoidStarImport { allow_class_imports, allow_static_member_imports, excludes });
            }
            "TodoComment" => {
                let format = p.format("TODO:")?;
                p.finish()?;
                let name = id.map_or("TodoComment", leak);
                self.todo_comments.push(TodoComment { name, format });
            }
            "EmptyCatchBlock" => {
                let non_default = |v: String, default: &str| (v != default).then_some(v);
                let exception_variable_name = p.take("exceptionVariableName").and_then(|v| non_default(v, "^$")).map(|v| Format::new(&v)).transpose()?;
                let comment_format = p.take("commentFormat").and_then(|v| non_default(v, ".*")).map(|v| Format::new(&v)).transpose()?;
                p.finish()?;
                self.empty_catch_block = Some(EmptyCatchBlock { exception_variable_name, comment_format });
            }
            "NeedBraces" => {
                p.expect_default("tokens", "LITERAL_DO,LITERAL_ELSE,LITERAL_FOR,LITERAL_IF,LITERAL_WHILE")?;
                p.expect_default("allowSingleLineStatement", "false")?;
                p.expect_default("allowEmptyLoopBody", "false")?;
                p.expect_default("allowEmptyLambdas", "false")?;
                p.expect_default("allowEmptyCatch", "false")?;
                p.expect_default("allowEmptyCase", "false")?;
                p.finish()?;
                self.need_braces = true;
            }
            "EqualsHashCode" => {
                p.finish()?;
                self.equals_hash_code = true;
            }
            "UnusedImports" => {
                let process_javadoc = p.bool("processJavadoc", true)?;
                p.expect_default("violateExecutionOnNonTightHtml", "false")?;
                p.finish()?;
                self.unused_imports = Some(UnusedImports { process_javadoc });
            }
            "TypeName" => {
                for a in access {
                    p.expect_default(a, "true")?;
                }
                let tokens = p.take("tokens");
                let format = p.format(r"^[A-Z][a-zA-Z0-9]*$")?;
                p.finish()?;
                let list: Vec<String> = match tokens {
                    Some(t) => t.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
                    None => ["CLASS_DEF", "INTERFACE_DEF", "ENUM_DEF", "ANNOTATION_DEF", "RECORD_DEF"].map(String::from).to_vec(),
                };
                // Validate every token before changing anything, so a rejected instance leaves no trace.
                let t = &mut self.naming.type_name;
                for tok in &list {
                    let slot = match tok.as_str() {
                        "CLASS_DEF" => &t.class,
                        "INTERFACE_DEF" => &t.interface,
                        "ENUM_DEF" => &t.enumeration,
                        "ANNOTATION_DEF" => &t.annotation,
                        "RECORD_DEF" => &t.record,
                        other => return Err(format!("token {other} is not supported")),
                    };
                    if slot.is_some() {
                        return Err(format!("token {tok} is already configured by an earlier TypeName"));
                    }
                }
                for tok in &list {
                    let slot = match tok.as_str() {
                        "CLASS_DEF" => &mut t.class,
                        "INTERFACE_DEF" => &mut t.interface,
                        "ENUM_DEF" => &mut t.enumeration,
                        "ANNOTATION_DEF" => &mut t.annotation,
                        _ => &mut t.record,
                    };
                    *slot = Some(format.clone());
                }
            }
            "MemberName" | "ConstantName" | "StaticVariableName" => {
                for a in access {
                    p.expect_default(a, "true")?;
                }
                let default = if module == "ConstantName" { r"^[A-Z][A-Z0-9]*(_[A-Z0-9]+)*$" } else { SIMPLE_LOWER };
                let format = p.format(default)?;
                p.finish()?;
                match module {
                    "MemberName" => self.naming.member_name = Some(format),
                    "ConstantName" => self.naming.constant_name = Some(format),
                    _ => self.naming.static_variable_name = Some(format),
                }
            }
            "MethodName" => {
                for a in access {
                    p.expect_default(a, "true")?;
                }
                let allow_class_name = p.bool("allowClassName", false)?;
                let format = p.format(SIMPLE_LOWER)?;
                p.finish()?;
                self.naming.method_name = Some(MethodName { format, allow_class_name });
            }
            "ParameterName" => {
                p.expect_default("accessModifiers", "public, protected, package, private")?;
                let ignore_overridden = p.bool("ignoreOverridden", false)?;
                let format = p.format(SIMPLE_LOWER)?;
                p.finish()?;
                self.naming.parameter_name = Some(ParameterName { format, ignore_overridden });
            }
            "LocalVariableName" => {
                let allow_one_char_var_in_for_loop = p.bool("allowOneCharVarInForLoop", false)?;
                let format = p.format(LOWER_OR_UNDERSCORE)?;
                p.finish()?;
                self.naming.local_variable_name = Some(LocalVariableName { format, allow_one_char_var_in_for_loop });
            }
            "LocalFinalVariableName" | "LambdaParameterName" | "PatternVariableName" => {
                if module == "LocalFinalVariableName" {
                    p.expect_default("tokens", "VARIABLE_DEF,PARAMETER_DEF,RESOURCE")?;
                }
                let format = p.format(LOWER_OR_UNDERSCORE)?;
                p.finish()?;
                match module {
                    "LocalFinalVariableName" => self.naming.local_final_variable_name = Some(format),
                    "LambdaParameterName" => self.naming.lambda_parameter_name = Some(format),
                    _ => self.naming.pattern_variable_name = Some(format),
                }
            }
            "PackageName" => {
                let format = p.format(r"^[a-z]+(\.[a-zA-Z_]\w*)*$")?;
                p.finish()?;
                self.naming.package_name = Some(format);
            }
            "ClassTypeParameterName" | "InterfaceTypeParameterName" | "MethodTypeParameterName" | "RecordTypeParameterName" => {
                let format = p.format(TYPE_PARAMETER)?;
                p.finish()?;
                match module {
                    "ClassTypeParameterName" => self.naming.class_type_parameter_name = Some(format),
                    "InterfaceTypeParameterName" => self.naming.interface_type_parameter_name = Some(format),
                    "MethodTypeParameterName" => self.naming.method_type_parameter_name = Some(format),
                    _ => self.naming.record_type_parameter_name = Some(format),
                }
            }
            "CatchParameterName" => {
                let format = p.format(r"^(e|t|ex|[a-z][a-z][a-zA-Z]+|_)$")?;
                p.finish()?;
                self.naming.catch_parameter_name = Some(format);
            }
            "RecordComponentName" => {
                let format = p.format(SIMPLE_LOWER)?;
                p.finish()?;
                self.naming.record_component_name = Some(format);
            }
            _ => return Err("not implemented".to_string()),
        }
        Ok(())
    }
}

fn allowed(only: Option<&[String]>, module: &str) -> bool {
    only.is_none_or(|names| names.iter().any(|n| n.eq_ignore_ascii_case(module)))
}

/// `com.puppycrawl.tools.checkstyle.checks.naming.TypeNameCheck` and `TypeName` are the same module.
fn short_name(full: &str) -> &str {
    let last = full.rsplit('.').next().unwrap_or(full);
    last.strip_suffix("Check").filter(|s| !s.is_empty()).unwrap_or(last)
}

fn collect_modules(node: roxmltree::Node, out: &mut Vec<RawModule>) {
    let is_module = |n: &roxmltree::Node| n.is_element() && n.tag_name().name() == "module";
    if is_module(&node) {
        let children = |tag: &'static str| node.children().filter(move |c| c.is_element() && c.tag_name().name() == tag);
        // `value="${name}" default="x"`: javaid has no property source, so every placeholder is unset,
        // and Checkstyle (probed, bench/oracle/probes/Probe7) then uses the default in place of the
        // whole value, even when the placeholder is only part of it. Without a default, Checkstyle
        // refuses to load the file; javaid skips just that module and says so (see `from_checkstyle_xml`).
        let props = children("property")
            .filter_map(|p| {
                let value = p.attribute("value").unwrap_or("");
                let value = match p.attribute("default") {
                    Some(default) if value.contains("${") => default,
                    _ => value,
                };
                Some((p.attribute("name")?.to_string(), value.to_string()))
            })
            .collect();
        let messages = children("message").filter_map(|m| m.attribute("key").map(String::from)).collect();
        out.push(RawModule { name: node.attribute("name").unwrap_or("").to_string(), props, messages });
    }
    for child in node.children().filter(is_module) {
        collect_modules(child, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_registry_and_apply_agree() {
        let mut seen = std::collections::HashSet::new();
        for m in REGISTRY {
            assert!(seen.insert(m.name), "{} is listed twice", m.name);
        }
        // Every row must be one `apply` understands: loading all defaults panics on 'not implemented'.
        let cfg = Config::checkstyle_defaults(None);
        assert!(cfg.need_braces && cfg.fall_through.is_some());
        assert!(find_module("TodoComment").unwrap().own_name && !find_module("MethodName").unwrap().multi);
    }

    #[test]
    fn java_shorthand_classes_are_ascii() {
        let f = Format::new(r"^[a-z]\w*$").unwrap();
        assert!(f.matches(b"abc_9"));
        assert!(!f.matches("größe".as_bytes()), "Java's \\w is ASCII-only");
        assert!(Format::new(r"^\d+$").unwrap().matches(b"123"));
        assert!(Format::new(r"^[\w.]+$").unwrap().matches(b"a.b_c"));
    }

    #[test]
    fn match_is_a_search_like_javas_find() {
        assert!(Format::new("bc").unwrap().matches(b"abcd"));
    }

    #[test]
    fn lookahead_patterns_fall_back_to_the_slower_engine() {
        // Google's naming formats use lookahead, which the fast engine cannot compile.
        let f = Format::new(r"^(_|[a-z](?=[a-z0-9]|$)(?:[a-zA-Z]|[0-9]+(?:_[0-9]+)*)*)$").unwrap();
        assert!(f.matches(b"ab") && f.matches(b"a") && f.matches(b"_") && f.matches(b"a1_2"));
        assert!(!f.matches(b"aB"), "a second letter must be lower-case or a digit");
        assert!(!f.matches(b"Ab"));
    }

    fn only_module(xml: &str) -> Loaded {
        let wrapped = format!(r#"<module name="Checker"><module name="TreeWalker">{xml}</module></module>"#);
        Config::from_checkstyle_xml(&wrapped, None).unwrap()
    }

    #[test]
    fn tokens_are_accepted_at_a_modules_default_and_refused_otherwise() {
        let same = only_module(r#"<module name="NeedBraces"><property name="tokens" value="LITERAL_WHILE, LITERAL_IF,
            LITERAL_FOR, LITERAL_ELSE, LITERAL_DO"/></module>"#);
        assert!(same.skipped.is_empty() && same.config.need_braces, "order, spacing and newlines do not matter");
        let narrower = only_module(r#"<module name="NeedBraces"><property name="tokens" value="LITERAL_IF"/></module>"#);
        assert!(!narrower.config.need_braces);
        assert!(narrower.skipped[0].1.contains("tokens"));
    }

    #[test]
    fn type_name_can_be_configured_once_per_type_kind() {
        let two = only_module(
            r#"<module name="TypeName"><property name="format" value="^C"/><property name="tokens" value="CLASS_DEF"/></module>
               <module name="TypeName"><property name="tokens" value="INTERFACE_DEF, ENUM_DEF, ANNOTATION_DEF, RECORD_DEF"/></module>"#,
        );
        assert!(two.skipped.is_empty(), "{:?}", two.skipped);
        let t = &two.config.naming.type_name;
        assert_eq!(t.class.as_ref().unwrap().source, "^C");
        assert_eq!(t.record.as_ref().unwrap().source, r"^[A-Z][a-zA-Z0-9]*$");

        let overlap = only_module(
            r#"<module name="TypeName"/>
               <module name="TypeName"><property name="tokens" value="CLASS_DEF"/></module>"#,
        );
        assert_eq!(overlap.skipped.len(), 1);
        assert!(overlap.skipped[0].1.contains("already configured"));
    }

    #[test]
    fn defaults_enable_everything() {
        let cfg = Config::checkstyle_defaults(None);
        assert!(cfg.empty_catch_block.is_some() && cfg.need_braces && cfg.equals_hash_code);
        assert!(cfg.unused_imports.as_ref().unwrap().process_javadoc);
        assert!(cfg.naming.type_name.class.is_some() && cfg.naming.record_component_name.is_some());
    }

    #[test]
    fn an_unset_placeholder_takes_its_default_as_checkstyle_does() {
        // Behaviour pinned by Probe7 against Checkstyle 14.4: the default replaces the whole value.
        let with = only_module(r#"<module name="MethodName"><property name="format" value="${unset}" default="^[A-Z]+$"/></module>"#);
        assert!(with.skipped.is_empty(), "{:?}", with.skipped);
        assert_eq!(with.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$");

        let partial = only_module(r#"<module name="MethodName"><property name="format" value="^[a-z]+${unset}$" default="^[A-Z]+$"/></module>"#);
        assert_eq!(partial.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$");

        let unused = only_module(r#"<module name="MethodName"><property name="format" value="^[A-Z]+$" default="^[a-z]+$"/></module>"#);
        assert_eq!(unused.config.naming.method_name.as_ref().unwrap().format.source, "^[A-Z]+$", "a default is ignored when nothing is unset");

        // Checkstyle fails the whole load here; javaid skips the one module and names the reason.
        let without = only_module(r#"<module name="MethodName"><property name="format" value="${unset}"/></module>"#);
        assert!(without.config.naming.method_name.is_none());
        assert!(without.skipped[0].1.contains("unresolved"));
    }

    #[test]
    fn file_level_modules_read_the_extensions_the_config_names() {
        let both = Config::from_checkstyle_xml(
            r#"<module name="Checker"><property name="fileExtensions" value="java, .properties, xml"/>
               <module name="FileTabCharacter"/>
               <module name="LineLength"><property name="fileExtensions" value="java"/></module></module>"#,
            None,
        )
        .unwrap();
        let cfg = &both.config;
        assert!(cfg.reads(&cfg.file_tab_character.as_ref().unwrap().extensions, "properties"), "a leading dot is accepted");
        assert!(cfg.reads(&cfg.file_tab_character.as_ref().unwrap().extensions, "xml"));
        assert!(!cfg.reads(&cfg.line_length.as_ref().unwrap().extensions, "xml"), "the module's own list narrows the Checker's");
        assert!(!cfg.reads(&None, "yaml"), "the Checker's list is a ceiling");
        let mut found = cfg.extensions();
        found.sort();
        assert_eq!(found, ["java", "properties", "xml"]);

        // Nothing narrows anything: Checkstyle would read every file, javaid reads Java and says so.
        let bare = Config::from_checkstyle_xml(r#"<module name="Checker"><module name="FileTabCharacter"/></module>"#, None).unwrap();
        assert!(bare.config.reads(&None, "java") && !bare.config.reads(&None, "xml"));
        assert_eq!(bare.config.extensions(), ["java"]);
        assert!(bare.notes.iter().any(|n| n.contains("fileExtensions")), "{:?}", bare.notes);
    }

    #[test]
    fn options_javaid_does_not_model_are_refused_by_name() {
        let refused = |xml: &str| only_module(xml).skipped.into_iter().map(|(n, why)| format!("{n}: {why}")).collect::<Vec<_>>();
        assert!(refused(r#"<module name="ArrayTypeStyle"><property name="javaStyle" value="false"/></module>"#)[0].contains("javaStyle"));
        assert!(refused(r#"<module name="ModifierOrder"><property name="modifiersOrder" value="public, static"/></module>"#)[0].contains("modifiersOrder"));
        assert!(refused(r#"<module name="FallThrough"><property name="reliefPattern" value="fall"/></module>"#)[0].contains("reliefPattern"));
        assert!(refused(r#"<module name="AvoidStarImport"><property name="maxAllowedStarImports" value="2"/></module>"#)[0].contains("maxAllowedStarImports"));
        // ...while the defaults, spelled out, are fine.
        for fine in [
            r#"<module name="ArrayTypeStyle"><property name="javaStyle" value="true"/></module>"#,
            r#"<module name="FallThrough"><property name="reliefPattern" value="falls?[ -]?thr(u|ough)"/><property name="checkLastCaseGroup" value="true"/></module>"#,
            r#"<module name="OneStatementPerLine"><property name="treatTryResourcesAsStatement" value="true"/></module>"#,
        ] {
            assert!(refused(fine).is_empty(), "{fine}");
        }
    }

    #[test]
    fn ids_name_the_findings_and_instances_keep_their_own() {
        let loaded = only_module(
            r#"<module name="NeedBraces"><property name="id" value="braces"/></module>
               <module name="TodoComment"><property name="id" value="first"/></module>
               <module name="TodoComment"/>"#,
        );
        assert!(loaded.skipped.is_empty(), "{:?}", loaded.skipped);
        assert_eq!(loaded.config.display("NeedBraces"), "braces");
        assert_eq!(loaded.config.display("MethodName"), "MethodName", "a module without an id keeps its name");
        let names: Vec<_> = loaded.config.todo_comments.iter().map(|t| t.name).collect();
        assert_eq!(names, ["first", "TodoComment"]);
    }

    #[test]
    fn reads_a_checkstyle_xml_and_reports_what_it_skips() {
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <module name="TreeWalker">
    <module name="UnusedImports"><property name="processJavadoc" value="false"/></module>
    <module name="MethodName"><property name="format" value="^[a-z]+$"/></module>
    <module name="NeedBraces"><property name="allowSingleLineStatement" value="true"/></module>
    <module name="Indentation"/>
    <module name="com.puppycrawl.tools.checkstyle.checks.naming.TypeNameCheck"/>
    <module name="EqualsHashCode"><property name="severity" value="ignore"/></module>
  </module>
</module>"#;
        let loaded = Config::from_checkstyle_xml(xml, None).unwrap();
        assert!(!loaded.config.unused_imports.as_ref().unwrap().process_javadoc);
        assert_eq!(loaded.config.naming.method_name.as_ref().unwrap().format.source, "^[a-z]+$");
        assert!(loaded.config.naming.type_name.class.is_some(), "fully-qualified names are understood");
        assert!(!loaded.config.equals_hash_code, "severity=ignore disables a module");
        let skipped: Vec<_> = loaded.skipped.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(skipped, vec!["NeedBraces", "Indentation"], "unsupported property and unimplemented module are named");
        assert!(loaded.skipped[0].1.contains("allowSingleLineStatement"));
    }
}
