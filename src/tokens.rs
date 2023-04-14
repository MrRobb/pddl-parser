use nom::IResult;

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};

/// Parse an identifier from the input stream. Identifiers are strings that do not start with a question mark.
///
/// # Errors
///
/// Returns an error if the next token is not an identifier.
pub fn id(i: TokenStream) -> IResult<TokenStream, String, ParserError> {
    match i.peek() {
        Some((Ok(Token::Id(s)), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedIdentifier)),
    }
}

/// Parse a variable from the input stream. Variables are identifiers that start with a question mark.
///
/// # Errors
///
/// Returns an error if the next token is not a variable.
pub fn var(i: TokenStream) -> IResult<TokenStream, String, ParserError> {
    match i.peek() {
        Some((Ok(Token::Var(s)), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedIdentifier)),
    }
}

/// Parse a floating point number from the input stream.
///
/// # Errors
///
/// Returns an error if the next token is not a floating point number.
pub fn float(i: TokenStream) -> IResult<TokenStream, f64, ParserError> {
    match i.peek() {
        Some((Ok(Token::Float(s)), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedFloat)),
    }
}

/// Parse an integer from the input stream.
///
/// # Errors
///
/// Returns an error if the next token is not an integer.
pub fn integer(i: TokenStream) -> IResult<TokenStream, i64, ParserError> {
    match i.peek() {
        Some((Ok(Token::Integer(s)), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedInteger)),
    }
}
