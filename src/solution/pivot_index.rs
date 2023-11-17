use crate::Solution;

impl Solution {
    /// # 724. Find Pivot Index
    ///
    /// Given an array of integers nums, calculate the pivot index of this array.
    /// The pivot index is the index where the sum of all the numbers strictly to
    /// the left of the index is equal to the sum of all the numbers strictly to
    /// the index's right. If the index is on the left edge of the array, then the
    /// left sum is 0 because there are no elements to the left. This also applies
    /// to the right edge of the array. Return the leftmost pivot index. If no such
    /// index exists, return -1.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** nums = [1,7,3,6,5,6]
    /// - **Output:** 3
    /// - **Explanation:**
    ///   <br> The pivot index is 3.
    ///   <br> Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
    ///   <br> Right sum = nums[4] + nums[5] = 5 + 6 = 11
    ///
    /// **Example** 2:
    ///
    /// - **Input:** nums = [1,2,3]
    /// - **Output:** -1
    /// - **Explanation:**
    ///   <br> There is no index that satisfies the conditions in the problem statement.
    ///
    /// **Example** 3:
    ///
    /// - **Input:** nums = [2,1,-1]
    /// - **Output:** 0
    /// - **Explanation:**
    ///   <br> The pivot index is 0.
    ///   <br> Left sum = 0 (no elements to the left of index 0)
    ///   <br> Right sum = nums[1] + nums[2] = 1 + -1 = 0
    ///
    /// **Constraints:**
    ///
    /// - 1 <= nums.length <= 104
    /// - -1000 <= nums[i] <= 1000
    ///
    /// ------
    ///
    /// ***Extracted from:*** [find-pivot-index](https://leetcode.com/problems/find-pivot-index/)
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut accumuate = 0;
        let mut it = nums.iter().enumerate();
        loop {
            if let Some((index, &value)) = it.next() {
                if sum - accumuate == value {
                    return index.try_into().unwrap();
                }
                accumuate += 2 * value;
            } else {
                return -1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(Solution::pivot_index(nums), 3)
    }
    #[test]
    fn example2() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::pivot_index(nums), -1)
    }
    #[test]
    fn example3() {
        let nums = vec![2, 1, -1];
        assert_eq!(Solution::pivot_index(nums), 0)
    }
}
