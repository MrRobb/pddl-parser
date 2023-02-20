use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::domain::typed_parameter::TypedParameter;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedPredicate {
    pub name: String,
    #[serde(default)]
    pub parameters: Vec<TypedParameter>,
}

impl TypedPredicate {
    pub fn parse_functions(input: TokenStream) -> IResult<TokenStream, Vec<TypedPredicate>, ParserError> {
        log::debug!("BEGIN > parse_functions {:?}", input.span());
        let (output, functions) = opt(delimited(
            Token::OpenParen,
            preceded(
                Token::Functions,
                many0(delimited(
                    Token::OpenParen,
                    pair(id, TypedParameter::parse_typed_parameters),
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
