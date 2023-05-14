use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let line = parse_util::read_line().unwrap();
    let (_line, (candies, extra_candies)) = parse_util::parse_list_and_i32(&line).unwrap();
    let candies: Vec<i32> = candies.iter().map(|c| c.parse().unwrap()).collect();
    let extra_candies: i32 = extra_candies.parse().unwrap();
    println!("{:?}", Solution::kids_with_candies(candies, extra_candies));
}
