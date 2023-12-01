use crate::Solution;

impl Solution {
    /// # [238. Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)
    ///
    /// Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`.
    ///
    /// The product of any prefix or suffix of `nums` is **guaranteed**  to fit in a **32-bit**  integer.
    ///
    /// You must write an algorithm that runs in`O(n)`time and without using the division operation.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,4]
    /// Output: [24,12,8,6]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [-1,1,0,-3,3]
    /// Output: [0,0,9,0,0]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= nums.length <= 10^5`
    /// - `-30 <= nums[i] <= 30`
    /// - The product of any prefix or suffix of `nums` is **guaranteed**  to fit in a **32-bit**  integer.
    ///
    /// **Follow up:** Can you solve the problem in `O(1)` extra space complexity? (The output array **does not**  count as extra space for space complexity analysis.)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mul_acc = |acc: &mut i32, x: &i32| {
            *acc *= *x;
            Some(*acc)
        };
        let mut a = nums.iter().scan(1, mul_acc);
        let mut b = nums.iter().rev().scan(1, mul_acc);
        let mut ret = vec![1; nums.len()];
        for v in ret.iter_mut().skip(1) {
            *v *= a.next().unwrap();
        }
        for v in ret.iter_mut().rev().skip(1) {
            *v *= b.next().unwrap();
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
