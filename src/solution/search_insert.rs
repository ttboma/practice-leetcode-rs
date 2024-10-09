use crate::*;

impl Solution {
    /// # [35. Search Insert Position](https://leetcode.com/problems/search-insert-position/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
    ///
    /// You must write an algorithm with`O(log n)` runtime complexity.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,3,5,6], target = 5
    /// Output: 2
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1,3,5,6], target = 2
    /// Output: 1
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [1,3,5,6], target = 7
    /// Output: 4
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `-10^4 <= nums[i] <= 10^4`
    /// - `nums` contains **distinct**  values sorted in **ascending**  order.
    /// - `-10^4 <= target <= 10^4`
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert_recursive(&nums, target, 0, nums.len())
    }

    fn search_insert_recursive(nums: &[i32], target: i32, start: usize, end: usize) -> i32 {
        if end == start {
            return end as i32;
        }
        let mid = start + (end - start) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => mid as i32,
            std::cmp::Ordering::Less => Self::search_insert_recursive(nums, target, mid + 1, end),
            std::cmp::Ordering::Greater => Self::search_insert_recursive(nums, target, start, mid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let output = 2;
        assert_eq!(Solution::search_insert(nums, target), output);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let output = 1;
        assert_eq!(Solution::search_insert(nums, target), output);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let output = 4;
        assert_eq!(Solution::search_insert(nums, target), output);
    }
}
