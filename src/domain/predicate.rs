use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::parameter::Parameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Predicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl Predicate {
    pub fn parse_predicates(input: TokenStream) -> IResult<TokenStream, Vec<Predicate>, ParserError> {
        log::debug!("BEGIN > parse_predicates {:?}", input.span());
        let (output, predicates) = delimited(
            Token::OpenParen,
            preceded(
                Token::Predicates,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, Parameter::parse_parameters),
                    Token::CloseParen,
                )),
            ),
            Token::CloseParen,
        )(input)?;
        let predicates = predicates
            .into_iter()
            .map(|(name, parameters)| Predicate { name, parameters })
            .collect();
        log::debug!("END < parse_predicates {:?}", output.span());
        Ok((output, predicates))
    }

    pub fn to_pddl(&self) -> String {
        format!(
            "({} {})",
            self.name,
            self.parameters
                .iter()
                .map(Parameter::to_pddl)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
