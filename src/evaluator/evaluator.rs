use crate::{parser::parser::Expr, token::token::Token};

pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::Number(val) => Ok(*val),
            Expr::Binary { left, operator, right } => {
                let left = self.eval(left)?;
                let right = self.eval(right)?;

                match operator {
                    Token::Plus => Ok(left + right),
                    Token::Minus => Ok(left - right),
                    Token::Star => Ok(left * right),
                    Token::Slash => {
                        if right == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left / right)
                        }
                    }
                    _ => Err(format!("Invalid operator: {:?}", operator)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Expr;
    use crate::token::token::Token;

    fn eval(expr: Expr) -> i64 {
        Evaluator.eval(&expr).unwrap()
    }

    fn binary(op: Token, l: i64, r: i64) -> Expr {
        Expr::Binary {
            left: Box::new(Expr::Number(l)),
            operator: op,
            right: Box::new(Expr::Number(r)),
        }
    }

    #[test]
    fn number_literal() {
        assert_eq!(eval(Expr::Number(7)), 7);
    }

    #[test]
    fn addition() {
        assert_eq!(eval(binary(Token::Plus, 3, 4)), 7);
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval(binary(Token::Minus, 10, 6)), 4);
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval(binary(Token::Star, 3, 5)), 15);
    }

    #[test]
    fn division() {
        assert_eq!(eval(binary(Token::Slash, 12, 4)), 3);
    }

    #[test]
    fn nested_expression() {
        // (2 + 3) * 4 = 20
        let inner = binary(Token::Plus, 2, 3);
        let outer = Expr::Binary {
            left: Box::new(inner),
            operator: Token::Star,
            right: Box::new(Expr::Number(4)),
        };
        assert_eq!(eval(outer), 20);
    }

    #[test]
    fn negative_result() {
        assert_eq!(eval(binary(Token::Minus, 3, 10)), -7);
    }
}