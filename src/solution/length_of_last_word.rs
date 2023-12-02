use super::*;

impl Solution {
    /// # [58. Length of Last Word](https://leetcode.com/problems/length-of-last-word/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a string `s` consisting of words and spaces, return the length of the **last**  word in the string.
    ///
    /// A **word**  is a maximal <div class="popover-wrapper inline-block" data-headlessui-state=""><div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rf:">substring<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(192px, 279px);"> consisting of non-space characters only.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "Hello World"
    /// Output: 5
    /// Explanation: The last word is "World" with length 5.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "   fly me   to   the moon  "
    /// Output: 4
    /// Explanation: The last word is "moon" with length 4.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "luffy is still joyboy"
    /// Output: 6
    /// Explanation: The last word is "joyboy" with length 6.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 10^4`
    /// - `s` consists of only English letters and spaces `' '`.
    /// - There will be at least one word in `s`.
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
