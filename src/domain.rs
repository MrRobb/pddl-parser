use std::string::ToString;

use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::{id, var};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Requirement {
    // PDDL 1
    Strips,
    Typing,
    DisjunctivePreconditions,
    Equality,
    ExistentialPreconditions,
    UniversalPreconditions,
    QuantifiedPreconditions,
    ConditionalEffects,
    ActionExpansions,
    ForeachExpansions,
    DagExpansions,
    DomainAxioms,
    SubgoalsThroughAxioms,
    SafetyConstraints,
    ExpressionEvaluation,
    Fluents,
    OpenWorld,
    TrueNegation,
    Adl,
    Ucpop,

    // PDDL 2.1
    NumericFluents,
    DurativeActions,
    DurativeInequalities,
    ContinuousEffects,
    NegativePreconditions,

    // PDDL 2.2
    DerivedPredicates,
    TimedInitialLiterals,

    // PDDL 3
    Preferences,
    Constraints,

    // PDDL 3.1
    ActionCosts,
    GoalUtilities,

    // PDDL+
    Time,
}

impl Requirement {
    fn parse_requirement(input: TokenStream) -> IResult<TokenStream, Requirement, ParserError> {
        alt((
            // PDDL 1
            alt((
                map(Token::Strips, |_| Requirement::Strips),
                map(Token::Typing, |_| Requirement::Typing),
                map(Token::DisjunctivePreconditions, |_| {
                    Requirement::DisjunctivePreconditions
                }),
                map(Token::Equality, |_| Requirement::Equality),
                map(Token::ExistentialPreconditions, |_| {
                    Requirement::ExistentialPreconditions
                }),
                map(Token::UniversalPreconditions, |_| Requirement::UniversalPreconditions),
                map(Token::QuantifiedPreconditions, |_| Requirement::QuantifiedPreconditions),
                map(Token::ConditionalEffects, |_| Requirement::ConditionalEffects),
                map(Token::ActionExpansions, |_| Requirement::ActionExpansions),
                map(Token::ForeachExpansions, |_| Requirement::ForeachExpansions),
                map(Token::DagExpansions, |_| Requirement::DagExpansions),
                map(Token::DomainAxioms, |_| Requirement::DomainAxioms),
                map(Token::SubgoalsThroughAxioms, |_| Requirement::SubgoalsThroughAxioms),
                map(Token::SafetyConstraints, |_| Requirement::SafetyConstraints),
                map(Token::ExpressionEvaluation, |_| Requirement::ExpressionEvaluation),
                map(Token::Fluents, |_| Requirement::Fluents),
                map(Token::OpenWorld, |_| Requirement::OpenWorld),
                map(Token::TrueNegation, |_| Requirement::TrueNegation),
                map(Token::Adl, |_| Requirement::Adl),
                map(Token::Ucpop, |_| Requirement::Ucpop),
            )),
            // PDDL 2.1
            alt((
                map(Token::NumericFluents, |_| Requirement::NumericFluents),
                map(Token::DurativeActions, |_| Requirement::DurativeActions),
                map(Token::DurativeInequalities, |_| Requirement::DurativeInequalities),
                map(Token::ContinuousEffects, |_| Requirement::ContinuousEffects),
                map(Token::NegativePreconditions, |_| Requirement::NegativePreconditions),
            )),
            // PDDL 2.2
            alt((
                map(Token::DerivedPredicates, |_| Requirement::DerivedPredicates),
                map(Token::TimedInitialLiterals, |_| Requirement::TimedInitialLiterals),
            )),
            // PDDL 3
            alt((
                map(Token::Preferences, |_| Requirement::Preferences),
                map(Token::Constraints, |_| Requirement::Constraints),
            )),
            // PDDL 3.1
            alt((
                map(Token::ActionCosts, |_| Requirement::ActionCosts),
                map(Token::GoalUtilities, |_| Requirement::GoalUtilities),
            )),
            // PDLL+
            map(Token::Time, |_| Requirement::Time),
        ))(input)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub parent: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

fn object() -> String {
    "object".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default = "object")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Expression {
    Predicate {
        name: String,
        #[serde(default)]
        parameters: Vec<Parameter>,
    },
    And(Vec<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    pub precondition: Expression,
    pub effect: Expression,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Domain {
    pub name: String,
    pub requirements: Vec<Requirement>,
    pub types: Vec<Type>,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>,
}

impl Domain {
    pub fn parse(input: TokenStream) -> Result<Self, ParserError> {
        puffin::profile_function!();
        let (_, domain) = delimited(
            Token::OpenParen,
            preceded(Token::Define, Domain::parse_domain),
            Token::CloseParen,
        )(input)?;
        Ok(domain)
    }

    fn parse_domain(input: TokenStream) -> IResult<TokenStream, Domain, ParserError> {
        puffin::profile_function!();
        let (output, (name, requirements, types, predicates, actions)) = tuple((
            Domain::parse_name,
            Domain::parse_requirements,
            opt(Domain::parse_types),
            Domain::parse_predicates,
            Domain::parse_actions,
        ))(input)?;
        Ok((
            output,
            Domain {
                name,
                requirements,
                types: types.unwrap_or_default(),
                predicates,
                actions,
            },
        ))
    }

    fn parse_name(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        puffin::profile_function!();
        let (output, name) = delimited(Token::OpenParen, preceded(Token::Domain, id), Token::CloseParen)(input)?;
        Ok((output, name))
    }

    fn parse_requirements(input: TokenStream) -> IResult<TokenStream, Vec<Requirement>, ParserError> {
        puffin::profile_function!();
        let (output, requirements) = delimited(
            Token::OpenParen,
            preceded(Token::Requirements, many0(Requirement::parse_requirement)),
            Token::CloseParen,
        )(input)?;

        for requirement in &requirements {
            match requirement {
                Requirement::Strips | Requirement::Typing => (),
                e => return Err(nom::Err::Error(ParserError::UnsupportedRequirement(e.clone()))),
            }
        }

        Ok((output, requirements))
    }

    fn parse_types(input: TokenStream) -> IResult<TokenStream, Vec<Type>, ParserError> {
        puffin::profile_function!();
        let (output, types) = delimited(
            Token::OpenParen,
            preceded(Token::Types, many0(separated_pair(many0(id), Token::Dash, id))),
            Token::CloseParen,
        )(input)?;
        let types = types
            .into_iter()
            .flat_map(|(names, parent)| {
                names.into_iter().map(move |name| Type {
                    name,
                    parent: parent.to_string(),
                })
            })
            .collect();
        Ok((output, types))
    }

    fn parse_predicates(input: TokenStream) -> IResult<TokenStream, Vec<Predicate>, ParserError> {
        puffin::profile_function!();
        let (output, predicates) = delimited(
            Token::OpenParen,
            preceded(
                Token::Predicates,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, parse_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        )(input)?;
        let predicates = predicates
            .into_iter()
            .map(|(name, parameters)| Predicate { name, parameters })
            .collect();
        Ok((output, predicates))
    }

    fn parse_actions(input: TokenStream) -> IResult<TokenStream, Vec<Action>, ParserError> {
        puffin::profile_function!();
        let (output, actions) = many0(map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Action,
                    tuple((
                        id,
                        preceded(
                            Token::Parameters,
                            delimited(Token::OpenParen, parse_parameters, Token::CloseParen),
                        ),
                        preceded(Token::Precondition, Expression::parse_expression),
                        preceded(Token::Effect, Expression::parse_expression),
                    )),
                ),
                Token::CloseParen,
            ),
            |(name, parameters, precondition, effect)| Action {
                name,
                parameters,
                precondition,
                effect,
            },
        ))(input)?;
        Ok((output, actions))
    }
}

fn parse_parameters(input: TokenStream) -> IResult<TokenStream, Vec<Parameter>, ParserError> {
    puffin::profile_function!();
    let (output, params) = many0(separated_pair(many1(var), opt(Token::Dash), opt(id)))(input)?;
    let params = params
        .into_iter()
        .flat_map(|(names, type_)| {
            names.into_iter().map(move |name| Parameter {
                name,
                type_: type_.as_ref().map_or_else(object, ToString::to_string),
            })
        })
        .collect();
    Ok((output, params))
}

impl Expression {
    pub fn parse_expression(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = alt((Self::parse_and, Self::parse_not, Self::parse_predicate))(input)?;
        Ok((output, expression))
    }

    fn parse_and(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expressions) = delimited(
            Token::OpenParen,
            preceded(Token::And, many0(Expression::parse_expression)),
            Token::CloseParen,
        )(input)?;
        Ok((output, Expression::And(expressions)))
    }

    fn parse_not(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = delimited(
            Token::OpenParen,
            preceded(Token::Not, Expression::parse_expression),
            Token::CloseParen,
        )(input)?;
        Ok((output, Expression::Not(Box::new(expression))))
    }

    fn parse_predicate(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = map(
            delimited(Token::OpenParen, pair(id, parse_parameters), Token::CloseParen),
            |(name, parameters)| Expression::Predicate { name, parameters },
        )(input)?;
        Ok((output, expression))
    }
}
