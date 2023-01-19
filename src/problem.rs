use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::tokens::{id, ws};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Object {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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

impl Problem {
    pub fn parse(input: &str) -> Result<Self, ParserError> {
        let (_, problem) = delimited(
            char('('),
            preceded(ws(tag("define")), ws(Problem::parse_problem)),
            char(')'),
        )(input)?;
        Ok(problem)
    }

    fn parse_problem(input: &str) -> IResult<&str, Problem, ParserError> {
        let (output, (name, domain, objects, init, goal)) = tuple((
            ws(Problem::parse_name),
            ws(Problem::parse_domain),
            ws(Problem::parse_objects),
            ws(Problem::parse_init),
            ws(Problem::parse_goal),
        ))(input)?;
        Ok((
            output,
            Problem {
                name,
                domain,
                objects,
                init,
                goal,
            },
        ))
    }

    fn parse_name(input: &str) -> IResult<&str, String, ParserError> {
        let (output, name) = delimited(char('('), preceded(ws(tag("problem")), ws(id)), char(')'))(input)?;
        Ok((output, name))
    }

    fn parse_domain(input: &str) -> IResult<&str, String, ParserError> {
        let (output, domain) = delimited(char('('), preceded(ws(tag(":domain")), ws(id)), char(')'))(input)?;
        Ok((output, domain))
    }

    fn parse_objects(input: &str) -> IResult<&str, Vec<Object>, ParserError> {
        let (output, objects) = delimited(
            char('('),
            preceded(
                ws(tag(":objects")),
                ws(many0(separated_pair(many0(ws(id)), char('-'), ws(id)))),
            ),
            char(')'),
        )(input)?;
        let objects = objects
            .into_iter()
            .flat_map(move |(names, type_)| {
                names
                    .into_iter()
                    .map(|name| Object {
                        name,
                        type_: type_.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok((output, objects))
    }

    fn parse_init(input: &str) -> IResult<&str, Vec<Predicate>, ParserError> {
        let (output, init) = delimited(
            char('('),
            preceded(
                ws(tag(":init")),
                many0(ws(delimited(
                    char('('),
                    pair(ws(id), Problem::parse_parameters),
                    char(')'),
                ))),
            ),
            char(')'),
        )(input)?;
        let init = init.into_iter().map(|(name, args)| Predicate { name, args }).collect();
        Ok((output, init))
    }

    fn parse_parameters(input: &str) -> IResult<&str, Vec<String>, ParserError> {
        let (output, parameters) = many0(ws(id))(input)?;
        Ok((output, parameters))
    }

    fn parse_goal(input: &str) -> IResult<&str, Vec<Predicate>, ParserError> {
        let (output, goal) = delimited(
            char('('),
            preceded(
                ws(tag(":goal")),
                many0(ws(delimited(
                    char('('),
                    pair(ws(id), Problem::parse_parameters),
                    char(')'),
                ))),
            ),
            char(')'),
        )(input)?;
        let goal = goal.into_iter().map(|(name, args)| Predicate { name, args }).collect();
        Ok((output, goal))
    }
}
