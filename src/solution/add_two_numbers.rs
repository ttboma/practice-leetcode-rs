use crate::{ListNode, Solution};

impl Solution {
    /// # [2. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given two **non-empty**  linked lists representing two non-negative integers. The digits are stored in **reverse order** , and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
    ///
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;">
    ///
    /// ```txt
    /// Input: l1 = [2,4,3], l2 = [5,6,4]
    /// Output: [7,0,8]
    /// Explanation: 342 + 465 = 807.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: l1 = [0], l2 = [0]
    /// Output: [0]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    /// Output: [8,9,9,9,0,0,0,1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in each linked list is in the range `[1, 100]`.
    /// - `0 <= Node.val <= 9`
    /// - It is guaranteed that the list represents a number that does not have leading zeros.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut output: Option<Box<ListNode>> = None;
        let mut it1 = l1.as_ref();
        let mut it2 = l2.as_ref();
        let mut it3 = &mut output;
        let mut carry = 0;
        while it1.is_some() || it2.is_some() {
            let a = it1.map_or(0, |p| p.val);
            let b = it2.map_or(0, |p| p.val);
            let mut val = a + b + carry;
            carry = val / 10;
            val %= 10;
            if let Some(node) = it3 {
                node.val = val;
                it3 = &mut node.next;
            } else {
                *it3 = Some(Box::new(ListNode { val, next: None }));
                it3 = &mut it3.as_mut().unwrap().next;
            }

            it1 = it1.and_then(|p| p.next.as_ref());
            it2 = it2.and_then(|p| p.next.as_ref());
        }
        if carry > 0 {
            *it3 = Some(Box::new(ListNode {
                val: carry,
                next: None,
            }));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let l1 = singly_linked_list![2, 4, 3];
        let l2 = singly_linked_list![5, 6, 4];
        let output = singly_linked_list![7, 0, 8];
        assert_eq!(Solution::add_two_numbers(l1.head, l2.head), output.head);
    }

    #[test]
    fn example2() {
        let l1 = singly_linked_list![0];
        let l2 = singly_linked_list![0];
        let output = singly_linked_list![0];
        assert_eq!(Solution::add_two_numbers(l1.head, l2.head), output.head);
    }

    #[test]
    fn example3() {
        let l1 = singly_linked_list![9, 9, 9, 9, 9, 9, 9];
        let l2 = singly_linked_list![9, 9, 9, 9];
        let output = singly_linked_list![8, 9, 9, 9, 0, 0, 0, 1];
        assert_eq!(Solution::add_two_numbers(l1.head, l2.head), output.head);
    }
}
