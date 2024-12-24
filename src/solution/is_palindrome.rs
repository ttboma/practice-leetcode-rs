use crate::ListNode;
use crate::SinglyLinkedList;
use crate::Solution;

impl Solution {
    /// # [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/description/)
    ///
    /// Given the `head` of a singly linked list, return `true` if it is a <div class="popover-wrapper inline-block" data-headlessui-state=""><div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rj:">palindrome<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(438px, 221px);"> or `false` otherwise.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal1linked-list.jpg" style="width: 422px; height: 62px;">
    ///
    /// ```txt
    /// Input: head = [1,2,2,1]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal2linked-list.jpg" style="width: 182px; height: 62px;">
    ///
    /// ```txt
    /// Input: head = [1,2]
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[1, 10^5]`.
    /// - `0 <= Node.val <= 9`
    ///
    /// **Follow up:**  Could you do it in `O(n)` time and `O(1)` space?
    pub fn is_palindrome_list(head: Option<Box<ListNode>>) -> bool {
        let mut list = SinglyLinkedList { head };
        let rev_list = list.splice_at_half().reverse();

        let mut it = list.iter();
        let mut rev_it = rev_list.iter();
        while let (Some(a), Some(b)) = (rev_it.next(), it.next()) {
            if a != b {
                return false;
            }
        }
        true
    }

    /// # [125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// A phrase is a **palindrome**  if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
    ///
    /// Given a string `s`, return `true` if it is a **palindrome** , or `false` otherwise.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "A man, a plan, a canal: Panama"
    /// Output: true
    /// Explanation: "amanaplanacanalpanama" is a palindrome.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "race a car"
    /// Output: false
    /// Explanation: "raceacar" is not a palindrome.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = " "
    /// Output: true
    /// Explanation: s is an empty string "" after removing non-alphanumeric characters.
    /// Since an empty string reads the same forward and backward, it is a palindrome.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 2 * 10^5`
    /// - `s` consists only of printable ASCII characters.
    pub fn is_palindrome_str(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let s: Vec<u8> = s
            .as_bytes()
            .iter()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        true
    }

    /// # [9. Palindrome Number](https://leetcode.com/problems/palindrome-number/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer `x`, return `true` if `x` is a <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rt:">**palindrome** <div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(332px, 183px);">, and `false` otherwise.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: x = 121
    /// Output: true
    /// Explanation: 121 reads as 121 from left to right and from right to left.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: x = -121
    /// Output: false
    /// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: x = 10
    /// Output: false
    /// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `-2^31<= x <= 2^31- 1`
    ///
    /// **Follow up:**  Could you solve it without converting the integer to a string?
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut tmp = x;
        let mut reverse_x = 0;
        while tmp != 0 {
            reverse_x *= 10;
            reverse_x += tmp % 10;
            tmp /= 10;
        }
        x == reverse_x
    }
}

#[cfg(test)]
mod tests {
    use crate::singly_linked_list;

    use super::*;

    #[test]
    fn example1_1() {
        let head = singly_linked_list![1, 2, 2, 1].head;
        assert!(Solution::is_palindrome_list(head));
    }

    #[test]
    fn example1_2() {
        let head = singly_linked_list![1, 2].head;
        assert!(!Solution::is_palindrome_list(head));
    }

    #[test]
    fn example1_3() {
        let head = singly_linked_list![2].head;
        assert!(Solution::is_palindrome_list(head));
    }

    #[test]
    fn example2_1() {
        let s = "A man, a plan, a canal: Panama".to_owned();
        assert!(Solution::is_palindrome_str(s));
    }

    #[test]
    fn example2_2() {
        let s = "race a car".to_owned();
        assert!(!Solution::is_palindrome_str(s));
    }

    #[test]
    fn example2_3() {
        let s = " ".to_owned();
        assert!(Solution::is_palindrome_str(s));
    }

    #[test]
    fn example3_1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn example3_2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn example3_3() {
        assert!(!Solution::is_palindrome(10));
    }
}
