pub mod lexer;
pub mod token;
pub mod parser;
pub mod evaluator;
pub mod interepter;

use crate::interepter::interepter::Interepter;

fn main() {
    let expression = "1 + 2 + 3 * (2 + 3)";
    let result = Interepter.execute(expression);
    match result {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err)
    } 
}
