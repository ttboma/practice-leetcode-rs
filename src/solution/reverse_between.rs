use super::Solution;
use crate::ListNode;

impl Solution {
    /// # [92. Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a singly linked list and two integers `left` and `right` where `left <= right`, reverse the nodes of the list from position `left` to position `right`, and return the reversed list.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev2ex2.jpg" style="width: 542px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5], left = 2, right = 4
    /// Output: [1,4,3,2,5]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = [5], left = 1, right = 1
    /// Output: [5]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is `n`.
    /// - `1 <= n <= 500`
    /// - `-500 <= Node.val <= 500`
    /// - `1 <= left <= right <= n`
    ///
    /// **Follow up:**  Could you do it in one pass?
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut i = &mut head;
        for _ in 1..left {
            i = &mut i.as_mut().unwrap().next;
        }
        let mut splice_off = i.take();

        let mut j = &mut splice_off;
        for _ in left..=right {
            j = &mut j.as_mut().unwrap().next;
        }
        let mut tail = j.take();

        for _ in left..=right {
            let mut tmp = splice_off.take();
            splice_off = tmp.as_mut().unwrap().next.take();
            tmp.as_mut().unwrap().next = tail.take();
            tail = tmp;
        }
        *i = tail;

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let head = singly_linked_list![1, 2, 3, 4, 5];
        let left = 2;
        let right = 4;
        let res = singly_linked_list![1, 4, 3, 2, 5];
        assert_eq!(
            SinglyLinkedList {
                head: Solution::reverse_between(head.head, left, right),
            },
            res
        );
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![5];
        let left = 1;
        let right = 1;
        let res = singly_linked_list![5];
        assert_eq!(
            SinglyLinkedList {
                head: Solution::reverse_between(head.head, left, right),
            },
            res
        );
    }
}
