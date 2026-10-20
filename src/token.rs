use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Class,
    Const,
    Variable,
    Number,
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Ident,
}

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: impl Into<String>) -> Self {
        Self {
            kind,
            literal: literal.into(),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Class => "Class",
            TokenKind::Const => "Const",
            TokenKind::Variable => "Variable",
            TokenKind::Number => "Number",
            TokenKind::Let => "Let",
            TokenKind::Function => "Function",
            TokenKind::True => "True",
            TokenKind::False => "False",
            TokenKind::If => "If",
            TokenKind::Else => "Else",
            TokenKind::Return => "Return",
            TokenKind::Ident => "Ident",
        };
        f.write_str(s)
    }
}
// The lookup_ident() function distinguishes reserved keywords from normal identifiers. For example, let becomes TokenKind::Let, while age becomes TokenKind::Ident

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
        "troop" => TokenKind::Class,
        "silverback" => TokenKind::Const,
        "monkey" => TokenKind::Variable,
        "pup" => TokenKind::Number,
        "let" => TokenKind::Let,
        "fn" => TokenKind::Function,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident,
    }
}
