use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{multispace0, one_of},
    combinator::{opt, recognize},
    error::ParseError,
    multi::{many1, separated_list0},
    sequence::{delimited, pair, terminated},
    IResult,
};

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn decimal(input: &str) -> IResult<&str, &str> {
    ws(recognize(pair(
        opt(one_of("-+")),
        many1(one_of("0123456789")),
    )))(input)
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

pub fn parse_2d_list(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    delimited(
        ws(tag("[")),
        terminated(separated_list0(ws(tag(",")), parse_list), opt(ws(tag(",")))),
        ws(tag("]")),
    )(input)
}

pub fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("\"")),
        take_while(|x: char| x.is_ascii() && x != '"'),
        ws(tag("\"")),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(decimal(" 123 "), Ok(("", "123")));
        assert_eq!(decimal("\n -123\n"), Ok(("", "-123")));
        assert_eq!(decimal(" +123 \n"), Ok(("", "+123")));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse_list("[1,2,3]"), Ok(("", vec!["1", "2", "3"])));
        assert_eq!(
            parse_list("[2,3,6,7] 7"),
            Ok(("7", vec!["2", "3", "6", "7"]))
        );
    }
}
