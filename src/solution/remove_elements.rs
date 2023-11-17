use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)
    ///
    /// Given the `head` of a linked list and an integer `val`, remove all the nodes of the linked list that has `Node.val == val`, and return the new head.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/06/removelinked-list.jpg" style="width: 500px; height: 142px;">
    ///
    /// ```txt
    /// Input: head = [1,2,6,3,4,5,6], val = 6
    /// Output: [1,2,3,4,5]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: head = [], val = 1
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: head = [7,7,7,7], val = 7
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[0, 10^4]`.
    /// - `1 <= Node.val <= 50`
    /// - `0 <= val <= 50`
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut list = None;
        let mut tail = &mut list;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val != val {
                *tail = Some(node);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode::new(6))),
                        })),
                    })),
                })),
            })),
        }));
        let output = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        }));
        assert_eq!(Solution::remove_elements(head, 6), output);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::remove_elements(None, 1), None);
    }

    #[test]
    fn example3() {
        let head = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode::new(7))),
                })),
            })),
        }));
        assert_eq!(Solution::remove_elements(head, 7), None);
    }
}
