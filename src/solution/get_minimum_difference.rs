use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [530. Minimum Absolute Difference in BST](https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst1.jpg" style="width: 292px; height: 301px;">
    ///
    /// ```
    /// Input: root = [4,2,6,1,3]
    /// Output: 1
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst2.jpg" style="width: 282px; height: 301px;">
    ///
    /// ```
    /// Input: root = [1,0,48,null,null,12,49]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[2, 10^4]`.
    /// - `0 <= Node.val <= 10^5`
    ///
    /// **Note:**  This question is the same as 783: <a href="https://leetcode.com/problems/minimum-distance-between-bst-nodes/" target="_blank">https://leetcode.com/problems/minimum-distance-between-bst-nodes/</a>
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev = None;
        let mut min_diff = i32::MAX;
        inorder(root, &mut prev, &mut min_diff);
        min_diff
    }
}

fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min_diff: &mut i32) {
    if let Some(node) = root {
        inorder(node.borrow().left.clone(), prev, min_diff);
        let val = node.borrow().val;
        if let Some(prev) = prev {
            *min_diff = (*min_diff).min(val - *prev);
        }
        *prev = Some(val);
        inorder(node.borrow().right.clone(), prev, min_diff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        assert_eq!(Solution::get_minimum_difference(tree.root), 1);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![
            Some(1),
            Some(0),
            Some(48),
            None,
            None,
            Some(12),
            Some(49),
        ]);
        assert_eq!(Solution::get_minimum_difference(tree.root), 1);
    }
}
