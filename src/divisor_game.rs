use crate::Solution;

impl Solution {
    /// # [1025. Divisor Game](https://leetcode.com/problems/divisor-game/description/)
    /// 
    /// Alice and Bob take turns playing a game, with Alice starting first.
    /// 
    /// Initially, there is a number `n` on the chalkboard. On each player's turn, that player makes a move consisting of:
    /// 
    /// - Choosing any `x` with `0 < x < n` and `n % x == 0`.
    /// - Replacing the number `n` on the chalkboard with `n - x`.
    /// 
    /// Also, if a player cannot make a move, they lose the game.
    /// 
    /// Return `true` if and only if Alice wins the game, assuming both players play optimally.
    /// 
    /// **Example 1:** 
    /// 
    /// ```
    /// Input: n = 2
    /// Output: true
    /// Explanation: Alice chooses 1, and Bob has no more moves.
    /// ```
    /// 
    /// **Example 2:** 
    /// 
    /// ```
    /// Input: n = 3
    /// Output: false
    /// Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
    /// ```
    /// 
    /// **Constraints:** 
    /// 
    /// - `1 <= n <= 1000`
    pub fn divisor_game(n: i32) -> bool {
        // proof by mathematical induction:
        // 
        // (1) proof if for all i in {1..n-1} that divisor_game(2*i) is true, then divisor_game(n) is true:
        // 
        // In divisor_game(2*i), Alice always chooses 1, Bob can only choose 1 odd number.
        // Then divisor_game(2*j) == true for all j in {1..i}
        // 
        // (2) divisor_game(2) == true 
        n % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::divisor_game(2), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::divisor_game(3), false);
    }
}