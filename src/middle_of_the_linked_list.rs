use crate::Solution;
use crate::ListNode;

impl Solution {
    /// # 876. Middle of the Linked List
    ///
    /// Given the head of a singly linked list, return the middle node of the linked list.
    /// If there are two middle nodes, return the second middle node.
    /// 
    /// **Example 1:**
    /// 
    /// 
    /// - **Input:** head = [1,2,3,4,5]
    /// - **Output:** [3,4,5]
    /// - **Explanation:** The middle node of the list is node 3.
    ///
    /// **Example 2:**
    /// 
    /// 
    /// - **Input:** head = [1,2,3,4,5,6]
    /// - **Output:** [4,5,6]
    /// - **Explanation:** Since the list has two middle nodes with values 3 and 4, we return the second one.
    ///  
    /// 
    /// **Constraints:**
    /// 
    /// - The number of nodes in the list is in the range [1, 100].
    /// - 1 <= Node.val <= 100 
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_p = &head;
        let mut fast_p = &head;
        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            slow_p = &slow_p.as_ref().unwrap().next;
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow_p.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode::new(5)));
        let head = Some(Box::new(ListNode {val: 4, next: head}));
        let head = Some(Box::new(ListNode {val: 3, next: head}));
        let head = Some(Box::new(ListNode {val: 2, next: head}));
        let head = Some(Box::new(ListNode {val: 1, next: head}));

        let output = Some(Box::new(ListNode::new(5)));
        let output = Some(Box::new(ListNode {val: 4, next: output}));
        let output = Some(Box::new(ListNode {val: 3, next: output}));

        assert_eq!(Solution::middle_node(head), output);
    }
    #[test]
    fn example2() {
        let head = Some(Box::new(ListNode::new(6)));
        let head = Some(Box::new(ListNode {val: 5, next: head}));
        let head = Some(Box::new(ListNode {val: 4, next: head}));
        let head = Some(Box::new(ListNode {val: 3, next: head}));
        let head = Some(Box::new(ListNode {val: 2, next: head}));
        let head = Some(Box::new(ListNode {val: 1, next: head}));

        let output = Some(Box::new(ListNode::new(6)));
        let output = Some(Box::new(ListNode {val: 5, next: output}));
        let output = Some(Box::new(ListNode {val: 4, next: output}));

        assert_eq!(Solution::middle_node(head), output);
    }
    #[test]
    fn example3() {
        let head = Some(Box::new(ListNode::new(1)));

        let output = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::middle_node(head), output);
    }
    #[test]
    fn example4() {
        let head = Some(Box::new(ListNode::new(2)));
        let head = Some(Box::new(ListNode {val: 1, next: head}));

        let output = Some(Box::new(ListNode::new(2)));

        assert_eq!(Solution::middle_node(head), output);
    }
}
