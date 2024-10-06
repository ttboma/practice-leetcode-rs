use crate::*;

impl Solution {
    /// # [918. Maximum Sum Circular Subarray](https://leetcode.com/problems/maximum-sum-circular-subarray/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a **circular integer array**  `nums` of length `n`, return the maximum possible sum of a non-empty **subarray**  of `nums`.
    ///
    /// A **circular array**  means the end of the array connects to the beginning of the array. Formally, the next element of `nums[i]` is `nums[(i + 1) % n]` and the previous element of `nums[i]` is `nums[(i - 1 + n) % n]`.
    ///
    /// A **subarray**  may only include each element of the fixed buffer `nums` at most once. Formally, for a subarray `nums[i], nums[i + 1], ..., nums[j]`, there does not exist `i <= k1`, `k2 <= j` with `k1 % n == k2 % n`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,-2,3,-2]
    /// Output: 3
    /// Explanation: Subarray [3] has maximum sum 3.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [5,-3,5]
    /// Output: 10
    /// Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [-3,-2,-3]
    /// Output: -2
    /// Explanation: Subarray [-2] has maximum sum -2.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == nums.length`
    /// - `1 <= n <= 3 * 10^4`
    /// - `-3 * 10^4 <= nums[i] <= 3 * 10^4`
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut max_ending_here = nums[0];
        for &num in nums.iter().skip(1) {
            max_ending_here = std::cmp::max(num, max_ending_here + num);
            max_so_far = std::cmp::max(max_so_far, max_ending_here);
        }

        if max_so_far < 0 {
            return max_so_far;
        }

        let mut sum = 0;
        let mut min_so_far = 0;
        let mut min_ending_here = 0;
        for &num in &nums {
            sum += num;
            min_ending_here = std::cmp::min(num, min_ending_here + num);
            min_so_far = std::cmp::min(min_so_far, min_ending_here);
        }

        max_so_far.max(sum - min_so_far)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, -2, 3, -2];
        let result = 3;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }

    #[test]
    fn example2() {
        let nums = vec![5, -3, 5];
        let result = 10;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }

    #[test]
    fn example3() {
        let nums = vec![-3, -2, -3];
        let result = -2;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }

    #[test]
    fn example4() {
        let nums = vec![3, -2, 2, -3];
        let result = 3;

        assert_eq!(Solution::max_subarray_sum_circular(nums), result);
    }
}
