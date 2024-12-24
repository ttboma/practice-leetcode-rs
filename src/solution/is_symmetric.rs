use super::*;
use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;">
    ///
    /// ```txt
    /// Input: root = [1,2,2,3,4,4,3]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;">
    ///
    /// ```txt
    /// Input: root = [1,2,2,null,3,null,3]
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 1000]`.
    /// - `-100 <= Node.val <= 100`
    ///
    /// **Follow up:**  Could you solve it both recursively and iteratively?
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map(|node| {
            is_symmetrically_equal(node.borrow().left.clone(), node.borrow().right.clone())
        })
        .unwrap()
    }
}

fn is_symmetrically_equal(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if left.is_none() && right.is_none() {
        return true;
    } else if left.is_none()
        || right.is_none()
        || left.as_ref().unwrap().borrow().val != right.as_ref().unwrap().borrow().val
    {
        return false;
    }

    is_symmetrically_equal(
        left.as_ref().unwrap().borrow_mut().right.take(),
        right.as_ref().unwrap().borrow_mut().left.take(),
    ) && is_symmetrically_equal(
        left.as_ref().unwrap().borrow_mut().left.take(),
        right.as_ref().unwrap().borrow_mut().right.take(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example_1() {
        let tree = Tree::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert!(Solution::is_symmetric(tree.root));
    }

    #[test]
    fn example_2() {
        let tree = Tree::from(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        assert!(!Solution::is_symmetric(tree.root));
    }
}
