// src/lexer.rs

//#[derive(Debug, PartialEq)]
pub enum CToken {
    // Keywords
    Auto,
    Break,
    Case,
    Char(Option<char>),
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float(Option<f64>),
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,

    // Identifiers
    Identifier(String),

    // Constants
    Integer(i64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    Amp,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    Inc,
    Dec,
    StarPointer,
    AmpAddress,
    Comma,

    // Punctuation
    Semicolon,
    Dot,
    Arrow,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    DoubleColon,
    QuestionMark,
    ExclamationMark,
    Backslash,

    // Preprocessor directives
    HashInclude,
    HashDefine,
    HashIfDef,
    HashIf,
    HashElse,
    HashElif,
    HashEndif,
    HashUndef,
    HashPragma,

    // Comments
    SingleLineComment(String),
    MultiLineComment(String),

    // End of file (EOF)
    EOF,
}
