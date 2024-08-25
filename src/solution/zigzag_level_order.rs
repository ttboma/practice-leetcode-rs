use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [103. Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: [[3],[20,9],[15,7]]
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
    /// - `-100 <= Node.val <= 100`
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut queue = vec![node];
            let mut level = 0;
            while !queue.is_empty() {
                let mut queue_next = vec![];
                let mut level_result = vec![];
                for node in queue.into_iter().rev() {
                    level_result.push(node.borrow().val);
                    if level % 2 == 0 {
                        if let Some(node) = node.borrow().left.clone() {
                            queue_next.push(node);
                        }
                        if let Some(node) = node.borrow().right.clone() {
                            queue_next.push(node);
                        }
                    } else {
                        if let Some(node) = node.borrow().right.clone() {
                            queue_next.push(node);
                        }
                        if let Some(node) = node.borrow().left.clone() {
                            queue_next.push(node);
                        }
                    }
                }
                queue = queue_next;
                level += 1;
                result.push(level_result);
            }
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
            Solution::zigzag_level_order(tree.root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(1)]);
        assert_eq!(Solution::zigzag_level_order(tree.root), vec![vec![1]]);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![]);
        assert_eq!(
            Solution::zigzag_level_order(tree.root),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn example4() {
        let tree = Tree::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
        ]);
        assert_eq!(
            Solution::zigzag_level_order(tree.root),
            vec![vec![1], vec![3, 2], vec![4, 5]]
        );
    }
}
