use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

use super::{expression::Expression, typed_parameter::TypedParameter};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<TypedParameter>,
    pub precondition: Option<Expression>,
    pub effect: Expression,
}

impl Action {
    pub fn parse_actions(input: TokenStream) -> IResult<TokenStream, Vec<Action>, ParserError> {
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
                            delimited(
                                Token::OpenParen,
                                TypedParameter::parse_typed_parameters,
                                Token::CloseParen,
                            ),
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
