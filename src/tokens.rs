use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char, line_ending, multispace0, not_line_ending};
use nom::character::is_alphanumeric;
use nom::combinator::{map, recognize};
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use nom::{branch::alt, multi::many0};

use crate::error::ParserError;

pub fn parse_id(input: &str) -> IResult<&str, String> {
    let (output, id) = map(take_while(|c: char| is_alphanumeric(c as u8) || c == '-'), String::from)(input)?;
    Ok((output, id))
}

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

pub fn comment<'a>(i: &'a str) -> IResult<&'a str, Vec<&str>, ParserError> {
    many1(delimited(tag(";"), not_line_ending, line_ending))(i)
}
