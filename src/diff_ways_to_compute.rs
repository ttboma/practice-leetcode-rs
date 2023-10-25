use crate::Solution;

impl Solution {
    /// # [241. Different Ways to Add Parentheses](https://leetcode.com/problems/different-ways-to-add-parentheses/)
    ///
    /// Given a string `expression` of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in **any order** .
    ///
    /// The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed `10^4`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: expression = "2-1-1"
    /// Output: [0,2]
    /// Explanation:
    /// ((2-1)-1) = 0
    /// (2-(1-1)) = 2
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: expression = "2*3-4*5"
    /// Output: [-34,-14,-10,-10,10]
    /// Explanation:
    /// (2*(3-(4*5))) = -34
    /// ((2*3)-(4*5)) = -14
    /// ((2*(3-4))*5) = -10
    /// (2*((3-4)*5)) = -10
    /// (((2*3)-4)*5) = 10
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= expression.length <= 20`
    /// - `expression` consists of digits and the operator `'+'`, `'-'`, and `'*'`.
    /// - All the integer values in the input expression are in the range `[0, 99]`.
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let operands: Vec<_> = expression
            .split(&['+', '-', '*', ' '][..])
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let operators: Vec<_> = expression
            .chars()
            .filter(|c| *c == '+' || *c == '-' || *c == '*')
            .collect();
        diff_ways_to_compute_impl(0, operands.len(), &operands, &operators)
    }
}

fn diff_ways_to_compute_impl(i: usize, j: usize, operands: &[i32], operators: &[char]) -> Vec<i32> {
    if j - i == 1 {
        return vec![operands[i]];
    }

    if j - i == 2 {
        return vec![operation(operators[i], operands[i], operands[i + 1])];
    }

    let mut result = vec![];
    for k in i + 1..j {
        let lvalues = diff_ways_to_compute_impl(i, k, operands, operators);
        let rvalues = diff_ways_to_compute_impl(k, j, operands, operators);
        for a in &lvalues {
            for b in &rvalues {
                result.push(operation(operators[k - 1], *a, *b));
            }
        }
    }
    result
}

fn operation(c: char, a: i32, b: i32) -> i32 {
    match c {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let expression = String::from("2-1-1");
        assert_eq!(Solution::diff_ways_to_compute(expression), vec![2, 0]);
    }

    #[test]
    fn example2() {
        let expression = String::from("2*3-4*5");
        assert_eq!(
            Solution::diff_ways_to_compute(expression),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
