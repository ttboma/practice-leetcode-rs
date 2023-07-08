use crate::Solution;

impl Solution {
    /// # 283. Move Zeroes
    ///
    /// Given an integer array `nums`, move all `0`'s to the end of it while maintaining the relative order of the non-zero elements.
    /// 
    /// Note that you must do this in-place without making a copy of the array.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** nums = [0,1,0,3,12]
    /// - **Output:** [1,3,12,0,0]
    ///
    /// **Example 2:**
    ///
    /// - **Input:** nums = [0]
    /// - **Output:** [0] 
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `-2^31 <= nums[i] <= 2^31 - 1`
    ///
    /// ------
    ///
    /// ***Extracted from:*** [move-zeroes](https://leetcode.com/problems/move-zeroes/)
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;

        while j != nums.len() {
            if nums[j] != 0 {
                if j != i {
                    nums[i] = nums[j];
                }
                i += 1;
            }
            j += 1;
        }

        while i != nums.len() {
            nums[i] = 0;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let output = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, output);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0];
        let output = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, output);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1];
        let output = vec![1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, output);
    }
}
