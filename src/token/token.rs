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