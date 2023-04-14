use std::fmt::Display;

use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// Action is a named sequence of steps that can be performed by an agent.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    /// The name of the action.
    pub name: String,
    /// The parameters of the action.
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl Action {
    /// Create a new action.
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

/// A plan is a sequence of actions.
///
/// The order of the actions is important. Plan is a wrapper around a `Vec<Action>` that implements `IntoIterator` and `FromIterator<Action>`. This might change in the future.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plan(pub Vec<Action>);

impl Plan {
    /// Parse a plan from a token stream.
    ///
    /// The plan must be a sequence of actions. The parser will fail if there are any tokens left after the plan.
    ///
    /// # Errors
    ///
    /// The parser will fail if there are any tokens left after the plan. It will also fail if the plan is empty or if any of the actions are invalid.
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (output, plan) = many0(Action::parse)(input)?;
        if !output.is_empty() {
            return Err(ParserError::ExpectedEndOfInput);
        }
        Ok(Plan(plan))
    }

    /// Get an iterator over the actions in the plan.
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
