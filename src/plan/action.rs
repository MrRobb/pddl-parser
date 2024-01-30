use serde::{Deserialize, Serialize};

use super::durative_action::DurativeAction;
use super::simple_action::SimpleAction;

/// Enum to represent either an `Action` or a `DurativeAction`.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum Action {
    Simple(SimpleAction),
    Durative(DurativeAction),
}
