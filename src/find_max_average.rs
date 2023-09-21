use crate::Solution;

impl Solution {
    /// # 643. Maximum Average Subarray I
    ///
    /// You are given an integer array `nums` consisting of `n` elements, and an integer `k`.
    ///
    /// Find a contiguous subarray whose length is equal to `k` that has the maximum average value and return this value. Any answer with a calculation error less than `10^-5` will be accepted.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** nums = [1,12,-5,-6,50,3], k = 4
    /// - **Output:** 12.75000
    /// - **Explanation:** Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
    ///
    /// **Example 2:**
    ///
    /// - **Input:** nums = [5], k = 1
    /// - **Output:** 5.00000
    ///
    /// **Constraints:**
    ///
    /// - `n == nums.length`
    /// - `1 <= k <= n <= 10^5`
    /// - `-10^4 <= nums[i] <= 10^4`
    ///
    /// ------
    ///
    /// ***Extracted from:*** [maximum-average-subarray-i](https://leetcode.com/problems/maximum-average-subarray-i/)
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        let mut sum: i32 = nums.iter().take(k).sum();
        let mut max_sum: i32 = sum;

        for i in 0..nums.len() - k {
            sum += nums[i + k] - nums[i];
            max_sum = max_sum.max(sum);
        }

        max_sum as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        assert_eq!(Solution::find_max_average(nums, k), 12.75);
    }

    #[test]
    fn example_2() {
        let nums = vec![5];
        let k = 1;
        assert_eq!(Solution::find_max_average(nums, k), 5.0)
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3, 4, 3, 0];
        let k = 3;
        assert_eq!(
            format!("{:.5}", Solution::find_max_average(nums, k)),
            "3.33333"
        )
    }
}
