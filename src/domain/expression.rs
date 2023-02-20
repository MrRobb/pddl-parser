use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::{id, integer};

use super::parameter::Parameter;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
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
