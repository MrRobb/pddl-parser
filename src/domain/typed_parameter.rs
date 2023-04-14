use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded};
use nom::IResult;
use serde::{Deserialize, Serialize};

use super::typing::Type;
use crate::error::ParserError;
use crate::lexer::{Token, TokenStream};
use crate::tokens::var;

/// A parameter with a type.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedParameter {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter. If not specified, the type is `object`.
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: Type,
}

impl TypedParameter {
    /// Parse a list of typed parameters from a token stream.
    pub fn parse_typed_parameters(input: TokenStream) -> IResult<TokenStream, Vec<TypedParameter>, ParserError> {
        log::debug!("BEGIN > parse_typed_parameters {:?}", input.span());
        let (output, params) = many0(pair(many1(var), opt(preceded(Token::Dash, Type::parse_type))))(input)?;
        let params = params
            .into_iter()
            .flat_map(|(names, type_)| {
                names.into_iter().map(move |name| TypedParameter {
                    name,
                    type_: type_.clone().unwrap_or_default(),
                })
            })
            .collect();
        log::debug!("Parsed typed parameters: {params:?}");
        log::debug!("END < parse_typed_parameters {:?}", output.span());
        Ok((output, params))
    }

    /// Convert the typed parameter to PDDL.
    pub fn to_pddl(&self) -> String {
        format!("{} - {}", self.name, self.type_.to_pddl())
    }
}
