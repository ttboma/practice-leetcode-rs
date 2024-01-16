use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::multispace0,
    combinator::opt,
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, terminated},
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

pub fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("\""),
        take_while(|x: char| x.is_ascii() && x != '"'),
        tag("\""),
    )(input)
}

pub fn parse_2d_list(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    delimited(
        ws(tag("[")),
        terminated(separated_list0(ws(tag(",")), parse_list), opt(ws(tag(",")))),
        ws(tag("]")),
    )(input)
}
