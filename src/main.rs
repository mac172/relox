mod lexer;
mod token;

use ecow::EcoString;

use crate::lexer::Lexer;
use crate::token::Token;

fn main() {
    let input = EcoString::from("x + 5 * (3 - y) / 2 > 8 != 8.9 // 8");
    let mut lexer = Lexer::new(&input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}
