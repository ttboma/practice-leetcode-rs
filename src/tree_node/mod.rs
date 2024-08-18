use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct Tree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl Tree {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Tree { root: None }
    }
}

use std::convert::From;
impl From<Vec<Option<i32>>> for Tree {
    fn from(value: Vec<Option<i32>>) -> Self {
        let mut fifo = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut idx = 0;
        if value.is_empty() || value[0].is_none() {
            return Tree::new();
        }
        let root_node = Rc::new(RefCell::new(TreeNode {
            val: value[0].unwrap(),
            left: None,
            right: None,
        }));
        fifo.push_back(root_node.clone());
        while let Some(node) = fifo.pop_front() {
            let left_node = value.get(idx + 1).and_then(|v| {
                (*v).map(|u| {
                    Rc::new(RefCell::new(TreeNode {
                        val: u,
                        left: None,
                        right: None,
                    }))
                })
            });
            let right_node = value.get(idx + 2).and_then(|v| {
                (*v).map(|u| {
                    Rc::new(RefCell::new(TreeNode {
                        val: u,
                        left: None,
                        right: None,
                    }))
                })
            });
            if let Some(v) = left_node {
                node.borrow_mut().left = Some(v.clone());
                fifo.push_back(v);
            }
            if let Some(v) = right_node {
                node.borrow_mut().right = Some(v.clone());
                fifo.push_back(v);
            }
            idx += 2
        }
        Tree {
            root: Some(root_node),
        }
    }
}

use std::fmt;
impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fifo = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        write!(f, "[")?;
        if self.root.is_none() {
            write!(f, "]")
        } else {
            write!(f, "{:?}", self.root.as_ref().unwrap().borrow().val)?;
            fifo.push_back(self.root.as_ref().unwrap().borrow().left.clone());
            fifo.push_back(self.root.as_ref().unwrap().borrow().right.clone());
            while let Some(node) = fifo.pop_front() {
                match node {
                    Some(node) => {
                        write!(f, ",{:?}", node.borrow().val)?;
                        fifo.push_back(node.borrow().left.clone());
                        fifo.push_back(node.borrow().right.clone());
                    }
                    None => write!(f, ",null")?,
                }
            }
            write!(f, "]")
        }
    }
}

/// # [173. Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Implement the `BSTIterator` class that represents an iterator over the **<a href="https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR)" target="_blank">in-order traversal</a>**  of a binary search tree (BST):
///
/// - `BSTIterator(TreeNode root)` Initializes an object of the `BSTIterator` class. The `root` of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
/// - `boolean hasNext()` Returns `true` if there exists a number in the traversal to the right of the pointer, otherwise returns `false`.
/// - `int next()` Moves the pointer to the right, then returns the number at the pointer.
///
/// Notice that by initializing the pointer to a non-existent smallest number, the first call to `next()` will return the smallest element in the BST.
///
/// You may assume that `next()` calls will always be valid. That is, there will be at least a next number in the in-order traversal when `next()` is called.
///
/// **Example 1:**
/// <img alt="" src="https://assets.leetcode.com/uploads/2018/12/25/bst-tree.png" style="width: 189px; height: 178px;">
///
/// ```txt
/// Input
///
/// ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
/// [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
/// Output
///
/// [null, 3, 7, true, 9, true, 15, true, 20, false]
///
/// Explanation
///
/// BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
/// bSTIterator.next();    // return 3
/// bSTIterator.next();    // return 7
/// bSTIterator.hasNext(); // return True
/// bSTIterator.next();    // return 9
/// bSTIterator.hasNext(); // return True
/// bSTIterator.next();    // return 15
/// bSTIterator.hasNext(); // return True
/// bSTIterator.next();    // return 20
/// bSTIterator.hasNext(); // return False
/// ```
///
/// **Constraints:**
///
/// - The number of nodes in the tree is in the range `[1, 10^5]`.
/// - `0 <= Node.val <= 10^6`
/// - At most `10^5` calls will be made to `hasNext`, and `next`.
///
/// **Follow up:**
///
/// - Could you implement `next()` and `hasNext()` to run in average `O(1)` time and use `O(h)` memory, where `h` is the height of the tree?
#[allow(dead_code)]
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    #[allow(dead_code)]
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut t = Self { stack: Vec::new() };
        t.push_left(root);
        t
    }

    #[allow(dead_code)]
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        self.push_left(node.borrow().right.clone());
        let val = node.borrow().val;
        val
    }

    #[allow(dead_code)]
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    #[allow(dead_code)]
    fn push_left(&mut self, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while root.is_some() {
            self.stack.push(root.clone().unwrap());
            root = root.unwrap().borrow().left.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let root = Tree::from(vec![
            Some(7),
            Some(3),
            Some(15),
            None,
            None,
            Some(9),
            Some(20),
        ]);
        let mut bst_iterator = BSTIterator::new(root.root.clone());
        assert_eq!(bst_iterator.next(), 3);
        assert_eq!(bst_iterator.next(), 7);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 9);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 15);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 20);
        assert_eq!(bst_iterator.has_next(), false);
    }
}
