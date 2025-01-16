use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar(char),
    EndOfInput,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar(c) => write!(f, "Unexpected character: {}", c),
            LexerError::EndOfInput => write!(f, "End of input"),
        }
    }
}
