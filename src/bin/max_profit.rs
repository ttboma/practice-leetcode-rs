use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let prices = parse_util::read_i32_list();
    println!("{}", Solution::max_profit(prices));
}
