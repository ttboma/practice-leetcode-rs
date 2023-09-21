use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let line = parse_util::read_line().unwrap();
    let (_line, str1) = parse_util::parse_string(&line).unwrap();
    println!("{}", Solution::reverse_vowels(str1.to_owned()));
}
