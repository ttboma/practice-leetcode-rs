use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let v: Vec<&str> = buffer.split_whitespace().collect();
    assert!(
        v.len() == 1,
        "Please enter two whitespace-splited string in one line."
    );
    println!(
        "{:?}",
        Solution::longest_palindrome(v[0].to_owned())
    );
}
