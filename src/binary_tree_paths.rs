use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [257. Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)
    ///
    /// Given the `root` of a binary tree, return all root-to-leaf paths in **any order** .
    ///
    /// A **leaf**  is a node with no children.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/12/paths-tree.jpg" style="width: 207px; height: 293px;">
    ///
    /// ```txt
    /// Input: root = [1,2,3,null,5]
    /// Output: ["1->2->5","1->3"]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: root = [1]
    /// Output: ["1"]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the tree is in the range `[1, 100]`.
    /// - `-100 <= Node.val <= 100`
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut t = InOrderTraversal::default();
        t.traverse(root);
        t.path()
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct InOrderTraversal {
    path: Vec<String>,
    position: Vec<i32>,
}

impl InOrderTraversal {
    fn path(self) -> Vec<String> {
        self.path
    }
    fn traverse(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        match root {
            None => {}
            Some(node) => {
                self.position.push(node.borrow().val);
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    let mut p = self.position[0].to_string();
                    for val in self.position.iter().skip(1) {
                        p.push_str(&format!("->{}", val));
                    }
                    self.path.push(p);
                } else {
                    self.traverse(node.borrow().left.clone());
                    self.traverse(node.borrow().right.clone());
                }
                self.position.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(
            Solution::binary_tree_paths(root),
            vec!["1->2->5".to_string(), "1->3".to_string()]
        );
    }

    #[test]
    fn example2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::binary_tree_paths(root), vec!["1".to_string()]);
    }
}
