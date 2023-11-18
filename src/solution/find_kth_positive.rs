use crate::Solution;

impl Solution {
    /// # [1539. Kth Missing Positive Number](https://leetcode.com/problems/kth-missing-positive-number/)
    ///
    /// Given an array `arr` of positive integers sorted in a **strictly increasing order** , and an integer `k`.
    ///
    /// Return the `k^th` **positive**  integer that is **missing**  from this array.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: arr = [2,3,4,7,11], k = 5
    /// Output: 9
    /// Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5^th missing positive integer is 9.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: arr = [1,2,3,4], k = 2
    /// Output: 6
    /// Explanation: The missing positive integers are [5,6,7,...]. The 2^nd missing positive integer is 6.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= arr.length <= 1000`
    /// - `1 <= arr[i] <= 1000`
    /// - `1 <= k <= 1000`
    /// - `arr[i] < arr[j]` for `1 <= i < j <= arr.length`
    ///
    /// **Follow up:**
    ///
    /// Could you solve this problem in less than O(n) complexity?
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut j = arr.len();
        while i < j {
            let m = i + (j - i) / 2;
            if arr[m] - 1 - (m as i32) < k {
                i = m + 1;
            } else {
                j = m;
            }
        }
        (i as i32) + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(Solution::find_kth_positive(arr, k), 9);
    }

    #[test]
    fn example2() {
        let arr = vec![1, 2, 3, 4];
        let k = 2;
        assert_eq!(Solution::find_kth_positive(arr, k), 6);
    }
}
