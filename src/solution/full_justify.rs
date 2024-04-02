use super::Solution;

impl Solution {
    /// # [68. Text Justification](https://leetcode.com/problems/text-justification/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of strings `words` and a width `maxWidth`, format the text such that each line has exactly `maxWidth` characters and is fully (left and right) justified.
    ///
    /// You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces `' '` when necessary so that each line has exactly `maxWidth` characters.
    ///
    /// Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.
    ///
    /// For the last line of text, it should be left-justified, and no extra space is inserted between words.
    ///
    /// **Note:**
    ///
    /// - A word is defined as a character sequence consisting of non-space characters only.
    /// - Each word's length is guaranteed to be greater than `0` and not exceed `maxWidth`.
    /// - The input array `words` contains at least one word.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
    /// Output:
    /// [
    ///  "This  is  an",
    ///  "example of text",
    ///  "justification. "
    /// ]```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
    /// Output:
    /// [
    ///  "What  must  be",
    ///  "acknowledgment ",
    ///  "shall be    "
    /// ]
    /// Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
    /// Note that the second line is also left-justified because it contains only one word.```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
    /// Output:
    /// [
    ///  "Science is what we",
    ///   "understand   well",
    ///  "enough to explain to",
    ///  "a computer. Art is",
    ///  "everything else we",
    ///  "do         "
    /// ]```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= words.length <= 300`
    /// - `1 <= words[i].length <= 20`
    /// - `words[i]` consists of only English letters and symbols.
    /// - `1 <= maxWidth <= 100`
    /// - `words[i].length <= maxWidth`
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut i = 0;
        let mut ret = Vec::new();
        while i < words.len() {
            let mut line = String::new();
            let mut j = i;
            let mut acc = 0;

            while j < words.len() && acc + words[j].len() as i32 <= max_width {
                acc += words[j].len() as i32 + 1;
                j += 1;
            }

            let mut slots = (j - i - 1) as i32;
            let spaces = max_width - (acc - slots - 1);
            if slots == 0 {
                line += words[i].as_str();
                line += " ".repeat(spaces as usize).as_str();
            } else if j == words.len() {
                line.push_str(&words[i]);
                i += 1;
                while i < j {
                    line.push(' ');
                    line.push_str(&words[i]);
                    i += 1;
                }
                line += " ".repeat(spaces as usize - slots as usize).as_str();
            } else {
                let distributed_space = " ".repeat(spaces as usize / slots as usize);
                let extra_space = spaces % slots;
                for _ in 0..extra_space {
                    line += words[i].as_str();
                    line += distributed_space.as_str();
                    line += " ";
                    slots -= 1;
                    i += 1;
                }
                for _ in 0..slots {
                    line += words[i].as_str();
                    line += distributed_space.as_str();
                    i += 1;
                }
                line += words[i].as_str();
            }
            ret.push(line);
            i += 1;
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
            Solution::full_justify(
                vec![
                    String::from("This"),
                    String::from("is"),
                    String::from("an"),
                    String::from("example"),
                    String::from("of"),
                    String::from("text"),
                    String::from("justification."),
                ],
                16
            ),
            vec!["This    is    an", "example  of text", "justification.  "]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    String::from("What"),
                    String::from("must"),
                    String::from("be"),
                    String::from("acknowledgment"),
                    String::from("shall"),
                    String::from("be"),
                ],
                16
            ),
            vec!["What   must   be", "acknowledgment  ", "shall be        "]
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    String::from("Science"),
                    String::from("is"),
                    String::from("what"),
                    String::from("we"),
                    String::from("understand"),
                    String::from("well"),
                    String::from("enough"),
                    String::from("to"),
                    String::from("explain"),
                    String::from("to"),
                    String::from("a"),
                    String::from("computer."),
                    String::from("Art"),
                    String::from("is"),
                    String::from("everything"),
                    String::from("else"),
                    String::from("we"),
                    String::from("do"),
                ],
                20
            ),
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
