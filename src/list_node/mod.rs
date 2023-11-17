#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SinglyLinkedList {
    pub head: Option<Box<ListNode>>,
}

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
