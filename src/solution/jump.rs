use crate::Solution;

impl Solution {
    /// # [45. Jump Game II](https://leetcode.com/problems/jump-game-ii/)
    ///
    /// You are given a **0-indexed**  array of integers `nums` of length `n`. You are initially positioned at `nums[0]`.
    ///
    /// Each element `nums[i]` represents the maximum length of a forward jump from index `i`. In other words, if you are at `nums[i]`, you can jump to any `nums[i + j]` where:
    ///
    /// - `0 <= j <= nums[i]` and
    /// - `i + j < n`
    ///
    /// Return the minimum number of jumps to reach `nums[n - 1]`. The test cases are generated such that you can reach `nums[n - 1]`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,3,1,1,4]
    /// Output: 2
    /// Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [2,3,0,1,4]
    /// Output: 2
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `0 <= nums[i] <= 1000`
    /// - It's guaranteed that you can reach `nums[n - 1]`.
    pub fn jump(nums: Vec<i32>) -> i32 {
        jump_impl(&nums, 0, 0, 0)
    }
}

fn jump_impl(nums: &[i32], mut i_end: usize, mut answer: i32, mut reach: usize) -> i32 {
    for i in 0..(nums.len() - 1) {
        let jump = i + <i32 as TryInto<usize>>::try_into(nums[i]).unwrap();
        if nums.len() - 1 <= jump {
            return answer + 1;
        }
        reach = reach.max(jump);
        if i == i_end {
            answer += 1;
            i_end = reach;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }
    #[test]
    fn example2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }
    #[test]
    fn example3() {
        let nums = vec![1, 2, 1, 1, 1];
        assert_eq!(Solution::jump(nums), 3);
    }
    #[test]
    fn example4() {
        let nums = vec![0];
        assert_eq!(Solution::jump(nums), 0);
    }
}
