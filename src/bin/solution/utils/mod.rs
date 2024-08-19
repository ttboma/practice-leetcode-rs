use std::io::{self, Read};
use std::path::Path;

pub fn parse_i32(input: &str) -> i32 {
    let (_, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    value.trim().parse::<i32>().expect(error_msg::I32_VALUE)
}

pub fn parse_two_i32(input: &str) -> (i32, i32) {
    let (input, value1) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, value2) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        value1.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        value2.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_i32_and_2d_i32_list(input: &str) -> (i32, Vec<Vec<i32>>) {
    let (input, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, result) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    (
        value.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        result
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn parse_i32_and_i32_list(input: &str) -> (i32, Vec<i32>) {
    let (input, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);

    (
        value.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        result
            .into_iter()
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
    )
}

pub fn parse_three_i32(input: &str) -> (i32, i32, i32) {
    let (input, value1) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (input, value2) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, value3) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        value1.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        value2.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        value3.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_u64(input: &str) -> u64 {
    let (_, value) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    value.trim().parse::<u64>().expect(error_msg::U64_VALUE)
}

pub fn parse_i32_list(input: &str) -> Vec<i32> {
    let (_, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    result
        .into_iter()
        .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
        .collect::<Vec<_>>()
}

pub fn parse_two_i32_list(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (input, s1) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, s2) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let bind = |x: &str| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32);
    (
        s1.into_iter().map(bind).collect::<Vec<_>>(),
        s2.into_iter().map(bind).collect::<Vec<_>>(),
    )
}

pub fn parse_i32_list_and_two_i32(input: &str) -> (Vec<i32>, i32, i32) {
    let (input, list1) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (input, n) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, m) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);

    (
        list1
            .into_iter()
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        n.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        m.trim().parse::<i32>().expect(error_msg::I32_VALUE),
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
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        n.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        list2
            .into_iter()
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
        m.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_i32_list_and_i32(input: &str) -> (Vec<i32>, i32) {
    let (input, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);

    (
        result
            .into_iter()
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
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

pub fn parse_str_list_and_2d_i32_list(input: &str) -> (Vec<String>, Vec<Vec<i32>>) {
    let (input, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, values) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    (
        result
            .into_iter()
            .map(|x| {
                nom_parser::parse_str(x)
                    .expect(error_msg::STR_FORMAT)
                    .1
                    .to_owned()
            })
            .collect::<Vec<_>>(),
        values
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn parse_str_and_str_list(input: &str) -> (String, Vec<String>) {
    let (input, r) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    let (_, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    (
        r.to_owned(),
        result
            .into_iter()
            .map(|x| {
                nom_parser::parse_str(x)
                    .expect(error_msg::STR_FORMAT)
                    .1
                    .to_owned()
            })
            .collect::<Vec<_>>(),
    )
}

pub fn parse_str_list_and_i32(input: &str) -> (Vec<String>, i32) {
    let (input, result) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, n) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        result
            .into_iter()
            .map(|x| {
                nom_parser::parse_str(x)
                    .expect(error_msg::STR_FORMAT)
                    .1
                    .to_owned()
            })
            .collect::<Vec<_>>(),
        n.parse::<i32>().expect(error_msg::I32_VALUE),
    )
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
                .map(|y| y.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn parse_2d_i32_list_and_i32_list(input: &str) -> (Vec<Vec<i32>>, Vec<i32>) {
    let (input, r1) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    let (_, r2) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);

    (
        r1.into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| y.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        r2.into_iter()
            .map(|x| x.trim().parse::<i32>().expect(error_msg::ITEM_OF_I32))
            .collect::<Vec<_>>(),
    )
}

pub fn parse_2d_char_list(input: &str) -> Vec<Vec<char>> {
    let (_, result) = nom_parser::parse_2d_list(input).expect(error_msg::TWO_DIMENSION_LIST_FORMAT);
    result
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| *y.trim().as_bytes().get(1).expect(error_msg::ITEM_OF_CHAR) as char)
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
                    .map(|y| *y.trim().as_bytes().get(1).expect(error_msg::ITEM_OF_CHAR) as char)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        s.to_owned(),
    )
}

pub fn parse_str(input: &str) -> String {
    let (_, s) = nom_parser::parse_str(input).expect(error_msg::STR_FORMAT);
    s.to_owned()
}

pub fn parse_opt_i32_list(input: &str) -> Vec<Option<i32>> {
    let (_, val) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    val.into_iter()
        .map(|v| {
            let v = v.trim();
            if v == "null" {
                None
            } else {
                Some(v.parse::<i32>().expect(error_msg::ITEM_OF_OPT_I32))
            }
        })
        .collect()
}

pub fn parse_two_opt_i32_list(input: &str) -> (Vec<Option<i32>>, Vec<Option<i32>>) {
    let (input, val1) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, val2) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    (
        val1.into_iter()
            .map(|v| {
                let v = v.trim();
                if v == "null" {
                    None
                } else {
                    Some(v.parse::<i32>().expect(error_msg::ITEM_OF_OPT_I32))
                }
            })
            .collect(),
        val2.into_iter()
            .map(|v| {
                let v = v.trim();
                if v == "null" {
                    None
                } else {
                    Some(v.parse::<i32>().expect(error_msg::ITEM_OF_OPT_I32))
                }
            })
            .collect(),
    )
}

pub fn parse_opt_i32_list_and_i32(input: &str) -> (Vec<Option<i32>>, i32) {
    let (input, val) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (_, n) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        val.into_iter()
            .map(|v| {
                let v = v.trim();
                if v == "null" {
                    None
                } else {
                    Some(v.parse::<i32>().expect(error_msg::ITEM_OF_OPT_I32))
                }
            })
            .collect(),
        n.trim().parse::<i32>().expect(error_msg::I32_VALUE),
    )
}

pub fn parse_opt_i32_list_and_two_i32(input: &str) -> (Vec<Option<i32>>, i32, i32) {
    let (input, val) = nom_parser::parse_list(input).expect(error_msg::LIST_FORMAT);
    let (input, p) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    let (_, q) = nom_parser::decimal(input).expect(error_msg::INTEGER_FORMAT);
    (
        val.into_iter()
            .map(|v| {
                let v = v.trim();
                if v == "null" {
                    None
                } else {
                    Some(v.parse::<i32>().expect(error_msg::ITEM_OF_OPT_I32))
                }
            })
            .collect(),
        p.trim().parse::<i32>().expect(error_msg::I32_VALUE),
        q.trim().parse::<i32>().expect(error_msg::I32_VALUE),
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
