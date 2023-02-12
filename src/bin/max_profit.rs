use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, list) = parse_util::parse_list(&buffer).unwrap();
    assert!(
        input.trim().len() == 0,
        "Please enter one square-bracket-enclosed-list in one line.",
    );
    let prices: Vec<i32> = list.iter().map(|s| s.trim().parse().unwrap()).collect();
    println!("{}", Solution::max_profit(prices));
}
