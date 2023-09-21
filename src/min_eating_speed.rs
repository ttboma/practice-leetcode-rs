use crate::Solution;

impl Solution {
    /// # 875. Koko Eating Bananas
    ///
    /// Koko loves to eat bananas. There are `n` piles of bananas, the `ith`
    /// pile has `piles[i]` bananas. The guards have gone and will come back
    /// in h hours. Koko can decide her bananas-per-hour eating speed of `k`.
    /// Each hour, she chooses some pile of bananas and eats `k` bananas from
    /// that pile. If the pile has less than `k` bananas, she eats all of
    /// them instead and will not eat any more bananas during this hour. Koko
    /// likes to eat slowly but still wants to finish eating all the bananas
    /// before the guards return. Return the minimum integer `k` such that
    /// she can eat all the bananas within `h` hours.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** piles = [3,6,7,11], h = 8
    /// - **Output:** 4
    ///
    /// **Example 2:**
    ///
    /// - **Input:** piles = [30,11,23,4,20], h = 5
    /// - **Output:** 30
    ///
    /// **Example 3:**
    ///
    /// - **Input:** piles = [30,11,23,4,20], h = 6
    /// - **Output:** 23
    ///
    /// **Constraints:**
    ///
    /// - 1 <= piles.length <= 104
    /// - piles.length <= h <= 109
    /// - 1 <= piles[i] <= 109
    ///
    /// ------
    ///
    /// ***Extracted from:*** [koko-eating-bananas](https://leetcode.com/problems/koko-eating-bananas/)
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut i = 1;
        let mut j = *piles.iter().max().unwrap();
        if h == piles.len() as i32 {
            return j;
        }
        while i != j {
            let m = i + (j - i) / 2;
            let total_hour = piles
                .iter()
                .map(|banana| {
                    if banana % m != 0 {
                        banana / m + 1
                    } else {
                        banana / m
                    }
                })
                .sum();
            if h < total_hour {
                i = m + 1;
            } else {
                j = m;
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(Solution::min_eating_speed(piles, h), 4);
    }

    #[test]
    fn example2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(Solution::min_eating_speed(piles, h), 30);
    }

    #[test]
    fn example3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(Solution::min_eating_speed(piles, h), 23);
    }
}
