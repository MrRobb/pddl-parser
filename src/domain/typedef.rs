use serde::{Deserialize, Serialize};

/// A type definition.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeDef {
    /// The name of the type.
    pub name: String,
    /// The parent type. If not specified, the parent type is `object`.
    pub parent: String,
}

impl TypeDef {
    /// Convert the type definition to PDDL.
    pub fn to_pddl(&self) -> String {
        format!("{} - {}", self.name, self.parent)
    }
}
