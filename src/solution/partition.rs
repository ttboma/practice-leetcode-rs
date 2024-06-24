use crate::ListNode;
use crate::Solution;
use std::mem;

impl Solution {
    /// # [86. Partition List](https://leetcode.com/problems/partition-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a linked list and a value `x`, partition it such that all nodes **less than**  `x` come before nodes **greater than or equal**  to `x`.
    ///
    /// You should **preserve**  the original relative order of the nodes in each of the two partitions.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/partition.jpg" style="width: 662px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,4,3,2,5,2], x = 3
    /// Output: [1,2,2,4,3,5]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = [2,1], x = 2
    /// Output: [1,2]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[0, 200]`.
    /// - `-100 <= Node.val <= 100`
    /// - `-200 <= x <= 200`
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        let mut tail = None;
        let mut q = &mut tail;
        while let Some(node) = p {
            if node.val < x {
                p = &mut p.as_mut().unwrap().next;
            } else {
                mem::swap(p, q);
                *p = q.as_mut().unwrap().next.take();
                q = &mut q.as_mut().unwrap().next;
            }
        }
        *p = tail;
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let list = singly_linked_list![1, 4, 3, 2, 5, 2];
        let x = 3;
        let result = singly_linked_list![1, 2, 2, 4, 3, 5];
        assert_eq!(Solution::partition(list.head, x), result.head);
    }

    #[test]
    fn example2() {
        let list = singly_linked_list![2, 1];
        let x = 2;
        let result = singly_linked_list![1, 2];
        assert_eq!(Solution::partition(list.head, x), result.head);
    }
}
