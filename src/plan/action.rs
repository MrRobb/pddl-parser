use std::fmt::Display;

use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::durative_action::DurativeAction;
use super::simple_action::SimpleAction;
use crate::error::ParserError;
use crate::lexer::TokenStream;

/// Enum to represent either an `Action` or a `DurativeAction`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum Action {
    /// An Action wrapper around a simple action. See [`SimpleAction`](../simple_action/struct.SimpleAction.html).
    Simple(SimpleAction),
    /// An Action wrapper around a durative action. See [`DurativeAction`](../durative_action/struct.DurativeAction.html).
    Durative(DurativeAction),
}

impl Action {
    /// Get the name of the action. This is the same as the name of the simple or durative action.
    pub fn name(&self) -> &str {
        match self {
            Self::Simple(action) => &action.name,
            Self::Durative(action) => &action.name,
        }
    }

    /// Get the parameters of the action. This is the same as the parameters of the simple or durative action.
    pub fn parameters(&self) -> &[crate::domain::parameter::Parameter] {
        match self {
            Self::Simple(action) => &action.parameters,
            Self::Durative(action) => &action.parameters,
        }
    }

    /// Get the precondition of the action. This is the same as the precondition of the simple or durative action.
    pub fn parse(input: TokenStream) -> IResult<TokenStream, Action, ParserError> {
        log::debug!("BEGIN > parse_actions {:?}", input.span());
        let (output, actions) = alt((
            map(SimpleAction::parse, Action::Simple),
            map(DurativeAction::parse, Action::Durative),
        ))(input)?;
        log::debug!("END < parse_actions {:?}", output.span());
        Ok((output, actions))
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Simple(action) => write!(f, "{action}"),
            Action::Durative(action) => write!(f, "{action}"),
        }
    }
}
