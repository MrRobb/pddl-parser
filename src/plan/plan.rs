use nom::multi::many0;
use serde::{Deserialize, Serialize};

use super::action::Action;
use super::durative_action::DurativeAction;
use nom::branch::alt;
use nom::combinator::map;

use crate::error::ParserError;
use crate::lexer::TokenStream;

/// Enum to represent either an `Action` or a `DurativeAction`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum PlanItem {
    Simple(Action),
    Durative(DurativeAction),
}

/// A plan is a sequence of actions.
///
/// The order of the actions is important. Plan is a wrapper around a `Vec<Action>` that implements `IntoIterator` and `FromIterator<Action>`. This might change in the future.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Plan(pub Vec<PlanItem>);

impl Plan {
    /// Parse a plan from a token stream.
    ///
    /// The plan must be a sequence of actions. The parser will fail if there are any tokens left after the plan.
    ///
    /// # Errors
    ///
    /// The parser will fail if there are any tokens left after the plan. It will also fail if the plan is empty or if any of the actions are invalid.
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (output, items) = alt((
            many0(map(DurativeAction::parse, PlanItem::Durative)),
            many0(map(Action::parse, PlanItem::Simple)),
        ))(input)?;
        if !output.is_empty() {
            log::error!("Plan parser failed: {:?}", output.to_string());
            return Err(ParserError::ExpectedEndOfInput);
        }
        Ok(Plan(items))
    }

    /// Get an iterator over the actions in the plan.
    pub fn actions(&self) -> impl Iterator<Item = &PlanItem> {
        self.0.iter()
    }
}

impl IntoIterator for Plan {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = PlanItem;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<PlanItem> for Plan {
    fn from_iter<T: IntoIterator<Item = PlanItem>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
