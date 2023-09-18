use std::iter::successors;

use crate::Solution;

impl Solution {
    /// # 238. Product of Array Except Self
    ///
    /// Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.
    ///
    /// The product of any prefix or suffix of `nums` is guaranteed to fit in a **32-bit** integer.
    ///
    /// You must write an algorithm that runs in `O(n)` time and without using the division
    /// operation.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** nums = [1,2,3,4]
    /// - **Output:** [24,12,8,6]
    ///
    /// **Example 2:**
    ///
    /// - **Input:** nums = [-1,1,0,-3,3]
    /// - **Output:** [0,0,9,0,0]
    ///
    /// **Constraints:**
    ///
    /// - `2 <= nums.length <= 10^5`
    /// - `-5000 <= Node.val <= 5000` 
    /// - The product of any prefix or suffix of `nums` is guaranteed to fit in a **32-bit** integer.
    ///  
    /// **Follow up:** Can you solve the problem in `O(1)` extra space complexity? (The output array does not count as extra space for space complexity analysis.)
    ///
    /// ------
    ///
    /// ***Extracted from:*** [product-of-array-except-self](https://leetcode.com/problems/product-of-array-except-self/)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut it = nums.iter();
        let mut ret: Vec<i32>= successors(Some(1), |n| {match it.next() {
            None => None,
            Some(x) => Some(n * x),
        }}).collect();
        ret.pop();
        
        let mut it2 = nums.iter().rev();
        let mut b = successors(Some(1), |n| {match it2.next() {
            None => None,
            Some(x) => Some(n * x),
        }});

        for i in ret.iter_mut().rev() {
            *i = *i * b.next().unwrap();
        } 
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4];
        let output = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), output);
    }

    #[test]
    fn example2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let output = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), output);
    }
}
