use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let input = parse_util::read_line().unwrap();
    let n: i32 = input.trim().parse().unwrap();
    println!("{:?}", Solution::count_bits(n));
}
