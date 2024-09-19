use crate::Solution;

impl Solution {
    /// # [127. Word Ladder](https://leetcode.com/problems/word-ladder/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// A **transformation sequence**  from word `beginWord` to word `endWord` using a dictionary `wordList` is a sequence of words `beginWord -> s<sub>1</sub> -> s<sub>2</sub> -> ... -> s<sub>k</sub>` such that:
    ///
    /// - Every adjacent pair of words differs by a single letter.
    /// - Every `s<sub>i</sub>` for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in `wordList`.
    /// - `s<sub>k</sub> == endWord`
    ///
    /// Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return the **number of words**  in the **shortest transformation sequence**  from `beginWord` to `endWord`, or `0` if no such sequence exists.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
    /// Output: 5
    /// Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
    /// Output: 0
    /// Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= beginWord.length <= 10`
    /// - `endWord.length == beginWord.length`
    /// - `1 <= wordList.length <= 5000`
    /// - `wordList[i].length == beginWord.length`
    /// - `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
    /// - `beginWord != endWord`
    /// - All the words in `wordList` are **unique** .
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        word_list.push(begin_word);
        let mut color = vec![0; word_list.len()];
        let mut dist = vec![0; word_list.len()];
        let mut queue = std::collections::VecDeque::new();
        *color.last_mut().unwrap() = 1;
        queue.push_back(word_list.len() - 1);
        while let Some(idx) = queue.pop_front() {
            for (i, ele) in word_list.iter().enumerate() {
                if color[i] != 0 {
                    continue;
                } else if ele
                    .chars()
                    .zip(word_list[idx].chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1
                {
                    if *ele == end_word {
                        return (dist[idx] + 2) as i32;
                    }
                    color[i] = 1;
                    dist[i] = dist[idx] + 1;
                    queue.push_back(i);
                }
            }
            color[idx] = 2;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(), // 1
            "dot".to_string(), // 2,0
            "dog".to_string(), // 3,1
            "lot".to_string(), // 2,0
            "log".to_string(), // 3,3
            "cog".to_string(),
        ];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
    }

    #[test]
    fn example2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }
}
