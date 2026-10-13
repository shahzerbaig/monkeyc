impl Token {
    pub fn new(kind: TokenKind, literal: impl Into<String>) -> Self {
        Self {
            kind,
            literal: literal.into(),
        }
    }
}

// The lookup_ident() function distinguishes reserved keywords from normal identifiers. For example, let becomes TokenKind::Let, while age becomes TokenKind::Ident

pub fn lookup_ident(ident: &str) -> TokenKind {
    match ident {
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
