/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct SinglyLinkedList {
    pub head: Option<Box<ListNode>>,
}

impl SinglyLinkedList {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }

    pub fn iter(&self) -> SinglyLinkedListIterator {
        SinglyLinkedListIterator {
            link: self.head.as_deref(),
        }
    }

    pub fn push(&mut self, item: i32) {
        self.head = Some(Box::new(ListNode {
            val: item,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn reverse(mut self) -> SinglyLinkedList {
        let mut prev_node: Option<Box<ListNode>> = None;
        while self.head.is_some() {
            let mut tmp = self.head.as_mut().unwrap().next.take();
            self.head.as_mut().unwrap().next = prev_node.take();
            prev_node = self.head.take();
            self = SinglyLinkedList { head: tmp.take() };
        }
        SinglyLinkedList { head: prev_node }
    }

    pub fn splice_at_half(&mut self) -> SinglyLinkedList {
        if self.head.is_none() {
            return SinglyLinkedList { head: None };
        }
        let mut slow = self.head.as_deref_mut().unwrap() as *mut ListNode;
        let mut fast = slow;
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                if (*fast).next.is_none() {
                    break;
                }
                fast = (*fast).next.as_deref_mut().unwrap() as *mut ListNode;
                slow = (*slow).next.as_deref_mut().unwrap() as *mut ListNode;
            }
            SinglyLinkedList {
                head: (*slow).next.take(),
            }
        }
    }
}

use std::fmt;
impl fmt::Debug for SinglyLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut it = self.head.as_ref();
        if it.is_some() {
            write!(f, "{:?}", it.unwrap().val)?;
            it = it.unwrap().next.as_ref();
            while let Some(node) = it {
                write!(f, ", {:?}", node.val)?;
                it = it.unwrap().next.as_ref();
            }
        }
        write!(f, "]")
    }
}

use std::convert::From;
impl From<Vec<i32>> for SinglyLinkedList {
    fn from(value: Vec<i32>) -> Self {
        let mut list = SinglyLinkedList::new();
        for item in value.into_iter().rev() {
            list.push(item);
        }
        list
    }
}

#[macro_export]
macro_rules! singly_linked_list {
    ($($x:expr),*) => {{
        let mut list = SinglyLinkedList::new();
        let mut last = &mut list.head;
        $(
            *last = Some(Box::new(ListNode::new($x)));
            last = &mut { last }.as_mut().unwrap().next;
        )*
        let _ = last; // to workaround unused assignments warning
        list
    }};
}

pub struct SinglyLinkedListIterator<'a> {
    link: Option<&'a ListNode>,
}

use std::iter::Iterator;
impl<'a> Iterator for SinglyLinkedListIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.link {
            Some(node) => {
                self.link = node.next.as_deref();
                Some(&node.val)
            }
            None => None,
        }
    }
}
