use ecow::EcoString;
use std::iter::Peekable;
use std::str::Chars;

use crate::lexer_error::LexerError;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            position: 0,
            read_position: 0,
            current_char: None,
        };

        lexer.advance();
        lexer
    }

    pub fn advance(&mut self) {
        self.current_char = self.input.next();
        self.position += 1;
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn single_char_token(&mut self, token: TokenType) -> Result<Token, LexerError> {
        self.advance();
        Ok(Token { token })
    }

    fn double_char_token(
        &mut self,
        next_char: Option<&char>,
        double_token: TokenType,
        single_token: TokenType,
    ) -> Result<Token, LexerError> {
        if next_char == self.peek_char() {
            self.advance();
            self.advance();
            return Ok(Token {
                token: double_token,
            });
        } else {
            self.advance();
            return Ok(Token {
                token: single_token,
            });
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        // If current_char is None, end of input reached, return EOF token
        if self.current_char.is_none() {
            return Ok(Token {
                token: TokenType::EOF,
            });
        }

        while let Some(c) = self.current_char {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }

                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.advance(); // consume '='
                        self.advance(); // consume next '='
                        return Ok(Token {
                            token: TokenType::Equal,
                        });
                    } else {
                        self.advance();
                        return Ok(Token {
                            token: TokenType::Assign,
                        });
                    }
                }

                '!' => {
                    return self.double_char_token(
                        Some(&'='),
                        TokenType::NotEqual,
                        TokenType::Bang,
                    );
                }

                '+' => {
                    return self.single_char_token(TokenType::Plus);
                }

                '-' => {
                    return self.single_char_token(TokenType::Minus);
                }

                '*' => {
                    return self.single_char_token(TokenType::Star);
                }

                '/' => {
                    return self.single_char_token(TokenType::Slash);
                }

                '>' => {
                    return self.double_char_token(Some(&'='), TokenType::GtEq, TokenType::Gt);
                }

                '<' => {
                    return self.double_char_token(Some(&'='), TokenType::LtEq, TokenType::Lt);
                }

                ',' => {
                    return self.single_char_token(TokenType::Comma);
                }

                ';' => {
                    self.advance();
                    return Ok(Token {
                        token: TokenType::Semicolon,
                    });
                }

                '(' => {
                    return self.single_char_token(TokenType::LParen);
                }

                ')' => {
                    return self.single_char_token(TokenType::RParen);
                }

                '{' => {
                    return self.single_char_token(TokenType::LBrace);
                }

                '}' => {
                    return self.single_char_token(TokenType::RBrace);
                }

                '0'..='9' => {
                    let (num, is_float) = self.is_number();
                    if is_float {
                        let value = TokenType::Float(num.parse::<f64>().unwrap());
                        return Ok(Token { token: value });
                    } else {
                        let value = num.parse::<i64>().unwrap();
                        return Ok(Token {
                            token: TokenType::Int(value),
                        });
                    }
                }

                'a'..='z' | 'A'..='Z' => {
                    return self.lex_identifier();
                }

                _ => return Err(LexerError::UnexpectedChar(c)),
                //panic!("Unknown Character: {}", c);
            }
        }
        // Ok(Token {
        //     token: TokenType::EOF,
        // })
        Ok(Token {
            token: TokenType::EOF,
        })
        // Err(LexerError::EndOfInput)
    }

    fn is_number(&mut self) -> (EcoString, bool) {
        let mut num_str = EcoString::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                num_str.push(c);
            } else if c == '.' && !is_float {
                is_float = true;
                num_str.push(c);
            } else {
                // println!("Unknown Digit {}", c);
                break;
            }
            self.advance();
        }
        (num_str, is_float)
    }

    fn lex_identifier(&mut self) -> Result<Token, LexerError> {
        let mut identifier = EcoString::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it matches a keyword or treat as generic identifier
        let token_type = match identifier.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "print" => TokenType::Print,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "fn" => TokenType::Function,
            "return" => TokenType::Return,
            _ => TokenType::Identifier(identifier.clone()),
        };

        Ok(Token { token: token_type })
    }
}
