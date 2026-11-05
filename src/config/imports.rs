//! Imports.

use super::*;

#[derive(Clone, Debug)]
pub struct UnusedImports {
    pub process_javadoc: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AvoidStarImport {
    pub allow_class_imports: bool,
    pub allow_static_member_imports: bool,
    /// Imported names (`java.io`, or `java.lang.Math` for a static import) whose star import is fine.
    pub excludes: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CustomImportOrder {
    /// The configured groups in order (`SAME_PACKAGE(3)` becomes `SAME_PACKAGE`).
    pub rules: Vec<String>,
    /// The same, with the catch-all group Checkstyle appends.
    pub rules_with_fallback: Vec<String>,
    pub same_package_depth: usize,
    pub standard_package: Format,
    pub third_party: Format,
    pub special_imports: Format,
    pub separate_line_between_groups: bool,
    pub sort_alphabetically: bool,
}

#[derive(Clone, Debug)]
pub struct IllegalImport {
    pub pkgs: Vec<String>,
    pub classes: Vec<String>,
    pub regexp: bool,
    /// With `regexp`, the patterns an import's package, and an import's full name, must match in full.
    pub pkg_formats: Vec<Format>,
    pub class_formats: Vec<Format>,
}

/// `ModuleDirectiveOrder`.
pub(super) fn module_directive_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let validate_block_separation = p.bool("validateBlockSeparation", true)?;
    let order: Vec<String> = match p.take("order") {
        None => super::structural::MODULE_DIRECTIVES.iter().map(|s| s.to_string()).collect(),
        Some(v) => v.split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect(),
    };
    if let Some(bad) = order.iter().find(|o| !super::structural::MODULE_DIRECTIVES.contains(&o.as_str())) {
        return Err(format!("property order: unable to parse {bad}"));
    }
    p.finish()?;
    cfg.gs.module_directive_order = Some(super::structural::ModuleDirectiveOrder { order, validate_block_separation });
    Ok(())
}

/// `AvoidModuleImport`.
pub(super) fn avoid_module_import(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let excludes = p.take("excludes").map(|v| v.split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect()).unwrap_or_default();
    let max_allowed = p.usize("maxAllowedModuleImports", 0)?;
    p.finish()?;
    cfg.gs.avoid_module_import = Some(super::structural::AvoidModuleImport { excludes, max_allowed });
    Ok(())
}

/// `RedundantImport`.
pub(super) fn redundant_import(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.redundant_import = true;
    Ok(())
}

/// `IllegalInstantiation`.
pub(super) fn illegal_instantiation(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let classes = p.take("classes").map(|v| v.split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect()).unwrap_or_default();
    p.finish()?;
    cfg.illegal_instantiation = Some(classes);
    Ok(())
}

/// `IllegalImport`.
pub(super) fn illegal_import(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let regexp = p.bool("regexp", false)?;
    if p.take("illegalModules").is_some() {
        return Err("property illegalModules is not supported yet".to_string());
    }
    let list = |v: Option<String>, default: &str| -> Vec<String> {
        v.unwrap_or_else(|| default.to_string()).split(',').map(|e| e.trim().to_string()).filter(|e| !e.is_empty()).collect()
    };
    let pkgs = list(p.take("illegalPkgs"), "sun");
    let classes = list(p.take("illegalClasses"), "");
    p.finish()?;
    let formats = |items: &[String]| -> Result<Vec<Format>, String> {
        if regexp { items.iter().map(|i| Format::new(&format!("^(?:{i})$"))).collect() } else { Ok(Vec::new()) }
    };
    let (pkg_formats, class_formats) = (formats(&pkgs)?, formats(&classes)?);
    cfg.illegal_import = Some(IllegalImport { pkgs, classes, regexp, pkg_formats, class_formats });
    Ok(())
}

/// `AvoidStarImport`.
pub(super) fn avoid_star_import(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_class_imports = p.bool("allowClassImports", false)?;
    let allow_static_member_imports = p.bool("allowStaticMemberImports", false)?;
    p.expect_default("maxAllowedStarImports", "0")?;
    // Entries are trimmed, and `java.io.*` means the same as `java.io`.
    let excludes =
        p.take("excludes").map_or_else(Vec::new, |v| v.split(',').map(|e| e.trim().trim_end_matches(".*").to_string()).filter(|e| !e.is_empty()).collect());
    p.finish()?;
    cfg.avoid_star_import = Some(AvoidStarImport { allow_class_imports, allow_static_member_imports, excludes });
    Ok(())
}

/// `CustomImportOrder`.
pub(super) fn custom_import_order(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let standard_package = Format::new(&p.take("standardPackageRegExp").unwrap_or_else(|| "^(java|javax)\\.".to_string()))?;
    let third_party = Format::new(&p.take("thirdPartyPackageRegExp").unwrap_or_else(|| ".*".to_string()))?;
    let special_imports = Format::new(&p.take("specialImportsRegExp").unwrap_or_else(|| "^$".to_string()))?;
    let separate_line_between_groups = p.bool("separateLineBetweenGroups", true)?;
    let sort_alphabetically = p.bool("sortImportsInGroupAlphabetically", false)?;
    let mut same_package_depth = 0usize;
    let mut rules = Vec::new();
    if let Some(v) = p.take("customImportOrderRules") {
        for part in v.split(',').flat_map(|r| r.split("###")) {
            let rule = part.trim();
            match rule {
                "" => {}
                "STATIC" | "THIRD_PARTY_PACKAGE" | "STANDARD_JAVA_PACKAGE" | "SPECIAL_IMPORTS" => rules.push(rule.to_string()),
                r if r.starts_with("SAME_PACKAGE") => {
                    let inner = r.split_once('(').and_then(|(_, t)| t.split_once(')')).map(|(n, _)| n.trim());
                    same_package_depth = inner
                        .and_then(|n| n.parse().ok())
                        .filter(|n| *n > 0)
                        .ok_or_else(|| format!("SAME_PACKAGE rule parameter should be positive integer: {r}"))?;
                    rules.push("SAME_PACKAGE".to_string());
                }
                other => return Err(format!("customImportOrderRules: unexpected rule {other}")),
            }
        }
    }
    p.finish()?;
    let mut rules_with_fallback = rules.clone();
    rules_with_fallback.push("NOT_ASSIGNED_TO_ANY_GROUP".to_string());
    cfg.custom_import_order = Some(CustomImportOrder {
        rules,
        rules_with_fallback,
        same_package_depth,
        standard_package,
        third_party,
        special_imports,
        separate_line_between_groups,
        sort_alphabetically,
    });
    Ok(())
}

/// `UnusedImports`.
pub(super) fn unused_imports(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let process_javadoc = p.bool("processJavadoc", true)?;
    p.expect_default("violateExecutionOnNonTightHtml", "false")?;
    p.finish()?;
    cfg.unused_imports = Some(UnusedImports { process_javadoc });
    Ok(())
}
