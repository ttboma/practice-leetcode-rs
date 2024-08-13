use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [222. Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a **complete**  binary tree, return the number of the nodes in the tree.
    ///
    /// According to **<a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>** , every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between `1` and `2^h` nodes inclusive at the last level `h`.
    ///
    /// Design an algorithm that runs in less than<code data-stringify-type="code">O(n)`time complexity.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/complete.jpg" style="width: 372px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3,4,5,6]
    /// Output: 6
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = []
    /// Output: 0
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: root = [1]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[0, 5 * 10^4]`.
    /// - `0 <= Node.val <= 5 * 10^4`
    /// - The tree is guaranteed to be **complete** .
    pub fn count_nodes(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        let mut a = left_depth(root.as_ref().unwrap().borrow().left.clone());
        let mut b = right_depth(root.as_ref().unwrap().borrow().left.clone());
        while root.is_some() {
            if a == b {
                sum += 2_i32.pow(a);
                root = root.unwrap().borrow().right.clone();
                a = left_depth(root.as_ref().and_then(|x| x.borrow().left.clone()));
                b = right_depth(root.as_ref().and_then(|x| x.borrow().left.clone()));
            } else {
                sum += 2_i32.pow(b);
                root = root.unwrap().borrow().left.clone();
                a -= 1;
                b = right_depth(root.as_ref().and_then(|x| x.borrow().left.clone()));
            }
        }
        sum
    }
}

fn left_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
    let mut left_depth = 0;
    while let Some(node) = root {
        left_depth += 1;
        root = node.borrow().left.clone();
    }
    left_depth
}

fn right_depth(mut root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
    let mut right_depth = 0;
    while let Some(node) = root {
        right_depth += 1;
        root = node.borrow().right.clone();
    }
    right_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let output = 6;
        assert_eq!(Solution::count_nodes(tree.root), output);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![]);
        let output = 0;
        assert_eq!(Solution::count_nodes(tree.root), output);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![Some(1)]);
        let output = 1;
        assert_eq!(Solution::count_nodes(tree.root), output);
    }
}
