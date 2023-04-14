use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::typing::Type;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::id;

/// A constant with a type.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Constant {
    /// The name of the constant.
    pub name: String,
    /// The type of the constant.
    #[serde(rename = "type")]
    pub type_: Type,
}

impl Constant {
    /// Parse a list of constants from a token stream.
    pub fn parse_constants(input: TokenStream) -> IResult<TokenStream, Vec<Constant>, ParserError> {
        log::debug!("BEGIN > parse_constants {:?}", input.span());
        let (output, constants) = delimited(
            Token::OpenParen,
            preceded(
                Token::Constants,
                many0(separated_pair(many1(id), Token::Dash, Type::parse_type)),
            ),
            Token::CloseParen,
        )(input)?;
        let constants = constants
            .into_iter()
            .flat_map(|(names, type_)| {
                names.into_iter().map(move |name| Constant {
                    name,
                    type_: type_.clone(),
                })
            })
            .collect();
        log::debug!("END < parse_constants {:?}", output.span());
        Ok((output, constants))
    }

    /// Convert the constant to PDDL.
    pub fn to_pddl(&self) -> String {
        format!("({} - {})", self.name, self.type_.to_pddl())
    }
}
