use std::fmt::Display;

use nom::sequence::{delimited, pair};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// Action is a named sequence of steps that can be performed by an agent.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleAction {
    /// The name of the action.
    pub name: String,
    /// The parameters of the action.
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl SimpleAction {
    /// Create a new action.
    pub const fn new(name: String, parameters: Vec<Parameter>) -> Self {
        Self { name, parameters }
    }

    /// Parse an action from a token stream.
    pub fn parse(input: TokenStream) -> IResult<TokenStream, Self, ParserError> {
        let (output, (name, parameters)) = delimited(
            Token::OpenParen,
            pair(Self::parse_name, Parameter::parse_parameters),
            Token::CloseParen,
        )(input)?;
        Ok((output, Self::new(name, parameters)))
    }

    fn parse_name(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        id(input)
    }
}

impl Display for SimpleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {})",
            self.name,
            self.parameters
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
