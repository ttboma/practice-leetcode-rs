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
