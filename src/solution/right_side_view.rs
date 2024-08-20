use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [199. Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, imagine yourself standing on the **right side**  of it, return the values of the nodes you can see ordered from top to bottom.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/14/tree.jpg" style="width: 401px; height: 301px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3,null,5,null,4]
    /// Output: [1,3,4]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = [1,null,3]
    /// Output: [1,3]
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        drl_traversal(&mut result, root, 0);
        result
    }
}

fn drl_traversal(result: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>, level: usize) {
    if let Some(node) = root {
        if result.len() == level {
            result.push(node.borrow().val);
        }
        drl_traversal(result, node.borrow().right.clone(), level + 1);
        drl_traversal(result, node.borrow().left.clone(), level + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);
        let expected = vec![1, 3, 4];
        assert_eq!(Solution::right_side_view(tree.root), expected);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(1), None, Some(3)]);
        let expected = vec![1, 3];
        assert_eq!(Solution::right_side_view(tree.root), expected);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![]);
        let expected = vec![];
        assert_eq!(Solution::right_side_view(tree.root), expected);
    }
}
