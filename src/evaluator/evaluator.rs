use crate::ast::{
    expr::{BinaryOperator, Expr, Literal},
    statement::Statements,
};
use std::collections::HashMap;

pub struct Evaluator {
    environment: HashMap<String, i64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn eval(&mut self, statements: Vec<Statements>) -> Result<Option<i64>, String> {
        let mut last_value = None;

        for statement in statements {
            last_value = self.eval_statement(statement)?;
        }

        Ok(last_value)
    }

    pub fn eval_statement(&mut self, statement: Statements) -> Result<Option<i64>, String> {
        match statement {
            Statements::Expression(expr) => {
                let value = self.eval_expr(&expr)?;
                Ok(Some(value))
            }
            Statements::VariableDeclaration { name, initializer } => {
                let value = self.eval_expr(&initializer)?;
                self.environment.insert(name, value);
                Ok(None)
            }
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::Literal(val) => self.parse_literal(val),

            Expr::Identifier(name) => self
                .environment
                .get(name)
                .copied()
                .ok_or_else(|| format!("Undefined variable: {}", name)),

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;

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
    use crate::ast::expr::Expr;

    fn eval(expr: Expr) -> i64 {
        Evaluator::new().eval_expr(&expr).unwrap()
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
    fn undefined_identifier_returns_err() {
        assert_eq!(
            Evaluator::new().eval_expr(&Expr::Identifier("x".to_string())),
            Err("Undefined variable: x".to_string())
        );
    }

    #[test]
    fn division_by_zero_returns_err() {
        let expr = binary(BinaryOperator::Divide, 10, 0);
        assert!(Evaluator::new().eval_expr(&expr).is_err());
    }

    #[test]
    fn variable_declaration_stores_value() {
        let mut evaluator = Evaluator::new();
        let result = evaluator
            .eval_statement(Statements::VariableDeclaration {
                name: "x".to_string(),
                initializer: Expr::Literal(Literal::Number(42)),
            })
            .unwrap();
        // A declaration produces no value of its own.
        assert_eq!(result, None);
        // ...but the binding is now usable.
        assert_eq!(
            evaluator
                .eval_expr(&Expr::Identifier("x".to_string()))
                .unwrap(),
            42
        );
    }

    #[test]
    fn variable_declaration_then_reference() {
        // let x = 7
        // x + 3   →  10
        let mut evaluator = Evaluator::new();
        let statements = vec![
            Statements::VariableDeclaration {
                name: "x".to_string(),
                initializer: Expr::Literal(Literal::Number(7)),
            },
            Statements::Expression(Expr::Binary {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: BinaryOperator::Add,
                right: Box::new(Expr::Literal(Literal::Number(3))),
            }),
        ];
        assert_eq!(evaluator.eval(statements).unwrap(), Some(10));
    }

    #[test]
    fn eval_returns_last_expression_value() {
        let mut evaluator = Evaluator::new();
        let statements = vec![
            Statements::Expression(Expr::Literal(Literal::Number(1))),
            Statements::Expression(Expr::Literal(Literal::Number(2))),
        ];
        assert_eq!(evaluator.eval(statements).unwrap(), Some(2));
    }

    #[test]
    fn declaration_only_yields_no_value() {
        let mut evaluator = Evaluator::new();
        let statements = vec![Statements::VariableDeclaration {
            name: "x".to_string(),
            initializer: Expr::Literal(Literal::Number(1)),
        }];
        assert_eq!(evaluator.eval(statements).unwrap(), None);
    }

    #[test]
    fn empty_statements_eval_to_none() {
        let mut evaluator = Evaluator::new();
        assert_eq!(evaluator.eval(vec![]).unwrap(), None);
    }

}
