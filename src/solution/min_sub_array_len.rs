use super::Solution;

impl Solution {
    /// # [209. Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of positive integers `nums` and a positive integer `target`, return the **minimal length**  of a <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rj:">subarray<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(54px, 204px);"> whose sum is greater than or equal to `target`. If there is no such subarray, return `0` instead.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: target = 7, nums = [2,3,1,2,4,3]
    /// Output: 2
    /// Explanation: The subarray [4,3] has the minimal length under the problem constraint.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: target = 4, nums = [1,4,4]
    /// Output: 1
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: target = 11, nums = [1,1,1,1,1,1,1,1]
    /// Output: 0
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= target <= 10^9`
    /// - `1 <= nums.length <= 10^5`
    /// - `1 <= nums[i] <= 10^4`
    ///
    /// **Follow up:**  If you have figured out the `O(n)` solution, try coding another solution of which the time complexity is `O(n log(n))`.
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        match nums
            .iter()
            .enumerate()
            .scan(0, |state, (k, x)| {
                *state += *x;
                Some((*state, k))
            })
            .find_map(|(acc, k)| if acc >= target { Some((acc, k)) } else { None })
        {
            Some((_, 1)) => 1,
            Some((mut acc, mut k)) => {
                let mut i = k;
                while i < nums.len() {
                    while acc >= target {
                        if k == 0 {
                            return 1;
                        }
                        acc -= nums[i - k];
                        k -= 1;
                    }
                    if i == nums.len() - 1 {
                        break;
                    }
                    acc += nums[i + 1] - nums[i - k];
                    i += 1;
                }
                k as i32 + 2
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::min_sub_array_len(213, vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12]),
            8
        );
    }
}
