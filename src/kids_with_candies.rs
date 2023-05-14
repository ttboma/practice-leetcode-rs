use crate::Solution;

impl Solution {
    /// # 1768. Merge Strings Alternately
    /// 
    /// There are `n` kids with candies. You are given an integer array `candies`, where each `candies[i]` 
    /// represents the number of candies the `i^th` kid has, and an integer `extraCandies`, denoting the number 
    /// of extra candies that you have.
    ///
    /// Return a boolean array `result` of length `n`, where `result[i]` is `true` if, after giving the `i^th` kid all the 
    /// `extraCandies`, they will have the greatest number of candies among all the kids, or `false` otherwise.
    /// 
    /// Note that **multiple** kids can have the **greatest** number of candies.
    ///
    /// **Example 1:**
    /// 
    /// - **Input:** candies = [2,3,5,1,3], extraCandies = 3
    /// - **Output:** [true,true,true,false,true] 
    /// - **Explanation:** If you give all extraCandies to:
    ///     - Kid 1, they will have 2 + 3 = 5 candies, which is the greatest among the kids.
    ///     - Kid 2, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
    ///     - Kid 3, they will have 5 + 3 = 8 candies, which is the greatest among the kids.
    ///     - Kid 4, they will have 1 + 3 = 4 candies, which is not the greatest among the kids.
    ///     - Kid 5, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** candies = [4,2,1,1,2], extraCandies = 1
    /// - **Output:** [true,false,false,false,false] 
    /// - **Explanation:** There is only 1 extra candy.
    ///   Kid 1 will always have the greatest number of candies, even if a different kid is given
    ///   the extra candy.
    /// 
    /// **Example 3:**
    /// 
    /// - **Input:** candies = [12,1,12], extraCandies = 10
    /// - **Output:** [true,false,true]
    ///     
    /// **Constraints:**
    /// 
    /// - `n == candies.length`
    /// - `2 <= n <= 100`
    /// - `1 <= candies[i] <= 100`
    /// - `1 <= extraCandies <= 50`
    ///
    /// ------
    ///
    /// ***Extracted from:*** [kids-with-the-greatest-number-of-candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        if candies.len() == 0 {
            Vec::<bool>::new()
        } else {
            let max = candies.iter().max();
            candies.iter().map(|c| {
                if *c + extra_candies < *max.unwrap() {
                    false
                } else {
                    true
                }
            }).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let output = vec![true, true, true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, extra_candies), output);
    }

    #[test]
    fn example2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let output = vec![true, false, false, false, false];
        assert_eq!(Solution::kids_with_candies(candies, extra_candies), output);
    }

    #[test]
    fn example3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let output = vec![true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, extra_candies), output);
    }
}
