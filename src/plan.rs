use std::error::Error;
use std::fmt::Display;

use nom::character::complete::{alphanumeric0, char, space1};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::{
    error::ParserError,
    tokens::{id, ws},
};

pub type Parameter = String;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl Action {
    fn parse(input: &str) -> IResult<&str, Self, ParserError> {
        let (output, (name, parameters)) = delimited(
            char('('),
            separated_pair(Action::parse_name, space1, Action::parse_parameters),
            char(')'),
        )(input)?;
        Ok((output, Action { name, parameters }))
    }

    fn parse_name(input: &str) -> IResult<&str, String, ParserError> {
        let (output, name) = id(input)?;
        Ok((output, name.to_string()))
    }

    fn parse_parameters(input: &str) -> IResult<&str, Vec<Parameter>, ParserError> {
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
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let (_, plan) = many0(ws(Action::parse))(input).map_err(|e| format!("Failed to parse plan: {e}"))?;
        Ok(Plan(plan))
    }

    pub fn actions(&self) -> impl Iterator<Item = &Action> {
        self.0.iter()
    }
}
