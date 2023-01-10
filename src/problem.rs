use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Problem {
	pub name: String,
	pub domain: String,
	#[serde(default)]
	pub objects: Vec<Object>,
	#[serde(default)]
	pub init: Vec<Predicate>,
	#[serde(default)]
	pub goal: Vec<Predicate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
	pub name: String,
	#[serde(rename = "type")]
	pub type_: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Predicate {
	pub name: String,
	#[serde(default)]
	pub args: Vec<String>,
}

impl Problem {
	pub fn parse(pddl_domain: &str) -> Result<Self, Box<dyn Error>> {
		let domain: Self = ron::from_str(pddl_domain)?;
		Ok(domain)
	}

	pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
		ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default().struct_names(true))
			.map_err(std::convert::Into::into)
	}
}
