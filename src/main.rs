pub mod lexer;
pub mod token;
pub mod parser;
pub mod evaluator;
pub mod interpreter;
pub mod ast;

use crate::interpreter::interpreter::Interpreter;

fn main() {
    let expression = "123 + (5 * 3)";
    let result = Interpreter.execute(expression);
    match result {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err)
    } 
}
