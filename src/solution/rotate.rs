use crate::Solution;

impl Solution {
    /// # [189. Rotate Array](https://leetcode.com/problems/rotate-array/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums`, rotate the array to the right by `k` steps, where `k` is non-negative.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,4,5,6,7], k = 3
    /// Output: [5,6,7,1,2,3,4]
    /// Explanation:
    /// rotate 1 steps to the right: [7,1,2,3,4,5,6]
    /// rotate 2 steps to the right: [6,7,1,2,3,4,5]
    /// rotate 3 steps to the right: [5,6,7,1,2,3,4]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [-1,-100,3,99], k = 2
    /// Output: [3,99,-1,-100]
    /// Explanation:
    /// rotate 1 steps to the right: [99,-1,-100,3]
    /// rotate 2 steps to the right: [3,99,-1,-100]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^5`
    /// - `-2^31 <= nums[i] <= 2^31 - 1`
    /// - `0 <= k <= 10^5`
    ///
    /// **Follow up:**
    ///
    /// - Try to come up with as many solutions as you can. There are at least **three**  different ways to solve this problem.
    /// - Could you do it in-place with `O(1)` extra space?
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        if k == 0 {
            return;
        }
        let rep = gcd(nums.len(), k);
        for i in 0..rep {
            let tmp = nums[i];
            let mut a = i;
            let mut b = (a + nums.len() - k) % nums.len();
            while b != i {
                nums[a] = nums[b];
                a = b;
                b = (b + nums.len() - k) % nums.len();
            }
            nums[a] = tmp;
        }
    }
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, [3, 99, -1, -100]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1];
        let k = 0;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, [1]);
    }

    #[test]
    fn example4() {
        let mut nums = vec![1, 2, 3];
        let k = 4;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, [3, 1, 2]);
    }

    #[test]
    fn example5() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        let k = 4;
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, [3, 4, 5, 6, 1, 2]);
    }
}
