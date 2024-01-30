use super::{durative_action::DurativeAction, simple_action::SimpleAction};
use serde::{Deserialize, Serialize};

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
    pub fn to_pddl(&self) -> String {
        match self {
            Self::Simple(action) => action.to_pddl(),
            Self::Durative(action) => action.to_pddl(),
        }
    }
}
