use std::ops::Range;

use nom::error::ParseError;
use nom::Needed;
use thiserror::Error;

use crate::domain::requirement::Requirement;
use crate::lexer::Token;

#[derive(Error, Debug, PartialEq, Clone, Default)]
pub enum ParserError {
    #[error("Unsupported PDDL Requirement: {0:?}")]
    UnsupportedRequirement(Requirement),

    #[error("Parse error: {0:?}")]
    ParseError(nom::error::ErrorKind, String),

    #[error("Incomplete input: {0:?}")]
    IncompleteInput(Needed),

    #[error("Expected identifier")]
    ExpectedIdentifier,

    #[error("Expected token: {0:?}")]
    ExpectedToken(Token, Range<usize>, Option<Vec<(Result<Token, ParserError>, String)>>),

    #[error("Expected float")]
    ExpectedFloat,

    #[error("Expected integer")]
    ExpectedInteger,

    #[error("Lexer error")]
    LexerError,

    #[default]
    #[error("Unknown error")]
    UnknownError,
}

impl<I: ToString> ParseError<I> for ParserError {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        ParserError::ParseError(kind, input.to_string())
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParserError::ExpectedInteger
    }
}

impl From<std::num::ParseFloatError> for ParserError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParserError::ExpectedFloat
    }
}

impl From<nom::Err<ParserError>> for ParserError {
    fn from(err: nom::Err<ParserError>) -> Self {
        match err {
            nom::Err::Incomplete(e) => ParserError::IncompleteInput(e),
            nom::Err::Error(e) | nom::Err::Failure(e) => match e {
                ParserError::ParseError(kind, string) => ParserError::ParseError(kind, string),
                ParserError::IncompleteInput(e) => ParserError::IncompleteInput(e),
                ParserError::UnsupportedRequirement(e) => ParserError::UnsupportedRequirement(e),
                ParserError::ExpectedIdentifier => ParserError::ExpectedIdentifier,
                ParserError::ExpectedToken(token, span, next_tokens) => {
                    ParserError::ExpectedToken(token, span, next_tokens)
                },
                ParserError::ExpectedFloat => ParserError::ExpectedFloat,
                ParserError::ExpectedInteger => ParserError::ExpectedInteger,
                ParserError::LexerError => ParserError::LexerError,
                ParserError::UnknownError => ParserError::UnknownError,
            },
        }
    }
}
