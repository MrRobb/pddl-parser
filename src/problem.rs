use std::string::ToString;

use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::Expression;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

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
        let (output, init) = delimited(
            Token::OpenParen,
            preceded(
                Token::Init,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, Problem::parse_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        )(input)?;
        let init = init.into_iter().map(|(name, args)| Predicate { name, args }).collect();
        Ok((output, init))
    }

    fn parse_parameters(input: TokenStream) -> IResult<TokenStream, Vec<String>, ParserError> {
        let (output, parameters) = many0(id)(input)?;
        Ok((output, parameters))
    }

    fn parse_goal(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        let (output, goal) = delimited(
            Token::OpenParen,
            preceded(Token::Goal, Expression::parse_expression),
            Token::CloseParen,
        )(input)?;
        Ok((output, goal))
    }
}
