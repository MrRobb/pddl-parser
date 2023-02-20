use nom::IResult;

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};

pub fn id(i: TokenStream) -> IResult<TokenStream, String, ParserError> {
    match i.peek() {
        Some((Token::Id(s), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedIdentifier)),
    }
}

pub fn var(i: TokenStream) -> IResult<TokenStream, String, ParserError> {
    match i.peek() {
        Some((Token::Var(s), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedIdentifier)),
    }
}

pub fn float(i: TokenStream) -> IResult<TokenStream, f64, ParserError> {
    match i.peek() {
        Some((Token::Float(s), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedFloat)),
    }
}

pub fn integer(i: TokenStream) -> IResult<TokenStream, i64, ParserError> {
    match i.peek() {
        Some((Token::Integer(s), _)) => Ok((i.advance(), s)),
        _ => Err(nom::Err::Error(ParserError::ExpectedInteger)),
    }
}
