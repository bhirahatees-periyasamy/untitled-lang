use crate::parser::parser::Expr;
use crate::parser::parser::{BinaryOperator, Literal};
pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::Literal(val) => self.parse_literal(val),

            Expr::Identifier(name) => Err(format!("Undefined variable: {}", name)),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.eval(left)?;
                let right = self.eval(right)?;

                match operator {
                    BinaryOperator::Add => Ok(left + right),
                    BinaryOperator::Subtract => Ok(left - right),
                    BinaryOperator::Multiply => Ok(left * right),
                    BinaryOperator::Divide => {
                        if right == 0 {
                            Err("Divide by zero".to_string())
                        } else {
                            Ok(left / right)
                        }
                    }
                }
            }
        }
    }

    fn parse_literal(&self, val: &Literal) -> Result<i64, String> {
        match val {
            Literal::Number(val) => Ok(*val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parser::Expr;

    fn eval(expr: Expr) -> i64 {
        Evaluator.eval(&expr).unwrap()
    }

    fn binary(op: BinaryOperator, l: i64, r: i64) -> Expr {
        Expr::Binary {
            left: Box::new(Expr::Literal(Literal::Number(l))),
            operator: op,
            right: Box::new(Expr::Literal(Literal::Number(r))),
        }
    }

    #[test]
    fn number_literal() {
        assert_eq!(eval(Expr::Literal(Literal::Number(7))), 7);
    }

    #[test]
    fn addition() {
        assert_eq!(eval(binary(BinaryOperator::Add, 3, 4)), 7);
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval(binary(BinaryOperator::Subtract, 10, 6)), 4);
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval(binary(BinaryOperator::Multiply, 3, 5)), 15);
    }

    #[test]
    fn division() {
        assert_eq!(eval(binary(BinaryOperator::Divide, 12, 4)), 3);
    }

    #[test]
    fn nested_expression() {
        // (2 + 3) * 4 = 20
        let inner = binary(BinaryOperator::Add, 2, 3);
        let outer = Expr::Binary {
            left: Box::new(inner),
            operator: BinaryOperator::Multiply,
            right: Box::new(Expr::Literal(Literal::Number(4))),
        };
        assert_eq!(eval(outer), 20);
    }

    #[test]
    fn negative_result() {
        assert_eq!(eval(binary(BinaryOperator::Subtract, 3, 10)), -7);
    }

    #[test]
    fn indentifier_return_zero() {
        assert_eq!(
            Evaluator.eval(&Expr::Identifier("x".to_string())),
            Err("Undefined variable: x".to_string())
        )
    }

    #[test]
    fn division_by_zero_returns_err() {
        let expr = binary(BinaryOperator::Divide, 10, 0);
        assert!(Evaluator.eval(&expr).is_err());
    }
}
