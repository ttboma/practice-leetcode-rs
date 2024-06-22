use crate::list_node::ListNode;
use crate::Solution;

impl Solution {
    /// # [82. Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list **sorted**  as well.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg" style="width: 500px; height: 142px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,3,4,4,5]
    /// Output: [1,2,5]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg" style="width: 500px; height: 205px;">
    ///
    /// ```txt
    /// Input: head = [1,1,1,2,3]
    /// Output: [2,3]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[0, 300]`.
    /// - `-100 <= Node.val <= 100`
    /// - The list is guaranteed to be **sorted**  in ascending order.
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(first) = p {
            let val = first.val;

            let mut q = &mut first.next;
            let mut find_duplicate = false;
            while let Some(second) = q {
                if second.val != val {
                    break;
                }
                find_duplicate = true;
                q = &mut q.as_mut().unwrap().next;
            }

            if find_duplicate {
                *p = q.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let head = singly_linked_list![1, 2, 3, 3, 4, 4, 5];
        let expected = singly_linked_list![1, 2, 5];
        assert_eq!(Solution::delete_duplicates(head.head), expected.head);
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![1, 1, 1, 2, 3];
        let expected = singly_linked_list![2, 3];
        assert_eq!(Solution::delete_duplicates(head.head), expected.head);
    }
}
