//! Checkstyle's token names, as the `tokens` property of the whitespace modules spells them.

macro_rules! tokens {
    ($($variant:ident => $name:literal,)*) => {
        /// A Checkstyle token type.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub enum Token { $($variant),* }

        const ALL: &[(Token, &str)] = &[$((Token::$variant, $name)),*];
    };
}

tokens! {
    Annotation => "ANNOTATION",
    Annotations => "ANNOTATIONS",
    AnnotationFieldDef => "ANNOTATION_FIELD_DEF",
    ArrayDeclarator => "ARRAY_DECLARATOR",
    ArrayInit => "ARRAY_INIT",
    Assign => "ASSIGN",
    Band => "BAND",
    BandAssign => "BAND_ASSIGN",
    Bor => "BOR",
    BorAssign => "BOR_ASSIGN",
    Bsr => "BSR",
    BsrAssign => "BSR_ASSIGN",
    Bxor => "BXOR",
    BxorAssign => "BXOR_ASSIGN",
    Colon => "COLON",
    Comma => "COMMA",
    CompactCtorDef => "COMPACT_CTOR_DEF",
    CtorCall => "CTOR_CALL",
    CtorDef => "CTOR_DEF",
    Dec => "DEC",
    Div => "DIV",
    DivAssign => "DIV_ASSIGN",
    DoWhile => "DO_WHILE",
    Dot => "DOT",
    Ellipsis => "ELLIPSIS",
    EnumConstantDef => "ENUM_CONSTANT_DEF",
    Equal => "EQUAL",
    Expr => "EXPR",
    Ge => "GE",
    GenericEnd => "GENERIC_END",
    GenericStart => "GENERIC_START",
    Gt => "GT",
    Inc => "INC",
    IndexOp => "INDEX_OP",
    LabeledStat => "LABELED_STAT",
    Lambda => "LAMBDA",
    Land => "LAND",
    Lparen => "LPAREN",
    Rparen => "RPAREN",
    Le => "LE",
    LiteralAssert => "LITERAL_ASSERT",
    LiteralCase => "LITERAL_CASE",
    LiteralCatch => "LITERAL_CATCH",
    LiteralDefault => "LITERAL_DEFAULT",
    LiteralDo => "LITERAL_DO",
    LiteralElse => "LITERAL_ELSE",
    LiteralFinally => "LITERAL_FINALLY",
    LiteralFor => "LITERAL_FOR",
    LiteralIf => "LITERAL_IF",
    LiteralNew => "LITERAL_NEW",
    LiteralReturn => "LITERAL_RETURN",
    LiteralSwitch => "LITERAL_SWITCH",
    LiteralSynchronized => "LITERAL_SYNCHRONIZED",
    LiteralThrows => "LITERAL_THROWS",
    LiteralTry => "LITERAL_TRY",
    LiteralWhile => "LITERAL_WHILE",
    LiteralYield => "LITERAL_YIELD",
    Lnot => "LNOT",
    Lor => "LOR",
    Lt => "LT",
    Minus => "MINUS",
    MinusAssign => "MINUS_ASSIGN",
    MethodCall => "METHOD_CALL",
    MethodDef => "METHOD_DEF",
    MethodRef => "METHOD_REF",
    Mod => "MOD",
    ModAssign => "MOD_ASSIGN",
    NotEqual => "NOT_EQUAL",
    Plus => "PLUS",
    PlusAssign => "PLUS_ASSIGN",
    PostDec => "POST_DEC",
    PostInc => "POST_INC",
    Question => "QUESTION",
    Rbrack => "RBRACK",
    RecordDef => "RECORD_DEF",
    RecordPatternDef => "RECORD_PATTERN_DEF",
    ResourceSpecification => "RESOURCE_SPECIFICATION",
    Semi => "SEMI",
    SingleLineComment => "SINGLE_LINE_COMMENT",
    Sl => "SL",
    SlAssign => "SL_ASSIGN",
    Sr => "SR",
    SrAssign => "SR_ASSIGN",
    Star => "STAR",
    StarAssign => "STAR_ASSIGN",
    StaticInit => "STATIC_INIT",
    SwitchRule => "SWITCH_RULE",
    SuperCtorCall => "SUPER_CTOR_CALL",
    Typecast => "TYPECAST",
    TypeExtensionAnd => "TYPE_EXTENSION_AND",
    UnaryMinus => "UNARY_MINUS",
    UnaryPlus => "UNARY_PLUS",
    InterfaceDef => "INTERFACE_DEF",
    ClassDef => "CLASS_DEF",
    EnumDef => "ENUM_DEF",
    AnnotationDef => "ANNOTATION_DEF",
    Objblock => "OBJBLOCK",
    Slist => "SLIST",
    LiteralInstanceof => "LITERAL_INSTANCEOF",
    LiteralWhen => "LITERAL_WHEN",
    Lcurly => "LCURLY",
    Rcurly => "RCURLY",
    WildcardType => "WILDCARD_TYPE",
}

const _: () = assert!(ALL.len() <= 128, "TokenSet is a u128 bit set");

impl Token {
    pub fn from_name(name: &str) -> Option<Token> {
        ALL.iter().find(|(_, n)| *n == name).map(|(t, _)| *t)
    }
}

/// A set of tokens, as written in `tokens=A, B, C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TokenSet(u128);

impl TokenSet {
    pub fn of(tokens: &[Token]) -> TokenSet {
        TokenSet(tokens.iter().fold(0, |acc, t| acc | 1 << *t as u8))
    }

    pub fn is_subset_of(&self, other: TokenSet) -> bool {
        self.0 & !other.0 == 0
    }

    pub fn contains(&self, t: Token) -> bool {
        self.0 >> t as u8 & 1 == 1
    }
}

/// Parses a `tokens` property. The error is the first name javaid does not know.
pub fn parse_tokens(list: &str) -> Result<TokenSet, String> {
    let mut set = TokenSet::default();
    for name in list.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        let t = Token::from_name(name).ok_or_else(|| name.to_string())?;
        set.0 |= 1 << t as u8;
    }
    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_list_and_names_the_unknown() {
        let set = parse_tokens("COMMA, SEMI").unwrap();
        assert!(set.contains(Token::Comma) && set.contains(Token::Semi) && !set.contains(Token::Assign));
        assert_eq!(parse_tokens("COMMA,NOPE").unwrap_err(), "NOPE");
    }
}
