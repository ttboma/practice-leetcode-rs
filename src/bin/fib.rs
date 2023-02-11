use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::parse_util;
fn main() {
    let n: i32 = parse_util::read_i32();
    println!("{}", Solution::fib(n));
}
