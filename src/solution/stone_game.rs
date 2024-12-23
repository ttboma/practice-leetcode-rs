use crate::Solution;

impl Solution {
    /// # [877. Stone Game](https://leetcode.com/problems/stone-game/)
    ///
    /// Alice and Bob play a game with piles of stones. There are an **even**  number of piles arranged in a row, and each pile has a **positive**  integer number of stones `piles[i]`.
    ///
    /// The objective of the game is to end with the most stones. The **total**  number of stones across all the piles is **odd** , so there are no ties.
    ///
    /// Alice and Bob take turns, with **Alice starting first** . Each turn, a player takes the entire pile of stones either from the **beginning**  or from the **end**  of the row. This continues until there are no more piles left, at which point the person with the **most stones wins** .
    ///
    /// Assuming Alice and Bob play optimally, return `true` if Alice wins the game, or `false` if Bob wins.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: piles = [5,3,4,5]
    /// Output: true
    /// Explanation:
    /// Alice starts first, and can only take the first 5 or the last 5.
    /// Say she takes the first 5, so that the row becomes [3, 4, 5].
    /// If Bob takes 3, then the board is [4, 5], and Alice takes 5 to win with 10 points.
    /// If Bob takes the last 5, then the board is [3, 4], and Alice takes 4 to win with 9 points.
    /// This demonstrated that taking the first 5 was a winning move for Alice, so we return true.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: piles = [3,7,2,3]
    /// Output: true
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= piles.length <= 500`
    /// - `piles.length` is **even** .
    /// - `1 <= piles[i] <= 500`
    /// - `sum(piles[i])` is **odd** .
    pub fn stone_game(_: Vec<i32>) -> bool {
        // let mut table = piles.clone();
        // let flat_index = |i, j| {
        //     let d = j - i;
        //     i + d * piles.len() - (d * d - d) / 2
        // };
        // for (i, j) in (1..piles.len()).flat_map(|j| (0..piles.len() - j).map(move |i| (i, i + j))) {
        //     let k = flat_index(i + 1, j);
        //     table.push(std::cmp::max(piles[i] - table[k], piles[j] + table[k - 1]))
        // }
        // *table.last().unwrap() > 0
        true // dp not even required, just return true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
    }

    #[test]
    fn example2() {
        assert!(Solution::stone_game(vec![3, 7, 2, 3]));
    }
}
