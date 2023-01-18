use thiserror::Error;

use crate::domain::Requirement;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unsupported PDDL Requirement: {0:?}")]
    UnsupportedRequirement(Requirement),

    #[error("Parse error")]
    ParseError(nom::Err<nom::error::Error<String>>),
}

impl From<nom::Err<nom::error::Error<&str>>> for ParserError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
        ParserError::ParseError(err.map(|nom::error::Error { input, code }| nom::error::Error {
            input: input.to_string(),
            code,
        }))
    }
}
