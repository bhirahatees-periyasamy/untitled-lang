use crate::token::keyword::Keyword;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(i64),

    Identifier(String),

    Keyword(Keyword),

    Equal,

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
        assert_eq!(TokenKind::Number(42), TokenKind::Number(42));
        assert_ne!(TokenKind::Number(1), TokenKind::Number(2));
        assert_eq!(TokenKind::Plus, TokenKind::Plus);
        assert_ne!(TokenKind::Plus, TokenKind::Minus);
    }

    #[test]
    fn token_clone() {
        let t = TokenKind::Number(7);
        assert_eq!(t.clone(), TokenKind::Number(7));
    }
}