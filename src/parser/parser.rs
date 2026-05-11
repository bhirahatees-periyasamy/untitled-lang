use crate::token::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn factor(&mut self) -> Result<Expr, String> {
        match self.current() {
            Some(Token::Number(value)) => {
                let value = *value;
                self.advance();
                Ok(Expr::Number(value))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.expression()?;
                match self.current() {
                    Some(Token::RParen) => self.advance(),
                    _ => return Err("Expected ')'".to_string()),
                }
                Ok(expr)
            }

            token => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;

        loop {
            if self.is_at_end() {
                break;
            }
            match self.current() {
                Some(Token::Star) | Some(Token::Slash) => {
                    let operator = self.current().unwrap().clone();
                    self.advance();
                    let right = self.factor()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        loop {
            if self.is_at_end() {
                break;
            }
            match self.current() {
                Some(Token::Plus) | Some(Token::Minus) => {
                    let operator = self.current().unwrap().clone();
                    self.advance();
                    let right = self.term()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token::Token;

    fn parse(tokens: Vec<Token>) -> Expr {
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn single_number() {
        let expr = parse(vec![Token::Number(5)]);
        assert!(matches!(expr, Expr::Number(5)));
    }

    #[test]
    fn addition() {
        let expr = parse(vec![Token::Number(1), Token::Plus, Token::Number(2)]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Plus,
                ..
            }
        ));
    }

    #[test]
    fn subtraction() {
        let expr = parse(vec![Token::Number(9), Token::Minus, Token::Number(3)]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Minus,
                ..
            }
        ));
    }

    #[test]
    fn multiplication() {
        let expr = parse(vec![Token::Number(4), Token::Star, Token::Number(5)]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Star,
                ..
            }
        ));
    }

    #[test]
    fn division() {
        let expr = parse(vec![Token::Number(8), Token::Slash, Token::Number(2)]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Slash,
                ..
            }
        ));
    }

    #[test]
    fn precedence_mul_before_add() {
        // 2 + 3 * 4  →  Binary(+, 2, Binary(*, 3, 4))
        let expr = parse(vec![
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::Star,
            Token::Number(4),
        ]);
        // Outer operator must be Plus
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Plus,
                ..
            }
        ));
        if let Expr::Binary { right, .. } = expr {
            assert!(matches!(
                *right,
                Expr::Binary {
                    operator: Token::Star,
                    ..
                }
            ));
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        // (2 + 3) * 4  →  Binary(*, Binary(+, 2, 3), 4)
        let expr = parse(vec![
            Token::LParen,
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::RParen,
            Token::Star,
            Token::Number(4),
        ]);
        assert!(matches!(
            expr,
            Expr::Binary {
                operator: Token::Star,
                ..
            }
        ));
        if let Expr::Binary { left, .. } = expr {
            assert!(matches!(
                *left,
                Expr::Binary {
                    operator: Token::Plus,
                    ..
                }
            ));
        }
    }
}
