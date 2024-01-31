use std::fmt::Display;

use nom::{branch::alt, combinator::map, IResult};
use serde::{Deserialize, Serialize};

use crate::{error::ParserError, lexer::TokenStream};

use super::durative_action::DurativeAction;
use super::simple_action::SimpleAction;

/// Enum to represent either an `Action` or a `DurativeAction`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum Action {
    Simple(SimpleAction),
    Durative(DurativeAction),
}

impl Action {
    pub fn name(&self) -> &str {
        match self {
            Self::Simple(action) => &action.name,
            Self::Durative(action) => &action.name,
        }
    }

    pub fn parameters(&self) -> &[crate::domain::parameter::Parameter] {
        match self {
            Self::Simple(action) => &action.parameters,
            Self::Durative(action) => &action.parameters,
        }
    }

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
