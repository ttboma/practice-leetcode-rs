use super::*;

impl Solution {
    /// # [191. Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a positive integer `n`, write a function that returns the number of
    /// set bits (A set bit refers to a bit in the binary representation of a number that has a value of 1.) in its binary representation (also known as the [Hamming weight](https://en.wikipedia.org/wiki/Hamming_weight)).
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 11
    ///
    /// Output: 3
    ///
    /// Explanation:
    ///
    /// The input binary string **1011**  has a total of three set bits.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 128
    ///
    /// Output: 1
    ///
    /// Explanation:
    ///
    /// The input binary string **10000000**  has a total of one set bit.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 2147483645
    ///
    /// Output: 30
    ///
    /// Explanation:
    ///
    /// The input binary string **1111111111111111111111111111101**  has a total of thirty set bits.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 2^31 - 1`
    ///
    /// **Follow up:**  If this function is called many times, how would you optimize it?
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut count = 0;
        while n != 0 {
            count += n & 1;
            n >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 11;
        let expected = 3;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let n = 128;
        let expected = 1;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, expected);
    }

    #[test]
    fn example3() {
        let n = 2147483645;
        let expected = 30;
        let result = Solution::hamming_weight(n);
        assert_eq!(result, expected);
    }
}
