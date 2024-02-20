use std::io::{self, Read};
use std::path::Path;

pub fn parse_i32(input: &str) -> i32 {
    let (_, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    value.parse::<i32>().expect(error_msg::I32_VALUE)
}

pub fn parse_two_i32(input: &str) -> (i32, i32) {
    let (input, value1) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, value2) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        value1.parse::<i32>().expect(error_msg::I32_VALUE),
        value2.parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_i32_and_2d_i32_list(input: &str) -> (i32, Vec<Vec<i32>>) {
    let (input, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, result) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    (
        value.parse::<i32>().expect(error_msg::I32_VALUE),
        result
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.parse::<i32>().expect(error_msg::ITEM_OF_I32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn parse_three_i32(input: &str) -> (i32, i32, i32) {
    let (input, value1) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (input, value2) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, value3) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        value1.parse::<i32>().expect(error_msg::I32_VALUE),
        value2.parse::<i32>().expect(error_msg::I32_VALUE),
        value3.parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_u64(input: &str) -> u64 {
    let (_, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    value.parse::<u64>().expect(error_msg::U64_VALUE)
}

pub fn parse_i32_list(input: &str) -> Vec<i32> {
    let (_, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    result
        .into_iter()
        .map(|x| x.parse::<i32>().expect(error_msg::ITEM_OF_I32))
        .collect::<Vec<_>>()
}

pub fn parse_two_i32_list(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (input, s1) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, s2) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let bind = |x: &str| x.parse::<i32>().expect(error_msg::ITEM_OF_I32);
    (
        s1.into_iter().map(bind).collect::<Vec<_>>(),
        s2.into_iter().map(bind).collect::<Vec<_>>(),
    )
}

pub fn parse_two_i32_list_and_two_i32(input: &str) -> (Vec<i32>, i32, Vec<i32>, i32) {
    let (input, list1) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (input, n) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (input, list2) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, m) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);

    (
        list1
            .into_iter()
            .map(|x| x.parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        n.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        list2
            .into_iter()
            .map(|x| x.parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        m.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_i32_list_and_i32(input: &str) -> (Vec<i32>, i32) {
    let (input, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);

    (
        result
            .into_iter()
            .map(|x| x.parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        input.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_str_list(input: &str) -> Vec<String> {
    let (_, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    result
        .into_iter()
        .map(|x| {
            nom_parser::parse_str(x)
                .expect(error_msg::STR_FORMAT)
                .1
                .to_owned()
        })
        .collect::<Vec<_>>()
}

pub fn parse_two_str(input: &str) -> (String, String) {
    let (input, s1) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    let (_, s2) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    (s1.to_owned(), s2.to_owned())
}

pub fn parse_str_and_i32(input: &str) -> (String, i32) {
    let (input, r) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    let n = input.trim().parse::<i32>().expect(error_msg::I32_VALUE);
    (r.to_owned(), n)
}

pub fn parse_2d_i32_list(input: &str) -> Vec<Vec<i32>> {
    let (_, result) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    result
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.parse::<i32>().expect(error_msg::ITEM_OF_I32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn parse_2d_char_list_and_str(input: &str) -> (Vec<Vec<char>>, String) {
    let (input, result) =
        nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    let (_, s) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    (
        result
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.parse::<char>().expect(error_msg::ITEM_OF_CHAR))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        s.to_owned(),
    )
}

pub fn read_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    match stdin.lock().read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub fn read_from_file(path: &Path) -> io::Result<String> {
    let file = std::fs::File::open(path).expect("cannot open file.");
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    match reader.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

pub mod error_msg;
pub mod help_msg;
pub mod nom_parser;
pub use help_msg::*;
