use crate::*;

impl Solution {
    /// # [148. Sort List](https://leetcode.com/problems/sort-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `head` of a linked list, return the list after sorting it in **ascending order** .
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg" style="width: 450px; height: 194px;">
    ///
    /// ```txt
    /// Input: head = [4,2,1,3]
    /// Output: [1,2,3,4]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg" style="width: 550px; height: 184px;">
    ///
    /// ```txt
    /// Input: head = [-1,5,3,4,0]
    /// Output: [-1,0,3,4,5]
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
    /// - The number of nodes in the list is in the range `[0, 5 * 10^4]`.
    /// - `-10^5 <= Node.val <= 10^5`
    ///
    /// **Follow up:**  Can you sort the linked list in `O(n logn)` time and `O(1)` memory (i.e. constant space)?
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_sort(head)
    }

    /// Implementation of [`Solution::sort_list`].
    ///
    /// Time complexity: O(nlogn), Space complexity: O(1)
    /// However, the quicksort implementation is not working for large n: Stack overflow occurs.
    pub fn quick_sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let p = &mut head as *mut Option<Box<ListNode>>;
        let mut q = &mut head;
        let mut count = 0;
        while q.is_some() {
            count += 1;
            q = &mut q.as_mut().unwrap().next;
        }
        Self::quick_sort_impl(p, count);
        head
    }

    fn quick_sort_impl(p: *mut Option<Box<ListNode>>, count: i32) {
        let (pivot_value, k, mut q) = if count <= 1 {
            return;
        } else {
            let node = unsafe { (*p).as_mut().unwrap() };
            (
                node.val,
                &mut node.next as *mut Option<Box<ListNode>>,
                &mut node.next,
            )
        };

        let mut count_smaller = 0;

        for _ in 1..count {
            let val = q.as_mut().unwrap().val;
            if val >= pivot_value {
                q = &mut q.as_mut().unwrap().next;
            } else {
                count_smaller += 1;
                unsafe {
                    let temp = (*p).take();
                    (*p) = q.take();
                    (*q) = (*p).as_mut().unwrap().next.take();
                    (*p).as_mut().unwrap().next = temp;
                }
            }
        }

        Self::quick_sort_impl(p, count_smaller);
        Self::quick_sort_impl(k, count - count_smaller - 1);
    }

    /// Implementation of [`Solution::sort_list`].
    ///
    // Time complexity: O(nlogn), Space complexity: O(1)
    pub fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(node) = head.as_ref() {
            if node.next.is_none() {
                head
            } else {
                let (mut left, mut right) = Self::halve(head);
                left = Self::merge_sort(left);
                right = Self::merge_sort(right);
                Self::merge_runs(left, right)
            }
        } else {
            None
        }
    }

    fn merge_runs(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cursor = &mut list1;
        while list2.is_some() {
            if cursor.is_none() || list2.as_ref()?.val < cursor.as_ref()?.val {
                std::mem::swap(cursor, &mut list2);
            }
            cursor = &mut cursor.as_mut()?.next;
        }
        list1
    }

    fn halve(mut list: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut len = 0usize;
        let mut counter = &list;
        while let Some(node) = counter.as_ref() {
            counter = &node.next;
            len += 1;
        }
        let mut cursor = &mut list;
        for _ in 0..(len / 2 - 1) {
            cursor = &mut cursor.as_mut().unwrap().next;
        }
        let right = cursor.as_mut().unwrap().next.take();
        (list, right)
    }

    /// Implementation of [`Solution::sort_list`].
    ///
    /// Time complexity: O(n), Space complexity: O(n)
    pub fn bucket_sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        // Find min and max values
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut current = &head;
        while let Some(node) = current {
            min = min.min(node.val);
            max = max.max(node.val);
            current = &node.next;
        }

        // Count occurrences
        let range = (max - min + 1) as usize;
        let mut count = vec![0; range];
        let mut current = head.take();
        while let Some(mut node) = current {
            count[(node.val - min) as usize] += 1;
            current = node.next.take();
        }

        // Reconstruct the sorted list
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for (i, &c) in count.iter().enumerate() {
            for _ in 0..c {
                tail.next = Some(Box::new(ListNode::new(i as i32 + min)));
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next.take()
    }

    /// Implementation of [`Solution::sort_list`].
    ///
    /// Cheat with Vec::sort
    pub fn vec_sort(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = vec![];
        while let Some(mut node) = head {
            vec.push(node.val);
            head = node.next.take();
        }
        vec.sort();
        let mut next = None;
        while let Some(val) = vec.pop() {
            next = Some(Box::new(ListNode { next, val }))
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = SinglyLinkedList::from(vec![4, 2, 1, 3]);
        let expected = SinglyLinkedList::from(vec![1, 2, 3, 4]);
        assert_eq!(Solution::sort_list(root.head), expected.head);
    }

    #[test]
    fn example2() {
        let root = SinglyLinkedList::from(vec![-1, 5, 3, 4, 0]);
        let expected = SinglyLinkedList::from(vec![-1, 0, 3, 4, 5]);
        assert_eq!(Solution::sort_list(root.head), expected.head);
    }

    #[test]
    fn example3() {
        let root = SinglyLinkedList::from(vec![]);
        let expected = SinglyLinkedList::from(vec![]);
        assert_eq!(Solution::sort_list(root.head), expected.head);
    }

    #[test]
    fn example4() {
        // NOTE: When n is large, stack overflow will occur. n differs depending on the Platform.
        //       E.g. The following is tested on my macOS:
        //            n = 4116 using Solution::quick_sort implementation,
        //            n = 11972 using Solution::vec_sort and Solution::bucket_sort implementation.
        //       Why does Solution::vec_sort stack overflow occur?
        //       I guess it is actually because of the recursive call in the assert_eq! macro, which
        //       recursively check for the equality of the linked list. The stack overflow occurs
        let n = 100;

        let random_values: Vec<i32> = (0..n).rev().collect();
        let root = SinglyLinkedList::from(random_values);
        let ans = Solution::sort_list(root.head);

        let expected_values: Vec<i32> = (0..n).collect();
        let expected = SinglyLinkedList::from(expected_values);
        assert_eq!(ans, expected.head);
    }
}
