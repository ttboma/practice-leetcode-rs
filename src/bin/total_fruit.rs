use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::parse_util;

fn main() {
    let fruits = parse_util::read_i32_list();
    println!("{}", Solution::total_fruit(fruits));
}
