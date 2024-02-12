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
pub struct DurativeAction {
    /// The name of the action.
    pub name: String,
    /// The parameters of the action.
    #[serde(default)]
    pub parameters: Vec<TypedParameter>,
    /// The duration of the action.
    pub duration: Expression,
    /// The condition of the action.
    pub condition: Option<Expression>,
    /// The effect of the action.
    pub effect: Expression,
}

impl DurativeAction {
    /// Parse a list of actions from a token stream.
    pub fn parse(input: TokenStream) -> IResult<TokenStream, DurativeAction, ParserError> {
        log::debug!("BEGIN > parse_durative_action {:?}", input.span());
        log::debug!("Parsing action: {:?}", input.peek_n(10));
        let (output, action) = map(
            delimited(
                Token::OpenParen,
                preceded(
                    Token::DurativeAction,
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
                        preceded(Token::Duration, Expression::parse_expression),
                        opt(preceded(Token::Condition, Expression::parse_expression)),
                        preceded(Token::Effect, Expression::parse_expression),
                    )),
                ),
                Token::CloseParen,
            ),
            |(name, parameters, duration, condition, effect)| DurativeAction {
                name,
                parameters,
                duration,
                condition,
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
        pddl.push_str(&format!("(:durative-action {}\n", self.name));

        // Parameters
        pddl.push_str(&format!(
            ":parameters ({})\n",
            self.parameters
                .iter()
                .map(TypedParameter::to_pddl)
                .collect::<Vec<_>>()
                .join(" ")
        ));

        // Duration
        pddl.push_str(&format!(":duration {}\n", self.duration.to_pddl()));

        // Condition
        if let Some(condition) = &self.condition {
            pddl.push_str(&format!(":condition {}\n", condition.to_pddl()));
        }

        // Effect
        pddl.push_str(&format!(":effect \n{}\n", self.effect.to_pddl()));

        pddl.push(')');
        pddl
    }
}
