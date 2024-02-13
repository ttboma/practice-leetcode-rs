use super::*;
use std::cmp::Ordering;

impl Solution {
    /// # [135. Candy](https://leetcode.com/problems/candy/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// There are `n` children standing in a line. Each child is assigned a rating value given in the integer array `ratings`.
    ///
    /// You are giving candies to these children subjected to the following requirements:
    ///
    /// - Each child must have at least one candy.
    /// - Children with a higher rating get more candies than their neighbors.
    ///
    /// Return the minimum number of candies you need to have to distribute the candies to the children.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: ratings = [1,0,2]
    /// Output: 5
    /// Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: ratings = [1,2,2]
    /// Output: 4
    /// Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
    /// The third child gets 1 candy because it satisfies the above two conditions.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == ratings.length`
    /// - `1 <= n <= 2 * 10^4`
    /// - `0 <= ratings[i] <= 2 * 10^4`
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        let mut i = 1;
        let mut ret = 1;
        let mut index = 1;
        while i < ratings.len() {
            match ratings[i - 1].cmp(&ratings[i]) {
                Ordering::Less => {
                    index += 1;
                    ret += index;
                    i += 1;
                }
                Ordering::Greater => {
                    let j = i;
                    while i < ratings.len() && ratings[i - 1] > ratings[i] {
                        i += 1;
                    }
                    let n = i as i32 - j as i32;
                    ret += n * (n + 1) / 2 + std::cmp::max(n + 1 - index, 0);
                    index = 1;
                }
                Ordering::Equal => {
                    index = 1;
                    ret += index;
                    i += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::candy(vec![1, 3, 2, 2, 1]), 7);
    }
}
