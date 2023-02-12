use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
fn read_string() -> io::Result<String> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}
pub fn read_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

use nom::{
    bytes::complete::tag, bytes::complete::take_while1, character::complete::multispace0,
    combinator::opt, error::ParseError, multi::many1, multi::separated_list0, sequence::delimited,
    sequence::preceded, sequence::terminated, IResult,
};
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
pub fn parse_list(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        ws(tag("[")),
        terminated(
            separated_list0(
                ws(tag(",")),
                take_while1(|x: char| x.is_ascii() && x != ',' && x != ']' && !x.is_whitespace()),
            ),
            opt(ws(tag(","))),
        ),
        ws(tag("]")),
    )(input)
}
pub fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many1(delimited(
        ws(tag("[")),
        terminated(
            separated_list0(
                ws(tag(",")),
                take_while1(|x: char| x.is_ascii() && x != ',' && x != ']' && !x.is_whitespace()),
            ),
            opt(ws(tag(","))),
        ),
        ws(tag("]")),
    ))(input)
}
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    preceded(
        multispace0,
        take_while1(|x: char| x.is_ascii() && !x.is_whitespace()),
    )(input)
}
pub fn read_i32() -> i32 {
    let buffer = read_line().unwrap();
    buffer.trim().parse().unwrap()
}
pub fn read_i32_list() -> Vec<i32> {
    let buffer = read_line().unwrap();
    let (_, list) = parse_list(&buffer).unwrap();
    list.iter().map(|s| s.parse().unwrap()).collect()
}
