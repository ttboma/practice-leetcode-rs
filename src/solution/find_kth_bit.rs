use crate::Solution;

impl Solution {
    /// # [1545. Find Kth Bit in Nth Binary String](https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/)
    ///
    /// Given two positive integers `n` and `k`, the binary string `S<sub>n</sub>` is formed as follows:
    ///
    /// - `S<sub>1</sub> = "0"`
    /// - `S<sub>i</sub> = S<sub>i - 1</sub> + "1" + reverse(invert(S<sub>i - 1</sub>))` for `i > 1`
    ///
    /// Where `+` denotes the concatenation operation, `reverse(x)` returns the reversed string `x`, and `invert(x)` inverts all the bits in `x` (`0` changes to `1` and `1` changes to `0`).
    ///
    /// For example, the first four strings in the above sequence are:
    ///
    /// - `S<sub>1 </sub>= "0"`
    /// - `S<sub>2 </sub>= "0**1** 1"`
    /// - `S<sub>3 </sub>= "011**1** 001"`
    /// - `S<sub>4</sub> = "0111001**1** 0110001"`
    ///
    /// Return the `k^th` bit in `S<sub>n</sub>`. It is guaranteed that `k` is valid for the given `n`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 3, k = 1
    /// Output: "0"
    /// Explanation: S<sub>3</sub> is "**0** 111001".
    /// The 1^st bit is "0".
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 4, k = 11
    /// Output: "1"
    /// Explanation: S<sub>4</sub> is "0111001101**1** 0001".
    /// The 11^th bit is "1".
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 20`
    /// - `1 <= k <= 2^n - 1`
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if k == 1 {
            '0'
        } else if k < i32::pow(2, n as u32 - 1) {
            Self::find_kth_bit(n - 1, k)
        } else if k == i32::pow(2, n as u32 - 1) {
            '1'
        } else {
            match Self::find_kth_bit(n, i32::pow(2, n as u32) - k) {
                '0' => '1',
                '1' => '0',
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
