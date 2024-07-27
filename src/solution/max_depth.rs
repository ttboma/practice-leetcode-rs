use super::*;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, return its maximum depth.
    ///
    /// A binary tree's **maximum depth** is the number of nodes along the longest path from the root node down to the farthest leaf node.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/tmp-tree.jpg" style="width: 400px; height: 277px;">
    ///
    /// ```txt
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: 3
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = [1,null,2]
    /// Output: 2
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 10^4]`.
    /// - `-100 <= Node.val <= 100`
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_ref().map_or(0, |node| {
            let left = (**node).borrow().left.clone();
            let right = (**node).borrow().right.clone();
            std::cmp::max(Solution::max_depth(left), Solution::max_depth(right)) + 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(tree.root), 3);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(tree.root), 2);
    }
}
