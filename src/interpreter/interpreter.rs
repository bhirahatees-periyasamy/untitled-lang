use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::evaluator::Evaluator;

pub struct Interpreter;

impl Interpreter {
    pub fn execute(&self, expression: &str) -> Result<i64, String> {
        let tokens = Lexer::tokenize(expression)?;
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse()?;
        let mut evaluator = Evaluator::new();
        Ok(evaluator.eval(parsed)?.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(expr: &str) -> i64 {
        Interpreter.execute(expr).unwrap()
    }

    #[test]
    fn single_number() {
        assert_eq!(run("42"), 42);
    }

    #[test]
    fn addition() {
        assert_eq!(run("1 + 2"), 3);
    }

    #[test]
    fn subtraction() {
        assert_eq!(run("10 - 4"), 6);
    }

    #[test]
    fn multiplication() {
        assert_eq!(run("3 * 5"), 15);
    }

    #[test]
    fn division() {
        assert_eq!(run("20 / 4"), 5);
    }

    #[test]
    fn operator_precedence() {
        assert_eq!(run("2 + 3 * 4"), 14);
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(run("(2 + 3) * 4"), 20);
    }

    #[test]
    fn complex_expression() {
        assert_eq!(run("10 + (2 * 3) - 4 / 2"), 14);
    }

    #[test]
    fn chained_addition() {
        assert_eq!(run("1 + 2 + 3 + 4"), 10);
    }
}
