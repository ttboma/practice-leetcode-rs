use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, list) = parse_util::parse_list(&buffer).unwrap();
    assert!(
        input.trim().is_empty(),
        "Please enter one square-bracket-enclosed-list in one line.",
    );
    let nums: Vec<i32> = list.iter().map(|s| s.parse().unwrap()).collect();
    println!("{}", Solution::jump(nums));
}
