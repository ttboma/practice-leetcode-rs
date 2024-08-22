use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: [[3],[9,20],[15,7]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = [1]
    /// Output: [[1]]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: root = []
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 2000]`.
    /// - `-1000 <= Node.val <= 1000`
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut queue = vec![root];
        let mut result = Vec::new();
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut node_values = Vec::new();
            for node in queue {
                let node = node.unwrap();
                next_queue.push(node.borrow().left.clone());
                next_queue.push(node.borrow().right.clone());
                node_values.push(node.borrow().val);
            }
            queue = next_queue.into_iter().filter(|x| x.is_some()).collect();
            result.push(node_values);
        }
        result
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
        assert_eq!(
            Solution::level_order(tree.root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(1)]);
        assert_eq!(Solution::level_order(tree.root), vec![vec![1]]);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![]);
        assert_eq!(Solution::level_order(tree.root), Vec::<Vec<i32>>::new());
    }
}
