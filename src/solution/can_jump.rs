use crate::Solution;

impl Solution {
    /// # [55. Jump Game](https://leetcode.com/problems/jump-game/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an integer array `nums`. You are initially positioned at the array's **first index** , and each element in the array represents your maximum jump length at that position.
    ///
    /// Return `true` if you can reach the last index, or `false` otherwise.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,3,1,1,4]
    /// Output: true
    /// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [3,2,1,0,4]
    /// Output: false
    /// Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `0 <= nums[i] <= 10^5`
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_index = 0;
        for (i, item) in nums.iter().enumerate().take(nums.len() - 1) {
            max_index = max_index.max(i + *item as usize);
            if max_index == i {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::can_jump(nums), true);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(Solution::can_jump(nums), false);
    }

    #[test]
    fn example3() {
        let nums = vec![0];
        assert_eq!(Solution::can_jump(nums), true);
    }
}
