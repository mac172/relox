use ecow::EcoString;

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

#[derive(Debug)]
pub enum ParserError {
    ExpectIdentifier,
    ExpectOperator(EcoString),
    UnexpectedToken(Option<TokenType>),
    EndOfInput,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.get_next_token().ok();
        let peek_token = lexer.get_next_token().ok();
        let mut parser = Parser {
            lexer,
            current_token,
            peek_token,
        };

        parser.advance_token();
        parser.advance_token();

        parser
    }

    pub fn advance_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.get_next_token().ok();
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut program = Vec::new();

        while self.current_token.is_some() {
            let stmt = self.parse_statement()?;
            program.push(stmt);
            self.advance_token();
        }

        Ok(program)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.as_ref().map(|t| &t.token) {
            Some(TokenType::Let) => self.parse_let(),
            Some(TokenType::Return) => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, ParserError> {
        self.advance_token(); // consume 'let'

        // Extract the identifier as a String
        let identifier = match self.current_token.as_ref().map(|t| &t.token) {
            Some(TokenType::Identifier(name)) => name.clone(),
            _ => return Err(ParserError::ExpectIdentifier),
        };

        self.advance_token(); // consume '='
        if self.current_token.as_ref().map(|t| &t.token) != Some(&TokenType::Assign) {
            return Err(ParserError::ExpectOperator(EcoString::from("=")));
        }

        self.advance_token();
        let expr = self.parse_expression()?;

        if let Some(TokenType::Semicolon) = self.peek_token.as_ref().map(|t| &t.token) {
            self.advance_token(); // Optionally consume the semicolon
        }

        Ok(Statement::Let {
            name: identifier,
            value: expr,
        })
    }

    fn parse_return(&mut self) -> Result<Statement, ParserError> {
        self.advance_token();

        let expr = self.parse_expression()?;

        // Optionally consume the semicolon if it's there
        if let Some(TokenType::Semicolon) = self.peek_token.as_ref().map(|t| &t.token) {
            self.advance_token();
        }

        Ok(Statement::Return(expr)) // Return the parsed Return statement
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression()?;

        if let Some(TokenType::Semicolon) = self.peek_token.as_ref().map(|t| &t.token) {
            self.advance_token(); // Optionally consume the semicolon
        }

        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let t = self.current_token.as_ref().map(|t| &t.token);
        match t {
            Some(TokenType::Identifier(name)) => Ok(Expression::Identifier(name.clone())),
            Some(TokenType::Int(value)) => Ok(Expression::IntegerLiteral(*value)),
            Some(TokenType::Float(value)) => Ok(Expression::FloatLiteral(*value)),
            _ => Err(ParserError::UnexpectedToken(t.cloned())),
        }
    }

    pub fn parse_statement_from_token(&mut self, token: Token) -> Result<Statement, ParserError> {
        // Use the token to guide parsing
        // You may need to store this token and potentially look ahead for multi-token statements

        // Example placeholder logic (implement based on actual parsing rules):
        match token.token {
            TokenType::Let => self.parse_let(),
            TokenType::Return => self.parse_return(),
            _ => Err(ParserError::UnexpectedToken(Some(token.token))),
        }
    }
}
