use serde::{Deserialize, Serialize};

pub type Parameter = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan(Vec<Action>);
