use super::*;
use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    /// # [1. Two Sum](https://leetcode.com/problems/two-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of integers `nums`and an integer `target`, return indices of the two numbers such that they add up to `target`.
    ///
    /// You may assume that each input would have **exactly one solution** , and you may not use the same element twice.
    ///
    /// You can return the answer in any order.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,7,11,15], target = 9
    /// Output: [0,1]
    /// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [3,2,4], target = 6
    /// Output: [1,2]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [3,3], target = 6
    /// Output: [0,1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= nums.length <= 10^4`
    /// - `-10^9 <= nums[i] <= 10^9`
    /// - `-10^9 <= target <= 10^9`
    /// - **Only one valid answer exists.**
    ///
    /// **Follow-up:** Can you come up with an algorithm that is less than `O(n^2)`time complexity?
    pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();
        for (i, &e) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - e)) {
                return vec![j as i32, i as i32];
            }
            map.insert(e, i);
        }
        vec![]
    }

    /// # [167. Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a **1-indexed**  array of integers `numbers` that is already **sorted in non-decreasing order** , find two numbers such that they add up to a specific `target` number. Let these two numbers be `numbers[index<sub>1</sub>]` and `numbers[index<sub>2</sub>]` where `1 <= index<sub>1</sub> < index<sub>2</sub> <= numbers.length`.
    ///
    /// Return the indices of the two numbers, `index<sub>1</sub>` and `index<sub>2</sub>`, **added by one**  as an integer array `[index<sub>1</sub>, index<sub>2</sub>]` of length 2.
    ///
    /// The tests are generated such that there is **exactly one solution** . You **may not**  use the same element twice.
    ///
    /// Your solution must use only constant extra space.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: numbers = [2,7,11,15], target = 9
    /// Output: [1,2]
    /// Explanation: The sum of 2 and 7 is 9. Therefore, index<sub>1</sub> = 1, index<sub>2</sub> = 2. We return [1, 2].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: numbers = [2,3,4], target = 6
    /// Output: [1,3]
    /// Explanation: The sum of 2 and 4 is 6. Therefore index<sub>1</sub> = 1, index<sub>2</sub> = 3. We return [1, 3].
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: numbers = [-1,0], target = -1
    /// Output: [1,2]
    /// Explanation: The sum of -1 and 0 is -1. Therefore index<sub>1</sub> = 1, index<sub>2</sub> = 2. We return [1, 2].
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= numbers.length <= 3 * 10^4`
    /// - `-1000 <= numbers[i] <= 1000`
    /// - `numbers` is sorted in **non-decreasing order** .
    /// - `-1000 <= target <= 1000`
    /// - The tests are generated such that there is **exactly one solution** .
    pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let sum = numbers[i] + numbers[j];
            match sum.cmp(&target) {
                Ordering::Less => {
                    i += 1;
                }
                Ordering::Greater => {
                    j -= 1;
                }
                Ordering::Equal => {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum_1(nums, target), vec![0, 1]);
    }

    #[test]
    fn example1_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum_1(nums, target), vec![1, 2]);
    }

    #[test]
    fn example1_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum_1(nums, target), vec![0, 1]);
    }

    #[test]
    fn example2_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum_2(nums, target), vec![1, 2]);
    }

    #[test]
    fn example2_2() {
        let nums = vec![2, 3, 4];
        let target = 6;
        assert_eq!(Solution::two_sum_2(nums, target), vec![1, 3]);
    }

    #[test]
    fn example2_3() {
        let nums = vec![-1, 0];
        let target = -1;
        assert_eq!(Solution::two_sum_2(nums, target), vec![1, 2]);
    }
}
