use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [129. Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given the `root` of a binary tree containing digits from `0` to `9` only.
    ///
    /// Each root-to-leaf path in the tree represents a number.
    ///
    /// - For example, the root-to-leaf path `1 -> 2 -> 3` represents the number `123`.
    ///
    /// Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a **32-bit**  integer.
    ///
    /// A **leaf**  node is a node with no children.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/num1tree.jpg" style="width: 212px; height: 182px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3]
    /// Output: 25
    /// Explanation:
    /// The root-to-leaf path `1->2` represents the number `12`.
    /// The root-to-leaf path `1->3` represents the number `13`.
    /// Therefore, sum = 12 + 13 = `25`.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/num2tree.jpg" style="width: 292px; height: 302px;">
    ///
    /// ```txt
    /// Input: root = [4,9,0,5,1]
    /// Output: 1026
    /// Explanation:
    /// The root-to-leaf path `4->9->5` represents the number 495.
    /// The root-to-leaf path `4->9->1` represents the number 491.
    /// The root-to-leaf path `4->0` represents the number 40.
    /// Therefore, sum = 495 + 491 + 40 = `1026`.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 1000]`.
    /// - `0 <= Node.val <= 9`
    /// - The depth of the tree will not exceed `10`.
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            unreachable!()
        }

        let mut sum = SumNumbers::new();
        sum.solve(root, 0);
        sum.sum
    }
}

struct SumNumbers {
    sum: i32,
}

impl SumNumbers {
    fn new() -> Self {
        Self { sum: 0 }
    }

    fn solve(&mut self, root: Option<Rc<RefCell<TreeNode>>>, num: i32) {
        let val = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        if left.is_none() && right.is_none() {
            self.sum += num + val;
            return;
        }

        if left.is_some() {
            self.solve(left, (num + val) * 10);
        }

        if right.is_some() {
            self.solve(right, (num + val) * 10);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::Tree;

    #[test]
    fn example1() {
        let tree = Tree::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::sum_numbers(tree.root), 25);
    }

    #[test]
    fn example2() {
        let tree = Tree::from(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]);
        assert_eq!(Solution::sum_numbers(tree.root), 1026);
    }
}
