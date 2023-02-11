use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let n: i32 = parse_util::read_i32();
    println!("{}", Solution::fib(n));
}
