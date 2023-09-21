use crate::Solution;
use std::cmp;

impl Solution {
    /// # 904. Fruit Into Baskets
    ///
    /// You are visiting a farm that has a single row of fruit trees arranged
    /// from left to right. The trees are represented by an integer array
    /// `fruits` where `fruits[i]` is the type of fruit the `ith` tree produces.
    /// You want to collect as much fruit as possible. However, the owner has
    /// some strict rules that you must follow:
    ///
    /// You only have two baskets, and each basket can only hold a single type
    /// of fruit. There is no limit on the amount of fruit each basket can hold.
    /// Starting from any tree of your choice, you must pick exactly one fruit
    /// from every tree (including the start tree) while moving to the right.
    /// The picked fruits must fit in one of your baskets. Once you reach a tree
    /// with fruit that cannot fit in your baskets, you must stop.
    /// Given the integer array `fruits`, return the maximum number of fruits
    /// you can pick.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** fruits = [1,2,1]
    /// - **Output:** 3
    /// - **Explanation:** We can pick from all 3 trees.
    ///
    /// **Example 2:**
    ///
    /// - **Input:** fruits = [0,1,2,2]
    /// - **Output:** 3
    /// - **Explanation:** We can pick from trees [1,2,2]. If we had started at
    ///   the first tree, we would only pick from trees [0,1].
    ///
    /// **Example 3:**
    ///
    /// - **Input:** fruits = [1,2,3,2,2]
    /// - **Output:** 4
    /// - **Explanation:** We can pick from trees [2,3,2,2]. If we had started
    ///   at the first tree, we would only pick from trees [1,2].
    ///
    /// **Constraints:**
    ///
    /// - 1 <= fruits.length <= 105
    /// - 0 <= fruits[i] < fruits.length
    ///
    /// ------
    ///
    /// ***Extracted from:*** [fruit-into-baskets](https://leetcode.com/problems/fruit-into-baskets/)
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        if fruits.len() < 3 {
            return fruits.len().try_into().unwrap();
        }
        let mut i = 1;
        while i != fruits.len() && fruits[i] == fruits[0] {
            i += 1;
        }
        if i == fruits.len() {
            return i.try_into().unwrap();
        }
        let mut bucket = (fruits[0], fruits[i]);
        i += 1;
        let mut max = i;
        let mut cur_max = i;
        let mut conseq = 1;
        for item in fruits.iter().skip(i) {
            if bucket.1 == *item {
                cur_max += 1;
                conseq += 1;
            } else if bucket.0 == *item {
                cur_max += 1;
                conseq = 1;
                bucket = (bucket.1, *item);
            } else {
                max = cmp::max(max, cur_max);
                bucket = (bucket.1, *item);
                cur_max = conseq + 1;
                conseq = 1;
            }
        }
        cmp::max(max, cur_max).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }
}
