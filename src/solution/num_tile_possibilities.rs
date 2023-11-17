use crate::Solution;
use std::collections::HashMap;

impl Solution {
    /// # [1079. Letter Tile Possibilities](https://leetcode.com/problems/letter-tile-possibilities/description/)
    ///
    /// You have `n``tiles`, where each tile has one letter `tiles[i]` printed on it.
    ///
    /// Return the number of possible non-empty sequences of letters you can make using the letters printed on those `tiles`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: tiles = "AAB"
    /// Output: 8
    /// Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: tiles = "AAABBC"
    /// Output: 188
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: tiles = "V"
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= tiles.length <= 7`
    /// - `tiles` consists of uppercase English letters.
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        NumTilePossibilities::new(tiles).solve()
    }
}

struct NumTilePossibilities {
    table: HashMap<char, i32>,
}

impl NumTilePossibilities {
    fn new(tiles: String) -> Self {
        let mut table = HashMap::<char, i32>::new();
        for ele in tiles.chars() {
            if let Some(cnt) = table.get_mut(&ele) {
                *cnt += 1;
            } else {
                table.insert(ele, 1);
            }
        }
        Self { table }
    }

    fn solve(&mut self) -> i32 {
        let table = &mut self.table as *mut HashMap<char, i32>;
        let mut cnt = 0;
        unsafe {
            for (_, v) in (*table).iter_mut() {
                if *v == 0 {
                    continue;
                }
                *v -= 1;
                cnt += self.solve() + 1;
                *v += 1;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
    }
}
