use std::fmt::Display;

use crate::tokens::parse_id;
use nom::{
	character::complete::line_ending,
	multi::many0,
	sequence::{delimited, terminated},
	IResult,
};
use nom::{
	character::complete::{alphanumeric0, char, space1},
	combinator::map,
	multi::separated_list0,
	sequence::separated_pair,
};
use serde::{Deserialize, Serialize};

pub type Parameter = String;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Action {
	pub name: String,
	#[serde(default)]
	pub parameters: Vec<Parameter>,
}

impl Action {
	fn parse(input: &str) -> IResult<&str, Self> {
		println!("parse: {}", input);
		let (output, (name, parameters)) = delimited(
			char('('),
			separated_pair(Action::parse_name, space1, Action::parse_parameters),
			char(')'),
		)(input)?;
		Ok((output, Action { name, parameters }))
	}

	fn parse_name(input: &str) -> IResult<&str, String> {
		println!("parse_name: {}", input);
		let (output, name) = parse_id(input)?;
		Ok((output, name))
	}

	fn parse_parameters(input: &str) -> IResult<&str, Vec<Parameter>> {
		println!("parse_parameters: {}", input);
		let (output, parameters) = separated_list0(space1, map(alphanumeric0, String::from))(input)?;
		Ok((output, parameters))
	}
}

impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.name, self.parameters.join(", "))
	}
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Plan(pub Vec<Action>);

impl Plan {
	pub fn parse(input: &str) -> IResult<&str, Self> {
		let (output, plan) = many0(terminated(Action::parse, line_ending))(input).unwrap();
		Ok((output, Plan(plan)))
	}

	pub fn actions(&self) -> impl Iterator<Item = &Action> {
		self.0.iter()
	}
}
