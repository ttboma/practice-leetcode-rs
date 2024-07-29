use super::*;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [100. Same Tree](https://leetcode.com/problems/same-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the roots of two binary trees `p` and `q`, write a function to check if they are the same or not.
    ///
    /// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex1.jpg" style="width: 622px; height: 182px;">
    ///
    /// ```txt
    /// Input: p = [1,2,3], q = [1,2,3]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex2.jpg" style="width: 382px; height: 182px;">
    ///
    /// ```txt
    /// Input: p = [1,2], q = [1,null,2]
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/ex3.jpg" style="width: 622px; height: 182px;">
    ///
    /// ```txt
    /// Input: p = [1,2,1], q = [1,1,2]
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in both trees is in the range `[0, 100]`.
    /// - `-10^4 <= Node.val <= 10^4`
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        } else if p.is_none() || q.is_none() {
            return false;
        }
        p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val
            && Solution::is_same_tree(
                p.as_ref().unwrap().borrow().left.clone(),
                q.as_ref().unwrap().borrow().left.clone(),
            )
            && Solution::is_same_tree(
                p.as_ref().unwrap().borrow().right.clone(),
                q.as_ref().unwrap().borrow().right.clone(),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let p = Tree::from(vec![Some(1), Some(2), Some(3)]);
        let q = Tree::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_same_tree(p.root, q.root), true);
    }

    #[test]
    fn example2() {
        let p = Tree::from(vec![Some(1), Some(2)]);
        let q = Tree::from(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::is_same_tree(p.root, q.root), false);
    }

    #[test]
    fn example3() {
        let p = Tree::from(vec![Some(1), Some(2), Some(1)]);
        let q = Tree::from(vec![Some(1), Some(1), Some(2)]);
        assert_eq!(Solution::is_same_tree(p.root, q.root), false);
    }
}
