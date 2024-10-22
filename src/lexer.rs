use crate::error::LexerError;
use crate::Token;
use ecow::EcoString;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    input_str: &'a EcoString,
    postion: usize,
    read_position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a EcoString) -> Self {
        let mut lexer = Lexer {
            input: source.chars().peekable(),
            current_char: None,
            input_str: source,
            postion: 0,
            line: 1,
            column: 0,
            read_position: 0,
        };

        lexer.advance();
        lexer
    }

    // next character in input
    pub fn advance(&mut self) {
        self.current_char = self.input.next();
        self.postion += 1;
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        } else {
            self.current_char = None;
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    // handle the number from the input source code
    fn number(&mut self) -> (EcoString, bool) {
        let mut num_str = EcoString::new();
        let mut is_float = false;

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                num_str.push(c);
            } else if c == '.' && !is_float {
                is_float = true;
                num_str.push(c);
            } else {
                break;
            }
            self.advance();
        }
        (num_str, is_float)
    }

    fn identifier(&mut self) -> Token {
        let start_pos = self.postion - 1;

        while let Some(c) = self.current_char {
            if c.is_alphabetic() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident_str = &self.input_str[start_pos..self.postion - 1];
        Token::Identifier(EcoString::from(ident_str))
    }

    // get next token from input
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        self.skip_single_comment();
        self.skip_multiline_comment();

        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                    continue;
                }
                '+' => {
                    self.advance();
                    return Ok(Token::Plus);
                }
                '-' => {
                    self.advance();
                    return Ok(Token::Minus);
                }
                '*' => {
                    self.advance();
                    return Ok(Token::Mul);
                }
                '/' => {
                    self.advance();
                    return Ok(Token::Slash);
                }
                '(' => {
                    self.advance();
                    return Ok(Token::LParen);
                }
                ')' => {
                    self.advance();
                    return Ok(Token::RParen);
                }
                '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::Equal);
                    } else {
                        return Ok(Token::Assign);
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::GreaterThanEqual);
                    } else {
                        return Ok(Token::GreaterThan);
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::LessThanEqual);
                    } else {
                        return Ok(Token::LessThan);
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        return Ok(Token::NotEqual);
                    }
                }
                '&' => {
                    self.advance();
                    if self.current_char == Some('&') {
                        self.advance();
                        return Ok(Token::And);
                    }
                    return Ok(Token::EOF);
                }
                '|' => {
                    self.advance();
                    if self.current_char == Some('|') {
                        self.advance();
                        return Ok(Token::Or);
                    }
                    return Ok(Token::EOF);
                }
                '0'..='9' => {
                    let (num, is_float) = self.number();
                    let line_content = self.get_current_line();
                    if is_float {
                        return Ok(Token::Float(num.parse::<f64>().map_err(|_| {
                            LexerError::new(
                                self.line,
                                self.column,
                                "invalid float literal".to_string(),
                                None,
                                line_content,
                            )
                        })?));
                    } else {
                        return Ok(Token::Int(num.parse::<i64>().map_err(|_| {
                            LexerError::new(
                                self.line,
                                self.column,
                                "Invalid int literal".to_string(),
                                None,
                                line_content,
                            )
                        })?));
                    }
                }
                'a'..='z' | 'A'..='Z' => return Ok(self.identifier()),
                '"' => {
                    let str_litr = self.parse_string_literal();
                    return Ok(Token::StringLiteral(str_litr));
                }
                _ => {
                    // Unknown char, return an error
                    let msg = format!("Unexpected character '{}'", c);
                    let hint = Some("Check for typos or invalid character".to_string());
                    let line_content = self.get_current_line();
                    return Err(LexerError::new(
                        self.line,
                        self.column,
                        msg,
                        hint,
                        line_content,
                    ));
                    //self.advance();
                }
            }
            self.skip_whitespace();
        }
        Ok(Token::EOF)
    }

    /*
    pub fn string_literal(&mut self) -> Token {
        self.advance(); // skip opening quote
        let start_pos = self.postion - 1;

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // skip the closing quote
                let string_litr = &self.input_str[start_pos..self.postion - 1];
                return Token::StringLiteral(EcoString::from(string_litr));
            }
            self.advance();
        }
        Token::EOF
    }*/

    fn parse_string_literal(&mut self) -> String {
        let mut string_value = String::new();
        self.advance(); // skip opening quote

        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(escape_char) = self.current_char {
                    match escape_char {
                        'n' => string_value.push('\n'),
                        't' => string_value.push('\t'),
                        '\\' => string_value.push('\\'),
                        '"' => string_value.push('"'),
                        _ => string_value.push(escape_char),
                    }
                } else {
                    string_value.push(c);
                }
                self.advance();
            }
        }
        self.advance();
        string_value
    }

    pub fn skip_single_comment(&mut self) {
        if self.current_char == Some('/') && self.peek() == Some(&'/') {
            while let Some(c) = self.current_char {
                if c == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    // Skip over multi-line comments
    fn skip_multiline_comment(&mut self) {
        if self.current_char == Some('/') && self.peek() == Some(&'*') {
            self.advance(); // Skip the '/'
            self.advance(); // Skip the '*'

            // Continue advancing until we find '*/'
            while let Some(c) = self.current_char {
                if c == '*' && self.peek() == Some(&'/') {
                    self.advance(); // Skip the '*'
                    self.advance(); // Skip the '/'
                    break;
                }
                self.advance(); // Continue advancing
            }
        }
    }

    fn get_current_line(&self) -> String {
        self.input_str
            .lines()
            .nth(self.line - 1)
            .unwrap_or("")
            .to_string()
    }
}
