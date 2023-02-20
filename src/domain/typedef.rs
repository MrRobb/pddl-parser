use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypeDef {
    pub name: String,
    pub parent: String,
}
