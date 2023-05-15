use crate::Solution;

impl Solution {
    /// # 605. Can Place Flowers
    /// 
    /// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers
    /// cannot be planted in adjacent plots.
    ///
    /// Given an integer array `flowerbed` containing `0`'s and `1`'s, where `0` means empty and `1` means
    /// not empty, and an integer `n`, return `true` if `n` new flowers can be planted in the `flowerbed`
    /// without violating the no-adjacent-flowers rule and `false` otherwise.
    ///
    /// **Example 1:**
    /// 
    /// - **Input:** flowerbed = [1,0,0,0,1], n = 1
    /// - **Output:** true
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** flowerbed = [1,0,0,0,1], n = 2
    /// - **Output:** false
    /// 
    /// **Constraints:**
    /// 
    /// - `1 <= flowerbed.length <= 2 * 10^4`
    /// - `flowerbed[i]` is `0` or `1`.
    /// - There are no two adjacent flowers in `flowerbed`.
    /// - `0 <= n <= flowerbed.length`
    ///
    /// ------
    ///
    /// ***Extracted from:*** [can-place-flowers](https://leetcode.com/problems/can-place-flowers/)
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let n = n as usize;
        let mut it = flowerbed.iter().enumerate().filter(|(_i, &v)| v == 1);
        let mut prev_i = 0;
        let mut max_plots = 0;

        if let Some((i, _v)) = it.next() {
            max_plots = i / 2;
            prev_i = i;
        } else {
            return n <= (flowerbed.len() + 1) / 2;
        }
        while let Some((i, _v)) = it.next() {
            max_plots += (i - prev_i - 2) / 2;
            prev_i = i;
        }
        max_plots += (flowerbed.len() - prev_i - 1) / 2;
        return n <= max_plots;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let output = true;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), output);
    }

    #[test]
    fn example2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let output = false;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), output);
    }

    #[test]
    fn example3() {
        let flowerbed = vec![0, 0, 1, 0, 1];
        let n = 1;
        let output = true;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), output);
    }
}
