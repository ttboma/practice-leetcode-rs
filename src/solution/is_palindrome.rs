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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
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
}

#[cfg(test)]
mod tests {
    use crate::singly_linked_list;

    use super::*;

    #[test]
    fn example1() {
        let head = singly_linked_list![1, 2, 2, 1].head;
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![1, 2].head;
        assert_eq!(Solution::is_palindrome(head), false);
    }

    #[test]
    fn example3() {
        let head = singly_linked_list![2].head;
        assert_eq!(Solution::is_palindrome(head), true);
    }
}
