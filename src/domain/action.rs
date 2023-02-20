use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::expression::Expression;
use super::typed_parameter::TypedParameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn to_pddl(&self) -> String {
        let mut pddl = String::new();

        // Action name
        pddl.push_str(&format!("(:action {}\n", self.name));

        // Parameters
        pddl.push_str(&format!(
            ":parameters ({})\n",
            self.parameters
                .iter()
                .map(TypedParameter::to_pddl)
                .collect::<Vec<_>>()
                .join(" ")
        ));

        // Precondition
        if let Some(precondition) = &self.precondition {
            pddl.push_str(&format!(":precondition {}\n", precondition.to_pddl()));
        }

        // Effect
        pddl.push_str(&format!(":effect \n{}\n", self.effect.to_pddl()));

        pddl.push(')');
        pddl
    }
}
