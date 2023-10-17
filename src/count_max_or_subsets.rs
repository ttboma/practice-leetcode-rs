use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// # [2044. Count Number of Maximum Bitwise-OR Subsets](https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/)
    ///
    /// Given an integer array `nums`, find the **maximum**  possible **bitwise OR**  of a subset of `nums` and return the **number of different non-empty subsets**  with the maximum bitwise OR.
    ///
    /// An array `a` is a **subset**  of an array `b` if `a` can be obtained from `b` by deleting some (possibly zero) elements of `b`. Two subsets are considered **different**  if the indices of the elements chosen are different.
    ///
    /// The bitwise OR of an array `a` is equal to `a[0] **OR**  a[1] **OR**  ... **OR**  a[a.length - 1]` (**0-indexed** ).
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [3,1]
    /// Output: 2
    /// Explanation: The maximum possible bitwise OR of a subset is 3. There are 2 subsets with a bitwise OR of 3:
    /// - [3]
    /// - [3,1]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [2,2,2]
    /// Output: 7
    /// Explanation: All non-empty subsets of [2,2,2] have a bitwise OR of 2. There are 2^3 - 1 = 7 total subsets.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [3,2,1,5]
    /// Output: 6
    /// Explanation: The maximum possible bitwise OR of a subset is 7. There are 6 subsets with a bitwise OR of 7:
    /// - [3,5]
    /// - [3,1,5]
    /// - [3,2,5]
    /// - [3,2,1,5]
    /// - [2,5]
    /// - [2,1,5]```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 16`
    /// - `1 <= nums[i] <= 10^5`
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        // map bitwise OR value to the number of subsets with that bitwise OR value
        let mut mp = HashMap::<i32, i32>::new();
        let mut max = 0;
        mp.insert(0, 1);
        for n in nums {
            max |= n;
            let update = mp.iter().map(|(v, cnt)| (v | n, *cnt)).collect::<Vec<_>>();
            update.iter().for_each(|(v, cnt)| match mp.get_mut(v) {
                Some(c) => {
                    *c += *cnt;
                }
                None => {
                    mp.insert(*v, *cnt);
                }
            });
        }
        *mp.get(&max).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3, 1];
        assert_eq!(Solution::count_max_or_subsets(nums), 2);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 2, 2];
        assert_eq!(Solution::count_max_or_subsets(nums), 7);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 2, 1, 5];
        assert_eq!(Solution::count_max_or_subsets(nums), 6);
    }
}
