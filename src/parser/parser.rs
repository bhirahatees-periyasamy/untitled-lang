use crate::token::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Binary{
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn factor(&mut self) -> Expr {
        match self.current() {
            Token::Number(value) => {
                let value = *value;

                self.advance();

                Expr::Number(value)
            }

            Token::LParen => {
                self.advance();

                let expr = self.expression();

                match self.current() {
                    Token::RParen => {
                        self.advance();
                    }

                    _ => panic!("Expected ')'"),
                }

                expr
            }

            _ => panic!("Unexpected token"),
        }
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        loop {
            if self.is_at_end() { break; }
            match self.current() {
                Token::Star | Token::Slash => {
                    let operator = self.current().clone();

                    self.advance();

                    let right = self.factor();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        left
    }
    fn expression(&mut self) -> Expr {
        let mut left = self.term();

        loop {
            if self.is_at_end() { break; }
            match self.current() {
                Token::Plus | Token::Minus => {
                    let operator = self.current().clone();

                    self.advance();

                    let right = self.term();

                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        left
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}


