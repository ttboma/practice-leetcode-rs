use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let nums: i32 = buffer.trim().parse().unwrap();
    println!("{:?}", Solution::generate(nums));
}
