use std::ops::RemAssign;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use ecow::EcoString;

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i64),
    Float(f64),
    Identifier(EcoString),
    Negation(Box<Expression>),
    Binary(Box<Expression>, TokenType, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(EcoString, Expression),
    Print(Expression),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token {
                token: TokenType::EOF,
            },
        };

        parser.advance();
        parser
    }

    /// Advance to next token after successfuly consume one
    /// if next token not found or end of file reach set token to "EOF"
    fn advance(&mut self) {
        self.current_token = self.lexer.get_next_token().unwrap_or_else(|_| Token {
            token: TokenType::EOF, // If error, set token to EOF
        });
    }

    /// This is function which parse keyword and lexer into parser
    /// !TODO:
    ///    [1] MAke more robust parser
    ///    [2] Handle more types of Error while parsing
    pub fn parse_statement(&mut self) -> Option<Result<Statement, String>> {
        // Check if the current token is EOF before parsing
        if self.current_token.token == TokenType::EOF {
            return None;
        }

        let stmt = match &self.current_token.token {
            TokenType::Let => self.parse_let(),
            TokenType::Print => self.parse_print(),
            _ => return Some(Err(format!("Unexpected token: {:?}", self.current_token))),
        };

        // Advance to the next token if successfully parsed a statement
        if stmt.is_ok() {
            self.advance();
        }

        Some(stmt)
    }

    /// Parse the "let" statement
    /// E.g.
    /// ```
    ///  let x = 45;
    ///  let y = x + 12;  
    ///```
    fn parse_let(&mut self) -> Result<Statement, String> {
        // consume the "let" keyword and move forward
        self.advance();

        // collect variable and make clone of it.
        // !TODO: [1] Avoid cloning of identifier
        if let TokenType::Identifier(name) = &self.current_token.token {
            let name = name.clone();
            self.advance();

            if let TokenType::Assign = self.current_token.token {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Statement::Let(name.clone(), expr))
            } else if let TokenType::EOF = self.current_token.token {
                // If we reached EOF, stop further parsing
                Err("Unexpected EOF while parsing let statement".into())
            } else {
                Err(format!("Expected '=', found {:?}", self.current_token))
            }
        } else if let TokenType::EOF = self.current_token.token {
            // Handle EOF at the point of expecting an identifier
            Err("Unexpected EOF while parsing let statement".into())
        } else {
            Err(format!(
                "Expected identifier, found {:?}",
                self.current_token
            ))
        }
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'print'

        if let TokenType::LParen = self.current_token.token {
            self.advance();
        }
        let expr = self.parse_expression()?;

        if let TokenType::RParen = self.current_token.token {
            self.advance();
        }

        Ok(Statement::Print(expr))
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?; // Parse the left side of the expression

        while let Some(op) = self.parse_operator() {
            let right = self.parse_primary()?; // Parse the right side of the expression
            left = Expression::Binary(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current_token.token {
            TokenType::Int(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::Int(value))
            }

            TokenType::Float(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::Float(value))
            }

            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            }

            // Explicitly handle EOF here for clarity
            TokenType::EOF => Err("Unexpected end of input while parsing expression.".into()),

            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token
            )),
        }
    }

    /// Parse the statement that contains Binary or Arithmetic operator
    /// E.g.
    /// ```let x = 5 + y - 20;```
    fn parse_operator(&mut self) -> Option<TokenType> {
        match &self.current_token.token {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                let op = self.current_token.token.clone();
                self.advance();
                Some(op)
            }
            _ => None,
        }
    }
}
