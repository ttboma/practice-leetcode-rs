use super::*;

impl Solution {
    /// # [136. Single Number](https://leetcode.com/problems/single-number/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a **non-empty** array of integers `nums`, every element appears twice except for one. Find that single one.
    ///
    /// You must implement a solution with a linear runtime complexity and use only constant extra space.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,2,1]
    /// Output: 1
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [4,1,2,1,2]
    /// Output: 4
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [1]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 3 * 10^4`
    /// - `-3 * 10^4 <= nums[i] <= 3 * 10^4`
    /// - Each element in the array appears twice except for one element which appears only once.
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &num| acc ^ num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 2, 1];
        let expected = 1;
        let result = Solution::single_number(nums);
        assert_eq!(expected, result);
    }

    #[test]
    fn example2() {
        let nums = vec![4, 1, 2, 1, 2];
        let expected = 4;
        let result = Solution::single_number(nums);
        assert_eq!(expected, result);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        let expected = 1;
        let result = Solution::single_number(nums);
        assert_eq!(expected, result);
    }
}
