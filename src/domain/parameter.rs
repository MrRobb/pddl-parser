use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};

use crate::error::ParserError;
use crate::lexer::TokenStream;
use crate::tokens::{id, var};

/// A parameter (untyped). This is a wrapper around a string.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parameter(String);

impl From<String> for Parameter {
    fn from(s: String) -> Self {
        Parameter(s)
    }
}

impl From<&str> for Parameter {
    fn from(s: &str) -> Self {
        Parameter(s.to_string())
    }
}

impl Parameter {
    /// Parse a list of parameters from a token stream.
    pub fn parse_parameters(input: TokenStream) -> IResult<TokenStream, Vec<Parameter>, ParserError> {
        log::debug!("BEGIN > parse_parameters {:?}", input.span());
        let (output, params) = many0(map(alt((id, var)), Into::into))(input)?;
        log::debug!("END < parse_parameters {:?}", output.span());
        Ok((output, params))
    }

    /// Convert the parameter to PDDL.
    pub fn to_pddl(&self) -> String {
        self.0.clone()
    }
}

impl ToString for Parameter {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
