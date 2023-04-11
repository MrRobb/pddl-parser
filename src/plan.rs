use std::fmt::Display;

use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl Action {
    pub const fn new(name: String, parameters: Vec<Parameter>) -> Self {
        Self { name, parameters }
    }

    fn parse(input: TokenStream) -> IResult<TokenStream, Self, ParserError> {
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

impl Display for Action {
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plan(pub Vec<Action>);

impl Plan {
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (_, plan) = many0(Action::parse)(input)?;
        Ok(Plan(plan))
    }

    pub fn actions(&self) -> impl Iterator<Item = &Action> {
        self.0.iter()
    }
}

impl IntoIterator for Plan {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Action;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl FromIterator<Action> for Plan {
    fn from_iter<T: IntoIterator<Item = Action>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
