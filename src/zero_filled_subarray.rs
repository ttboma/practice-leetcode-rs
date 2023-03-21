use crate::Solution;

impl Solution {
    /// # 2348. Number of Zero-Filled Subarrays
    ///
    /// Given an integer array `nums`, return the number of subarrays filled with
    /// `0`. A **subarray** is a contiguous non-empty sequence of elements within
    /// an array.
    ///
    /// ## Example 1:
    ///
    /// - **Input:** nums = [1,3,0,0,2,0,0,4]
    /// - **Output:** 6
    /// - **Explanation:** 
    ///     - There are 4 occurrences of [0] as a subarray.
    ///     - There are 2 occurrences of [0,0] as a subarray.
    ///     - There is no occurrence of a subarray with a size more than 2 filled with 0. Therefore, we return 6.
    /// 
    /// ## Example 2:
    /// 
    /// - **Input:** nums = [0,0,0,2,0,0]
    /// - **Output:** 9
    /// - **Explanation:**
    ///     - There are 5 occurrences of [0] as a subarray.
    ///     - There are 3 occurrences of [0,0] as a subarray.
    ///     - There is 1 occurrence of [0,0,0] as a subarray.
    ///     - There is no occurrence of a subarray with a size more than 3 filled with 0. Therefore, we return 9.
    /// 
    /// ## Example 3:
    /// 
    /// - **Input:** nums = [2,10,2019]
    /// - **Output:** 0
    /// - **Explanation:** There is no subarray filled with 0. Therefore, we return 0.
    /// 
    /// ## Constraints:
    /// 
    /// - 1 <= nums.length <= 10^5
    /// - -10^9 <= nums[i] <= 10^9
    /// 
    /// ------
    ///
    /// ***Extracted from:*** [number-of-zero-filled-subarrays](https://leetcode.com/problems/number-of-zero-filled-subarrays/)
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut output = 0;
        let mut zero_detected = 0;
        for num in nums {
            if num == 0 {
                zero_detected += 1;
                output += zero_detected;
            } else if zero_detected != 0 {
                zero_detected = 0;
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        assert_eq!(Solution::zero_filled_subarray(nums), 6);
    }
    #[test]
    fn example2() {
        let nums = vec![0, 0, 0, 2, 0, 0];
        assert_eq!(Solution::zero_filled_subarray(nums), 9);
    }
    #[test]
    fn example3() {
        let nums = vec![2, 10, 2019];
        assert_eq!(Solution::zero_filled_subarray(nums), 0);
    }
}
