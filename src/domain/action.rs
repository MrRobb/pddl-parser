use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::durative_action::DurativeAction;
use super::expression::Expression;
use super::simple_action::SimpleAction;
use crate::domain::typed_parameter::TypedParameter;
use crate::error::ParserError;
use crate::lexer::TokenStream;

/// Enum to represent either an `Action` or a `DurativeAction`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    /// An Action wrapper around a simple action. See [`SimpleAction`](../simple_action/struct.SimpleAction.html).
    Simple(SimpleAction),
    /// An Action wrapper around a durative action. See [`DurativeAction`](../durative_action/struct.DurativeAction.html).
    Durative(DurativeAction),
}

impl From<SimpleAction> for Action {
    fn from(action: SimpleAction) -> Self {
        Self::Simple(action)
    }
}

impl From<DurativeAction> for Action {
    fn from(action: DurativeAction) -> Self {
        Self::Durative(action)
    }
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
    pub fn parameters(&self) -> &[TypedParameter] {
        match self {
            Self::Simple(action) => &action.parameters,
            Self::Durative(action) => &action.parameters,
        }
    }

    /// Get the precondition of the action. This is the same as the precondition of the simple or durative action.
    pub fn precondition(&self) -> Option<Expression> {
        match self {
            Self::Simple(action) => action.precondition.clone(),
            Self::Durative(action) => action.condition.clone(),
        }
    }

    /// Get the effect of the action. This is the same as the effect of the simple or durative action.
    pub fn effect(&self) -> Expression {
        match self {
            Self::Simple(action) => action.effect.clone(),
            Self::Durative(action) => action.effect.clone(),
        }
    }

    /// Parse an action from a token stream.
    pub fn parse(input: TokenStream) -> IResult<TokenStream, Action, ParserError> {
        alt((
            map(SimpleAction::parse, Self::Simple),
            map(DurativeAction::parse, Self::Durative),
        ))(input)
    }

    /// Convert the action to PDDL.
    pub fn to_pddl(&self) -> String {
        match self {
            Self::Simple(action) => action.to_pddl(),
            Self::Durative(action) => action.to_pddl(),
        }
    }
}
