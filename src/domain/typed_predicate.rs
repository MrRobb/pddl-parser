use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::lexer::{Token, TokenStream};
use crate::tokens::id;
use crate::{domain::typed_parameter::TypedParameter, error::ParserError};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
}
