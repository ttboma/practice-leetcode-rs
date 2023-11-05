use crate::Solution;

impl Solution {
    /// # [1969. Minimum Non-Zero Product of the Array Elements](https://leetcode.com/problems/minimum-non-zero-product-of-the-array-elements/)
    ///
    /// You are given a positive integer `p`. Consider an array `nums` (**1-indexed** ) that consists of the integers in the **inclusive**  range `[1, 2^p - 1]` in their binary representations. You are allowed to do the following operation **any**  number of times:
    ///
    /// - Choose two elements `x` and `y` from `nums`.
    /// - Choose a bit in `x` and swap it with its corresponding bit in `y`. Corresponding bit refers to the bit that is in the **same position**  in the other integer.
    ///
    /// For example, if `x = 1101` and `y = 0011`, after swapping the `2^nd` bit from the right, we have `x = 1111` and `y = 0001`.
    ///
    /// Find the **minimum non-zero**  product of `nums` after performing the above operation **any**  number of times. Return this product **modulo**  `10^9 + 7`.
    ///
    /// **Note:**  The answer should be the minimum product **before**  the modulo operation is done.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: p = 1
    /// Output: 1
    /// Explanation: nums = [1].
    /// There is only one element, so the product equals that element.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: p = 2
    /// Output: 6
    /// Explanation: nums = [01, 10, 11].
    /// Any swap would either make the product 0 or stay the same.
    /// Thus, the array product of 1 * 2 * 3 = 6 is already minimized.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: p = 3
    /// Output: 1512
    /// Explanation: nums = [001, 010, 011, 100, 101, 110, 111]
    /// - In the first operation we can swap the leftmost bit of the second and fifth elements.
    ///     - The resulting array is [001, 110, 011, 100, 001, 110, 111].
    /// - In the second operation we can swap the middle bit of the third and fourth elements.
    ///     - The resulting array is [001, 110, 001, 110, 001, 110, 111].
    /// The array product is 1 * 6 * 1 * 6 * 1 * 6 * 7 = 1512, which is the minimum possible product.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= p <= 60`
    pub fn min_non_zero_product(p: u64) -> u64 {
        let modulus = 1_000_000_007;
        let a = 2_u64.pow(p as u32);
        let b = a / 2;
        let c = mod_pow(a - 2, b - 1, modulus);

        // (2^p - 2)^(2^(p-1) - 1) * (2^p - 1)
        mod_mul(c, (a - 1 + modulus) % modulus, modulus)
    }
}

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if exponent == 0 {
        return 1;
    }
    while exponent % 2 == 0 {
        base = mod_mul(base, base, modulus);
        exponent /= 2;
    }
    if exponent == 1 {
        base
    } else {
        mod_pow_acc(base, mod_mul(base, base, modulus), exponent / 2, modulus)
    }
}

fn mod_pow_acc(mut r: u64, mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    loop {
        if exponent % 2 == 1 {
            r = mod_mul(r, base, modulus);
            if exponent == 1 {
                return r;
            }
        }
        exponent /= 2;
        base = mod_mul(base, base, modulus);
    }
}

/// (a * n) mod m
fn mod_mul(mut a: u64, mut n: u64, m: u64) -> u64 {
    while n % 2 == 0 {
        a = (a + a) % m;
        n /= 2;
    }
    if n == 1 {
        a
    } else {
        mod_mul_acc(a, (a + a) % m, n / 2, m)
    }
}

fn mod_mul_acc(mut r: u64, mut a: u64, mut n: u64, m: u64) -> u64 {
    loop {
        if n % 2 == 1 {
            r = (r + a) % m;
            if n == 1 {
                return r;
            }
        }
        n /= 2;
        a = (a + a) % m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_non_zero_product(1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_non_zero_product(2), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_non_zero_product(3), 1512);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::min_non_zero_product(4), 581202553);
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::min_non_zero_product(31), 138191773);
    }
}
