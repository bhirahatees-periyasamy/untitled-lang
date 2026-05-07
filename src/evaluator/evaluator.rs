use crate::{parser::parser::Expr, token::token::Token};

pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self,expr: &Expr) -> i64 {
        match expr {
            Expr::Number(val) => *val,
            Expr::Binary { left, operator, right } => {
                let left = self.eval(left);
                let right = self.eval(right);

                match operator {

                    Token::Plus => {
                        left + right
                    }

                    Token::Minus => {
                        left - right
                    }

                    Token::Star => {
                        left * right
                    }

                    Token::Slash => {
                        left / right
                    }

                    _ => panic!("Invalid operator"),
                }
            }
        }
    }
}