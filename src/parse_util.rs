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
pub fn read_i32() -> i32 {
    let buffer = read_line().unwrap();
    buffer.trim().parse().unwrap()
}

use nom::{
    bytes::complete::is_not, bytes::complete::tag, bytes::complete::take_till,
    character::complete::char, combinator::fail, combinator::opt, multi::many0,
    sequence::delimited, IResult,
};
fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, value) = take_till(|c| c == ',')(input)?;
    let (input, comma) = opt(tag(","))(input)?;
    let value = value.trim();
    if comma.is_some() || value.len() != 0 {
        Ok((input, value.parse::<i32>().unwrap()))
    } else {
        fail(input)
    }
}
fn parse_list(input: &str) -> IResult<&str, Vec<i32>> {
    let input = input.trim();
    if input.len() == 0 {
        fail(input)
    } else if input == "[]" {
        Ok((input, vec![]))
    } else {
        let (_input, list) = delimited(char('['), is_not("]"), char(']'))(input)?;
        many0(parse_i32)(list)
    }
}
pub fn read_i32_list() -> Vec<i32> {
    let buffer = read_line().unwrap();
    let (_, list) = parse_list(&buffer).unwrap();
    list
}
