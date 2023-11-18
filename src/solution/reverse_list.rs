use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)
    ///
    /// Given the `head` of a singly linked list, reverse the list, and return the reversed list.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5]
    /// Output: [5,4,3,2,1]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2]
    /// Output: [2,1]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: head = []
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is the range `[0, 5000]`.
    /// - `-5000 <= Node.val <= 5000`
    ///
    /// **Follow up:**  A linked list can be reversed either iteratively or recursively. Could you implement both?
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut out: Option<Box<ListNode>> = None;

        while let Some(mut v) = head {
            head = v.next;
            v.next = out;
            out = Some(v);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode { val: 4, next: head }));
        let head = Some(Box::new(ListNode { val: 3, next: head }));
        let head = Some(Box::new(ListNode { val: 2, next: head }));
        let head = Some(Box::new(ListNode { val: 1, next: head }));

        let output = Some(Box::new(ListNode::new(1)));
        let output = Some(Box::new(ListNode {
            val: 2,
            next: output,
        }));
        let output = Some(Box::new(ListNode {
            val: 3,
            next: output,
        }));
        let output = Some(Box::new(ListNode {
            val: 4,
            next: output,
        }));
        let output = Some(Box::new(ListNode {
            val: 5,
            next: output,
        }));
        assert_eq!(Solution::reverse_list(head), output);
    }
    #[test]
    fn example2() {
        let head = Some(Box::new(ListNode::new(2)));
        let head = Some(Box::new(ListNode { val: 1, next: head }));

        let output = Some(Box::new(ListNode::new(1)));
        let output = Some(Box::new(ListNode {
            val: 2,
            next: output,
        }));

        assert_eq!(Solution::reverse_list(head), output);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::reverse_list(None), None)
    }
}
