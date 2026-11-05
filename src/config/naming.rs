//! Naming.

use super::*;

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

#[derive(Clone, Debug)]
pub struct AbbreviationAsWordInName {
    pub allowed_length: usize,
    pub allowed_abbreviations: Vec<String>,
    pub ignore_final: bool,
    pub ignore_static: bool,
    pub ignore_static_final: bool,
    pub ignore_overridden_methods: bool,
    pub tokens: Vec<String>,
}

/// `GoogleMethodName`.
pub(super) fn google_method_name(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.google_method_name = true;
    Ok(())
}

/// `GoogleNonConstantFieldName`.
pub(super) fn google_non_constant_field_name(cfg: &mut Config, _module: &str, p: Props, _id: Option<&str>) -> Result<(), String> {
    p.finish()?;
    cfg.google_non_constant_field_name = true;
    Ok(())
}

/// `AbbreviationAsWordInName`.
pub(super) fn abbreviation_as_word_in_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allowed_length = p.usize("allowedAbbreviationLength", 3)?;
    let allowed_abbreviations =
        p.take("allowedAbbreviations").map_or_else(Vec::new, |v| v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect());
    let ignore_final = p.bool("ignoreFinal", true)?;
    let ignore_static = p.bool("ignoreStatic", true)?;
    let ignore_static_final = p.bool("ignoreStaticFinal", true)?;
    let ignore_overridden_methods = p.bool("ignoreOverriddenMethods", true)?;
    const ACCEPTED: &[&str] = &[
        "CLASS_DEF",
        "INTERFACE_DEF",
        "ENUM_DEF",
        "ANNOTATION_DEF",
        "ANNOTATION_FIELD_DEF",
        "PARAMETER_DEF",
        "VARIABLE_DEF",
        "METHOD_DEF",
        "ENUM_CONSTANT_DEF",
        "PATTERN_VARIABLE_DEF",
        "RECORD_DEF",
        "RECORD_COMPONENT_DEF",
    ];
    let tokens = match p.take("tokens") {
        None => ACCEPTED.iter().filter(|t| **t != "ENUM_CONSTANT_DEF").map(|t| t.to_string()).collect(),
        Some(v) => {
            let list: Vec<String> = v.split(',').map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect();
            if let Some(bad) = list.iter().find(|t| !ACCEPTED.contains(&t.as_str())) {
                return Err(format!("tokens: {bad} is not a token this module accepts"));
            }
            list
        }
    };
    p.finish()?;
    cfg.abbreviation_as_word_in_name = Some(AbbreviationAsWordInName {
        allowed_length,
        allowed_abbreviations,
        ignore_final,
        ignore_static,
        ignore_static_final,
        ignore_overridden_methods,
        tokens,
    });
    Ok(())
}

/// `TypeName`.
pub(super) fn type_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    for a in ACCESS {
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
    let t = &mut cfg.naming.type_name;
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
    Ok(())
}

/// `MemberName`, `ConstantName`, `StaticVariableName`.
pub(super) fn member_name(cfg: &mut Config, module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    for a in ACCESS {
        p.expect_default(a, "true")?;
    }
    let default = if module == "ConstantName" { r"^[A-Z][A-Z0-9]*(_[A-Z0-9]+)*$" } else { SIMPLE_LOWER };
    let format = p.format(default)?;
    p.finish()?;
    match module {
        "MemberName" => cfg.naming.member_name = Some(format),
        "ConstantName" => cfg.naming.constant_name = Some(format),
        _ => cfg.naming.static_variable_name = Some(format),
    }
    Ok(())
}

/// `MethodName`.
pub(super) fn method_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    for a in ACCESS {
        p.expect_default(a, "true")?;
    }
    let allow_class_name = p.bool("allowClassName", false)?;
    let format = p.format(SIMPLE_LOWER)?;
    p.finish()?;
    cfg.naming.method_name = Some(MethodName { format, allow_class_name });
    Ok(())
}

/// `ParameterName`.
pub(super) fn parameter_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    p.expect_default("accessModifiers", "public, protected, package, private")?;
    let ignore_overridden = p.bool("ignoreOverridden", false)?;
    let format = p.format(SIMPLE_LOWER)?;
    p.finish()?;
    cfg.naming.parameter_name = Some(ParameterName { format, ignore_overridden });
    Ok(())
}

/// `LocalVariableName`.
pub(super) fn local_variable_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let allow_one_char_var_in_for_loop = p.bool("allowOneCharVarInForLoop", false)?;
    let format = p.format(LOWER_OR_UNDERSCORE)?;
    p.finish()?;
    cfg.naming.local_variable_name = Some(LocalVariableName { format, allow_one_char_var_in_for_loop });
    Ok(())
}

/// `LocalFinalVariableName`, `LambdaParameterName`, `PatternVariableName`.
pub(super) fn local_final_variable_name(cfg: &mut Config, module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    if module == "LocalFinalVariableName" {
        p.expect_default("tokens", "VARIABLE_DEF,PARAMETER_DEF,RESOURCE")?;
    }
    let format = p.format(LOWER_OR_UNDERSCORE)?;
    p.finish()?;
    match module {
        "LocalFinalVariableName" => cfg.naming.local_final_variable_name = Some(format),
        "LambdaParameterName" => cfg.naming.lambda_parameter_name = Some(format),
        _ => cfg.naming.pattern_variable_name = Some(format),
    }
    Ok(())
}

/// `PackageName`.
pub(super) fn package_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let format = p.format(r"^[a-z]+(\.[a-zA-Z_]\w*)*$")?;
    p.finish()?;
    cfg.naming.package_name = Some(format);
    Ok(())
}

/// `ClassTypeParameterName`, `InterfaceTypeParameterName`, `MethodTypeParameterName`, `RecordTypeParameterName`.
pub(super) fn class_type_parameter_name(cfg: &mut Config, module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let format = p.format(TYPE_PARAMETER)?;
    p.finish()?;
    match module {
        "ClassTypeParameterName" => cfg.naming.class_type_parameter_name = Some(format),
        "InterfaceTypeParameterName" => cfg.naming.interface_type_parameter_name = Some(format),
        "MethodTypeParameterName" => cfg.naming.method_type_parameter_name = Some(format),
        _ => cfg.naming.record_type_parameter_name = Some(format),
    }
    Ok(())
}

/// `CatchParameterName`.
pub(super) fn catch_parameter_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let format = p.format(r"^(e|t|ex|[a-z][a-z][a-zA-Z]+|_)$")?;
    p.finish()?;
    cfg.naming.catch_parameter_name = Some(format);
    Ok(())
}

/// `RecordComponentName`.
pub(super) fn record_component_name(cfg: &mut Config, _module: &str, mut p: Props, _id: Option<&str>) -> Result<(), String> {
    let format = p.format(SIMPLE_LOWER)?;
    p.finish()?;
    cfg.naming.record_component_name = Some(format);
    Ok(())
}
