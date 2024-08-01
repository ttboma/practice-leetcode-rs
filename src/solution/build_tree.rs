use super::*;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [105. Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two integer arrays `preorder` and `inorder` where `preorder` is the preorder traversal of a binary tree and `inorder` is the inorder traversal of the same tree, construct and return the binary tree.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;">
    ///
    /// ```txt
    /// Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
    /// Output: [3,9,20,null,null,15,7]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: preorder = [-1], inorder = [-1]
    /// Output: [-1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= preorder.length <= 3000`
    /// - `inorder.length == preorder.length`
    /// - `-3000 <= preorder[i], inorder[i] <= 3000`
    /// - `preorder` and `inorder` consist of **unique**  values.
    /// - Each value of `inorder` also appears in `preorder`.
    /// - `preorder` is **guaranteed**  to be the preorder traversal of the tree.
    /// - `inorder` is **guaranteed**  to be the inorder traversal of the tree.
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree_impl(&preorder, &inorder)
    }
}

fn build_tree_impl(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        None
    } else {
        let i = inorder
            .iter()
            .enumerate()
            .find_map(|(i, val)| if *val == preorder[0] { Some(i) } else { None })
            .unwrap();
        let left = build_tree_impl(&preorder[1..1 + i], &inorder[0..i]);
        let right = build_tree_impl(&preorder[1 + i..], &inorder[i + 1..]);
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left,
            right,
        })))
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
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree.root
        );
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(-1)]);
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), tree.root);
    }
}
