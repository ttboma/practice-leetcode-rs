use crate::Solution;

impl Solution {
    /// # [486. Predict the Winner](https://leetcode.com/problems/predict-the-winner/)
    ///
    /// You are given an integer array `nums`. Two players are playing a game with this array: player 1 and player 2.
    ///
    /// Player 1 and player 2 take turns, with player 1 starting first. Both players start the game with a score of `0`. At each turn, the player takes one of the numbers from either end of the array (i.e., `nums[0]` or `nums[nums.length - 1]`) which reduces the size of the array by `1`. The player adds the chosen number to their score. The game ends when there are no more elements in the array.
    ///
    /// Return `true` if Player 1 can win the game. If the scores of both players are equal, then player 1 is still the winner, and you should also return `true`. You may assume that both players are playing optimally.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,5,2]
    /// Output: false
    /// Explanation: Initially, player 1 can choose between 1 and 2.
    /// If he chooses 2 (or 1), then player 2 can choose from 1 (or 2) and 5. If player 2 chooses 5, then player 1 will be left with 1 (or 2).
    /// So, final score of player 1 is 1 + 2 = 3, and player 2 is 5.
    /// Hence, player 1 will never be the winner and you need to return false.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1,5,233,7]
    /// Output: true
    /// Explanation: Player 1 first chooses 1. Then player 2 has to choose between 5 and 7. No matter which number player 2 choose, player 1 can choose 233.
    /// Finally, player 1 has more score (234) than player 2 (12), so you need to return True representing player1 can win.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 20`
    /// - `0 <= nums[i] <= 10^7`
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        // predict_the_winner_impl_recursive(&nums, 0, nums.len() - 1) >= 0
        predict_the_winner_impl_dp(nums)
    }
}

struct UpperTriangleMatrix {
    dimension: usize,
    data: Vec<i32>,
}

impl UpperTriangleMatrix {
    fn get(&self, i: usize, j: usize) -> &i32 {
        let k = i + (j - i) * (self.dimension + 1) - (j - i) * (j - i + 1) / 2;
        &self.data[k]
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
}

fn predict_the_winner_impl_dp(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut table = UpperTriangleMatrix {
        dimension: n,
        data: nums,
    };
    for offset in 1..n {
        for i in 0..n - offset {
            let j = i + offset;
            let c1 = table.get(i, i) - table.get(i + 1, j);
            let c2 = table.get(j, j) - table.get(i, j - 1);
            table.push(c1.max(c2));
        }
    }
    table.pop().unwrap() >= 0
}

#[allow(dead_code)]
fn predict_the_winner_impl_recursive(nums: &Vec<i32>, i: usize, j: usize) -> i32 {
    if j - i == 0 {
        return nums[0];
    } else if j - i == 1 {
        return i32::abs(nums[i] - nums[j]);
    }
    let c1 = nums[j] - predict_the_winner_impl_recursive(nums, i, j - 1);
    let c2 = nums[i] - predict_the_winner_impl_recursive(nums, i + 1, j);
    i32::max(c1, c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 5, 2];
        assert!(!Solution::predict_the_winner(nums));
    }

    #[test]
    fn example2() {
        let nums = vec![1, 5, 233, 7];
        assert!(Solution::predict_the_winner(nums));
    }

    #[test]
    fn example3() {
        let nums = vec![0];
        assert!(Solution::predict_the_winner(nums));
    }
}
