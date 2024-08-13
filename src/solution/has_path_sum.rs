use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [112. Path Sum](https://leetcode.com/problems/path-sum/)
    ///
    /// Given the `root` of a binary tree and an integer `targetSum`, return `true` if the tree has a **root-to-leaf**  path such that adding up all the values along the path equals `targetSum`.
    ///
    /// A **leaf**  is a node with no children.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum1.jpg" style="width: 500px; height: 356px;">
    ///
    /// ```txt
    /// Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
    /// Output: true
    /// Explanation: The root-to-leaf path with the target sum is shown.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg">
    ///
    /// ```txt
    /// Input: root = [1,2,3], targetSum = 5
    /// Output: false
    /// Explanation: There two root-to-leaf paths in the tree:
    /// (1 --> 2): The sum is 3.
    /// (1 --> 3): The sum is 4.
    /// There is no root-to-leaf path with sum = 5.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: root = [], targetSum = 0
    /// Output: false
    /// Explanation: Since the tree is empty, there are no root-to-leaf paths.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 5000]`.
    /// - `-1000 <= Node.val <= 1000`
    /// - `-1000 <= targetSum <= 1000`
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let val = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if target_sum - val == 0 && left.is_none() && right.is_none() {
            return true;
        }
        Solution::has_path_sum(left, target_sum - val)
            || Solution::has_path_sum(right, target_sum - val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
        })));
        let target_sum = 22;
        assert_eq!(Solution::has_path_sum(root, target_sum), true);
    }

    #[test]
    fn example2() {
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let target_sum = 5;
        assert_eq!(Solution::has_path_sum(root, target_sum), false);
    }

    #[test]
    fn example3() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let target_sum = 0;
        assert_eq!(Solution::has_path_sum(root, target_sum), false);
    }
}
