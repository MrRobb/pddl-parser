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
