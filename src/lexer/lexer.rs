use std::iter::Peekable;
use std::str::Chars;

use crate::token::keyword::Keyword;
use crate::token::token::TokenKind;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<TokenKind>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    let value = Self::read_numbers(&mut chars, &ch);
                    tokens.push(TokenKind::Number(value));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let value = Self::read_identifier(&mut chars, &ch);
                    match Self::lookup_keyword(&value) {
                        Some(keyword) => tokens.push(TokenKind::Keyword(keyword)),
                        None => tokens.push(TokenKind::Identifier(value)),
                    }
                },
                '=' => tokens.push(TokenKind::Equal),
                '+' => tokens.push(TokenKind::Plus),
                '-' => tokens.push(TokenKind::Minus),
                '*' => tokens.push(TokenKind::Star),
                '/' => tokens.push(TokenKind::Slash),
                '(' => tokens.push(TokenKind::LParen),
                ')' => tokens.push(TokenKind::RParen),
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

    fn lookup_keyword(value: &str) -> Option<Keyword> {
        match value {
            "let" => Some(Keyword::Let),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token::TokenKind;

    #[test]
    fn single_number() {
        assert_eq!(Lexer::tokenize("42").unwrap(), vec![TokenKind::Number(42)]);
    }

    #[test]
    fn multi_digit_number() {
        assert_eq!(
            Lexer::tokenize("1234").unwrap(),
            vec![TokenKind::Number(1234)]
        );
    }

    #[test]
    fn all_operators() {
        let tokens = Lexer::tokenize("+ - * /").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash
            ]
        );
    }

    #[test]
    fn parens() {
        let tokens = Lexer::tokenize("()").unwrap();
        assert_eq!(tokens, vec![TokenKind::LParen, TokenKind::RParen]);
    }

    #[test]
    fn whitespace_ignored() {
        let tokens = Lexer::tokenize("1 + 2").unwrap();
        assert_eq!(
            tokens,
            vec![TokenKind::Number(1), TokenKind::Plus, TokenKind::Number(2)]
        );
    }

    #[test]
    fn newline_and_tab_ignored() {
        let tokens = Lexer::tokenize("3\n*\t4").unwrap();
        assert_eq!(
            tokens,
            vec![TokenKind::Number(3), TokenKind::Star, TokenKind::Number(4)]
        );
    }

    #[test]
    fn full_expression() {
        let tokens = Lexer::tokenize("(1+2)*3").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::LParen,
                TokenKind::Number(1),
                TokenKind::Plus,
                TokenKind::Number(2),
                TokenKind::RParen,
                TokenKind::Star,
                TokenKind::Number(3),
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
            vec![TokenKind::Identifier("hello".to_string())]
        );
    }

    #[test]
    fn identifier_with_numbers() {
        assert_eq!(
            Lexer::tokenize("total123").unwrap(),
            vec![TokenKind::Identifier("total123".to_string())]
        );
    }

    #[test]
    fn identifier_with_underscore() {
        assert_eq!(
            Lexer::tokenize("my_var").unwrap(),
            vec![TokenKind::Identifier("my_var".to_string())]
        );
    }

    #[test]
    fn identifier_expression() {
        assert_eq!(
            Lexer::tokenize("x + y").unwrap(),
            vec![
                TokenKind::Identifier("x".to_string()),
                TokenKind::Plus,
                TokenKind::Identifier("y".to_string()),
            ]
        );
    }

    #[test]
    fn equal_sign() {
        assert_eq!(Lexer::tokenize("=").unwrap(), vec![TokenKind::Equal]);
    }

    #[test]
    fn assignment_expression() {
        assert_eq!(
            Lexer::tokenize("x = 5").unwrap(),
            vec![
                TokenKind::Identifier("x".to_string()),
                TokenKind::Equal,
                TokenKind::Number(5),
            ]
        );
    }

    #[test]
    fn let_binding_with_equal() {
        use crate::token::keyword::Keyword;
        assert_eq!(
            Lexer::tokenize("let x = 10").unwrap(),
            vec![
                TokenKind::Keyword(Keyword::Let),
                TokenKind::Identifier("x".to_string()),
                TokenKind::Equal,
                TokenKind::Number(10),
            ]
        );
    }
}
