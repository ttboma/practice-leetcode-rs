use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let fruits = parse_util::read_i32_list();
    println!("{}", Solution::total_fruit(fruits));
}
