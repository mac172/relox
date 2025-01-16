use ecow::EcoString;

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i64),
    Float(f64),
    StringLiteral(String),
    Plus,
    Mul,
    Minus,
    Slash,
    LParen,
    RParen,
    Identifier(EcoString),
    Equal,
    Assign,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    And,
    Or,
    EOF,
}
