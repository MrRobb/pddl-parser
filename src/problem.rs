use std::string::ToString;

use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::expression::Expression;
use crate::domain::parameter::Parameter;
use crate::domain::predicate::Predicate;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Object {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Problem {
    pub name: String,
    pub domain: String,
    #[serde(default)]
    pub objects: Vec<Object>,
    #[serde(default)]
    pub init: Vec<Predicate>,
    pub goal: Expression,
}

impl Problem {
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (_, problem) = delimited(
            Token::OpenParen,
            preceded(Token::Define, Problem::parse_problem),
            Token::CloseParen,
        )(input)?;
        Ok(problem)
    }

    fn parse_problem(input: TokenStream) -> IResult<TokenStream, Problem, ParserError> {
        let (output, (name, domain, objects, init, goal)) = tuple((
            Problem::parse_name,
            Problem::parse_domain,
            Problem::parse_objects,
            Problem::parse_init,
            Problem::parse_goal,
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

    fn parse_name(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        let (output, name) = delimited(Token::OpenParen, preceded(Token::Problem, id), Token::CloseParen)(input)?;
        Ok((output, name))
    }

    fn parse_domain(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        let (output, domain) =
            delimited(Token::OpenParen, preceded(Token::ProblemDomain, id), Token::CloseParen)(input)?;
        Ok((output, domain))
    }

    fn parse_objects(input: TokenStream) -> IResult<TokenStream, Vec<Object>, ParserError> {
        let (output, objects) = delimited(
            Token::OpenParen,
            preceded(Token::Objects, many0(separated_pair(many0(id), Token::Dash, id))),
            Token::CloseParen,
        )(input)?;
        let objects = objects
            .into_iter()
            .flat_map(move |(names, type_)| {
                names
                    .into_iter()
                    .map(|name| Object {
                        name,
                        type_: type_.to_string(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok((output, objects))
    }

    fn parse_init(input: TokenStream) -> IResult<TokenStream, Vec<Predicate>, ParserError> {
        log::debug!("BEGIN > parse_init {:?}", input.span());
        let (output, init) = delimited(
            Token::OpenParen,
            preceded(
                Token::Init,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, Parameter::parse_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        )(input)?;
        let init = init
            .into_iter()
            .map(|(name, parameters)| Predicate { name, parameters })
            .collect();
        log::debug!("END < parse_init {:?}", output.span());
        Ok((output, init))
    }

    fn parse_goal(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        let (output, goal) = delimited(
            Token::OpenParen,
            preceded(Token::Goal, Expression::parse_expression),
            Token::CloseParen,
        )(input)?;
        Ok((output, goal))
    }

    pub fn to_pddl(&self) -> String {
        let mut pddl = String::new();

        // Name and domain
        pddl.push_str(&format!("(define (problem {})\n", self.name));
        pddl.push_str(&format!("(:domain {})\n", self.domain));

        // Objects
        pddl.push_str("(:objects\n");
        for object in &self.objects {
            pddl.push_str(&format!("{} - {}\n", object.name, object.type_));
        }
        pddl.push_str(")\n");

        // Init
        pddl.push_str("(:init\n");
        for predicate in &self.init {
            pddl.push_str(&predicate.to_pddl());
        }
        pddl.push_str(")\n");

        // Goal
        pddl.push_str(&format!("(:goal {})\n", &self.goal.to_pddl()));

        pddl
    }
}
