use crate::*;

impl Solution {
    /// # [53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums`, find the <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:r11:">subarray<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(285px, 182px);"> with the largest sum, and return its sum.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
    /// Output: 6
    /// Explanation: The subarray [4,-1,2,1] has the largest sum 6.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1]
    /// Output: 1
    /// Explanation: The subarray [1] has the largest sum 1.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [5,4,-1,7,8]
    /// Output: 23
    /// Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^5`
    /// - `-10^4 <= nums[i] <= 10^4`
    ///
    /// **Follow up:**  If you have figured out the `O(n)` solution, try coding another solution using the **divide and conquer**  approach, which is more subtle.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_at_end = nums[0];
        let mut max_so_far = nums[0];
        for num in nums.into_iter().skip(1) {
            max_at_end = num.max(max_at_end + num);
            max_so_far = max_so_far.max(max_at_end);
        }
        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let ans = 6;
        assert_eq!(Solution::max_sub_array(nums), ans);
    }

    #[test]
    fn example2() {
        let nums = vec![1];
        let ans = 1;
        assert_eq!(Solution::max_sub_array(nums), ans);
    }

    #[test]
    fn example3() {
        let nums = vec![5, 4, -1, 7, 8];
        let ans = 23;
        assert_eq!(Solution::max_sub_array(nums), ans);
    }
}
