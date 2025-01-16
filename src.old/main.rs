mod error;
mod lexer;
mod parser;
mod sym_table;
mod token;

use ecow::EcoString;

use crate::lexer::Lexer;
use crate::token::Token;

// write function to add two numbers

fn main() {
    let input = EcoString::from("x + 5 * (3 - y) / 2 > 8 != 8.9 - 67 // 8");
    let mut lexer = Lexer::new(&input);

    loop {
        if let Ok(token) = lexer.next_token() {
            if token == Token::EOF {
                break;
            } else {
                println!("{:?}", token);
            }
        } else if let Err(err) = lexer.next_token() {
            err.report();
            break;
        }
    }
}
