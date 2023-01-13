use std::error::Error;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type Parameter = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Plan(Vec<Action>);

impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.name, self.parameters.join(", "))
	}
}

impl Plan {
	pub fn parse(pddl_domain: &str) -> Result<Self, Box<dyn Error>> {
		let domain: Self = ron::from_str(pddl_domain)?;
		Ok(domain)
	}

	pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
		ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default().struct_names(true))
			.map_err(std::convert::Into::into)
	}

	pub fn actions(&self) -> impl Iterator<Item = &Action> {
		self.0.iter()
	}
}
