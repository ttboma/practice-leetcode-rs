use std::io;

pub fn read_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{digit1, multispace0},
    combinator::opt,
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
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
                ws(take_while1(|x: char| {
                    x.is_ascii() && x != ',' && x != ']' && !x.is_whitespace()
                })),
            ),
            opt(ws(tag(","))),
        ),
        ws(tag("]")),
    )(input)
}
pub fn parse_list_2d(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    delimited(
        ws(tag("[")),
        terminated(separated_list0(ws(tag(",")), parse_list), opt(ws(tag(",")))),
        ws(tag("]")),
    )(input)
}
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("\"")),
        take_while(|x: char| x.is_ascii() && x != '"'),
        ws(tag("\"")),
    )(input)
}
pub fn parse_i32_and_list_2d(input: &str) -> IResult<&str, (&str, Vec<Vec<&str>>)> {
    tuple((ws(digit1), parse_list_2d))(input)
}
pub fn parse_list_and_i32(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    tuple((parse_list, ws(digit1)))(input)
}
