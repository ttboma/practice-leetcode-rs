use super::*;

impl Solution {
    /// # [190. Reverse Bits](https://leetcode.com/problems/reverse-bits/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Reverse bits of a given 32 bits unsigned integer.
    ///
    /// **Note:**
    ///
    /// - Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
    /// - In Java, the compiler represents the signed integers using <a href="https://en.wikipedia.org/wiki/Two%27s_complement" target="_blank">2's complement notation</a>. Therefore, in **Example 2**  above, the input represents the signed integer `-3` and the output represents the signed integer `-1073741825`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 00000010100101000001111010011100
    /// Output:    964176192 (00111001011110000010100101000000)
    /// Explanation: The input binary string **00000010100101000001111010011100**  represents the unsigned integer 43261596, so return 964176192 which its binary representation is **00111001011110000010100101000000** .
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 11111111111111111111111111111101
    /// Output:   3221225471 (10111111111111111111111111111111)
    /// Explanation: The input binary string **11111111111111111111111111111101**  represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is **10111111111111111111111111111111** .
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The input must be a **binary string**  of length `32`
    ///
    /// **Follow up:**  If this function is called many times, how would you optimize it?
    pub fn reverse_bits(mut x: u32) -> u32 {
        x = x.rotate_left(16);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
        x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let x = 0b00000010100101000001111010011100;
        let expected = 0b00111001011110000010100101000000;
        let result = Solution::reverse_bits(x);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let x = 0b11111111111111111111111111111101;
        let expected = 0b10111111111111111111111111111111;
        let result = Solution::reverse_bits(x);
        assert_eq!(result, expected);
    }
}
