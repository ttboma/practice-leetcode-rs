use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [236. Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
    ///
    /// According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: “The lowest common ancestor is defined between two nodes `p` and `q` as the lowest node in `T` that has both `p` and `q` as descendants (where we allow <b>a node to be a descendant of itself</b>).”
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;">
    ///
    /// ```txt
    /// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
    /// Output: 3
    /// Explanation: The LCA of nodes 5 and 1 is 3.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;">
    ///
    /// ```txt
    /// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
    /// Output: 5
    /// Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: root = [1,2], p = 1, q = 2
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[2, 10^5]`.
    /// - `-10^9 <= Node.val <= 10^9`
    /// - All `Node.val` are **unique** .
    /// - `p != q`
    /// - `p` and `q` will exist in the tree.
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut solver = Search::new(p, q);
        solver.post_order_search(root);
        solver.ans
    }
}

struct Search {
    ans: Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
}

impl Search {
    fn new(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            ans: None,
            p: p.unwrap().borrow().val,
            q: q.unwrap().borrow().val,
        }
    }

    fn post_order_search(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let val = root.as_ref().unwrap().borrow().val;
        let left = self.post_order_search(root.as_ref().unwrap().borrow().left.clone());
        let right = self.post_order_search(root.as_ref().unwrap().borrow().right.clone());
        if left == 2 || right == 2 {
            2
        } else if val == self.p || val == self.q {
            if left == 1 || right == 1 {
                self.ans = root;
                return 2;
            }
            1
        } else {
            if left == 1 && right == 1 {
                self.ans = root;
                return 2;
            }
            left + right
        }
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
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = tree.root.as_ref().unwrap().borrow().left.clone();
        let q = tree.root.as_ref().unwrap().borrow().right.clone();
        let ans = Solution::lowest_common_ancestor(tree.root, p, q);
        assert_eq!(ans.as_ref().unwrap().borrow().val, 3);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = tree.root.as_ref().unwrap().borrow().left.clone();
        let q = p
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .clone()
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .clone();
        let ans = Solution::lowest_common_ancestor(tree.root, p, q);
        assert_eq!(ans.as_ref().unwrap().borrow().val, 5);
    }

    #[test]
    fn example3() {
        let tree = Tree::from(vec![Some(1), Some(2)]);
        let p = tree.root.clone();
        let q = tree.root.as_ref().unwrap().borrow().left.clone();
        let ans = Solution::lowest_common_ancestor(tree.root, p, q);
        assert_eq!(ans.as_ref().unwrap().borrow().val, 1);
    }
}
