use crate::*;

impl Solution {
    /// # [4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median**  of the two sorted arrays.
    ///
    /// The overall run time complexity should be `O(log (m+n))`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums1 = [1,3], nums2 = [2]
    /// Output: 2.00000
    /// Explanation: merged array = [1,2,3] and median is 2.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums1 = [1,2], nums2 = [3,4]
    /// Output: 2.50000
    /// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `nums1.length == m`
    /// - `nums2.length == n`
    /// - `0 <= m <= 1000`
    /// - `0 <= n <= 1000`
    /// - `1 <= m + n <= 2000`
    /// - `-10^6 <= nums1[i], nums2[i] <= 10^6`
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len();
        let m = nums2.len();
        let median = (n + m) / 2;
        if (n + m) % 2 == 1 {
            Self::find_median_sorted_arrays_recursive(&nums1, 0, n, &nums2, 0, m, median, true)
        } else {
            Self::find_median_sorted_arrays_recursive(&nums1, 0, n, &nums2, 0, m, median, true)
        }
    }

    fn find_median_sorted_arrays_recursive(
        nums1: &[i32],
        s1: usize,
        e1: usize,
        nums2: &[i32],
        s2: usize,
        e2: usize,
        median: usize,
        is_odd: bool,
    ) -> f64 {
        let n = e1 - s1;
        let m = e2 - s2;
        if n == 0 {
            return if is_odd {
                nums2[s2 + median] as f64
            } else {
                (nums2[s2 + median] + nums2[s2 + median - 1]) as f64 / 2.0
            };
        } else if m == 0 {
            return if is_odd {
                nums1[s1 + median] as f64
            } else {
                (nums1[s1 + median] + nums1[s1 + median - 1]) as f64 / 2.0
            };
        }  
        let j = m / 2;
        let i = nums1[s1..e1]
            .binary_search(&nums2[j])
            .unwrap_or_else(|idx| idx);
        match (i + j).cmp(&median) {
            std::cmp::Ordering::Less => Self::find_median_sorted_arrays_recursive(
                nums1,
                i,
                e1,
                nums2,
                j,
                e2,
                median - i - j,
                is_odd
            ),
            std::cmp::Ordering::Equal => {
                if is_odd {
                    nums2[j] as f64
                } else if j != 0{
                    (nums2[j] + nums2[j - 1].min(nums1[i])) as f64 / 2.0
                } else {
                    (nums2[j] + nums1[i-1]) as f64 / 2.0
                }
            }
            std::cmp::Ordering::Greater => {
                Self::find_median_sorted_arrays_recursive(&nums1, s1, i, nums2, s2, j, median, is_odd)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }
}
