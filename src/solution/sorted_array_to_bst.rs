use super::*;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [108. Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums` where the elements are sorted in **ascending order** , convert it to a <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rt:">**height-balanced** <div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(83px, 236px);"> binary search tree.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree1.jpg" style="width: 302px; height: 222px;">
    ///
    /// ```txt
    /// Input: nums = [-10,-3,0,5,9]
    /// Output: [0,-3,9,-10,null,5]
    /// Explanation: [0,-10,5,null,-3,null,9] is also accepted:
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree2.jpg" style="width: 302px; height: 222px;">
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree.jpg" style="width: 342px; height: 142px;">
    ///
    /// ```txt
    /// Input: nums = [1,3]
    /// Output: [3,1]
    /// Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `-10^4 <= nums[i] <= 10^4`
    /// - `nums` is sorted in a **strictly increasing**  order.
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_array_to_bst(&nums)
    }
}

fn sorted_array_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        None
    } else if nums.len() == 1 {
        Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))))
    } else {
        let (left, right) = nums.split_at(nums.len() / 2);
        Some(Rc::new(RefCell::new(TreeNode {
            val: right[0],
            left: sorted_array_to_bst(left),
            right: sorted_array_to_bst(&right[1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![-10, -3, 0, 5, 9];
        assert_eq!(
            Solution::sorted_array_to_bst(nums),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                    right: None
                })))
            })))
        );
    }

    #[test]
    fn example2() {
        let nums = vec![1, 3];
        assert_eq!(
            Solution::sorted_array_to_bst(nums),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None
            })))
        );
    }
}
