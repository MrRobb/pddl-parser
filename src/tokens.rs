use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0, not_line_ending};
use nom::combinator::{map, recognize};
use nom::error::ParseError;
use nom::multi::many0_count;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use nom::{
    character::{complete::line_ending, is_alphanumeric},
    combinator::opt,
};

pub fn parse_id(input: &str) -> IResult<&str, String> {
    let (output, id) = map(take_while(|c: char| is_alphanumeric(c as u8) || c == '-'), String::from)(input)?;
    Ok((output, id))
}

pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        delimited(
            opt(delimited(multispace0, comment, multispace0)),
            inner,
            opt(delimited(multispace0, comment, multispace0)),
        ),
        multispace0,
    )
}

pub fn id(i: &str) -> IResult<&str, String> {
    let (input, identifier) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"), tag("-")))),
    ))(i)?;
    Ok((input, identifier.to_string()))
}

pub fn var(i: &str) -> IResult<&str, String> {
    let (input, identifier) = preceded(char('?'), id)(i)?;
    Ok((input, identifier))
}

pub fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &str, E> {
    delimited(tag(";"), not_line_ending, line_ending)(i)
}
