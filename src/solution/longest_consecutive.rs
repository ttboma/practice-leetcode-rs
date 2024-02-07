use super::*;
use std::collections::HashSet;

impl Solution {
    /// # [128. Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an unsorted array of integers `nums`, return the length of the longest consecutive elements sequence.
    ///
    /// You must write an algorithm that runs in `O(n)` time.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [100,4,200,1,3,2]
    /// Output: 4
    /// Explanation: The longest consecutive elements sequence is `[1, 2, 3, 4]`. Therefore its length is 4.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,3,7,2,5,8,4,6,0,1]
    /// Output: 9
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= nums.length <= 10^5`
    /// - `-10^9 <= nums[i] <= 10^9`
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let nums: HashSet<_> = nums.into_iter().collect();
        nums.iter()
            .filter_map(|&n| {
                if !nums.contains(&(n - 1)) {
                    Some((n..).take_while(|i| nums.contains(i)).count())
                } else {
                    None
                }
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
