use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # 21. Merge Two Sorted Lists
    ///
    /// You are given the heads of two sorted linked lists list1 and list2.
    /// Merge the two lists in a one sorted list. The list should be made by
    /// splicing together the nodes of the first two lists. Return the head
    /// of the merged linked list.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** list1 = [1,2,4], list2 = [1,3,4]
    /// - **Output:** [1,1,2,3,4,4]
    ///
    /// **Example 2:**
    ///
    /// - **Input:** list1 = [], list2 = []
    /// - **Output:** []
    ///
    /// **Example 3:**
    ///
    /// - **Input:** list1 = [], list2 = [0]
    /// - **Output:** [0]
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in both lists is in the range [0, 50].
    /// - -100 <= Node.val <= 100
    /// - Both list1 and list2 are sorted in non-decreasing order.
    ///
    /// ------ 
    ///
    /// ***Extracted from:*** [merge-two-sorted-lists](https://leetcode.com/problems/merge-two-sorted-lists/)
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let None = list1 {
            return list2;
        }
        if let None = list2 {
            return list1;
        }

        let (mut list1, mut list2) = if list2.as_ref().unwrap().val < list1.as_ref().unwrap().val {
            (list2, list1)
        } else {
            (list1, list2)
        };

        let mut i: &mut ListNode = list1.as_mut().unwrap();

        while let Some(ref mut v) = list2 {
            while !(i.next.is_none() || i.next.as_mut().unwrap().val > v.val) {
                i = i.next.as_mut().unwrap();
            }
            std::mem::swap(&mut i.next, &mut list2);
        }

        list1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let list1 = Some(Box::new(ListNode::new(4)));
        let list1 = Some(Box::new(ListNode {
            val: 2,
            next: list1,
        }));
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: list1,
        }));

        let list2 = Some(Box::new(ListNode::new(4)));
        let list2 = Some(Box::new(ListNode {
            val: 3,
            next: list2,
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: list2,
        }));

        let list3 = Some(Box::new(ListNode::new(4)));
        let list3 = Some(Box::new(ListNode {
            val: 4,
            next: list3,
        }));
        let list3 = Some(Box::new(ListNode {
            val: 3,
            next: list3,
        }));
        let list3 = Some(Box::new(ListNode {
            val: 2,
            next: list3,
        }));
        let list3 = Some(Box::new(ListNode {
            val: 1,
            next: list3,
        }));
        let list3 = Some(Box::new(ListNode {
            val: 1,
            next: list3,
        }));

        assert_eq!(Solution::merge_two_lists(list1, list2), list3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }
    #[test]
    fn example3() {
        let list2 = Some(Box::new(ListNode::new(0)));
        let list3 = Some(Box::new(ListNode::new(0)));
        assert_eq!(Solution::merge_two_lists(None, list2), list3);
    }
}
