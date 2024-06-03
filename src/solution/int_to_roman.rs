use super::*;
use std::ops::Rem;

impl Solution {
    /// # [12. Integer to Roman](https://leetcode.com/problems/integer-to-roman/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Roman numerals are represented by seven different symbols:`I`, `V`, `X`, `L`, `C`, `D` and `M`.
    ///
    /// ```txt
    /// **Symbol**        **Value**
    /// I             1
    /// V             5
    /// X             10
    /// L             50
    /// C             100
    /// D             500
    /// M             1000```
    ///
    /// For example,`2` is written as `II`in Roman numeral, just two ones added together. `12` is written as`XII`, which is simply `X + II`. The number `27` is written as `XXVII`, which is `XX + V + II`.
    ///
    /// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not `IIII`. Instead, the number four is written as `IV`. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as `IX`. There are six instances where subtraction is used:
    ///
    /// - `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
    /// - `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
    /// - `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: num = 3
    /// Output: "III"
    /// Explanation: 3 is represented as 3 ones.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: num = 58
    /// Output: "LVIII"
    /// Explanation: L = 50, V = 5, III = 3.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: num = 1994
    /// Output: "MCMXCIV"
    /// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= num <= 3999`
    pub fn int_to_roman(num: i32) -> String {
        let mut roman_numeral = String::new();
        let roman_symbols = [('M', 'M'), ('C', 'D'), ('X', 'L'), ('I', 'V')];
        for (i, d) in (0..=3)
            .rev()
            .map(|i| (num / 10i32.pow(i)).rem(10))
            .enumerate()
        {
            if d == 0 {
            } else if d < 4 {
                (0..d).for_each(|_| roman_numeral.push(roman_symbols[i].0));
            } else if d == 4 {
                roman_numeral.push(roman_symbols[i].0);
                roman_numeral.push(roman_symbols[i].1);
            } else if d == 9 {
                roman_numeral.push(roman_symbols[i].0);
                roman_numeral.push(roman_symbols[i - 1].0);
            } else {
                roman_numeral.push(roman_symbols[i].1);
                (0..d - 5).for_each(|_| roman_numeral.push(roman_symbols[i].0));
            }
        }
        roman_numeral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
