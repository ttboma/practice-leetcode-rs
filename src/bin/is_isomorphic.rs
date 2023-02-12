use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, s) = parse_util::parse_string(&buffer).unwrap();
    let (input, t) = parse_util::parse_string(&input).unwrap();
    assert!(
        input.trim().len() == 0,
        "Please enter two whitespace-splited string in one line.",
    );
    println!("{:?}", Solution::is_isomorphic(s.to_owned(), t.to_owned()));
}
