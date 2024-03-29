use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::expression::Expression;
use super::typed_parameter::TypedParameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// An action with typed parameters.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleAction {
    /// The name of the action.
    pub name: String,
    /// The parameters of the action.
    #[serde(default)]
    pub parameters: Vec<TypedParameter>,
    /// The precondition of the action.
    pub precondition: Option<Expression>,
    /// The effect of the action.
    pub effect: Expression,
}

impl SimpleAction {
    /// Parse a list of actions from a token stream.
    pub fn parse(input: TokenStream) -> IResult<TokenStream, SimpleAction, ParserError> {
        log::debug!("BEGIN > parse_action {:?}", input.span());
        log::debug!("Parsing action: {:?}", input.peek_n(10));
        let (output, action) = map(
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
            |(name, parameters, precondition, effect)| SimpleAction {
                name,
                parameters,
                precondition,
                effect,
            },
        )(input)?;
        log::debug!("END < parse_action {:?}", output.span());
        Ok((output, action))
    }

    /// Convert the action to PDDL.
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
