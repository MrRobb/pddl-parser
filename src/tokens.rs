use nom::bytes::complete::take_while;
use nom::character::is_alphanumeric;
use nom::combinator::map;
use nom::IResult;

pub fn parse_id(input: &str) -> IResult<&str, String> {
    println!("parse_id: {}", input);
    let (output, id) = map(take_while(|c: char| is_alphanumeric(c as u8) || c == '-'), String::from)(input)?;
    Ok((output, id))
}
