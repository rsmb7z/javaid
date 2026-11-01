//! Rule families and the dispatch from node kind to rule.

pub mod blocks;
pub mod coding;
pub mod comments;
pub mod fallthrough;
pub mod imports;
pub mod misc;
pub mod modifiers;
pub mod naming;
pub mod statements;
pub mod text;
pub mod util;

#[cfg(test)]
mod tests;

use crate::engine::Ctx;
use crate::syntax::Act;
use tree_sitter::Node;

/// Per-file state that rules carry across nodes.
#[derive(Default)]
pub struct State<'a> {
    pub imports: imports::State<'a>,
    pub statements: statements::State,
}

/// Called for every node in document order. Returns whether to descend into its children.
pub fn visit<'a>(cx: &mut Ctx<'a>, st: &mut State<'a>, node: Node) -> bool {
    imports::leave_scopes(&mut st.imports, node);
    let kind = cx.syn.kind(node);
    if kind.documented {
        imports::on_documented(cx, &mut st.imports, node);
    }
    if kind.statement {
        statements::on_statement(cx, &mut st.statements, node);
    }
    match kind.act {
        Act::None => {}
        Act::Comment => {
            imports::on_comment(cx, &mut st.imports, node);
            comments::on_comment(cx, node);
        }
        Act::Ident => imports::on_ident(cx, &mut st.imports, node),
        Act::Import => {
            imports::on_star_import(cx, node);
            imports::on_import(cx, &mut st.imports, node);
            return false;
        }
        Act::Package => {
            naming::on_package(cx, node);
            imports::on_package(cx, &mut st.imports, node);
            return false;
        }
        Act::TypeDecl => naming::on_type_decl(cx, node),
        Act::TypeBody => {
            coding::on_type_body(cx, node);
            imports::on_type_body(cx, &mut st.imports, node);
        }
        Act::Method => naming::on_method(cx, node),
        Act::Field => {
            naming::on_field(cx, node);
            coding::on_declaration(cx, node);
        }
        Act::LocalVar => {
            naming::on_local_var(cx, node);
            coding::on_declaration(cx, node);
        }
        Act::Param => naming::on_param(cx, node),
        Act::CatchParam => naming::on_catch_param(cx, node),
        Act::TypeParam => naming::on_type_param(cx, node),
        Act::Resource => {
            naming::on_resource(cx, node);
            statements::on_resource(cx, node);
        }
        Act::Semi => statements::on_semicolon(cx, &mut st.statements, node),
        Act::Lambda => naming::on_lambda(cx, node),
        Act::InstanceOf => naming::on_instanceof(cx, node),
        Act::TypePattern => naming::on_type_pattern(cx, node),
        Act::RecordPatternComponent => naming::on_record_pattern_component(cx, node),
        Act::Catch => blocks::on_catch(cx, node),
        Act::If => blocks::on_if(cx, node),
        Act::While => blocks::on_loop(cx, node, "while"),
        Act::For => blocks::on_loop(cx, node, "for"),
        Act::EnhancedFor => {
            blocks::on_loop(cx, node, "for");
            naming::on_enhanced_for(cx, node);
        }
        Act::Do => blocks::on_loop(cx, node, "do"),
        Act::Dimensions => misc::on_dimensions(cx, node),
        Act::IntegerLiteral => misc::on_integer_literal(cx, node),
        Act::Modifiers => modifiers::on_modifiers(cx, node),
        Act::Switch => {
            coding::on_switch(cx, node);
            fallthrough::on_switch(cx, node);
        }
    }
    true
}

/// Called once after the walk, for rules that decide at the end of the file.
pub fn finish<'a>(cx: &mut Ctx<'a>, st: &mut State<'a>) {
    imports::finish(cx, &mut st.imports);
}
