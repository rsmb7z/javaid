//! The grammar handle and the per-node-kind lookup table.
//!
//! Everything about the tree-sitter grammar that rules depend on is resolved once, here: which
//! node kinds trigger which rule family, and the numeric ids of the grammar fields. Rules then
//! work with small integers, never strings, on the hot path.

use tree_sitter::{Language, Node, Parser, Tree};

/// Which tree-sitter Java grammar to parse with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum Grammar {
    /// tree-sitter/tree-sitter-java 0.23.5 (last release Dec 2024).
    Upstream,
    /// Maintained fork that parses annotations in varargs and catch types.
    #[default]
    Orchard,
}

/// What the walker does when it meets a node of a given kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    None,
    Ident,
    Import,
    Package,
    TypeDecl,
    TypeBody,
    Method,
    Field,
    LocalVar,
    Param,
    CatchParam,
    TypeParam,
    Resource,
    Lambda,
    InstanceOf,
    TypePattern,
    RecordPatternComponent,
    Catch,
    If,
    While,
    For,
    EnhancedFor,
    Do,
    Comment,
    /// Array brackets (`[]`), which sit in a type or, C-style, after a declared name.
    Dimensions,
    IntegerLiteral,
    Modifiers,
    /// An annotation, which `WhitespaceAfter` looks at when it is not a declaration's modifier.
    Annotation,
    /// The `;` that ends a statement: the only unnamed token a rule looks at.
    Semi,
    /// A `(` or `)`, which `ParenPad` looks at.
    Paren,
    /// A `switch`, which is a statement or an expression depending on where it sits.
    Switch,
}

#[derive(Debug, Clone, Copy)]
pub struct KindInfo {
    pub act: Act,
    pub is_block: bool,
    pub is_if: bool,
    /// A declaration that can own a Javadoc comment, as far as `UnusedImports` is concerned.
    pub documented: bool,
    /// A statement or declaration that `OneStatementPerLine` looks at.
    pub statement: bool,
    /// An unnamed token whose padding some rule checks: punctuation, and the keywords that take a parenthesis or a block.
    pub pad_token: bool,
}

const NONE: KindInfo = KindInfo { act: Act::None, is_block: false, is_if: false, documented: false, statement: false, pad_token: false };

/// Grammar field ids, resolved once.
pub struct Fields {
    pub body: u16,
    pub consequence: u16,
    pub alternative: u16,
    pub name: u16,
    pub parameters: u16,
    pub type_: u16,
    pub field: u16,
    pub object: u16,
    pub scope: u16,
    pub condition: u16,
    /// Kept as a non-zero id (tree-sitter's `FieldId`) because it is used to iterate *all*
    /// children carrying the field.
    pub declarator: std::num::NonZeroU16,
}

pub struct Syntax {
    pub language: Language,
    info: Vec<KindInfo>,
    pub f: Fields,
}

impl Syntax {
    pub fn new(grammar: Grammar) -> Self {
        let language: Language = match grammar {
            Grammar::Upstream => tree_sitter_java::LANGUAGE.into(),
            Grammar::Orchard => tree_sitter_java_orchard::LANGUAGE.into(),
        };
        let mut info = vec![NONE; language.node_kind_count() + 1];
        // Scan ids rather than looking names up: one name can map to several ids (aliases).
        for id in 0..language.node_kind_count() as u16 {
            if !language.node_kind_is_named(id) {
                let kind = language.node_kind_for_id(id);
                info[id as usize].pad_token = matches!(
                    kind,
                    Some(
                        "," | ";" | "." | "..." | "::" | "++" | "--" | ":" | "->" | "if" | "else" | "while" | "do" | "for" | "finally" | "return" | "yield"
                            | "catch" | "switch" | "synchronized" | "try" | "case" | "when" | "assert" | "instanceof" | "[" | "]" | "{" | "}" | "?" | "=" | "+=" | "-=" | "*="
                            | "/=" | "%=" | "<<=" | ">>=" | ">>>=" | "&=" | "|=" | "^=" | "+" | "-" | "*" | "/" | "%" | "<<" | ">>" | ">>>" | "&"
                            | "|" | "^" | "<" | ">" | "<=" | ">=" | "==" | "!=" | "&&" | "||"
                    )
                );
                match kind {
                    Some(";") => info[id as usize].act = Act::Semi,
                    Some("(" | ")") => info[id as usize].act = Act::Paren,
                    _ => {}
                }
                continue;
            }
            let Some(name) = language.node_kind_for_id(id) else { continue };
            let entry = &mut info[id as usize];
            entry.act = match name {
                "identifier" | "type_identifier" => Act::Ident,
                "import_declaration" => Act::Import,
                "package_declaration" => Act::Package,
                "class_declaration" | "interface_declaration" | "enum_declaration" | "record_declaration" | "annotation_type_declaration" => {
                    Act::TypeDecl
                }
                "class_body" | "enum_body_declarations" => Act::TypeBody,
                "method_declaration" => Act::Method,
                "field_declaration" | "constant_declaration" => Act::Field,
                "local_variable_declaration" => Act::LocalVar,
                "formal_parameter" | "spread_parameter" => Act::Param,
                "catch_formal_parameter" => Act::CatchParam,
                "type_parameter" => Act::TypeParam,
                "resource" => Act::Resource,
                "lambda_expression" => Act::Lambda,
                "instanceof_expression" => Act::InstanceOf,
                "type_pattern" => Act::TypePattern,
                "record_pattern_component" => Act::RecordPatternComponent,
                "catch_clause" => Act::Catch,
                "if_statement" => Act::If,
                "while_statement" => Act::While,
                "for_statement" => Act::For,
                "enhanced_for_statement" => Act::EnhancedFor,
                "do_statement" => Act::Do,
                "block_comment" | "line_comment" => Act::Comment,
                "dimensions" => Act::Dimensions,
                "modifiers" => Act::Modifiers,
                "annotation" | "marker_annotation" => Act::Annotation,
                "switch_expression" => Act::Switch,
                "decimal_integer_literal" | "hex_integer_literal" | "octal_integer_literal" | "binary_integer_literal" => Act::IntegerLiteral,
                _ => Act::None,
            };
            entry.statement = matches!(
                name,
                "local_variable_declaration"
                    | "field_declaration"
                    | "constant_declaration"
                    | "expression_statement"
                    | "return_statement"
                    | "throw_statement"
                    | "assert_statement"
                    | "break_statement"
                    | "continue_statement"
                    | "yield_statement"
                    | "import_declaration"
                    | "package_declaration"
                    | "annotation_type_element_declaration"
                    | "method_declaration"
                    | "class_declaration"
                    | "interface_declaration"
                    | "enum_declaration"
                    | "record_declaration"
                    | "annotation_type_declaration"
            );
            entry.is_block = name == "block";
            entry.is_if = name == "if_statement";
            // Found by probing Checkstyle: Javadoc before a type (even a local one), method,
            // constructor, field, enum constant, annotation element or the package statement counts.
            // Javadoc before a local variable, an initializer block or a record component does not.
            entry.documented = matches!(
                name,
                "class_declaration"
                    | "interface_declaration"
                    | "enum_declaration"
                    | "record_declaration"
                    | "annotation_type_declaration"
                    | "method_declaration"
                    | "constructor_declaration"
                    | "compact_constructor_declaration"
                    | "field_declaration"
                    | "constant_declaration"
                    | "enum_constant"
                    | "annotation_type_element_declaration"
                    | "package_declaration"
            );
        }
        let field = |n: &str| language.field_id_for_name(n).unwrap_or_else(|| panic!("grammar has no field {n}")).get();
        let f = Fields {
            body: field("body"),
            consequence: field("consequence"),
            alternative: field("alternative"),
            name: field("name"),
            parameters: field("parameters"),
            type_: field("type"),
            field: field("field"),
            object: field("object"),
            scope: field("scope"),
            condition: field("condition"),
            declarator: language.field_id_for_name("declarator").expect("grammar has no field declarator"),
        };
        Syntax { language, info, f }
    }

    pub fn new_parser(&self) -> Parser {
        let mut p = Parser::new();
        p.set_language(&self.language).expect("tree-sitter-java ABI mismatch");
        p
    }

    pub fn kind(&self, node: Node) -> KindInfo {
        self.info.get(node.kind_id() as usize).copied().unwrap_or(NONE)
    }

    pub fn act(&self, node: Node) -> Act {
        self.kind(node).act
    }
}

/// First ERROR or MISSING node in document order, if the tree has any.
pub fn first_syntax_error(tree: &Tree) -> Option<Node<'_>> {
    fn go(n: Node<'_>) -> Option<Node<'_>> {
        if n.is_error() || n.is_missing() {
            return Some(n);
        }
        if !n.has_error() {
            return None;
        }
        let mut cur = n.walk();
        n.children(&mut cur).find_map(go)
    }
    go(tree.root_node())
}

/// Visit every node without running any rule: measures raw traversal cost.
pub fn walk_only(tree: &Tree) -> usize {
    let mut cursor = tree.walk();
    let mut n = 0usize;
    'outer: loop {
        n += 1;
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break 'outer;
            }
        }
    }
    n
}
