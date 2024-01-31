use std::string::ToString;

use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::typedef::TypeDef;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// A type is either a simple type or one of the list of specified types.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// The type is a simple type.
    Simple(String),
    /// The type must be one of the specified types.
    Either(Vec<String>),
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        Type::Simple(s.to_string())
    }
}

impl Default for Type {
    fn default() -> Self {
        "object".into()
    }
}

impl Type {
    /// Parse a type from a token stream.
    pub fn parse_type(input: TokenStream) -> IResult<TokenStream, Type, ParserError> {
        log::debug!("BEGIN > parse_type {:?}", input.span());
        let (output, type_) = alt((
            map(id, Type::Simple),
            map(
                delimited(Token::OpenParen, preceded(Token::Either, many1(id)), Token::CloseParen),
                Type::Either,
            ),
        ))(input)?;
        log::debug!("END < parse_type {:?}", output.span());
        Ok((output, type_))
    }

    /// Parse a list of types from a token stream.
    pub fn parse_types(input: TokenStream) -> IResult<TokenStream, Vec<TypeDef>, ParserError> {
        log::debug!("BEGIN > parse_types {:?}", input.span());
        let (output, types) = delimited(
            Token::OpenParen,
            preceded(Token::Types, many0(pair(many1(id), opt(preceded(Token::Dash, id))))),
            Token::CloseParen,
        )(input)?;
        let types = types
            .into_iter()
            .flat_map(|(names, parent)| {
                names.into_iter().map(move |name| TypeDef {
                    name,
                    parent: parent.clone(),
                })
            })
            .collect();
        log::debug!("END < parse_types {:?}", output.span());
        Ok((output, types))
    }

    /// Convert the type to PDDL.
    pub fn to_pddl(&self) -> String {
        match self {
            Type::Simple(s) => s.to_string(),
            Type::Either(v) => format!("(either {})", v.join(" ")),
        }
    }
}
