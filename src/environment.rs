use crate::parser::{Expression, Statement};
use crate::token::TokenType;
use ecow::EcoString;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    pub variables: HashMap<EcoString, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: EcoString, value: Expression) {
        self.variables.insert(name, value);
    }

    // Function to evaluate expressions based on the current environment
    pub fn eval_expression(&mut self, expr: &Expression) -> Result<i64, String> {
        match expr {
            Expression::Int(value) => Ok(*value),
            Expression::Float(value) => Ok(*value as i64),
            Expression::Identifier(name) => {
                // Extract the value from the environment to avoid borrowing conflicts.
                let expr_to_eval = self.get(name).cloned();

                if let Some(e) = expr_to_eval {
                    // Evaluate the expression using the mutable reference.
                    self.eval_expression(&e)
                } else {
                    Err(format!("Variable '{}' not found", name))
                }
            }
            Expression::Negation(expr) => {
                let result = self.eval_expression(expr)?;
                Ok(-result)
            }
            Expression::Binary(left, op, right) => {
                let left_value = self.eval_expression(left)?;
                let right_value = self.eval_expression(right)?;
                match op {
                    TokenType::Plus => Ok(left_value + right_value),
                    TokenType::Minus => Ok(left_value - right_value),
                    TokenType::Star => Ok(left_value * right_value),
                    TokenType::Slash => {
                        if right_value != 0 {
                            Ok(left_value / right_value)
                        } else {
                            Err("Division by zero".into())
                        }
                    }
                    _ => Err(format!("Unsupported operator: {:?}", op)),
                }
            }
        }
    }

    // Function to execute a statement
    pub fn execute(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Let(name, expr) => {
                let value = self.eval_expression(expr)?;
                self.set(name.clone(), Expression::Int(value));
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.eval_expression(expr)?;
                println!("{}", value);
                Ok(())
            }
        }
    }
}
