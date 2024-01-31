use nom::multi::many0;
use serde::{Deserialize, Serialize};

use super::simple_action::SimpleAction;
use super::{action::Action, durative_action::DurativeAction};
use nom::branch::alt;
use nom::combinator::map;

use crate::error::ParserError;
use crate::lexer::TokenStream;

/// A plan is a sequence of actions.
///
/// The order of the actions is important. Plan is a wrapper around a `Vec<Action>` that implements `IntoIterator` and `FromIterator<Action>`. This might change in the future.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
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
        let (output, items) = many0(Action::parse)(input)?;
        if !output.is_empty() {
            log::error!("Plan parser failed: {:?}", output.to_string());
            return Err(ParserError::ExpectedEndOfInput);
        }
        Ok(Plan(items))
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
