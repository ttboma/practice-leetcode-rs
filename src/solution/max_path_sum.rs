use super::Solution;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [124. Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// A **path**  in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence **at most once** . Note that the path does not need to pass through the root.
    ///
    /// The **path sum**  of a path is the sum of the node's values in the path.
    ///
    /// Given the `root` of a binary tree, return the maximum **path sum**  of any **non-empty**  path.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg" style="width: 322px; height: 182px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3]
    /// Output: 6
    /// Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg">
    ///
    /// ```txt
    /// Input: root = [-10,9,20,null,null,15,7]
    /// Output: 42
    /// Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 3 * 10^4]`.
    /// - `-1000 <= Node.val <= 1000`
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        MaxPathSum::new().solve(root)
    }
}

struct MaxPathSum {
    max: i32,
}

impl MaxPathSum {
    fn new() -> Self {
        Self { max: i32::MIN }
    }

    fn solve(mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        self.apply(root);
        self.max
    }

    fn apply(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = self.apply(node.borrow().left.clone());
            let right = self.apply(node.borrow().right.clone());
            self.max = self.max.max(left.max(0) + right.max(0) + node.borrow().val);
            node.borrow().val + left.max(right).max(0)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::max_path_sum(tree.root), 6);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_path_sum(tree.root), 42);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![Some(-3)]);
        assert_eq!(Solution::max_path_sum(tree.root), -3);
    }
}
