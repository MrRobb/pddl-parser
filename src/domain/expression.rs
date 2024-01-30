use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::{id, integer};
use crate::{domain::typed_parameter::TypedParameter, tokens::var};

/// An enumeration of binary operations that can be used in expressions.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    /// Addition operation.
    Add,
    /// Subtraction operation.
    Subtract,
    /// Multiplication operation.
    Multiply,
    /// Division operation.
    Divide,
    /// Equality operation.
    Equal,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DurationInstant {
    Start,
    End,
    All,
}

/// An enumeration of expressions that can be used in PDDL planning domains and problems.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expression {
    /// An atomic expression consisting of a name and an optional list of parameters.
    Atom {
        /// The name of the atom.
        name: String,
        /// The parameters of the atom.
        #[serde(default)]
        parameters: Vec<Parameter>,
    },
    /// A logical "and" expression that takes a list of sub-expressions as arguments.
    And(Vec<Expression>),
    /// A logical "not" expression that takes a single sub-expression as an argument.
    Not(Box<Expression>),

    // Assign operator
    /// An assignment expression that assigns the value of the second sub-expression to the first sub-expression.
    Assign(Box<Expression>, Box<Expression>),
    /// An increase expression that increases the value of the first sub-expression by the value of the second sub-expression.
    Increase(Box<Expression>, Box<Expression>),
    /// A decrease expression that decreases the value of the first sub-expression by the value of the second sub-expression.
    Decrease(Box<Expression>, Box<Expression>),
    /// A scale-up expression that multiplies the value of the first sub-expression by the value of the second sub-expression.
    ScaleUp(Box<Expression>, Box<Expression>),
    /// A scale-down expression that divides the value of the first sub-expression by the value of the second sub-expression.
    ScaleDown(Box<Expression>, Box<Expression>),
    /// A binary operation expression that applies a binary operation to two sub-expressions.
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    /// A numeric constant expression.
    Number(i64),

    // Forall
    /// A forall expression that takes a list of typed parameters and a sub-expression as arguments.
    Forall(Vec<TypedParameter>, Box<Expression>),

    // Duration
    Duration(DurationInstant, Box<Expression>),
}

impl Expression {
    /// Parse an expression from a token stream.
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
            Self::parse_duration,
            Self::parse_forall,
            Self::parse_comparison,
        ))(input)?;
        log::debug!("END < parse_expression {:?}", output.span());
        Ok((output, expression))
    }

    /// Convert the expression to PDDL.
    pub fn to_pddl(&self) -> String {
        match self {
            Expression::Atom { name, parameters } => format!(
                "({} {})",
                name,
                parameters.iter().map(Parameter::to_pddl).collect::<Vec<_>>().join(" ")
            ),
            Expression::And(expressions) => format!(
                "(and {})",
                expressions
                    .iter()
                    .map(Expression::to_pddl)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Expression::Not(expression) => format!("(not {})", expression.to_pddl()),
            Expression::Assign(exp1, exp2) => format!("(assign {} {})", exp1.to_pddl(), exp2.to_pddl()),
            Expression::Increase(exp1, exp2) => {
                format!("(increase {} {})", exp1.to_pddl(), exp2.to_pddl())
            },
            Expression::Decrease(exp1, exp2) => {
                format!("(decrease {} {})", exp1.to_pddl(), exp2.to_pddl())
            },
            Expression::ScaleUp(exp1, exp2) => {
                format!("(scale-up {} {})", exp1.to_pddl(), exp2.to_pddl())
            },
            Expression::ScaleDown(exp1, exp2) => {
                format!("(scale-down {} {})", exp1.to_pddl(), exp2.to_pddl())
            },
            Expression::BinaryOp(op, exp1, exp2) => format!(
                "({} {} {})",
                match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                    BinaryOp::Equal => "=",
                },
                exp1.to_pddl(),
                exp2.to_pddl()
            ),
            Expression::Number(n) => n.to_string(),
            Expression::Duration(instant, exp) => format!(
                "({} {})",
                match instant {
                    DurationInstant::Start => "at start",
                    DurationInstant::End => "at end",
                    DurationInstant::All => "over all",
                },
                exp.to_pddl()
            ),
            Expression::Forall(parameters, expression) => format!(
                "(forall ({}) {})",
                parameters
                    .iter()
                    .map(TypedParameter::to_pddl)
                    .collect::<Vec<_>>()
                    .join(" "),
                expression.to_pddl()
            ),
        }
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
            delimited(
                Token::OpenParen,
                pair(id, Parameter::parse_parameters),
                Token::CloseParen,
            ),
            |(name, parameters)| Expression::Atom { name, parameters },
        )(input)?;
        log::debug!("END < parse_atom {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_var(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_var {:?}", input.span());
        let (output, expression) = map(var, |name| Expression::Atom {
            name,
            parameters: Vec::new(),
        })(input)?;
        log::debug!("END < parse_var {:?}", output.span());
        Ok((output, expression))
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
            map(Token::Equal, |_| BinaryOp::Equal),
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
                    alt((
                        Self::parse_number,
                        Self::parse_comparison,
                        Self::parse_atom,
                        Self::parse_var,
                    )),
                    alt((
                        Self::parse_number,
                        Self::parse_comparison,
                        Self::parse_atom,
                        Self::parse_var,
                    )),
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

    fn parse_forall(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_forall {:?}", input.span());
        let (output, expression) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::Forall,
                    tuple((
                        delimited(
                            Token::OpenParen,
                            TypedParameter::parse_typed_parameters,
                            Token::CloseParen,
                        ),
                        Expression::parse_expression,
                    )),
                ),
                Token::CloseParen,
            ),
            |(parameters, expression)| Expression::Forall(parameters, Box::new(expression)),
        )(input)?;
        log::debug!("END < parse_forall {:?}", output.span());
        Ok((output, expression))
    }

    fn parse_duration(input: TokenStream) -> IResult<TokenStream, Expression, ParserError> {
        log::debug!("BEGIN > parse_duration {:?}", input.span());
        let (output, expression) = delimited(
            Token::OpenParen,
            alt((
                map(
                    tuple((Token::At, Token::Start, Expression::parse_expression)),
                    |(_, _, expression)| Expression::Duration(DurationInstant::Start, Box::new(expression)),
                ),
                map(
                    tuple((Token::At, Token::End, Expression::parse_expression)),
                    |(_, _, expression)| Expression::Duration(DurationInstant::End, Box::new(expression)),
                ),
                map(
                    tuple((Token::Over, Token::All, Expression::parse_expression)),
                    |(_, _, expression)| Expression::Duration(DurationInstant::All, Box::new(expression)),
                ),
            )),
            Token::CloseParen,
        )(input)?;
        log::debug!("END < parse_duration {:?}", output.span());
        Ok((output, expression))
    }
}
