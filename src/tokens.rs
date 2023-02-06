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
