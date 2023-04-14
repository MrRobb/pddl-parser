use std::ops::Range;

use nom::error::ParseError;
use nom::Needed;
use thiserror::Error;

use crate::domain::requirement::Requirement;
use crate::lexer::Token;

/// A PDDL parser error
#[derive(Error, Debug, PartialEq, Clone, Default)]
pub enum ParserError {
    /// The PDDL file contains an unsupported requirement (e.g. `:fluents`). See the `Requirement` section of the [README.md](https://github.com/MrRobb/pddl-parser#pddl-requirements-supported) for a list of supported requirements.
    #[error("Unsupported PDDL Requirement: {0:?}")]
    UnsupportedRequirement(Requirement),

    /// A generic parse error.
    #[error("Parse error: {0:?}")]
    ParseError(nom::error::ErrorKind, String),

    /// An incomplete input error. This is returned by the parser when it needs more input to continue parsing.
    #[error("Incomplete input: {0:?}")]
    IncompleteInput(Needed),

    /// The parser expected an identifier, but found something else.
    #[error("Expected identifier")]
    ExpectedIdentifier,

    /// The parser expected a token, but found something else.
    #[error("Expected token: {0:?}")]
    ExpectedToken(Token, Range<usize>, Option<Vec<(Result<Token, ParserError>, String)>>),

    /// The parser expected a float, but found something else.
    #[error("Expected float")]
    ExpectedFloat,

    /// The parser expected an integer, but found something else.
    #[error("Expected integer")]
    ExpectedInteger,

    /// The lexer encountered an error. This is returned by the lexer when it encounters an invalid token.
    #[error("Lexer error")]
    LexerError,

    /// The parser expected the end of input, but there was more input. This is returned by the parser when it encounters more input after the end of the PDDL file.
    #[error("Expected end of input")]
    ExpectedEndOfInput,

    /// An unknown error. Default error variant. This should never be returned.
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
                ParserError::ExpectedEndOfInput => ParserError::ExpectedEndOfInput,
            },
        }
    }
}
