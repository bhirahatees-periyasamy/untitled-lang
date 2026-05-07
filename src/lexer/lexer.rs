use crate::token::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '0'..='9' => {
                    let mut number = ch.to_string();
                    while let Some(next_char) = chars.peek() {
                        match next_char {
                            '0'..='9' => {
                                number.push(*next_char);
                                chars.next();
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                    let value = number.parse::<i64>().unwrap();
                    tokens.push(Token::Number(value));
                },
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
        assert_eq!(tokens, vec![Token::Plus, Token::Minus, Token::Star, Token::Slash]);
    }

    #[test]
    fn parens() {
        let tokens = Lexer::tokenize("()").unwrap();
        assert_eq!(tokens, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn whitespace_ignored() {
        let tokens = Lexer::tokenize("1 + 2").unwrap();
        assert_eq!(tokens, vec![Token::Number(1), Token::Plus, Token::Number(2)]);
    }

    #[test]
    fn newline_and_tab_ignored() {
        let tokens = Lexer::tokenize("3\n*\t4").unwrap();
        assert_eq!(tokens, vec![Token::Number(3), Token::Star, Token::Number(4)]);
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
        assert!(Lexer::tokenize("1 + x").is_err());
    }

    #[test]
    fn empty_input_produces_no_tokens() {
        assert_eq!(Lexer::tokenize("").unwrap(), vec![]);
    }
}
