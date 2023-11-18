use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [24. Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
    ///
    /// Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4]
    /// Output: [2,1,4,3]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = []
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: head = [1]
    /// Output: [1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[0, 100]`.
    /// - `0 <= Node.val <= 100`
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_none() {
            return None;
        } else if head.as_mut().unwrap().next.is_none() {
            return head;
        }
        let mut next = head.as_mut().unwrap().next.take();
        let tail = next.as_mut().unwrap().next.take();

        if tail.is_some() {
            head.as_mut().unwrap().next = Self::swap_pairs(tail);
        }
        next.as_mut().unwrap().next = head.take();
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let head = singly_linked_list![1, 2, 3, 4].head;
        let expected = singly_linked_list![2, 1, 4, 3].head;
        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![1].head;
        let expected = singly_linked_list![1].head;
        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::swap_pairs(None), None);
    }
}
