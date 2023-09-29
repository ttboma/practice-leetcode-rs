use crate::Solution;

impl Solution {
    /// # [1043. Partition Array for Maximum Sum](https://leetcode.com/problems/partition-array-for-maximum-sum/)
    ///
    /// Given an integer array `arr`, partition the array into (contiguous) subarrays of length **at most**  `k`. After partitioning, each subarray has their values changed to become the maximum value of that subarray.
    ///
    /// Return the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a **32-bit**  integer.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: arr = [1,15,7,9,2,5,10], k = 3
    /// Output: 84
    /// Explanation: arr becomes [15,15,15,9,10,10,10]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
    /// Output: 83
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: arr = [1], k = 1
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= arr.length <= 500`
    /// - `0 <= arr[i] <= 10^9`
    /// - `1 <= k <= arr.length`
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut table = compute_max_sum_table_for_the_first_k_value(&arr, k);
        if k < arr.len() {
            for _ in 0..arr.len() - k {
                compute_max_sum_table_for_the_next_value(&mut table, &arr, k);
            }
        }
        table.pop().unwrap()
    }
}

fn compute_max_sum_table_for_the_first_k_value(arr: &[i32], k: usize) -> Vec<i32> {
    (0..k)
        .map(|i| arr[0..=i].iter().max().unwrap() * (i as i32 + 1))
        .collect::<Vec<i32>>()
}

fn compute_max_sum_table_for_the_next_value(table: &mut Vec<i32>, arr: &[i32], k: usize) {
    let i = table.len();
    let v = (0..k)
        .map(|j| arr[i - j..=i].iter().max().unwrap() * (j as i32 + 1) + table[i - j - 1])
        .max()
        .unwrap();
    table.push(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_sum_after_partitioning(vec![1], 1), 1);
    }
}
