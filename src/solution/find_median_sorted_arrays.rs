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
        if nums1.len() > nums2.len() {
            Solution::find_median_sorted_arrays(nums2, nums1)
        } else if nums1.is_empty() {
            if (nums1.len() + nums2.len()) % 2 == 1 {
                nums2[nums2.len() / 2] as f64
            } else {
                let median = (nums1.len() + nums2.len() - 1) / 2;
                (nums2[median] + nums2[median + 1]) as f64 / 2.0
            }
        } else {
            find_median_sorted_arrays_impl(&nums1, &nums2)
        }
    }
}

fn find_median_sorted_arrays_impl(nums1: &[i32], nums2: &[i32]) -> f64 {
    let n = nums1.len();
    let m = nums2.len();
    let median = (n + m) / 2;
    let mut range = 0..n;

    loop {
        let i = range.start + (range.end - range.start) / 2;
        let j = median - i;
        let a = i.checked_sub(1).map_or(i32::MIN, |idx| nums1[idx]);
        let b = nums1.get(i).copied().unwrap_or(i32::MAX);
        let c = j.checked_sub(1).map_or(i32::MIN, |idx| nums2[idx]);
        let d = nums2.get(j).copied().unwrap_or(i32::MAX);

        if range.is_empty() || (a <= d && c <= b) {
            break if (n + m) % 2 == 1 {
                b.min(d) as f64
            } else {
                (b.min(d) + a.max(c)) as f64 / 2.0
            };
        } else if a > d {
            range.end = i;
        } else if c > b {
            range.start = i + 1;
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![2];
        let nums2 = vec![];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example3() {
        let nums1 = vec![1];
        let nums2 = vec![2, 3];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example4() {
        let nums1 = vec![2, 3];
        let nums2 = vec![1];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example5() {
        let nums1 = vec![2, 2, 4, 4];
        let nums2 = vec![2, 2, 2, 4, 4];
        let result = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example6() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let result = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example7() {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![-1, 0, 0, 0, 0, 0, 1];
        let result = 0.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example8() {
        let nums1 = vec![0];
        let nums2 = vec![0];
        let result = 0.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example9() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2, 7];
        let result = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example10() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-1, 3];
        let result = 1.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example11() {
        let nums1 = vec![1];
        let nums2 = vec![2, 3, 4];
        let result = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example12() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4, 5, 6, 7, 8];
        let result = 4.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example13() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let result = 9.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example14() {
        let nums1 = vec![1, 2];
        let nums2 = vec![1, 1];
        let result = 1.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }

    #[test]
    fn example15() {
        let nums1 = vec![2];
        let nums2 = vec![1, 3, 4];
        let result = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), result);
    }
}
