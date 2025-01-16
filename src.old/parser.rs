use crate::error::LexerError;
use crate::lexer::Lexer;
use crate::token::Token;
use ecow::EcoString;

pub struct Parser<'a> {
    lexer: Lexer<'a>, // Hold an owned instance of Lexer
    current_token: Token,
}

#[derive(Debug)]
pub enum Expr {
    Identifier(EcoString),
}

#[derive(Debug)]
pub struct IfStmt {
    if_block: Vec<Expr>,
    then_block: Vec<Expr>,
    else_block: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self, LexerError> {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF, // Initialize with a default token
        };
        parser.current_token = parser.lexer.next_token()?; // Get the first token
        Ok(parser)
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next_token().unwrap();
    }

    pub fn expect_token(&mut self, token: Token) -> Result<(), String> {
        if token == self.current_token {
            self.advance();
            Ok(())
        } else {
            Err("Error while checking Token ".to_string())
        }
    }

    //fn factor(&mut self)
}
