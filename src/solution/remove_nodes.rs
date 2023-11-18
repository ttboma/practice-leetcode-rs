use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [2487. Remove Nodes From Linked List](https://leetcode.com/problems/remove-nodes-from-linked-list/)
    ///
    /// You are given the `head` of a linked list.
    ///
    /// Remove every node which has a node with a greater value anywhere to the right side of it.
    ///
    /// Return the `head` of the modified linked list.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2022/10/02/drawio.png" style="width: 631px; height: 51px;">
    ///
    /// ```txt
    /// Input: head = [5,2,13,3,8]
    /// Output: [13,8]
    /// Explanation: The nodes that should be removed are 5, 2 and 3.
    /// - Node 13 is to the right of node 5.
    /// - Node 13 is to the right of node 2.
    /// - Node 8 is to the right of node 3.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = [1,1,1,1]
    /// Output: [1,1,1,1]
    /// Explanation: Every node has value 1, so no nodes are removed.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of the nodes in the given list is in the range `[1, 10^5]`.
    /// - `1 <= Node.val <= 10^5`
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        remove_nodes_impl(&mut head);
        head
    }
}

fn remove_nodes_impl(iter: &mut Option<Box<ListNode>>) -> i32 {
    if iter.is_none() {
        return 0;
    }

    if iter.as_ref().unwrap().next.is_none() {
        return iter.as_ref().unwrap().val;
    }

    let max_value_of_right_side = remove_nodes_impl(&mut iter.as_mut().unwrap().next);

    if iter.as_ref().unwrap().val < max_value_of_right_side {
        *iter = iter.as_mut().unwrap().next.take();
        max_value_of_right_side
    } else {
        iter.as_ref().unwrap().val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let head = singly_linked_list![5, 2, 13, 3, 8].head;
        let expected = singly_linked_list![13, 8].head;
        assert_eq!(Solution::remove_nodes(head), expected);
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![1, 1, 1, 1].head;
        let expected = singly_linked_list![1, 1, 1, 1].head;
        assert_eq!(Solution::remove_nodes(head), expected);
    }
}
