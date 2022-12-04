/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// LeetCode solutions provided by Shieh, Yueh-chang <ttboma@gmail.com>
pub struct Solution {}

/// Each of the methods for Solution struct are placed in its own module
mod running_sum_of_1d_array; 
mod find_pivot_index;
mod isomorphic_strings;
mod is_subsequence;
mod merge_two_sorted_lists;
mod reverse_linked_list;
mod middle_of_the_linked_list;
mod best_time_to_buy_and_sell_stock;
mod longest_palindrome;
mod fibonacci_number;
mod pascals_triangle;
