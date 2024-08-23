use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [637. Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within `10^-5` of the actual answer will be accepted.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/avg1-tree.jpg" style="width: 277px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: [3.00000,14.50000,11.00000]
    /// Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
    /// Hence return [3, 14.5, 11].
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/avg2-tree.jpg" style="width: 292px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [3,9,20,15,7]
    /// Output: [3.00000,14.50000,11.00000]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 10^4]`.
    /// - `-2^31 <= Node.val <= 2^31 - 1`
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue = vec![root.unwrap()];
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut sum = 0f64;
            let count = queue.len();
            for node in queue {
                let node = node.borrow();
                sum += node.val as f64;
                if node.left.is_some() {
                    next_queue.push(node.left.clone().unwrap());
                }
                if node.right.is_some() {
                    next_queue.push(node.right.clone().unwrap());
                }
            }
            result.push(sum / count as f64);
            queue = next_queue;
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
            Solution::average_of_levels(tree.root),
            vec![3.0, 14.5, 11.0]
        );
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(3), Some(9), Some(20), Some(15), Some(7)]);
        assert_eq!(
            Solution::average_of_levels(tree.root),
            vec![3.0, 14.5, 11.0]
        );
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![Some(2147483647), Some(2147483647), Some(2147483647)]);
        assert_eq!(
            Solution::average_of_levels(tree.root),
            vec![2147483647.00000, 2147483647.00000]
        );
    }
}
