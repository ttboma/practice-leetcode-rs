use crate::*;
use std::cmp::Ordering::*;

impl Solution {
    /// # [153. Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Suppose an array of length `n` sorted in ascending order is **rotated**  between `1` and `n` times. For example, the array `nums = [0,1,2,4,5,6,7]` might become:
    ///
    /// - `[4,5,6,7,0,1,2]` if it was rotated `4` times.
    /// - `[0,1,2,4,5,6,7]` if it was rotated `7` times.
    ///
    /// Notice that **rotating**  an array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time results in the array `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.
    ///
    /// Given the sorted rotated array `nums` of **unique**  elements, return the minimum element of this array.
    ///
    /// You must write an algorithm that runs in`O(log n) time`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [3,4,5,1,2]
    /// Output: 1
    /// Explanation: The original array was [1,2,3,4,5] rotated 3 times.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [4,5,6,7,0,1,2]
    /// Output: 0
    /// Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [11,13,15,17]
    /// Output: 11
    /// Explanation: The original array was [11,13,15,17] and it was rotated 4 times.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == nums.length`
    /// - `1 <= n <= 5000`
    /// - `-5000 <= nums[i] <= 5000`
    /// - All the integers of `nums` are **unique**.
    /// - `nums` is sorted and rotated between `1` and `n` times.
    pub fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min_recursive(&nums, 0, nums.len())
    }

    fn find_min_recursive(nums: &[i32], start: usize, end: usize) -> i32 {
        if end - start == 1 {
            nums[start]
        } else {
            let mid = start + (end - start) / 2;
            match nums[start].cmp(&nums[end - 1]) {
                Less => {
                    // zero-rotated
                    nums[start]
                }
                Equal => {
                    unreachable!("All the integers of `nums` are unique")
                }
                Greater => {
                    match nums[start].cmp(&nums[mid]) {
                        Less => {
                            // rotated
                            Solution::find_min_recursive(nums, mid + 1, end)
                        }
                        Equal => {
                            unreachable!("All the integers of `nums` are unique")
                        }
                        Greater => {
                            if nums[mid - 1] > nums[mid] {
                                nums[mid]
                            } else {
                                Solution::find_min_recursive(nums, start + 1, mid)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3, 4, 5, 1, 2];
        let ans = 1;
        assert_eq!(Solution::find_min(nums), ans);
    }

    #[test]
    fn example2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let ans = 0;
        assert_eq!(Solution::find_min(nums), ans);
    }

    #[test]
    fn example3() {
        let nums = vec![11, 13, 15, 17];
        let ans = 11;
        assert_eq!(Solution::find_min(nums), ans);
    }
}
