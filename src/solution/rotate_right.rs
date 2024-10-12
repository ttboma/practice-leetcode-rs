use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [61. Rotate List](https://leetcode.com/problems/rotate-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a linked list, rotate the list to the right by `k` places.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg" style="width: 450px; height: 191px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5], k = 2
    /// Output: [4,5,1,2,3]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg" style="width: 305px; height: 350px;">
    ///
    /// ```txt
    /// Input: head = [0,1,2], k = 4
    /// Output: [2,0,1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[0, 500]`.
    /// - `-100 <= Node.val <= 100`
    /// - `0 <= k <= 2 * 10^9`
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let n = size(&head);
        if n == 0 {
            return head;
        }
        let k = n - k % n;
        if k == 0 {
            return head;
        }

        let mut p = &mut head;
        for _ in 0..k {
            p = &mut p.as_mut().unwrap().next;
        }
        let mut tmp = p.take();
        let mut p = &mut tmp;
        for _ in k..n {
            p = &mut p.as_mut().unwrap().next;
        }
        *p = head;
        tmp
    }
}

fn size(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut n = 0;
    while let Some(node) = head {
        n += 1;
        head = &node.next;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let list = singly_linked_list![1, 2, 3, 4, 5];
        let k = 2;
        let result = singly_linked_list![4, 5, 1, 2, 3];
        assert_eq!(Solution::rotate_right(list.head, k), result.head);
    }

    #[test]
    fn example2() {
        let list = singly_linked_list![0, 1, 2];
        let k = 4;
        let result = singly_linked_list![2, 0, 1];
        assert_eq!(Solution::rotate_right(list.head, k), result.head);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::rotate_right(None, 0), None);
    }
}
