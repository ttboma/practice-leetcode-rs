use crate::Solution;

impl Solution {
    /// # [150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an array of strings `tokens` that represents an arithmetic expression in a <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
    ///
    /// Evaluate the expression. Return an integer that represents the value of the expression.
    ///
    /// **Note**  that:
    ///
    /// - The valid operators are `'+'`, `'-'`, `'*'`, and `'/'`.
    /// - Each operand may be an integer or another expression.
    /// - The division between two integers always **truncates toward zero** .
    /// - There will not be any division by zero.
    /// - The input represents a valid arithmetic expression in a reverse polish notation.
    /// - The answer and all the intermediate calculations can be represented in a **32-bit**  integer.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: tokens = ["2","1","+","3","*"]
    /// Output: 9
    /// Explanation: ((2 + 1) * 3) = 9
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: tokens = ["4","13","5","/","+"]
    /// Output: 6
    /// Explanation: (4 + (13 / 5)) = 6
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
    /// Output: 22
    /// Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
    /// = ((10 * (6 / (12 * -11))) + 17) + 5
    /// = ((10 * (6 / -132)) + 17) + 5
    /// = ((10 * 0) + 17) + 5
    /// = (0 + 17) + 5
    /// = 17 + 5
    /// = 22
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= tokens.length <= 10^4`
    /// - `tokens[i]` is either an operator: `"+"`, `"-"`, `"*"`, or `"/"`, or an integer in the range `[-200, 200]`.
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        eval(&mut tokens)
    }
}

fn eval(tokens: &mut Vec<String>) -> i32 {
    let token = tokens.pop();
    match token.as_deref() {
        Some("+") => eval(tokens) + eval(tokens),
        Some("-") => {
            let a = eval(tokens);
            let b = eval(tokens);
            b - a
        }
        Some("*") => eval(tokens) * eval(tokens),
        Some("/") => {
            let a = eval(tokens);
            let b = eval(tokens);
            b / a
        }
        Some(val) => val.parse::<i32>().unwrap(),
        _ => {
            panic!("Invalid token!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tokens = ["2", "1", "+", "3", "*"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn example2() {
        let tokens = ["4", "13", "5", "/", "+"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn example3() {
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
