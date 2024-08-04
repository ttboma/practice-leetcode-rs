use crate::tree_node::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [114. Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a binary tree
    ///
    /// ```
    /// struct Node {
    ///   int val;
    ///   Node *left;
    ///   Node *right;
    ///   Node *next;
    /// }
    /// ```
    ///
    /// Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to `NULL`.
    ///
    /// Initially, all next pointers are set to `NULL`.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2019/02/15/117_sample.png" style="width: 500px; height: 171px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3,4,5,null,7]
    /// Output: [1,#,2,3,#,4,5,7,#]
    /// Explanation: Given the above binary tree (Figure A), your function should populate each next pointer to point to its next right node, just like in Figure B. The serialized output is in level order as connected by the next pointers, with '#' signifying the end of each level.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = []
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 6000]`.
    /// - `-100 <= Node.val <= 100`
    ///
    /// **Follow-up:**
    ///
    /// - You may only use constant extra space.
    /// - The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut right = root.as_ref().unwrap().borrow_mut().right.take();
        let mut left = root.as_ref().unwrap().borrow_mut().left.take();
        Self::flatten(&mut right);
        Self::flatten(&mut left);
        root.as_ref().unwrap().borrow_mut().right = left;

        let mut p = root;
        unsafe {
            while let Some(node) = p {
                let raw_p = &mut (**node).borrow_mut().right as *mut Option<Rc<RefCell<TreeNode>>>;
                p = &mut *raw_p;
            }
        }
        *p = right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let mut root = Tree::from(vec![
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ])
        .root;
        Solution::flatten(&mut root);
        let expected = Tree::from(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            None,
        ])
        .root;
        assert_eq!(root, expected);
    }

    #[test]
    fn example2() {
        let mut root = Tree::from(vec![]).root;
        Solution::flatten(&mut root);
        let expected = Tree::from(vec![]).root;
        assert_eq!(root, expected);
    }
}
