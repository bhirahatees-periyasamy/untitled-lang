use std::fmt::Error;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;

pub struct Interepter;

impl Interepter {
    pub fn execute(&self, expression: &str) -> Result<i64, Error> {
        let tokens = Lexer::tokenize(expression);

        match tokens {
            Ok(tokenized) => {
                let mut parser = Parser::new(tokenized);
                let parsed = parser.parse();
                let evaluator = Evaluator.eval(&parsed);
                Ok(evaluator)
            }
            Err(err) => {
                panic!("{}",err)
            }
        }
    }
}
