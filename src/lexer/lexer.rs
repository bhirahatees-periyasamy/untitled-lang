use std::iter::Peekable;
use std::str::Chars;

use crate::token::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    let value = Self::read_numbers(&mut chars, &ch);
                    tokens.push(Token::Number(value));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let value = Self::read_identifier(&mut chars, &ch);
                    tokens.push(Token::Identifier(value));
                }
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ' ' | '\n' | '\t' => {}
                _ => {
                    return Err(format!("Unexpected character: {}", ch));
                }
            }
        }
        Ok(tokens)
    }

    fn read_numbers(chs: &mut Peekable<Chars>, ch: &char) -> i64 {
        let mut value = ch.to_string();
        while let Some(next_char) = chs.peek() {
            match next_char {
                '0'..='9' => {
                    value.push(*next_char);
                    chs.next();
                }
                _ => {
                    break;
                }
            }
        }
        value.parse::<i64>().unwrap()
    }

    fn read_identifier(chs: &mut Peekable<Chars>, ch: &char) -> String {
        let mut value = ch.to_string();
        while let Some(next_char) = chs.peek() {
            match next_char {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    value.push(*next_char);
                    chs.next();
                }
                _ => {
                    break;
                }
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token::Token;

    #[test]
    fn single_number() {
        assert_eq!(Lexer::tokenize("42").unwrap(), vec![Token::Number(42)]);
    }

    #[test]
    fn multi_digit_number() {
        assert_eq!(Lexer::tokenize("1234").unwrap(), vec![Token::Number(1234)]);
    }

    #[test]
    fn all_operators() {
        let tokens = Lexer::tokenize("+ - * /").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Plus, Token::Minus, Token::Star, Token::Slash]
        );
    }

    #[test]
    fn parens() {
        let tokens = Lexer::tokenize("()").unwrap();
        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn whitespace_ignored() {
        let tokens = Lexer::tokenize("1 + 2").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(1), Token::Plus, Token::Number(2)]
        );
    }

    #[test]
    fn newline_and_tab_ignored() {
        let tokens = Lexer::tokenize("3\n*\t4").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(3), Token::Star, Token::Number(4)]
        );
    }

    #[test]
    fn full_expression() {
        let tokens = Lexer::tokenize("(1+2)*3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Number(1),
                Token::Plus,
                Token::Number(2),
                Token::RParen,
                Token::Star,
                Token::Number(3),
            ]
        );
    }

    #[test]
    fn unexpected_character_returns_err() {
        assert!(Lexer::tokenize("1 + $").is_err());
    }

    #[test]
    fn empty_input_produces_no_tokens() {
        assert_eq!(Lexer::tokenize("").unwrap(), vec![]);
    }

    #[test]
fn identifier() {
    assert_eq!(
        Lexer::tokenize("hello").unwrap(),
        vec![Token::Identifier("hello".to_string())]
    );
}

#[test]
fn identifier_with_numbers() {
    assert_eq!(
        Lexer::tokenize("total123").unwrap(),
        vec![Token::Identifier("total123".to_string())]
    );
}

#[test]
fn identifier_with_underscore() {
    assert_eq!(
        Lexer::tokenize("my_var").unwrap(),
        vec![Token::Identifier("my_var".to_string())]
    );
}

#[test]
fn identifier_expression() {
    assert_eq!(
        Lexer::tokenize("x + y").unwrap(),
        vec![
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
        ]
    );
}
}
