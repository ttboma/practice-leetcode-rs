use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let line = parse_util::read_line().unwrap();
    let (line, str1) = parse_util::parse_string(&line).unwrap();
    let (_line, str2) = parse_util::parse_string(line).unwrap();
    println!(
        "{}",
        Solution::gcd_of_strings(str1.to_owned(), str2.to_owned())
    );
}
