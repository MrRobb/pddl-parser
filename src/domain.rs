use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::{
    error::ParserError,
    tokens::{id, var, ws},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Requirement {
    Strips,
    Typing,
    NegativePreconditions,
    DisjunctivePreconditions,
    Equality,
    ExistentialPreconditions,
    UniversalPreconditions,
    QuantifiedPreconditions,
    ConditionalEffects,
    FluentActions,
    Adl,
    DurativeActions,
    DurativeAtEnd,
    ContinuousEffects,
    DerivedPredicates,
    TimedInitialLiterals,
    Preferences,
    Constraints,
    ActionCosts,
}

impl Requirement {
    fn parse_requirement(input: &str) -> IResult<&str, Requirement> {
        alt((
            map(tag(":strips"), |_| Requirement::Strips),
            map(tag(":typing"), |_| Requirement::Typing),
            map(tag(":negative-preconditions"), |_| Requirement::NegativePreconditions),
            map(tag(":disjunctive-preconditions"), |_| {
                Requirement::DisjunctivePreconditions
            }),
            map(tag(":equality"), |_| Requirement::Equality),
            map(tag(":existential-preconditions"), |_| {
                Requirement::ExistentialPreconditions
            }),
            map(tag(":universal-preconditions"), |_| Requirement::UniversalPreconditions),
            map(tag(":quantified-preconditions"), |_| {
                Requirement::QuantifiedPreconditions
            }),
            map(tag(":conditional-effects"), |_| Requirement::ConditionalEffects),
            map(tag(":fluent-actions"), |_| Requirement::FluentActions),
            map(tag(":adl"), |_| Requirement::Adl),
            map(tag(":durative-actions"), |_| Requirement::DurativeActions),
            map(tag(":durative-at-end"), |_| Requirement::DurativeAtEnd),
            map(tag(":continuous-effects"), |_| Requirement::ContinuousEffects),
            map(tag(":derived-predicates"), |_| Requirement::DerivedPredicates),
            map(tag(":timed-initial-literals"), |_| Requirement::TimedInitialLiterals),
            map(tag(":preferences"), |_| Requirement::Preferences),
            map(tag(":constraints"), |_| Requirement::Constraints),
            map(tag(":action-costs"), |_| Requirement::ActionCosts),
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
    pub fn parse(input: &str) -> Result<Self, ParserError> {
        let (_, domain) = delimited(
            char('('),
            preceded(ws(tag("define")), ws(Domain::parse_domain)),
            char(')'),
        )(input)?;
        Ok(domain)
    }

    fn parse_domain(input: &str) -> IResult<&str, Domain> {
        let (output, (name, requirements, types, predicates, actions)) = tuple((
            ws(Domain::parse_name),
            ws(Domain::parse_requirements),
            ws(opt(Domain::parse_types)),
            ws(Domain::parse_predicates),
            ws(Domain::parse_actions),
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

    fn parse_name(input: &str) -> IResult<&str, String> {
        let (output, name) = delimited(char('('), preceded(ws(tag("domain")), ws(id)), char(')'))(input)?;
        Ok((output, name))
    }

    fn parse_requirements(input: &str) -> IResult<&str, Vec<Requirement>> {
        let (output, requirements) = delimited(
            char('('),
            preceded(ws(tag(":requirements")), many0(ws(Requirement::parse_requirement))),
            char(')'),
        )(input)?;
        Ok((output, requirements))
    }

    fn parse_types(input: &str) -> IResult<&str, Vec<Type>> {
        let (output, types) = delimited(
            char('('),
            preceded(
                ws(tag(":types")),
                ws(many0(separated_pair(many0(ws(id)), char('-'), ws(id)))),
            ),
            char(')'),
        )(input)?;
        let types = types
            .into_iter()
            .flat_map(|(names, parent)| {
                names.into_iter().map(move |name| Type {
                    name,
                    parent: parent.clone(),
                })
            })
            .collect();
        Ok((output, types))
    }

    fn parse_predicates(input: &str) -> IResult<&str, Vec<Predicate>> {
        let (output, predicates) = delimited(
            char('('),
            preceded(
                ws(tag(":predicates")),
                many0(ws(delimited(
                    char('('),
                    pair(ws(id), Domain::parse_parameters),
                    char(')'),
                ))),
            ),
            char(')'),
        )(input)?;
        let predicates = predicates
            .into_iter()
            .map(|(name, parameters)| Predicate { name, parameters })
            .collect();
        Ok((output, predicates))
    }

    fn parse_parameters(input: &str) -> IResult<&str, Vec<Parameter>> {
        let (output, params) = many0(separated_pair(many1(ws(var)), opt(char('-')), opt(ws(id))))(input)?;
        let params = params
            .into_iter()
            .flat_map(|(names, type_)| {
                names.into_iter().map(move |name| Parameter {
                    name,
                    type_: type_.clone().unwrap_or_else(object),
                })
            })
            .collect();
        Ok((output, params))
    }

    fn parse_actions(input: &str) -> IResult<&str, Vec<Action>> {
        let (output, actions) = many0(ws(delimited(
            char('('),
            preceded(
                ws(tag(":action")),
                tuple((
                    ws(id),
                    ws(preceded(
                        tag(":parameters"),
                        ws(delimited(char('('), ws(Domain::parse_parameters), char(')'))),
                    )),
                    ws(preceded(tag(":precondition"), ws(Domain::parse_expression))),
                    ws(preceded(tag(":effect"), ws(Domain::parse_expression))),
                )),
            ),
            char(')'),
        )))(input)?;
        let actions = actions
            .into_iter()
            .map(|(name, parameters, precondition, effect)| Action {
                name,
                parameters,
                precondition,
                effect,
            })
            .collect();
        Ok((output, actions))
    }

    fn parse_expression(input: &str) -> IResult<&str, Expression> {
        let (output, expression) = alt((Self::parse_and, Self::parse_not, Self::parse_pred))(input)?;
        Ok((output, expression))
    }

    fn parse_and(input: &str) -> IResult<&str, Expression> {
        let (output, expressions) = map(
            ws(delimited(
                char('('),
                preceded(ws(tag("and")), many0(ws(Domain::parse_expression))),
                char(')'),
            )),
            Expression::And,
        )(input)?;
        Ok((output, expressions))
    }

    fn parse_not(input: &str) -> IResult<&str, Expression> {
        let (output, expressions) = map(
            ws(delimited(
                char('('),
                preceded(ws(tag("not")), ws(Domain::parse_expression)),
                char(')'),
            )),
            |e| Expression::Not(Box::new(e)),
        )(input)?;
        Ok((output, expressions))
    }

    fn parse_pred(input: &str) -> IResult<&str, Expression> {
        let (output, expressions) = map(
            ws(delimited(
                char('('),
                pair(ws(id), ws(Self::parse_parameters)),
                char(')'),
            )),
            |(name, parameters)| Expression::Predicate { name, parameters },
        )(input)?;
        Ok((output, expressions))
    }
}
