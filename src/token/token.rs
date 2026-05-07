#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    Plus,
    Minus,
    Star,
    Slash,

    LParen,
    RParen,

    EOF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_equality() {
        assert_eq!(Token::Number(42), Token::Number(42));
        assert_ne!(Token::Number(1), Token::Number(2));
        assert_eq!(Token::Plus, Token::Plus);
        assert_ne!(Token::Plus, Token::Minus);
    }

    #[test]
    fn token_clone() {
        let t = Token::Number(7);
        assert_eq!(t.clone(), Token::Number(7));
    }
}