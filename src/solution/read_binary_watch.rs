use crate::Solution;

impl Solution {
    /// # [401. Binary Watch](https://leetcode.com/problems/binary-watch/)
    ///
    /// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
    ///
    /// - For example, the below binary watch reads `"4:51"`.
    ///
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/binarywatch.jpg" style="width: 500px; height: 500px;">
    ///
    /// Given an integer `turnedOn` which represents the number of LEDs that are currently on (ignoring the PM), return all possible times the watch could represent. You may return the answer in **any order** .
    ///
    /// The hour must not contain a leading zero.
    ///
    /// - For example, `"01:00"` is not valid. It should be `"1:00"`.
    ///
    /// The minute mustconsist of two digits and may contain a leading zero.
    ///
    /// - For example, `"10:2"` is not valid. It should be `"10:02"`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: turnedOn = 1
    /// Output: ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: turnedOn = 9
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= turnedOn <= 10`
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..=turned_on as u32 {
            let j = turned_on as u32 - i;
            let h: Vec<u32> = BinaryClockValues::build(4, i, 12);
            let m: Vec<u32> = BinaryClockValues::build(6, j, 60);
            let mut combinations: Vec<String> = h
                .iter()
                .flat_map(|hour| {
                    m.iter()
                        .map(move |minute| format!("{}:{:02}", hour, minute))
                })
                .collect();
            result.append(&mut combinations);
        }
        result
    }
}

struct BinaryClockValues {
    n: u32,
    m: u32,
    limit: u32,
    val: Vec<u32>,
}

impl BinaryClockValues {
    fn build(n: u32, m: u32, limit: u32) -> Vec<u32> {
        let mut item = BinaryClockValues {
            n,
            m,
            limit,
            val: vec![],
        };
        if m <= n {
            item.back_trace(0, 0, 0);
        }
        item.val
    }

    fn back_trace(&mut self, p: u32, depth: u32, sum: u32) -> bool {
        if sum >= self.limit {
            return false;
        }
        if self.n - p < self.m - depth {
            return false;
        }
        if self.m == depth {
            self.val.push(sum);
            return true;
        }
        for i in p..self.n {
            if !self.back_trace(i + 1, depth + 1, sum + 2_u32.pow(i)) {
                break;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let turned_on = 1;
        let output = [
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ];
        assert_eq!(Solution::read_binary_watch(turned_on), output);
    }

    #[test]
    fn example2() {
        let turned_on = 9;
        let output: Vec<String> = vec![];
        assert_eq!(Solution::read_binary_watch(turned_on), output);
    }

    #[test]
    fn example3() {
        let turned_on = 2;
        let output = [
            "0:03", "0:05", "0:09", "0:17", "0:33", "0:06", "0:10", "0:18", "0:34", "0:12", "0:20",
            "0:36", "0:24", "0:40", "0:48", "1:01", "1:02", "1:04", "1:08", "1:16", "1:32", "2:01",
            "2:02", "2:04", "2:08", "2:16", "2:32", "4:01", "4:02", "4:04", "4:08", "4:16", "4:32",
            "8:01", "8:02", "8:04", "8:08", "8:16", "8:32", "3:00", "5:00", "9:00", "6:00",
            "10:00",
        ];
        assert_eq!(Solution::read_binary_watch(turned_on), output);
    }
}
