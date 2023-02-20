use std::string::ToString;

use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::{id, integer, var};

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
pub struct TypeDef {
    pub name: String,
    pub parent: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Type {
    Simple(String),
    Either(Vec<String>),
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        Type::Simple(s.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Constant {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: Type,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypedPredicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<TypedParameter>,
}

fn object() -> Type {
    "object".into()
}

pub type Parameter = String;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TypedParameter {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default = "object")]
    pub type_: Type,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Expression {
    Atom {
        name: String,
        #[serde(default)]
        parameters: Vec<Parameter>,
    },
    And(Vec<Expression>),
    Not(Box<Expression>),

    // Assign operator
    Assign(Box<Expression>, Box<Expression>),
    Increase(Box<Expression>, Box<Expression>),
    Decrease(Box<Expression>, Box<Expression>),
    ScaleUp(Box<Expression>, Box<Expression>),
    ScaleDown(Box<Expression>, Box<Expression>),
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    Number(i64),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum TypedExpression {
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
    pub parameters: Vec<TypedParameter>,
    pub precondition: Option<Expression>,
    pub effect: Expression,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Domain {
    pub name: String,
    pub requirements: Vec<Requirement>,
    pub types: Vec<TypeDef>,
    pub constants: Vec<Constant>,
    pub predicates: Vec<TypedPredicate>,
    pub functions: Vec<TypedPredicate>,
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
        log::debug!("BEGIN > parse_domain {:?}", input.span());
        let (output, (name, requirements, types, constants, predicates, functions, actions)) = tuple((
            Domain::parse_name,
            Domain::parse_requirements,
            opt(Domain::parse_types),
            opt(Domain::parse_constants),
            Domain::parse_predicates,
            Domain::parse_functions,
            Domain::parse_actions,
        ))(input)?;
        let domain = Domain {
            name,
            requirements,
            types: types.unwrap_or_default(),
            constants: constants.unwrap_or_default(),
            predicates,
            functions,
            actions,
        };
        log::debug!("END < parse_domain {:?}", output.span());
        // log::info!("Parsed domain: \n{domain:#?}");
        Ok((output, domain))
    }

    fn parse_name(input: TokenStream) -> IResult<TokenStream, String, ParserError> {
        log::debug!("BEGIN > parse_name {:?}", input.span());
        let (output, name) = delimited(Token::OpenParen, preceded(Token::Domain, id), Token::CloseParen)(input)?;
        log::debug!("END < parse_name {:?}", output.span());
        Ok((output, name))
    }

    fn parse_requirements(input: TokenStream) -> IResult<TokenStream, Vec<Requirement>, ParserError> {
        log::debug!("BEGIN > parse_requirements {:?}", input.span());
        let (output, requirements) = opt(delimited(
            Token::OpenParen,
            preceded(Token::Requirements, many0(Requirement::parse_requirement)),
            Token::CloseParen,
        ))(input)?;

        if let Some(requirements) = &requirements {
            for requirement in requirements.iter() {
                match requirement {
                    Requirement::Strips | Requirement::Typing => (),
                    e => return Err(nom::Err::Error(ParserError::UnsupportedRequirement(e.clone()))),
                }
            }
        }

        log::debug!("Requirements: {requirements:?}");
        log::debug!("END < parse_requirements {:?}", output.span());
        Ok((output, requirements.unwrap_or_default()))
    }

    fn parse_types(input: TokenStream) -> IResult<TokenStream, Vec<TypeDef>, ParserError> {
        log::debug!("BEGIN > parse_types {:?}", input.span());
        let (output, types) = delimited(
            Token::OpenParen,
            preceded(Token::Types, many0(pair(many1(id), opt(preceded(Token::Dash, id))))),
            Token::CloseParen,
        )(input)?;
        let types = types
            .into_iter()
            .flat_map(|(names, parent)| {
                names.into_iter().map(move |name| TypeDef {
                    name,
                    parent: parent.clone().unwrap_or_else(|| "object".to_string()),
                })
            })
            .collect();
        log::debug!("END < parse_types {:?}", output.span());
        Ok((output, types))
    }

    fn parse_typed_parameters(input: TokenStream) -> IResult<TokenStream, Vec<TypedParameter>, ParserError> {
        log::debug!("BEGIN > parse_typed_parameters {:?}", input.span());
        let (output, params) = many0(pair(many1(var), opt(preceded(Token::Dash, Self::parse_type))))(input)?;
        let params = params
            .into_iter()
            .flat_map(|(names, type_)| {
                names.into_iter().map(move |name| TypedParameter {
                    name,
                    type_: type_.clone().unwrap_or_else(object),
                })
            })
            .collect();
        log::debug!("Parsed typed parameters: {params:?}");
        log::debug!("END < parse_typed_parameters {:?}", output.span());
        Ok((output, params))
    }

    fn parse_type(input: TokenStream) -> IResult<TokenStream, Type, ParserError> {
        log::debug!("BEGIN > parse_type {:?}", input.span());
        let (output, type_) = alt((
            map(id, Type::Simple),
            map(
                delimited(Token::OpenParen, preceded(Token::Either, many1(id)), Token::CloseParen),
                Type::Either,
            ),
        ))(input)?;
        log::debug!("END < parse_type {:?}", output.span());
        Ok((output, type_))
    }

    fn parse_constants(input: TokenStream) -> IResult<TokenStream, Vec<Constant>, ParserError> {
        log::debug!("BEGIN > parse_constants {:?}", input.span());
        let (output, constants) = delimited(
            Token::OpenParen,
            preceded(
                Token::Constants,
                many0(separated_pair(many1(id), Token::Dash, Self::parse_type)),
            ),
            Token::CloseParen,
        )(input)?;
        let constants = constants
            .into_iter()
            .flat_map(|(names, type_)| {
                names.into_iter().map(move |name| Constant {
                    name,
                    type_: type_.clone(),
                })
            })
            .collect();
        log::debug!("END < parse_constants {:?}", output.span());
        Ok((output, constants))
    }

    fn parse_predicates(input: TokenStream) -> IResult<TokenStream, Vec<TypedPredicate>, ParserError> {
        log::debug!("BEGIN > parse_predicates {:?}", input.span());
        let (output, predicates) = delimited(
            Token::OpenParen,
            preceded(
                Token::Predicates,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, Self::parse_typed_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        )(input)?;
        let predicates = predicates
            .into_iter()
            .map(|(name, parameters)| TypedPredicate { name, parameters })
            .collect();
        log::debug!("END < parse_predicates {:?}", output.span());
        Ok((output, predicates))
    }

    fn parse_functions(input: TokenStream) -> IResult<TokenStream, Vec<TypedPredicate>, ParserError> {
        log::debug!("BEGIN > parse_functions {:?}", input.span());
        let (output, functions) = opt(delimited(
            Token::OpenParen,
            preceded(
                Token::Functions,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, Self::parse_typed_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        ))(input)?;
        let functions = functions
            .unwrap_or_default()
            .into_iter()
            .map(|(name, parameters)| TypedPredicate { name, parameters })
            .collect();
        log::debug!("END < parse_functions {:?}", output.span());
        Ok((output, functions))
    }

    fn parse_actions(input: TokenStream) -> IResult<TokenStream, Vec<Action>, ParserError> {
        log::debug!("BEGIN > parse_actions {:?}", input.span());
        log::debug!("Parsing actions: {:?}", input.peek_n(10));
        let (output, actions) = many0(map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Action,
                    tuple((
                        id,
                        preceded(
                            Token::Parameters,
                            delimited(Token::OpenParen, Self::parse_typed_parameters, Token::CloseParen),
                        ),
                        opt(preceded(Token::Precondition, Expression::parse_expression)),
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
        log::debug!("END < parse_actions {:?}", output.span());
        Ok((output, actions))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Expression {
    pub fn parse_expression(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_expression {:?}", input.span());
        let (output, expression) = alt((
            Self::parse_and,
            Self::parse_not,
            Self::parse_atom,
            // Assign op
            alt((
                Self::parse_assign,
                Self::parse_scale_up,
                Self::parse_scale_down,
                Self::parse_increase,
                Self::parse_decrease,
            )),
        ))(input)?;
        log::debug!("END < parse_expression {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_and(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_and {:?}", input.span());
        let (output, expressions) = delimited(
            Token::OpenParen,
            preceded(Token::And, many0(Expression::parse_expression)),
            Token::CloseParen,
        )(input)?;
        log::debug!("END < parse_and {:?}", output.span());
        Ok((output, Expression::And(expressions)))
    }

    fn parse_not(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_not {:?}", input.span());
        let (output, expression) = delimited(
            Token::OpenParen,
            preceded(Token::Not, Expression::parse_expression),
            Token::CloseParen,
        )(input)?;
        log::debug!("END < parse_not {:?}", output.span());
        Ok((output, Expression::Not(Box::new(expression))))
    }

    fn parse_atom(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_atom {:?}", input.span());
        let (output, expression) = map(
            delimited(Token::OpenParen, pair(id, Self::parse_parameters), Token::CloseParen),
            |(name, parameters)| Expression::Atom { name, parameters },
        )(input)?;
        log::debug!("END < parse_atom {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_parameters(input: TokenStream) -> IResult<TokenStream, Vec<String>, ParserError> {
        log::debug!("BEGIN > parse_parameters {:?}", input.span());
        let (output, params) = many0(alt((id, var)))(input)?;
        log::debug!("END < parse_parameters {:?}", output.span());
        Ok((output, params))
    }

    fn parse_assign(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_assign {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Assign,
                    tuple((
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    )),
                ),
                Token::CloseParen,
            ),
            |(exp1, exp2)| Expression::Assign(Box::new(exp1), Box::new(exp2)),
        )(input)?;
        log::debug!("END < parse_assign {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_binary_operator(input: TokenStream) -> IResult<TokenStream, BinaryOp, ParserError> {
        log::debug!("BEGIN > parse_binary_operator {:?}", input.span());
        let (output, op) = alt((
            map(Token::Plus, |_| BinaryOp::Add),
            map(Token::Dash, |_| BinaryOp::Subtract),
            map(Token::Times, |_| BinaryOp::Multiply),
            map(Token::Divide, |_| BinaryOp::Divide),
        ))(input)?;
        log::debug!("END < parse_binary_operator {:?}", output.span());
        Ok((output, op))
    }

    fn parse_comparison(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_comparison {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                tuple((
                    Self::parse_binary_operator,
                    alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                )),
                Token::CloseParen,
            ),
            |(name, parameters, value)| Expression::BinaryOp(name, Box::new(parameters), Box::new(value)),
        )(input)?;
        log::debug!("END < parse_comparison {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_number(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_number {:?}", input.span());
        let (output, number) = integer(input)?;
        log::debug!("END < parse_number {:?}", output.span());
        Ok((output, Expression::Number(number)))
    }

    fn parse_scale_up(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_scale_up {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::ScaleUp,
                    tuple((
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    )),
                ),
                Token::CloseParen,
            ),
            |(exp1, exp2)| Expression::ScaleUp(Box::new(exp1), Box::new(exp2)),
        )(input)?;
        log::debug!("END < parse_scale_up {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_scale_down(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_scale_down {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::ScaleDown,
                    tuple((
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    )),
                ),
                Token::CloseParen,
            ),
            |(exp1, exp2)| Expression::ScaleDown(Box::new(exp1), Box::new(exp2)),
        )(input)?;
        log::debug!("END < parse_scale_down {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_increase(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_increase {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Increase,
                    tuple((
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    )),
                ),
                Token::CloseParen,
            ),
            |(exp1, exp2)| Expression::Increase(Box::new(exp1), Box::new(exp2)),
        )(input)?;
        log::debug!("END < parse_increase {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_decrease(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_decrease {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Decrease,
                    tuple((
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                        alt((Self::parse_number, Self::parse_comparison, Self::parse_atom)),
                    )),
                ),
                Token::CloseParen,
            ),
            |(exp1, exp2)| Expression::Decrease(Box::new(exp1), Box::new(exp2)),
        )(input)?;
        log::debug!("END < parse_decrease {:?}", output.span());
        Ok((output, expression))
    }
}
