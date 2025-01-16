use ecow::EcoString;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Int(i64),
    Float(f64),
    Identifier(EcoString),

    // Operators
    Assign,
    Bang,
    Plus,
    Minus,
    Star,
    Slash,

    Lt,
    Gt,
    LtEq,
    GtEq,
    Equal,
    NotEqual,

    // Delimeters
    Semicolon,
    RParen,
    LParen,
    RBrace,
    LBrace,
    Comma,

    // Keywords
    Let,
    Print,
    If,
    Else,
    Return,
    Function,
    True,
    False,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
}
