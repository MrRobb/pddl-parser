use nom::error::ParseError;
use nom::Needed;
use thiserror::Error;

use crate::domain::Requirement;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Unsupported PDDL Requirement: {0:?}")]
    UnsupportedRequirement(Requirement),

    #[error("Parse error: {0:?}")]
    ParseError(nom::error::Error<String>),

    #[error("Incomplete input: {0:?}")]
    IncompleteInput(Needed),
}

impl<I: ToString> ParseError<I> for ParserError {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ParserError::ParseError(nom::error::Error::from_error_kind(input.to_string(), kind))
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<nom::Err<ParserError>> for ParserError {
    fn from(err: nom::Err<ParserError>) -> Self {
        match err {
            nom::Err::Incomplete(e) => ParserError::IncompleteInput(e),
            nom::Err::Error(e) | nom::Err::Failure(e) => match e {
                ParserError::ParseError(e) => ParserError::ParseError(nom::error::Error {
                    input: e.input.to_string(),
                    code: e.code,
                }),
                ParserError::IncompleteInput(e) => ParserError::IncompleteInput(e),
                ParserError::UnsupportedRequirement(e) => ParserError::UnsupportedRequirement(e),
            },
        }
    }
}
