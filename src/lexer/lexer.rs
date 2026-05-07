use crate::token::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        input
            .chars()
            .filter_map(|ch| {
                let token = match ch {
                    '0'..='9' => {
                        Some(Token::Number(ch.to_digit(10).unwrap() as i64))
                    }
                    '+' => Some(Token::Plus),
                    '-' => Some(Token::Minus),
                    '*' => Some(Token::Star),
                    '/' => Some(Token::Slash),
                    '(' => Some(Token::LParen),
                    ')' => Some(Token::RParen),

                    // Ignore whitespace
                    ' ' | '\n' | '\t' => None,

                    _ => {
                        return Some(
                            Err(format!("Unexpected character: {}", ch))
                        );
                    }
                };

                token.map(Ok)
            })
            .collect()
    }
}