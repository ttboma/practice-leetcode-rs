use crate::tree_node::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [114. Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, flatten the tree into a "linked list":
    ///
    /// - The "linked list" should use the same `TreeNode` class where the `right` child pointer points to the next node in the list and the `left` child pointer is always `null`.
    /// - The "linked list" should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank">**pre-order** ** traversal** </a> of the binary tree.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;">
    ///
    /// ```txt
    /// Input: root = [1,2,5,3,4,null,6]
    /// Output: [1,null,2,null,3,null,4,null,5,null,6]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = []
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: root = [0]
    /// Output: [0]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 2000]`.
    /// - `-100 <= Node.val <= 100`
    ///
    /// **Follow up:**  Can you flatten the tree in-place (with `O(1)` extra space)?
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut right = root.as_ref().unwrap().borrow_mut().right.take();
        let mut left = root.as_ref().unwrap().borrow_mut().left.take();
        Self::flatten(&mut right);
        Self::flatten(&mut left);
        root.as_ref().unwrap().borrow_mut().right = left;

        // Use unsafe code to find the rightmost node
        // let mut p = root;
        // unsafe {
        //     while let Some(node) = p {
        //         let raw_p = &mut (**node).borrow_mut().right as *mut Option<Rc<RefCell<TreeNode>>>;
        //         p = &mut *raw_p;
        //     }
        // }
        // *p = right;

        // Use safe code to find the rightmost node
        let mut p = root.clone();
        loop {
            let next = p.as_ref().unwrap().borrow_mut().right.clone();
            if next.is_none() {
                break;
            } else {
                p = next;
            }
        }
        p.as_ref().unwrap().borrow_mut().right = right;
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
