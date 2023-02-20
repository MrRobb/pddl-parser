use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::lexer::{Token, TokenStream};
use crate::tokens::id;
use crate::{domain::typed_parameter::TypedParameter, error::ParserError};

use super::{parameter::Parameter, typed_predicate::TypedPredicate};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

impl Predicate {
    pub fn parse_predicates(input: TokenStream) -> IResult<TokenStream, Vec<TypedPredicate>, ParserError> {
        log::debug!("BEGIN > parse_predicates {:?}", input.span());
        let (output, predicates) = delimited(
            Token::OpenParen,
            preceded(
                Token::Predicates,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, TypedParameter::parse_typed_parameters),
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
}
