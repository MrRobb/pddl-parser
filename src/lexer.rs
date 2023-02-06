use derive_more::Display;
use logos::Logos;
use nom::InputLength;

use crate::error::ParserError;

#[derive(Debug, Display, Clone, PartialEq, Eq, Logos)]
pub enum Token {
    // Open parenthesis
    #[token("(")]
    OpenParen,

    // Close parenthesis
    #[token(")")]
    CloseParen,

    // PDDL Identifier
    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |lex| lex.slice().to_string())]
    Id(String),

    // Comments
    #[regex(r";.*", logos::skip)]
    Comment,

    // Whitespace
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub struct TokenStream<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl Clone for TokenStream<'_> {
    fn clone(&self) -> Self {
        Self {
            lexer: self.lexer.clone(),
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }

    pub fn len(&self) -> usize {
        self.lexer.source().len() - self.lexer.span().end
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek(&self) -> Option<(Token, &'a str)> {
        let mut iter = self.lexer.clone().spanned();
        iter.next().map(|(t, span)| (t, &self.lexer.source()[span]))
    }

    pub fn advance(mut self) -> Self {
        self.lexer.next();
        self
    }
}

impl<'a> nom::Parser<TokenStream<'a>, &'a str, ParserError> for Token {
    fn parse(&mut self, input: TokenStream<'a>) -> nom::IResult<TokenStream<'a>, &'a str, ParserError> {
        match input.peek() {
            Some((t, s)) if t == *self => Ok((input.advance(), s)),
            _ => Err(nom::Err::Error(ParserError::ExpectedToken(self.clone()))),
        }
    }
}

impl ToString for TokenStream<'_> {
    fn to_string(&self) -> String {
        self.lexer.source().to_string()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

impl InputLength for TokenStream<'_> {
    fn input_len(&self) -> usize {
        self.len()
    }
}
