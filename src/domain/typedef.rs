use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeDef {
    pub name: String,
    pub parent: String,
}

impl TypeDef {
    pub fn to_pddl(&self) -> String {
        format!("{} - {})", self.name, self.parent)
    }
}
