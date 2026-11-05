//! Rule families and the dispatch from node kind to rule.

pub mod blocks;
pub mod coding;
pub mod comments;
pub mod declorder;
pub mod descendant;
pub mod design;
pub mod extension;
pub mod fallthrough;
pub mod gs;
pub mod hidden;
pub mod importcheck;
pub mod indentation;
pub mod imports;
pub mod javadoc;
pub mod javadoc_style;
pub mod javadoc_tags;
pub mod javadoc_tree;
pub mod javadoc_tree_checks;
pub mod magic;
pub mod misc;
pub mod modifiers;
pub mod abbr;
pub mod commentfilter;
pub mod customimport;
pub mod unicode;
pub mod cs;
pub mod textblock;
pub mod googlerightcurly;
pub mod emptyline;
pub mod googlenames;
pub mod naming;
pub mod pad;
pub mod redundant;
pub mod size;
pub mod statements;
pub mod text;
pub mod util;
pub mod whitespace;
pub mod wrap;
pub mod vdud;
pub mod commentsindent;
pub mod csast_checks;
pub mod xpathfilter;

#[cfg(test)]
mod tests;

use crate::engine::Ctx;
use crate::syntax::KindExt;
use crate::syntax::Act;
use tree_sitter::Node;

/// Per-file state that rules carry across nodes.
#[derive(Default)]
pub struct State<'a> {
    pub imports: imports::State<'a>,
    pub statements: statements::State,
    pub design: design::State,
    pub declorder: declorder::State,
    pub importcheck: importcheck::State,
    pub unicode: unicode::State,
    pub commentfilter: commentfilter::State,
    pub customimport: customimport::State,
    /// End of the `package` or `import` declaration being walked into. Only the padding rules look
    /// inside one; to every other rule it is a leaf, as before.
    pub inert_end: usize,
}

/// Called for every node in document order. Returns whether to descend into its children.
pub fn visit<'a>(cx: &mut Ctx<'a>, st: &mut State<'a>, node: Node) -> bool {
    let kind = cx.syn.kind(node);
    cx.node_id = node.id();
    cx.kind_name = if kind.name.is_empty() { node.kind_s() } else { kind.name };
    if node.start_byte() < st.inert_end {
        match kind.act {
            Act::Paren => {
                whitespace::on_paren(cx, node);
                wrap::on_paren(cx, node);
            }
            Act::Annotation => whitespace::on_annotation(cx, node),
            Act::Comment => whitespace::on_comment(cx, node),
            Act::Semi => misc::on_empty_statement(cx, node),
            _ => {}
        }
        if kind.pad_token {
            whitespace::on_token(cx, node);
            wrap::on_token(cx, node);
        }
        return true;
    }
    imports::leave_scopes(&mut st.imports, node);
    declorder::on_node(cx, &mut st.declorder, node);
    gs::on_node(cx, node);
    match cx.kind_name {
        "enum_constant" => javadoc::on_variable(cx, node),
        "annotation_type_element_declaration" => {
            javadoc::on_method(cx, node);
            javadoc::on_javadoc_method(cx, node);
        }
        _ => {}
    }
    if kind.pad_token {
        whitespace::on_token(cx, node);
        wrap::on_token(cx, node);
    }
    if kind.documented {
        imports::on_documented(cx, &mut st.imports, node);
    }
    if kind.statement {
        statements::on_statement(cx, &mut st.statements, node);
    }
    match kind.act {
        Act::None => {}
        Act::StringLiteral => unicode::on_literal(cx, &mut st.unicode, node),
        Act::Comment => {
            unicode::on_comment(cx, &mut st.unicode, node);
            commentfilter::on_comment(cx, &mut st.commentfilter, node);
            imports::on_comment(cx, &mut st.imports, node);
            javadoc::on_comment(cx, node);
            comments::on_comment(cx, node);
            whitespace::on_comment(cx, node);
        }
        Act::Ident => imports::on_ident(cx, &mut st.imports, node),
        Act::Import => {
            importcheck::on_import(cx, &mut st.importcheck, node);
            customimport::on_import(cx, &mut st.customimport, node);
            imports::on_star_import(cx, node);
            imports::on_import(cx, &mut st.imports, node);
            return enter_for_padding(cx, st, node);
        }
        Act::Package => {
            design::on_package(cx, &mut st.design, node);
            importcheck::on_package(cx, &mut st.importcheck, node);
            customimport::on_package(cx, &mut st.customimport, node);
            naming::on_package(cx, node);
            imports::on_package(cx, &mut st.imports, node);
            return enter_for_padding(cx, st, node);
        }
        Act::TypeDecl => {
            naming::on_type_decl(cx, node);
            abbr::on_type_decl(cx, node);
            javadoc::on_type(cx, node);
            javadoc::on_missing_type(cx, node);
            design::on_type_decl(cx, &mut st.design, node);
            importcheck::on_class_decl(cx, &mut st.importcheck, node);
            design::on_class(cx, node);
            if node.kind_s() == "interface_declaration" {
                design::on_interface(cx, node);
            }
        }
        Act::New => {
            design::on_new(cx, &mut st.design, node);
            importcheck::on_new(cx, &mut st.importcheck, node);
        }
        Act::TypeBody => {
            coding::on_type_body(cx, node);
            imports::on_type_body(cx, &mut st.imports, node);
        }
        Act::Method => {
            naming::on_method(cx, node);
            abbr::on_method(cx, node);
            googlenames::on_method(cx, node);
            size::on_method_like(cx, node);
            extension::on_method(cx, node);
            javadoc::on_method(cx, node);
            javadoc::on_javadoc_method(cx, node);
        }
        Act::Constructor => {
            size::on_method_like(cx, node);
            design::on_constructor(cx, &mut st.design, node);
            javadoc::on_method(cx, node);
            javadoc::on_javadoc_method(cx, node);
        }
        Act::Field => {
            naming::on_field(cx, node);
            abbr::on_field(cx, node);
            googlenames::on_field(cx, node);
            design::on_visibility(cx, node);
            javadoc::on_variable(cx, node);
            coding::on_declaration(cx, node);
        }
        Act::LocalVar => {
            naming::on_local_var(cx, node);
            abbr::on_local_var(cx, node);
            hidden::on_local(cx, node);
            coding::on_declaration(cx, node);
        }
        Act::Param => {
            naming::on_param(cx, node);
            abbr::on_param(cx, node);
            hidden::on_param(cx, node);
            design::on_final_parameter(cx, node);
        }
        Act::CatchParam => {
            naming::on_catch_param(cx, node);
            abbr::on_param(cx, node);
            hidden::on_param(cx, node);
            design::on_final_parameter(cx, node);
        }
        Act::TypeParam => naming::on_type_param(cx, node),
        Act::Resource => {
            naming::on_resource(cx, node);
            statements::on_resource(cx, node);
        }
        Act::Paren => {
            whitespace::on_paren(cx, node);
            wrap::on_paren(cx, node);
        }
        Act::Semi => {
            statements::on_semicolon(cx, &mut st.statements, node);
            misc::on_empty_statement(cx, node);
        }
        Act::Lambda => {
            naming::on_lambda(cx, node);
            abbr::on_lambda(cx, node);
            hidden::on_lambda(cx, node);
        }
        Act::InstanceOf => {
            naming::on_instanceof(cx, node);
            abbr::on_instanceof(cx, node);
            hidden::on_instanceof(cx, node);
        }
        Act::TypePattern => {
            naming::on_type_pattern(cx, node);
            abbr::on_pattern(cx, node);
            hidden::on_pattern(cx, node);
        }
        Act::RecordPatternComponent => {
            naming::on_record_pattern_component(cx, node);
            abbr::on_pattern(cx, node);
            hidden::on_pattern(cx, node);
        }
        Act::Catch => blocks::on_catch(cx, node),
        Act::If => {
            blocks::on_if(cx, node);
            coding::on_simplify_return(cx, node);
        }
        Act::While => blocks::on_loop(cx, node, "while"),
        Act::For => {
            blocks::on_loop(cx, node, "for");
            descendant::on_for(cx, node);
        }
        Act::EnhancedFor => {
            design::on_final_parameter(cx, node);
            hidden::on_enhanced_for(cx, node);
            blocks::on_loop(cx, node, "for");
            naming::on_enhanced_for(cx, node);
            abbr::on_enhanced_for(cx, node);
        }
        Act::Do => blocks::on_loop(cx, node, "do"),
        Act::Dimensions => misc::on_dimensions(cx, node),
        Act::IntegerLiteral => {
            misc::on_integer_literal(cx, node);
            magic::on_number(cx, node);
        }
        Act::FloatLiteral => magic::on_number(cx, node),
        Act::Modifiers => {
            modifiers::on_modifiers(cx, node);
            redundant::on_modifiers(cx, node);
        }
        Act::Annotation => whitespace::on_annotation(cx, node),
        Act::Block => blocks::on_block(cx, node),
        Act::Assignment => coding::on_assignment(cx, node),
        Act::BooleanLiteral => coding::on_boolean_literal(cx, node),
        Act::Switch => {
            coding::on_switch(cx, node);
            fallthrough::on_switch(cx, node);
        }
    }
    true
}

/// Imports and the package declaration are leaves to most rules. The padding rules still need
/// their tokens (`import a . b ;`) and the annotations on a package, so descend when one is on.
fn enter_for_padding(cx: &Ctx, st: &mut State, node: Node) -> bool {
    if cx.cfg.looks_inside_declarations() {
        st.inert_end = node.end_byte();
        return true;
    }
    false
}

/// Called once after the walk, for rules that decide at the end of the file.
pub fn finish<'a>(cx: &mut Ctx<'a>, st: &mut State<'a>) {
    imports::finish(cx, &mut st.imports);
    design::finish(cx, &mut st.design);
    importcheck::finish(cx, &mut st.importcheck);
    unicode::finish(cx, &mut st.unicode);
    customimport::finish(cx, &mut st.customimport);
    commentfilter::finish(cx, &mut st.commentfilter);
}
