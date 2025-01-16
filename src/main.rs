//mod ast;
mod environment;
mod lexer;
mod lexer_error;
mod parser;
mod token;

use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    // Sample input
    let input = "let x = 5 + 3; let y = x * 2; print(y + 5);";

    // Create lexer and parser
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);

    // Create an environment to store variables
    let mut environment = Environment::new();

    // Parse and execute statements
    while let Some(stmt) = parser.parse_statement() {
        match stmt {
            Ok(statement) => {
                // Execute each statement
                if let Err(e) = environment.execute(&statement) {
                    eprintln!("Error executing statement: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error parsing statement: {}", e);
                break;
            }
        }
    }
}
/*

fn main() {
    let input = "let x = 10 - 45; let x = y; print (x); print y+ 7;";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);

    // Continue parsing until parse_statement returns None
    while let Some(result) = parser.parse_statement() {
        match result {
            Ok(stmt) => println!("{:?}", stmt),
            Err(err) => {
                eprintln!("Error parsing statement: {}", err);
                break;
            }
        }
    }
}*/
