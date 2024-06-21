use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [19. Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a linked list, remove the `n^th` node from the end of the list and return its head.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5], n = 2
    /// Output: [1,2,3,5]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = [1], n = 1
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: head = [1,2], n = 1
    /// Output: [1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is `sz`.
    /// - `1 <= sz <= 30`
    /// - `0 <= Node.val <= 100`
    /// - `1 <= n <= sz`
    ///
    /// **Follow up:**  Could you do this in one pass?
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut p = head.as_ref();
        let mut list_size = 0;
        while let Some(node) = p {
            list_size += 1;
            p = node.next.as_ref();
        }

        let i = list_size - n;
        let mut p = &mut head;
        for _ in 0..i {
            p = &mut p.as_mut().unwrap().next;
        }
        *p = p.take().as_mut().unwrap().next.take();

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let input = singly_linked_list![1, 2, 3, 4, 5];
        let n = 2;
        let result = singly_linked_list![1, 2, 3, 5];
        assert_eq!(Solution::remove_nth_from_end(input.head, n), result.head);
    }

    #[test]
    fn example2() {
        let input = singly_linked_list![1];
        let n = 1;
        let result = singly_linked_list![];
        assert_eq!(Solution::remove_nth_from_end(input.head, n), result.head);
    }

    #[test]
    fn example3() {
        let input = singly_linked_list![1, 2];
        let n = 1;
        let result = singly_linked_list![1];
        assert_eq!(Solution::remove_nth_from_end(input.head, n), result.head);
    }
}
