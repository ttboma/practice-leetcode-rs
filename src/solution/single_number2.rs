use super::*;

impl Solution {
    /// # [137. Single Number II](https://leetcode.com/problems/single-number-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums` where every element appears **three times**  except for one, which appears **exactly once** . Find the single element and return it.
    ///
    /// You must implement a solution with a linear runtime complexity and use only constant extra space.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,2,3,2]
    /// Output: 3
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,1,0,1,0,1,99]
    /// Output: 99
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 3 * 10^4`
    /// - `-2^31 <= nums[i] <= 2^31 - 1`
    /// - Each element in `nums` appears exactly **three times**  except for one element which appears **once** .
    pub fn single_number2(nums: Vec<i32>) -> i32 {
        (0..32).fold(0, |ans, i| {
            ans | ((nums.iter().fold(0, |sum, &num| sum + ((num >> i) & 1)) % 3) << i)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 2, 3, 2];
        let expected = 3;
        let result = Solution::single_number2(nums);
        assert_eq!(expected, result);
    }

    #[test]
    fn example2() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let expected = 99;
        let result = Solution::single_number2(nums);
        assert_eq!(expected, result);
    }
}
