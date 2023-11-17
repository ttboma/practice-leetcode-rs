use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [143. Reorder List](https://leetcode.com/problems/reorder-list/)
    ///
    /// You are given the head of a singly linked-list. The list can be represented as:
    ///
    /// ```txt
    /// L<sub>0</sub> → L<sub>1</sub> → … → L<sub>n - 1</sub> → L<sub>n</sub>
    /// ```
    ///
    /// Reorder the list to be on the following form:
    ///
    /// ```txt
    /// L<sub>0</sub> → L<sub>n</sub> → L<sub>1</sub> → L<sub>n - 1</sub> → L<sub>2</sub> → L<sub>n - 2</sub> → …
    /// ```
    ///
    /// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg" style="width: 422px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4]
    /// Output: [1,4,2,3]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg" style="width: 542px; height: 222px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5]
    /// Output: [1,5,2,4,3]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[1, 5 * 10^4]`.
    /// - `1 <= Node.val <= 1000`
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut list2 = find_middle_node(head).take();
        let mut list2 = reverse_list(&mut list2);
        let mut new_list = None;
        let mut tail = &mut new_list;
        while head.is_some() {
            std::mem::swap(tail, &mut head.as_mut().unwrap().next);
            std::mem::swap(tail, head);
            tail = &mut tail.as_mut().unwrap().next;
            if list2.is_some() {
                std::mem::swap(tail, &mut list2.as_mut().unwrap().next);
                std::mem::swap(tail, &mut list2);
                tail = &mut tail.as_mut().unwrap().next;
            }
        }
        *head = new_list;
    }
}

fn find_middle_node(head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
    let mut slow = head as *mut Option<Box<ListNode>>;
    let mut fast = head as *mut Option<Box<ListNode>>;
    unsafe {
        while (*fast).is_some() {
            slow = &mut (*slow).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            fast = &mut (*fast).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            if (*fast).is_some() {
                fast = &mut (*fast).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            }
        }
        &mut (*slow)
    }
}

fn reverse_list(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = None;
    while head.is_some() {
        let mut tail = head.as_mut().unwrap().next.take();
        head.as_mut().unwrap().next = new_head.take();
        new_head = head.take();
        *head = tail.take();
    }
    new_head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }

    #[test]
    fn example2() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);
        assert_eq!(head, expected);
    }
}
