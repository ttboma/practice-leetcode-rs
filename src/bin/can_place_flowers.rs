use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let line = parse_util::read_line().unwrap();
    let (_line, (flowerbed, n)) = parse_util::parse_list_and_i32(&line).unwrap();
    let flowerbed: Vec<i32> = flowerbed.iter().map(|c| c.parse().unwrap()).collect();
    let n: i32 = n.parse().unwrap();
    println!("{:?}", Solution::can_place_flowers(flowerbed, n));
}
