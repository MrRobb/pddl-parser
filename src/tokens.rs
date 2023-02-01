use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, line_ending, multispace0, not_line_ending};
use nom::combinator::recognize;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;

use crate::error::ParserError;

pub fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, ParserError>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, ParserError>,
{
    puffin::profile_function!();
    delimited(
        multispace0,
        delimited(
            many0(delimited(multispace0, comment, multispace0)),
            inner,
            many0(delimited(multispace0, comment, multispace0)),
        ),
        multispace0,
    )
}

pub fn id(i: &str) -> IResult<&str, &str, ParserError> {
    puffin::profile_function!();
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_"), tag("-")))),
    ))(i)
}

pub fn var(i: &str) -> IResult<&str, &str, ParserError> {
    puffin::profile_function!();
    preceded(char('?'), id)(i)
}

pub fn comment(i: &str) -> IResult<&str, Vec<&str>, ParserError> {
    many1(delimited(tag(";"), not_line_ending, line_ending))(i)
}
