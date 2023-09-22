//! # Development Note
//!
//! * Use chrome and [Clip LeetCode](https://chrome.google.com/webstore/detail/clip-leetcode/cnghimckckgcmhbdokjielmhkmnagdcp/related)
//!   extension to maintain documentation of each method of [`Solution`]

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
mod all_possible_fbt;
mod can_place_flowers;
mod contains_duplicate;
mod count_bits;
mod count_odds;
mod count_pairs;
mod divisor_game;
mod fib;
mod find_difference;
mod find_kth_positive;
mod find_max_average;
mod gcd_of_strings;
mod generate;
mod get_maximum_generated;
mod get_row;
mod is_anagram;
mod is_isomorphic;
mod is_subsequence;
mod jump;
mod kids_with_candies;
mod largest_altitude;
mod longest_palindrome;
mod majority_element;
mod max_profit;
mod merge_alternately;
mod merge_two_lists;
mod middle_node;
mod min_cost_climbing_stairs;
mod min_eating_speed;
mod min_score;
mod move_zeroes;
mod pivot_index;
mod product_except_self;
mod reverse_list;
mod reverse_vowels;
mod reverse_words;
mod running_sum;
mod search;
mod ship_within_days;
mod total_fruit;
mod zero_filled_subarray;

// parser utilities
pub mod parse_util;
