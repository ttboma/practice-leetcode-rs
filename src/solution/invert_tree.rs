use super::*;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, invert the tree, and return its root.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert1-tree.jpg" style="width: 500px; height: 165px;">
    ///
    /// ```txt
    /// Input: root = [4,2,7,1,3,6,9]
    /// Output: [4,7,2,9,6,3,1]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert2-tree.jpg" style="width: 500px; height: 120px;">
    ///
    /// ```txt
    /// Input: root = [2,1,3]
    /// Output: [2,3,1]
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
    /// - The number of nodes in the tree is in the range `[0, 100]`.
    /// - `-100 <= Node.val <= 100`
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Solution::invert_tree(right);
            node.borrow_mut().right = Solution::invert_tree(left);
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example_1() {
        let tree = Tree::from(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let expected = Tree::from(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(tree.root), expected.root);
    }

    #[test]
    fn example_2() {
        let tree = Tree::from(vec![Some(2), Some(1), Some(3)]);
        let expected = Tree::from(vec![Some(2), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(tree.root), expected.root);
    }

    #[test]
    fn example_3() {
        let tree = Tree::from(vec![]);
        let expected = Tree::from(vec![]);
        assert_eq!(Solution::invert_tree(tree.root), expected.root);
    }
}
