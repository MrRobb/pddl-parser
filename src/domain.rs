use std::string::ToString;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::tokens::{id, var, ws};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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

    // PDDL+
    Time,
}

impl Requirement {
    fn parse_requirement(input: &str) -> IResult<&str, Requirement, ParserError> {
        alt((
            // PDDL 1
            alt((
                map(tag(":strips"), |_| Requirement::Strips),
                map(tag(":typing"), |_| Requirement::Typing),
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
                map(tag(":action-expansions"), |_| Requirement::ActionExpansions),
                map(tag(":foreach-expansions"), |_| Requirement::ForeachExpansions),
                map(tag(":dag-expansions"), |_| Requirement::DagExpansions),
                map(tag(":domain-axioms"), |_| Requirement::DomainAxioms),
                map(tag(":subgoals-through-axioms"), |_| Requirement::SubgoalsThroughAxioms),
                map(tag(":safety-constraints"), |_| Requirement::SafetyConstraints),
                map(tag(":expression-evaluation"), |_| Requirement::ExpressionEvaluation),
                map(tag(":fluents"), |_| Requirement::Fluents),
                map(tag(":open-world"), |_| Requirement::OpenWorld),
                map(tag(":true-negation"), |_| Requirement::TrueNegation),
                map(tag(":adl"), |_| Requirement::Adl),
                map(tag(":ucpop"), |_| Requirement::Ucpop),
            )),
            // PDDL 2.1
            alt((
                map(tag(":numeric-fluents"), |_| Requirement::NumericFluents),
                map(tag(":durative-actions"), |_| Requirement::DurativeActions),
                map(tag(":durative-inequalities"), |_| Requirement::DurativeInequalities),
                map(tag(":continuous-effects"), |_| Requirement::ContinuousEffects),
                map(tag(":negative-preconditions"), |_| Requirement::NegativePreconditions),
            )),
            // PDDL 2.2
            alt((
                map(tag(":derived-predicates"), |_| Requirement::DerivedPredicates),
                map(tag(":timed-initial-literals"), |_| Requirement::TimedInitialLiterals),
            )),
            // PDDL 3
            alt((
                map(tag(":preferences"), |_| Requirement::Preferences),
                map(tag(":constraints"), |_| Requirement::Constraints),
            )),
            // PDDL+
            alt((map(tag(":time"), |_| Requirement::Time),)),
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
        puffin::profile_function!();
        let (_, domain) = ws(delimited(
            char('('),
            preceded(ws(tag("define")), ws(Domain::parse_domain)),
            char(')'),
        ))(input)?;
        Ok(domain)
    }

    fn parse_domain(input: &str) -> IResult<&str, Domain, ParserError> {
        puffin::profile_function!();
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

    fn parse_name(input: &str) -> IResult<&str, String, ParserError> {
        puffin::profile_function!();
        let (output, name) = delimited(char('('), preceded(ws(tag("domain")), ws(id)), char(')'))(input)?;
        Ok((output, name.to_string()))
    }

    fn parse_requirements(input: &str) -> IResult<&str, Vec<Requirement>, ParserError> {
        puffin::profile_function!();
        let (output, requirements) = delimited(
            char('('),
            preceded(ws(tag(":requirements")), many0(ws(Requirement::parse_requirement))),
            char(')'),
        )(input)?;

        for requirement in &requirements {
            match requirement {
                Requirement::Strips | Requirement::Typing => (),
                e => return Err(nom::Err::Error(ParserError::UnsupportedRequirement(e.clone()))),
            }
        }

        Ok((output, requirements))
    }

    fn parse_types(input: &str) -> IResult<&str, Vec<Type>, ParserError> {
        puffin::profile_function!();
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
                    name: name.to_string(),
                    parent: parent.to_string(),
                })
            })
            .collect();
        Ok((output, types))
    }

    fn parse_predicates(input: &str) -> IResult<&str, Vec<Predicate>, ParserError> {
        puffin::profile_function!();
        let (output, predicates) = delimited(
            char('('),
            preceded(
                ws(tag(":predicates")),
                many0(ws(delimited(char('('), pair(ws(id), parse_parameters), char(')')))),
            ),
            char(')'),
        )(input)?;
        let predicates = predicates
            .into_iter()
            .map(|(name, parameters)| Predicate {
                name: name.to_string(),
                parameters,
            })
            .collect();
        Ok((output, predicates))
    }

    fn parse_actions(input: &str) -> IResult<&str, Vec<Action>, ParserError> {
        puffin::profile_function!();
        let (output, actions) = many0(ws(map(
            delimited(
                char('('),
                preceded(
                    ws(tag(":action")),
                    tuple((
                        ws(id),
                        ws(preceded(
                            tag(":parameters"),
                            ws(delimited(char('('), ws(parse_parameters), char(')'))),
                        )),
                        ws(preceded(tag(":precondition"), ws(Expression::parse_expression))),
                        ws(preceded(tag(":effect"), ws(Expression::parse_expression))),
                    )),
                ),
                char(')'),
            ),
            |(name, parameters, precondition, effect)| Action {
                name: name.to_string(),
                parameters,
                precondition,
                effect,
            },
        )))(input)?;
        Ok((output, actions))
    }
}

fn parse_parameters(input: &str) -> IResult<&str, Vec<Parameter>, ParserError> {
    puffin::profile_function!();
    let (output, params) = ws(many0(separated_pair(many1(ws(var)), opt(char('-')), opt(ws(id)))))(input)?;
    let params = params
        .into_iter()
        .flat_map(|(names, type_)| {
            names.into_iter().map(move |name| Parameter {
                name: name.to_string(),
                type_: type_.as_ref().map_or_else(object, ToString::to_string),
            })
        })
        .collect();
    Ok((output, params))
}

impl Expression {
    pub fn parse_expression(input: &str) -> IResult<&str, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = ws(alt((Self::parse_and, Self::parse_not, ws(Self::parse_predicate))))(input)?;
        Ok((output, expression))
    }

    fn parse_and(input: &str) -> IResult<&str, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expressions) = delimited(
            char('('),
            preceded(ws(tag("and")), many0(Expression::parse_expression)),
            char(')'),
        )(input)?;
        Ok((output, Expression::And(expressions)))
    }

    fn parse_not(input: &str) -> IResult<&str, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = delimited(
            char('('),
            preceded(ws(tag("not")), Expression::parse_expression),
            char(')'),
        )(input)?;
        Ok((output, Expression::Not(Box::new(expression))))
    }

    fn parse_predicate(input: &str) -> IResult<&str, Expression, ParserError> {
        puffin::profile_function!();
        let (output, expression) = map(
            delimited(char('('), pair(ws(id), ws(parse_parameters)), char(')')),
            |(name, parameters)| Expression::Predicate {
                name: name.to_string(),
                parameters,
            },
        )(input)?;
        Ok((output, expression))
    }
}
