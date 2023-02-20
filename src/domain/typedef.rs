use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeDef {
    pub name: String,
    pub parent: String,
}
