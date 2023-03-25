use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let input = parse_util::read_line().unwrap();
    let (_, (n, edges)) = parse_util::parse_i32_and_list_2d(&input).unwrap();
    let n: i32 = n.parse().unwrap();
    let edges: Vec<Vec<i32>> = edges
        .iter()
        .map(|list| list.iter().map(|list| list.parse().unwrap()).collect())
        .collect();
    println!("{}", Solution::count_pairs(n, edges));
}
