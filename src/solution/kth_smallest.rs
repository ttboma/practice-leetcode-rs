use crate::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [230. Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given the `root` of a binary search tree, and an integer `k`, return the `k^th` smallest value (**1-indexed** ) of all the values of the nodes in the tree.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree1.jpg" style="width: 212px; height: 301px;">
    ///
    /// ```txt
    /// Input: root = [3,1,4,null,2], k = 1
    /// Output: 1
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree2.jpg" style="width: 382px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [5,3,6,2,4,null,null,1], k = 3
    /// Output: 3
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is `n`.
    /// - `1 <= k <= n <= 10^4`
    /// - `0 <= Node.val <= 10^4`
    ///
    /// **Follow up:**  If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        kth_smallest_impl(root, &mut k).unwrap()
    }
}

fn kth_smallest_impl(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
    root.and_then(|node| {
        if let Some(val) = kth_smallest_impl(node.borrow().left.clone(), k) {
            Some(val)
        } else if *k == 1 {
            Some(node.borrow().val)
        } else {
            *k -= 1;
            kth_smallest_impl(node.borrow().right.clone(), k)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(Solution::kth_smallest(tree.root, 1), 1);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::kth_smallest(tree.root, 3), 3);
    }
}
