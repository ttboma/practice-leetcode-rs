use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::parse_util;

fn main() {
    let line = parse_util::read_line().unwrap(); 
    let (_line, str1) = parse_util::parse_string(&line).unwrap();
    println!("{}", Solution::reverse_words(str1.to_owned()));
}

