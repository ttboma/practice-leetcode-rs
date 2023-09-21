use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, s) = parse_util::parse_string(&buffer).unwrap();
    assert!(
        input.trim().is_empty(),
        "Please enter one whitespace-splited string in one line.",
    );
    println!("{:?}", Solution::longest_palindrome(s.to_owned()));
}
