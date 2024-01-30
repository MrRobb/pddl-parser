use crate::domain::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens;
use crate::tokens::id;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use serde::{Deserialize, Serialize};

/// A durative action is an action that has a duration.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct DurativeAction {
    /// The name of the action.
    pub name: String,
    /// The parameters of the action.
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    /// The duration of the action.
    pub duration: f64,
    /// The condition of the action.
    pub timestamp: f64,
}

impl DurativeAction {
    pub const fn new(name: String, parameters: Vec<Parameter>, duration: f64, timestamp: f64) -> Self {
        Self {
            name,
            parameters,
            duration,
            timestamp,
        }
    }

    pub fn parse(input: TokenStream) -> IResult<TokenStream, Self, ParserError> {
        let (output, (timestamp, (name, parameters), duration)) = tuple((
            terminated(tokens::float, Token::Colon),
            delimited(
                Token::OpenParen,
                pair(id, Parameter::parse_parameters),
                Token::CloseParen,
            ),
            delimited(Token::OpenBracket, tokens::float, Token::CloseBracket),
        ))(input)?;
        Ok((output, Self::new(name, parameters, duration, timestamp)))
    }
}
