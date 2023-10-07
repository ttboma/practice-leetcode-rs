use crate::ListNode;
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
        let mut head = head;
        if head.as_mut().unwrap().next.is_none() {
            return true;
        }

        let mut head2 = head.splice_at_half().reverse();
        loop {
            if head.is_none() {
                break true;
            }
            if head.as_mut().unwrap().val != head2.as_deref_mut().unwrap().val {
                break false;
            }
            head = head.as_mut().unwrap().next.take();
            head2 = head2.as_mut().unwrap().next.take();
        }
    }
}

trait SingleLinkedList {
    fn reverse(self) -> Option<Box<ListNode>>;
    fn splice_at_half(&mut self) -> Option<Box<ListNode>>;
}

impl SingleLinkedList for Option<Box<ListNode>> {
    fn reverse(mut self) -> Option<Box<ListNode>> {
        let mut prev_node: Option<Box<ListNode>> = None;
        while self.is_some() {
            let mut tmp = self.as_mut().unwrap().next.take();
            self.as_mut().unwrap().next = prev_node.take();
            prev_node = self.take();
            self = tmp.take();
        }
        prev_node
    }

    fn splice_at_half(&mut self) -> Option<Box<ListNode>> {
        let mut slow = self.as_deref_mut().unwrap() as *mut ListNode;
        let mut fast = slow;
        unsafe {
            loop {
                if (*fast).next.is_none() {
                    break;
                }
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                if (*fast).next.is_none() {
                    break;
                }
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                if (*fast).next.is_none() {
                    break;
                }
                slow = (*slow).next.as_deref_mut().unwrap() as *mut ListNode;
            }
            (*slow).next.take()
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
