use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::parse_util;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, nums) = parse_util::parse_list(&buffer).unwrap();
    let nums: Vec<i32> = nums.iter().map(|s| s.parse().unwrap()).collect();
    let target: i32 = input.trim().parse().unwrap();
    println!("{}", Solution::search(nums, target));
}
