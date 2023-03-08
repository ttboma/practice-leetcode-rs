use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, list) = parse_util::parse_list(&buffer).unwrap();
    let piles: Vec<i32> = list.iter().map(|s| s.parse().unwrap()).collect();
    let h: i32 = input.trim().parse().unwrap();
    println!("{}", Solution::min_eating_speed(piles, h));
}
