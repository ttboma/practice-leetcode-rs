use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let input = parse_util::read_line().unwrap();
    let (_, input) = parse_util::parse_list(&input).unwrap();
    let nums: Vec<i32> = input.iter().map(|s| s.parse().unwrap()).collect();
    println!("{:?}", Solution::product_except_self(nums));
}
