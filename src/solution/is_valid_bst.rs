use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, determine if it is a valid binary search tree (BST).
    ///
    /// A **valid BST**  is defined as follows:
    ///
    /// - The left <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:r11:">subtree<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(120px, 289px);"> of a node contains only nodes with keys **less than**  the node's key.
    /// - The right subtree of a node contains only nodes with keys **greater than**  the node's key.
    /// - Both the left and right subtrees must also be binary search trees.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg" style="width: 302px; height: 182px;">
    ///
    /// ```txt
    /// Input: root = [2,1,3]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg" style="width: 422px; height: 292px;">
    ///
    /// ```txt
    /// Input: root = [5,1,4,null,null,3,6]
    /// Output: false
    /// Explanation: The root node's value is 5 but its right child's value is 4.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 10^4]`.
    /// - `-2^31 <= Node.val <= 2^31 - 1`
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_valid_bst_with_limit(root, None, None)
    }
}

fn is_valid_bst_with_limit(
    root: Option<Rc<RefCell<TreeNode>>>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
) -> bool {
    if let Some(node) = root {
        let val = node.borrow().val;
        lower_bound.map(|bound| bound < val).unwrap_or(true)
            && upper_bound.map(|bound| val < bound).unwrap_or(true)
            && is_valid_bst_with_limit(node.borrow().left.clone(), lower_bound, Some(val))
            && is_valid_bst_with_limit(node.borrow().right.clone(), Some(val), upper_bound)
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use std::i32;

    use super::*;
    use crate::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::is_valid_bst(tree.root), true);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert_eq!(Solution::is_valid_bst(tree.root), false);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![Some(2), Some(2), Some(2)]);
        assert_eq!(Solution::is_valid_bst(tree.root), false);
    }

    #[test]
    fn example4() {
        let tree = Tree::from(vec![Some(1), None, Some(1)]);
        assert_eq!(Solution::is_valid_bst(tree.root), false);
    }

    #[test]
    fn example5() {
        let tree = Tree::from(vec![Some(i32::MIN)]);
        assert_eq!(Solution::is_valid_bst(tree.root), true);
    }

    #[test]
    fn example6() {
        let tree = Tree::from(vec![Some(i32::MAX)]);
        assert_eq!(Solution::is_valid_bst(tree.root), true);
    }
}
