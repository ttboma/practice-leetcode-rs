use crate::*;

impl Solution {
    /// # [162. Find Peak Element](https://leetcode.com/problems/find-peak-element/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// A peak element is an element that is strictly greater than its neighbors.
    ///
    /// Given a **0-indexed**  integer array `nums`, find a peak element, and return its index. If the array contains multiple peaks, return the index to **any of the peaks** .
    ///
    /// You may imagine that `nums[-1] = nums[n] = -∞`. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.
    ///
    /// You must write an algorithm that runs in `O(log n)` time.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,1]
    /// Output: 2
    /// Explanation: 3 is a peak element and your function should return the index number 2.```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1,2,1,3,5,6,4]
    /// Output: 5
    /// Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 1000`
    /// - `-2^31 <= nums[i] <= 2^31 - 1`
    /// - `nums[i] != nums[i + 1]` for all valid `i`.
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::binary_search_find_peak_element(&nums, 0, nums.len())
    }

    fn binary_search_find_peak_element(nums: &[i32], start: usize, end: usize) -> i32 {
        if end - start <= 1 {
            return start as i32;
        }
        let mid = start + (end - start) / 2;
        if nums[mid - 1] < nums[mid] {
            Self::binary_search_find_peak_element(nums, mid, end)
        } else {
            Self::binary_search_find_peak_element(nums, start, mid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 1];
        let expected = 2;
        assert_eq!(Solution::find_peak_element(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let expected = 5;
        assert_eq!(Solution::find_peak_element(nums), expected);
    }
}
