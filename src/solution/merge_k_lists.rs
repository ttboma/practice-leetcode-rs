use crate::*;
use std::collections::{binary_heap::PeekMut, BinaryHeap};

impl Solution {
    /// # [23. Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order.
    ///
    /// Merge all the linked-lists into one sorted linked-list and return it.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: lists = [[1,4,5],[1,3,4],[2,6]]
    /// Output: [1,1,2,3,4,4,5,6]
    /// Explanation: The linked-lists are:
    /// [
    ///   1->4->5,
    ///   1->3->4,
    ///   2->6
    /// ]
    /// merging them into one sorted list:
    /// 1->1->2->3->4->4->5->6
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: lists = []
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: lists = [[]]
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `k == lists.length`
    /// - `0 <= k <= 10^4`
    /// - `0 <= lists[i].length <= 500`
    /// - `-10^4 <= lists[i][j] <= 10^4`
    /// - `lists[i]` is sorted in **ascending order** .
    /// - The sum of `lists[i].length` will not exceed `10^4`.
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut merged_list = None;
        let mut insert_pos = &mut merged_list;

        let mut loser_tree: BinaryHeap<Option<Box<ListNode>>> =
            lists.into_iter().filter(|list| list.is_some()).collect();

        while let Some(mut loser) = loser_tree.peek_mut() {
            *insert_pos = loser.take();
            insert_pos = &mut insert_pos.as_mut().unwrap().next;
            *loser = insert_pos.take();
            if loser.is_none() {
                PeekMut::pop(loser);
            }
        }

        merged_list
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order to make it a min heap
        other.val.cmp(&self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            singly_linked_list![1, 4, 5].head,
            singly_linked_list![1, 3, 4].head,
            singly_linked_list![2, 6].head,
        ];
        assert_eq!(
            Solution::merge_k_lists(input),
            singly_linked_list![1, 1, 2, 3, 4, 4, 5, 6].head
        );
    }

    #[test]
    fn example2() {
        let input = vec![singly_linked_list![].head];
        assert_eq!(Solution::merge_k_lists(input), None);
    }

    #[test]
    fn example3() {
        let input = vec![];
        assert_eq!(Solution::merge_k_lists(input), None);
    }
}
