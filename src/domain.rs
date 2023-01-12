use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Domain {
	pub name: String,
	pub requirements: Vec<String>,
	pub types: Vec<Type>,
	pub predicates: Vec<Predicate>,
	pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Type {
	pub name: String,
	pub parent: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Predicate {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
	pub precondition: Expression,
	pub effect: Expression,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Parameter {
	pub name: String,
	#[serde(rename = "type")]
	#[serde(default = "object")]
	pub type_: String,
}

fn object() -> String {
	"object".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Expression {
	Predicate(Predicate),
	And(And),
	Not(Not),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct And(pub Vec<Expression>);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Not(pub Box<Expression>);

impl Domain {
	pub fn parse(pddl_domain: &str) -> Result<Self, Box<dyn Error>> {
		let domain: Self = ron::from_str(pddl_domain)?;
		Ok(domain)
	}

	pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
		ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default().struct_names(true))
			.map_err(std::convert::Into::into)
	}
}
