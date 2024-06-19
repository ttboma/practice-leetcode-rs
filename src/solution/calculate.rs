use crate::Solution;

impl Solution {
    /// # [224. Basic Calculator](https://leetcode.com/problems/basic-calculator/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a string `s` representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.
    ///
    /// **Note:**  You are **not**  allowed to use any built-in function which evaluates strings as mathematical expressions, such as `eval()`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "1 + 1"
    /// Output: 2
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = " 2-1 + 2 "
    /// Output: 3
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "(1+(4+5+2)-3)+(6+8)"
    /// Output: 23
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 3 * 10^5`
    /// - `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`.
    /// - `s` represents a valid expression.
    /// - `'+'` is **not**  used as a unary operation (i.e., `"+1"` and `"+(2 + 3)"` is invalid).
    /// - `'-'` could be used as a unary operation (i.e., `"-1"` and `"-(2 + 3)"` is valid).
    /// - There will be no two consecutive operators in the input.
    /// - Every number and running calculation will fit in a signed 32-bit integer.
    pub fn calculate(s: String) -> i32 {
        expr(s.trim_start()).unwrap().1
    }
}

enum Token {
    Plus,
    Minus,
    LParen,
    RParen,
}

/// expr -> terminal_expr (PLUS terminal_expr | MINUS terminal_expr)*
/// terminal_expr -> NUMBER | LPAREN expr RPAREN | MINUS terminal_expr
/// NUMBER    -> [0-9]+
/// PLUS      -> '+'
/// MINUS     -> '-'
/// LPAREN    -> '('
/// RPAREN    -> ')'
fn expr(input: &str) -> Result<(&str, i32), &str> {
    if let Ok((input, num)) = terminal_expr(input) {
        let (input, v) = plus_or_minus_expr(input)?;
        return Ok((input, num + v));
    }
    Err(input)
}

fn terminal_expr(input: &str) -> Result<(&str, i32), &str> {
    if let Ok((input, num)) = number(input) {
        return Ok((input, num));
    } else if let Ok((input, Token::LParen)) = lparen(input) {
        let (input, num) = expr(input)?;
        if let Ok((input, Token::RParen)) = rparen(input) {
            return Ok((input, num));
        }
    } else if let Ok((input, Token::Minus)) = minus(input) {
        let (input, num) = terminal_expr(input)?;
        return Ok((input, -num));
    }
    Err(input)
}

fn plus_or_minus_expr(mut input: &str) -> Result<(&str, i32), &str> {
    let mut num = 0;
    loop {
        if let Ok((rem, v)) = plus_expr(input) {
            input = rem;
            num += v;
        } else if let Ok((rem, v)) = minus_expr(input) {
            input = rem;
            num += v;
        } else {
            break;
        }
    }
    Ok((input, num))
}

fn minus_expr(input: &str) -> Result<(&str, i32), &str> {
    if let Ok((input, Token::Minus)) = minus(input) {
        let (input, num) = terminal_expr(input)?;
        return Ok((input, -num));
    }
    Err(input)
}

fn plus_expr(input: &str) -> Result<(&str, i32), &str> {
    if let Ok((input, Token::Plus)) = plus(input) {
        let (input, num) = terminal_expr(input)?;
        return Ok((input, num));
    }
    Err(input)
}

fn number(input: &str) -> Result<(&str, i32), &str> {
    let num = input.find(|c: char| !c.is_ascii_digit());
    match num {
        Some(0) => Err(input),
        Some(n) => Ok((input[n..].trim_start(), input[..n].parse().unwrap())),
        _ => {
            let n = input.len();
            Ok((&input[n..], input[..n].parse().unwrap()))
        }
    }
}

fn minus(input: &str) -> Result<(&str, Token), &str> {
    let m = input.chars().next();
    match m {
        Some('-') => Ok((input[1..].trim_start(), Token::Minus)),
        _ => Err(input),
    }
}

fn plus(input: &str) -> Result<(&str, Token), &str> {
    let m = input.chars().next();
    match m {
        Some('+') => Ok((input[1..].trim_start(), Token::Plus)),
        _ => Err(input),
    }
}

fn lparen(input: &str) -> Result<(&str, Token), &str> {
    let m = input.chars().next();
    match m {
        Some('(') => Ok((input[1..].trim_start(), Token::LParen)),
        _ => Err(input),
    }
}

fn rparen(input: &str) -> Result<(&str, Token), &str> {
    let m = input.chars().next();
    match m {
        Some(')') => Ok((input[1..].trim_start(), Token::RParen)),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "1 + 1".to_string();
        assert_eq!(Solution::calculate(s), 2);
    }

    #[test]
    fn example2() {
        let s = " 2-1 + 2 ".to_string();
        assert_eq!(Solution::calculate(s), 3);
    }

    #[test]
    fn example3() {
        let s = "(1+(4+5+2)-3)+(6+8)".to_string();
        assert_eq!(Solution::calculate(s), 23);
    }

    #[test]
    fn example4() {
        let s = "- (3 + (4 + 5))".to_string();
        assert_eq!(Solution::calculate(s), -12);
    }
}
