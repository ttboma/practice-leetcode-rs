use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// # [1387. Sort Integers by The Power Value](https://leetcode.com/problems/sort-integers-by-the-power-value/)
    ///
    /// The power of an integer `x` is defined as the number of steps needed to transform `x` into `1` using the following steps:
    ///
    /// - if `x` is even then `x = x / 2`
    /// - if `x` is odd then `x = 3 * x + 1`
    ///
    /// For example, the power of `x = 3` is `7` because `3` needs `7` steps to become `1` (`3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1`).
    ///
    /// Given three integers `lo`, `hi` and `k`. The task is to sort all integers in the interval `[lo, hi]` by the power value in **ascending order** , if two or more integers have **the same**  power value sort them by **ascending order** .
    ///
    /// Return the `k^th` integer in the range `[lo, hi]` sorted by the power value.
    ///
    /// Notice that for any integer `x` `(lo <= x <= hi)` it is **guaranteed**  that `x` will transform into `1` using these steps and that the power of `x` is will **fit**  in a 32-bit signed integer.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: lo = 12, hi = 15, k = 2
    /// Output: 13
    /// Explanation: The power of 12 is 9 (12 --> 6 --> 3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1)
    /// The power of 13 is 9
    /// The power of 14 is 17
    /// The power of 15 is 17
    /// The interval sorted by the power value [12,13,14,15]. For k = 2 answer is the second element which is 13.
    /// Notice that 12 and 13 have the same power value and we sorted them in ascending order. Same for 14 and 15.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: lo = 7, hi = 11, k = 4
    /// Output: 7
    /// Explanation: The power array corresponding to the interval [7, 8, 9, 10, 11] is [16, 3, 19, 6, 14].
    /// The interval sorted by power is [8, 10, 11, 7, 9].
    /// The fourth number in the sorted array is 7.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= lo <= hi <= 1000`
    /// - `1 <= k <= hi - lo + 1`
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut lookup = HashMap::<i32, i32>::from([(2, 1)]);
        let mut table: Vec<(i32, i32)> = (lo..=hi).map(|i| (power(&mut lookup, i), i)).collect();
        table.select_nth_unstable_by(k as usize - 1, |a, b| a.cmp(b));
        table[k as usize - 1].1
    }
}

fn power(lookup: &mut HashMap<i32, i32>, mut i: i32) -> i32 {
    let mut derivation = vec![];
    loop {
        derivation.push(i);
        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }
        if let Some(p) = lookup.get(&i) {
            let mut p = *p;
            for k in derivation.into_iter().rev() {
                p += 1;
                lookup.insert(k, p);
            }
            break p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
