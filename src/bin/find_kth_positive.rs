use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (input, list) = parse_util::parse_list(&buffer).unwrap();
    let arr: Vec<i32> = list.iter().map(|s| s.parse().unwrap()).collect();
    let k: i32 = input.trim().parse().unwrap();
    println!("{}", Solution::find_kth_positive(arr, k));
}
