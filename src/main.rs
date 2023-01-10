use ron::ser::{to_string_pretty, PrettyConfig};

use serde::{Deserialize, Serialize};

fn main() {
	let domain: Domain = ron::from_str(include_str!("../tests/domain.ron")).unwrap();
	println!(
		"{}",
		to_string_pretty(&domain, PrettyConfig::new().struct_names(true)).unwrap()
	);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
	pub name: String,
	pub requirements: Vec<String>,
	pub types: Vec<Type>,
	pub predicates: Vec<Predicate>,
	pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
	pub name: String,
	pub parent: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Predicate {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
	pub precondition: Expression,
	pub effect: Expression,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameter {
	pub name: String,
	#[serde(rename = "type")]
	#[serde(default = "object")]
	pub type_: String,
}

fn object() -> String {
	"object".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Expression {
	Predicate(Predicate),
	And(And),
	Not(Not),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct And(Vec<Expression>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Not(Box<Expression>);
