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

        let mut it = list.head.as_ref();
        let mut rev_it = rev_list.head.as_ref();
        while rev_it.is_some() {
            if it.unwrap().val != rev_it.unwrap().val {
                return false;
            }
            it = it.unwrap().next.as_ref();
            rev_it = rev_it.unwrap().next.as_ref();
        }
        true
    }
}

impl SinglyLinkedList {
    fn reverse(mut self) -> SinglyLinkedList {
        let mut prev_node: Option<Box<ListNode>> = None;
        while self.head.is_some() {
            let mut tmp = self.head.as_mut().unwrap().next.take();
            self.head.as_mut().unwrap().next = prev_node.take();
            prev_node = self.head.take();
            self = SinglyLinkedList { head: tmp.take() };
        }
        SinglyLinkedList { head: prev_node }
    }

    fn splice_at_half(&mut self) -> SinglyLinkedList {
        if self.head.is_none() {
            return SinglyLinkedList { head: None };
        }
        let mut slow = self.head.as_deref_mut().unwrap() as *mut ListNode;
        let mut fast = slow;
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                if (*fast).next.is_none() {
                    break;
                }
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                slow = (*slow).next.as_deref_mut().unwrap() as *mut ListNode;
            }
            SinglyLinkedList {
                head: (*slow).next.take(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(1))),
                })),
            })),
        }));
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn example2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        assert_eq!(Solution::is_palindrome(head), false);
    }

    #[test]
    fn example3() {
        let head = Some(Box::new(ListNode::new(2)));
        assert_eq!(Solution::is_palindrome(head), true);
    }
}
