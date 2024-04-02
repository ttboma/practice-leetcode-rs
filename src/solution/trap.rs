use super::Solution;
use std::cmp::Ordering;

impl Solution {
    /// # [42. Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given `n` non-negative integers representing an elevation map where the width of each bar is `1`, compute how much water it can trap after raining.
    ///
    /// **Example 1:**
    /// <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;">
    ///
    /// ```txt
    /// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
    /// Output: 6
    /// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: height = [4,2,0,3,2,5]
    /// Output: 9
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == height.length`
    /// - `1 <= n <= 2 * 10^4`
    /// - `0 <= height[i] <= 10^5`
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        while left <= right {
            match left_max.cmp(&right_max) {
                Ordering::Greater => {
                    result += 0.max(right_max - height[right]);
                    right_max = height[right].max(right_max);
                    right -= 1;
                }
                _ => {
                    result += 0.max(left_max - height[left]);
                    left_max = height[left].max(left_max);
                    left += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
