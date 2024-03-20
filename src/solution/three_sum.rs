use super::*;
use std::cmp::Ordering;

impl Solution {
    /// # [15. 3Sum](https://leetcode.com/problems/3sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
    ///
    /// Notice that the solution set must not contain duplicate triplets.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [-1,0,1,2,-1,-4]
    /// Output: [[-1,-1,2],[-1,0,1]]
    /// Explanation:
    /// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    /// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    /// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are [-1,0,1] and [-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,1,1]
    /// Output: []
    /// Explanation: The only possible triplet does not sum up to 0.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [0,0,0]
    /// Output: [[0,0,0]]
    /// Explanation: The only possible triplet sums up to 0.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `3 <= nums.length <= 3000`
    /// - `-10^5 <= nums[i] <= 10^5`
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut k = i + 1;
            let mut j = nums.len() - 1;
            while k < j {
                let sum = nums[i] + nums[k] + nums[j];
                match sum.cmp(&0) {
                    Ordering::Less => {
                        while k < j && nums[k] == nums[k + 1] {
                            k += 1;
                        }
                        k += 1;
                    }
                    Ordering::Greater => {
                        while k < j && nums[j] == nums[j - 1] {
                            j -= 1;
                        }
                        j -= 1;
                    }
                    Ordering::Equal => {
                        result.push([nums[i], nums[k], nums[j]].to_vec());
                        while k < j && nums[k] == nums[k + 1] {
                            k += 1;
                        }
                        while k < j && nums[j] == nums[j - 1] {
                            j -= 1;
                        }
                        k += 1;
                        j -= 1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec::<i32>>::new());
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
