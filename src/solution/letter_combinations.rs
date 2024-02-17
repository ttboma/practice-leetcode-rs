use super::*;
impl Solution {
    /// # [17. Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order** .
    ///
    /// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
    /// <img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;">
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: digits = "23"
    /// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: digits = ""
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: digits = "2"
    /// Output: ["a","b","c"]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= digits.length <= 4`
    /// - `digits[i]` is a digit in the range `['2', '9']`.
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits = digits
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let phone = vec![
            vec![],
            vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut ret = phone[digits[0]].iter().map(|s| s.to_string()).collect();
        if digits.len() == 1 {
            return ret;
        }
        // digits
        // ret <- FlatMap
        // phone -> phone[*i].iter(), closure1
        for i in digits.iter().skip(1) {
            ret = ret
                .iter()
                .flat_map(|s| {
                    phone[*i].iter().map(move |&c| {
                        let mut s = s.clone();
                        s.push(c);
                        s
                    })
                })
                .collect();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
