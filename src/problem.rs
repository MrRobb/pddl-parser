use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::expression::Expression;
use crate::domain::typing::Type;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// A PDDL object
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Object {
    /// The name of the object
    pub name: String,
    /// The type of the object
    #[serde(rename = "type")]
    pub type_: Type,
}

impl Object {
    /// Convert a typed object to a PDDL format. That is `name - type`.
    pub fn to_pddl(&self) -> String {
        format!("{} - {}", self.name, self.type_.to_pddl())
    }
}

/// A PDDL problem
///
/// A problem is a description of a particular planning problem. It consists of a domain, a set of objects, an initial state, and a goal state.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Problem {
    /// The name of the problem
    pub name: String,
    /// The name of the domain of the problem
    pub domain: String,
    /// The objects of the problem
    #[serde(default)]
    pub objects: Vec<Object>,
    /// The initial state of the problem
    #[serde(default)]
    pub init: Vec<Expression>,
    /// The goal of the problem
    pub goal: Expression,
}

impl Problem {
    /// Parse a PDDL problem
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        let (output, problem) = delimited(
            Token::OpenParen,
            preceded(Token::Define, Problem::parse_problem),
            Token::CloseParen,
        )(input)?;
        if !output.is_empty() {
            return Err(ParserError::ExpectedEndOfInput);
        }
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
            preceded(
                Token::Objects,
                many0(pair(many1(id), opt(preceded(Token::Dash, Type::parse_type)))),
            ),
            Token::CloseParen,
        )(input)?;
        let objects = objects
            .into_iter()
            .flat_map(move |(names, type_)| {
                names
                    .into_iter()
                    .map(|name| Object {
                        name,
                        type_: type_.clone().unwrap_or_default(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok((output, objects))
    }

    fn parse_init(input: TokenStream) -> IResult<TokenStream, Vec<Expression>, ParserError> {
        log::debug!("BEGIN > parse_init {:?}", input.span());
        let (output, init) = delimited(
            Token::OpenParen,
            preceded(Token::Init, many0(Expression::parse_expression)),
            Token::CloseParen,
        )(input)?;
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

    /// Convert the problem to PDDL format (as a string) for writing to a file
    pub fn to_pddl(&self) -> String {
        let mut pddl = String::new();

        // Name and domain
        pddl.push_str(&format!("(define (problem {})\n", self.name));
        pddl.push_str(&format!("(:domain {})\n", self.domain));

        // Objects
        pddl.push_str(&format!(
            "(:objects\n{}\n)\n",
            self.objects.iter().map(Object::to_pddl).collect::<Vec<_>>().join("\n")
        ));

        // Init
        pddl.push_str(&format!(
            "(:init\n{}\n)\n",
            self.init.iter().map(Expression::to_pddl).collect::<Vec<_>>().join("\n")
        ));

        // Goal
        pddl.push_str(&format!("(:goal\n{}\n)\n", &self.goal.to_pddl()));

        // End
        pddl.push(')');

        pddl
    }
}
