use crate::Solution;
use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    /// # [894. All Possible Full Binary Trees](https://leetcode.com/problems/all-possible-full-binary-trees/)
    /// Given an integer `n`, return a list of all possible **full binary trees**  with `n` nodes. Each node of each tree in the answer must have `Node.val == 0`.
    ///
    /// Each element of the answer is the root node of one possible tree. You may return the final list of trees in **any order** .
    ///
    /// A **full binary tree**  is a binary tree where each node has exactly `0` or `2` children.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/22/fivetrees.png" style="width: 700px; height: 400px;">
    ///
    /// ```txt
    /// Input: n = 7
    /// Output: [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 3
    /// Output: [[0,0,0]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 20`
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            vec![]
        } else {
            fbt((n as usize - 1) / 2)
        }
    }
}

fn fbt(i: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    match i {
        0 => vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))],
        1 => vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })))],
        _ => {
            let mut res = vec![fbt(0), fbt(1)];
            for _ in 0..i - 1 {
                add_next_fbt(&mut res);
            }
            res.pop().unwrap()
        }
    }
}

fn add_next_fbt(res: &mut Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>) {
    let mut next = vec![];
    for j in 0..res.len() {
        for left in &res[j] {
            for right in &res[res.len() - j - 1] {
                next.push(Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(left.clone().unwrap()),
                    right: Some(right.clone().unwrap()),
                }))));
            }
        }
    }
    res.push(next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let fbt = [
            Solution::all_possible_fbt(1),
            Solution::all_possible_fbt(3),
            Solution::all_possible_fbt(5),
        ];
        let output = [
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(fbt[0][0].clone().unwrap()),
                right: Some(fbt[2][0].clone().unwrap()),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(fbt[0][0].clone().unwrap()),
                right: Some(fbt[2][1].clone().unwrap()),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(fbt[1][0].clone().unwrap()),
                right: Some(fbt[1][0].clone().unwrap()),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(fbt[2][0].clone().unwrap()),
                right: Some(fbt[0][0].clone().unwrap()),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(fbt[2][1].clone().unwrap()),
                right: Some(fbt[0][0].clone().unwrap()),
            }))),
        ];
        assert_eq!(Solution::all_possible_fbt(7), output);
    }

    #[test]
    fn example2() {
        let output = vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })))];
        assert_eq!(Solution::all_possible_fbt(3), output);
    }

    #[test]
    fn example3() {
        let output = vec![];
        assert_eq!(Solution::all_possible_fbt(2), output);
    }
}
