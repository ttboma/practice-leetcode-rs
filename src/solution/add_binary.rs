use super::*;

impl Solution {
    /// # [67. Add Binary](https://leetcode.com/problems/add-binary/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two binary strings `a` and `b`, return their sum as a binary string.
    ///
    /// **Example 1:**
    ///
    /// ```
    /// Input: a = "11", b = "1"
    /// Output: "100"
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: a = "1010", b = "1011"
    /// Output: "10101"
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= a.length, b.length <= 10^4`
    /// - `a` and `b` consistonly of `'0'` or `'1'` characters.
    /// - Each string does not contain leading zeros except for the zero itself.
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if a.len() > b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let m = a.len();
        let n = b.len();
        if m == 0 {
            return b;
        } else if m < n {
            a = std::iter::repeat('0')
                .take(n - m)
                .chain(a.chars())
                .collect();
        }

        let mut output = String::new();
        a.chars()
            .zip(b.chars())
            .fold(State::Initial, |state, digits| {
                state.transit(digits, &mut output)
            })
            .transit_to_end(&mut output);

        output
    }
}

enum State {
    Initial,
    AllOnes(usize),     // number of ones
    ZeroAndOnes(usize), // one zero and number of ones
}

impl State {
    fn transit(self, digits: (char, char), output: &mut String) -> Self {
        match self {
            State::Initial => Self::transit_from_initial(digits, output),
            State::AllOnes(count) => Self::transit_from_all_ones(digits, count, output),
            State::ZeroAndOnes(count) => Self::transit_from_zero_and_ones(digits, count, output),
        }
    }

    fn transit_to_end(&self, output: &mut String) {
        match self {
            Self::Initial => output.push('0'),
            Self::AllOnes(count) => output.extend(std::iter::repeat('1').take(*count)),
            Self::ZeroAndOnes(count) => {
                output.push('0');
                output.extend(std::iter::repeat('1').take(*count));
            }
        }
    }

    fn transit_from_initial(digits: (char, char), output: &mut String) -> Self {
        match digits {
            ('0', '0') => Self::Initial,
            ('0', '1') | ('1', '0') => Self::AllOnes(1),
            ('1', '1') => {
                output.push('1');
                Self::ZeroAndOnes(0)
            }
            _ => unreachable!(),
        }
    }

    fn transit_from_all_ones(digits: (char, char), count: usize, output: &mut String) -> Self {
        match digits {
            ('0', '0') => {
                output.extend(std::iter::repeat('1').take(count));
                Self::ZeroAndOnes(0)
            }
            ('0', '1') | ('1', '0') => Self::AllOnes(count + 1),
            ('1', '1') => {
                output.push('1');
                output.extend(std::iter::repeat('0').take(count));
                Self::ZeroAndOnes(0)
            }
            _ => unreachable!(),
        }
    }

    fn transit_from_zero_and_ones(digits: (char, char), count: usize, output: &mut String) -> Self {
        match digits {
            ('0', '0') => {
                output.push('0');
                output.extend(std::iter::repeat('1').take(count));
                Self::ZeroAndOnes(0)
            }
            ('0', '1') | ('1', '0') => Self::ZeroAndOnes(count + 1),
            ('1', '1') => {
                output.push('1');
                output.extend(std::iter::repeat('0').take(count));
                Self::ZeroAndOnes(0)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let res = "100".to_string();
        assert_eq!(Solution::add_binary(a, b), res);
    }

    #[test]
    fn example2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let res = "10101".to_string();
        assert_eq!(Solution::add_binary(a, b), res);
    }

    #[test]
    fn example3() {
        let a = "0".to_string();
        let b = "0".to_string();
        let res = "0".to_string();
        assert_eq!(Solution::add_binary(a, b), res);
    }

    #[test]
    fn example4() {
        let a = "0".to_string();
        let b = "10".to_string();
        let res = "10".to_string();
        assert_eq!(Solution::add_binary(a, b), res);
    }
}
