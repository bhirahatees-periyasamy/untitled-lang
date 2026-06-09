use crate::ast::expr::{BinaryOperator, Expr, Literal};
use crate::ast::statement::Statements;
use crate::token::keyword::Keyword;
use crate::token::token::TokenKind;

pub struct Parser {
    tokens: Vec<TokenKind>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn previous(&self) -> Option<&TokenKind> {
        if self.position == 0 {
            None
        } else {
            self.tokens.get(self.position - 1)
        }
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&TokenKind> {
        if !self.is_at_end() {
            self.position += 1;
        }

        self.previous()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(TokenKind::Number(value)) => {
                let value = *value;
                self.advance();
                Ok(Expr::Literal(Literal::Number(value)))
            }

            Some(TokenKind::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }

            Some(TokenKind::LParen) => {
                self.advance();

                let expr = self.expression()?;

                self.consume(&TokenKind::RParen, "Expected ')'")?;

                Ok(expr)
            }

            token => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.primary()?;

        while self.matches(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = match self.previous() {
                Some(TokenKind::Star) => BinaryOperator::Multiply,
                Some(TokenKind::Slash) => BinaryOperator::Divide,
                _ => unreachable!(),
            };

            let right = self.primary()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        while self.matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = match self.previous() {
                Some(TokenKind::Plus) => BinaryOperator::Add,
                Some(TokenKind::Minus) => BinaryOperator::Subtract,
                _ => unreachable!(),
            };

            let right = self.term()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Vec<Statements>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn check(&self, token: &TokenKind) -> bool {
        match self.peek() {
            Some(current_token) => {
                std::mem::discriminant(current_token) == std::mem::discriminant(token)
            }

            None => false,
        }
    }

    fn matches(&mut self, tokens: &[TokenKind]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<&TokenKind, String> {
        if self.check(kind) {
            let token = self
                .advance()
                .ok_or_else(|| "Unexpected end of input".to_string())?;
            return Ok(token);
        }

        Err(message.to_string())
    }

    fn parse_statement(&mut self) -> Result<Statements, String> {
        match self.peek() {
            Some(TokenKind::Keyword(Keyword::Let)) => self.parse_variable_declaration(),
            _ => self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> Result<Statements, String> {
        Ok(Statements::Expression(self.expression()?))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statements, String> {
        let _ = self.consume(
            &TokenKind::Keyword(Keyword::Let),
            "Expected 'let' but got unexpected keyword",
        );

        let name = match self.advance() {
            Some(TokenKind::Identifier(name)) => name.to_string(),
            _ => return Err("Expected variable name".to_string()),
        };

        let _ = self.consume(&TokenKind::Equal, "Expected '=' after variable name");

        let initializer = self.expression()?;

        Ok(Statements::VariableDeclaration { name, initializer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token::TokenKind;

    fn parse_statements(tokens: Vec<TokenKind>) -> Vec<Statements> {
        Parser::new(tokens).parse().unwrap()
    }

    fn parse(tokens: Vec<TokenKind>) -> Expr {
        let mut statements = parse_statements(tokens);
        assert_eq!(
            statements.len(),
            1,
            "Expected exactly one statement, got {:?}",
            statements
        );
        match statements.remove(0) {
            Statements::Expression(expr) => expr,
            other => panic!("Expected an expression statement, got {:?}", other),
        }
    }

    #[test]
    fn single_number() {
        let expr = parse(vec![TokenKind::Number(5)]);
        assert!(matches!(expr, Expr::Literal(Literal::Number(5))));
    }

    #[test]
    fn single_identifier() {
        let expr = parse(vec![TokenKind::Identifier("x".to_string())]);
        assert!(matches!(expr, Expr::Identifier(name) if name == "x"));
    }

    #[test]
    fn addition() {
        let expr = parse(vec![
            TokenKind::Number(1),
            TokenKind::Plus,
            TokenKind::Number(2),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Add,
                ..
            }
        ));
    }

    #[test]
    fn subtraction() {
        let expr = parse(vec![
            TokenKind::Number(9),
            TokenKind::Minus,
            TokenKind::Number(3),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Subtract,
                ..
            }
        ));
    }

    #[test]
    fn multiplication() {
        let expr = parse(vec![
            TokenKind::Number(4),
            TokenKind::Star,
            TokenKind::Number(5),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Multiply,
                ..
            }
        ));
    }

    #[test]
    fn division() {
        let expr = parse(vec![
            TokenKind::Number(8),
            TokenKind::Slash,
            TokenKind::Number(2),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Divide,
                ..
            }
        ));
    }

    #[test]
    fn precedence_mul_before_add() {
        // 2 + 3 * 4  →  Binary(+, 2, Binary(*, 3, 4))
        let expr = parse(vec![
            TokenKind::Number(2),
            TokenKind::Plus,
            TokenKind::Number(3),
            TokenKind::Star,
            TokenKind::Number(4),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Add,
                ..
            }
        ));
        if let Expr::Binary { right, .. } = expr {
            assert!(matches!(
                *right,
                Expr::Binary {
                    operator: BinaryOperator::Multiply,
                    ..
                }
            ));
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        // (2 + 3) * 4  →  Binary(*, Binary(+, 2, 3), 4)
        let expr = parse(vec![
            TokenKind::LParen,
            TokenKind::Number(2),
            TokenKind::Plus,
            TokenKind::Number(3),
            TokenKind::RParen,
            TokenKind::Star,
            TokenKind::Number(4),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Multiply,
                ..
            }
        ));
        if let Expr::Binary { left, .. } = expr {
            assert!(matches!(
                *left,
                Expr::Binary {
                    operator: BinaryOperator::Add,
                    ..
                }
            ));
        }
    }

    #[test]
    fn identifier_in_binary_expr() {
        // x + 1  →  Binary(+, Identifier("x"), Number(1))
        let expr = parse(vec![
            TokenKind::Identifier("x".to_string()),
            TokenKind::Plus,
            TokenKind::Number(1),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Add,
                ..
            }
        ));
        if let Expr::Binary { left, .. } = expr {
            assert!(matches!(*left, Expr::Identifier(name) if name == "x"));
        }
    }

    #[test]
    fn missing_rparen_returns_err() {
        let result = Parser::new(vec![
            TokenKind::LParen,
            TokenKind::Number(1),
            TokenKind::Plus,
            TokenKind::Number(2),
        ])
        .parse();
        assert!(result.is_err());
    }

    #[test]
    fn chained_addition_is_left_associative() {
        // 1 + 2 + 3  →  Binary(+, Binary(+, 1, 2), 3)
        let expr = parse(vec![
            TokenKind::Number(1),
            TokenKind::Plus,
            TokenKind::Number(2),
            TokenKind::Plus,
            TokenKind::Number(3),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Add,
                ..
            }
        ));
        if let Expr::Binary { left, .. } = expr {
            assert!(matches!(
                *left,
                Expr::Binary {
                    operator: BinaryOperator::Add,
                    ..
                }
            ));
        }
    }

    #[test]
    fn division_then_subtraction_precedence() {
        // 8 - 4 / 2  →  Binary(-, 8, Binary(/, 4, 2))
        let expr = parse(vec![
            TokenKind::Number(8),
            TokenKind::Minus,
            TokenKind::Number(4),
            TokenKind::Slash,
            TokenKind::Number(2),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: BinaryOperator::Subtract,
                ..
            }
        ));
        if let Expr::Binary { right, .. } = expr {
            assert!(matches!(
                *right,
                Expr::Binary {
                    operator: BinaryOperator::Divide,
                    ..
                }
            ));
        }
    }

    #[test]
    fn variable_declaration() {
        // let x = 1 + 2
        let statements = parse_statements(vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier("x".to_string()),
            TokenKind::Equal,
            TokenKind::Number(1),
            TokenKind::Plus,
            TokenKind::Number(2),
        ]);
        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Statements::VariableDeclaration { name, initializer } => {
                assert_eq!(name, "x");
                assert!(matches!(
                    initializer,
                    Expr::Binary {
                        operator: BinaryOperator::Add,
                        ..
                    }
                ));
            }
            other => panic!("Expected a variable declaration, got {:?}", other),
        }
    }

    #[test]
    fn variable_declaration_missing_name_returns_err() {
        let result = Parser::new(vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Equal,
            TokenKind::Number(1),
        ])
        .parse();
        assert!(result.is_err());
    }

    #[test]
    fn multiple_statements() {
        // let x = 1
        // 2 + 3
        let statements = parse_statements(vec![
            TokenKind::Keyword(Keyword::Let),
            TokenKind::Identifier("x".to_string()),
            TokenKind::Equal,
            TokenKind::Number(1),
            TokenKind::Number(2),
            TokenKind::Plus,
            TokenKind::Number(3),
        ]);
        assert_eq!(statements.len(), 2);
        assert!(matches!(
            statements[0],
            Statements::VariableDeclaration { .. }
        ));
        assert!(matches!(statements[1], Statements::Expression(_)));
    }

    #[test]
    fn empty_input_yields_no_statements() {
        let statements = parse_statements(vec![]);
        assert!(statements.is_empty());
    }
}
