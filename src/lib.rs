use std::cell::RefCell;
use std::rc::Rc;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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


/// LeetCode solutions provided by Shieh, Yueh-chang <ttboma@gmail.com>
/// Each of the methods for Solution struct are placed under its own module
pub struct Solution {}
mod count_bits;
mod count_odds;
mod count_pairs;
mod fib;
mod find_kth_positive;
mod generate;
mod is_isomorphic;
mod is_subsequence;
mod jump;
mod longest_palindrome;
mod max_profit;
mod merge_two_lists;
mod middle_node;
mod min_eating_speed;
mod min_score;
mod pivot_index;
mod reverse_list;
mod running_sum;
mod search;
mod ship_within_days;
mod total_fruit;
mod zero_filled_subarray;
mod merge_alternately;
mod gcd_of_strings;
mod kids_with_candies;

// parser utilities
pub mod parse_util;
