use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let n: Vec<i32> = buffer.split_whitespace().map(|s|s.parse().unwrap()).collect();
    assert!(n.len() == 2, "Please enter to numbers in one line.");
    println!("{}", Solution::count_odds(n[0], n[1]));
}

