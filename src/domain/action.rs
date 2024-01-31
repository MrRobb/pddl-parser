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
    Simple(SimpleAction),
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
    pub fn name(&self) -> &str {
        match self {
            Self::Simple(action) => &action.name,
            Self::Durative(action) => &action.name,
        }
    }

    pub fn parameters(&self) -> &[TypedParameter] {
        match self {
            Self::Simple(action) => &action.parameters,
            Self::Durative(action) => &action.parameters,
        }
    }

    pub fn precondition(&self) -> Option<Expression> {
        match self {
            Self::Simple(action) => action.precondition.clone(),
            Self::Durative(action) => action.condition.clone(),
        }
    }

    pub fn effect(&self) -> Expression {
        match self {
            Self::Simple(action) => action.effect.clone(),
            Self::Durative(action) => action.effect.clone(),
        }
    }

    pub fn parse(input: TokenStream) -> IResult<TokenStream, Action, ParserError> {
        alt((
            map(SimpleAction::parse, Self::Simple),
            map(DurativeAction::parse, Self::Durative),
        ))(input)
    }

    pub fn to_pddl(&self) -> String {
        match self {
            Self::Simple(action) => action.to_pddl(),
            Self::Durative(action) => action.to_pddl(),
        }
    }
}
