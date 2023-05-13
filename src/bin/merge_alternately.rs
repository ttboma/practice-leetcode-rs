use syc_leetcode_solution_rs::Solution;
use syc_leetcode_solution_rs::parse_util;

fn main() {
    let line = parse_util::read_line().unwrap(); 
    let (line, word1) = parse_util::parse_string(&line).unwrap();
    let (_line, word2) = parse_util::parse_string(&line).unwrap();
    println!("{}", Solution::merge_alternately(word1.to_owned(), word2.to_owned()));
}
