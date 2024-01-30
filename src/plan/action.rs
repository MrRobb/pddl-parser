use serde::{Deserialize, Serialize};

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
}
