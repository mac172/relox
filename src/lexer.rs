use crate::Token;
use ecow::EcoString;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    input_str: &'a EcoString,
    postion: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a EcoString) -> Self {
        let mut lexer = Lexer {
            input: source.chars().peekable(),
            current_char: None,
            input_str: source,
            postion: 0,
        };

        lexer.advance();
        lexer
    }

    // next character in input
    pub fn advance(&mut self) {
        self.current_char = self.input.next();
        self.postion += 1;
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
    fn number(&mut self) -> EcoString {
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
        num_str
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
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_single_comment();
        self.skip_multiline_comment();

        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                    continue;
                }
                '+' => return self.single_char_token(Token::Plus),
                '-' => return self.single_char_token(Token::Minus),
                '*' => return self.single_char_token(Token::Mul),
                '/' => return self.single_char_token(Token::Slash),
                '(' => return self.single_char_token(Token::LParen),
                ')' => return self.single_char_token(Token::RParen),
                '=' => return self.double_char_token('=', Token::Equal, Token::Assign),
                '>' => return self.double_char_token('=', Token::GreaterThanEqual, Token::GreaterThan),
                '<' => return self.double_char_token('=', Token::LessThanEqual, Token::LessThan),
                '!' => return self.double_char_token('=', Token::NotEqual, Token::EOF),
                '&' => return self.double_char_token('&', Token::And, Token::EOF),
                '|' => return self.double_char_token('|', Token::Or, Token::EOF),
                '0'..='9' => {
                    let num = self.number();
                    if num.contains('.') {
                        return Token::Float(num.parse().unwrap());
                    } else {
                        return Token::Int(num.parse().unwrap());
                    }
                }
                'a'..='z' | 'A'..='Z' => return self.identifier(),
                '"' => {
                    let str_litr = self.parse_string_literal();
                    return Token::StringLiteral(str_litr);
                }
                _ => {
                    println!("Error: Unexpected char {}", c);
                    self.advance();
                }
            }
            self.skip_whitespace();
        }
        Token::EOF
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

    // helper methods
    fn single_char_token(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn double_char_token(&mut self, next_char: char, double_token: Token, single_token: Token) -> Token {
        self.advance();
        if self.current_char == Some(next_char) {
            self.advance();
            double_token
        } else {
            single_token
        }
    }
}
