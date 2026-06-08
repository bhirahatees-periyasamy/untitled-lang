use crate::token::token::TokenKind;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Option<TokenKind>),
    ExpectedRightParen,
    ExpectedVariableName,
    ExpectedEquals,
    UnexpectedEndOfInput,
}