use crate::Solution;

impl Solution {
    /// # [2305. Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/)
    ///
    /// You are given an integer array `cookies`, where `cookies[i]` denotes the number of cookies in the `i^th` bag. You are also given an integer `k` that denotes the number of children to distribute **all**  the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.
    ///
    /// The **unfairness**  of a distribution is defined as the **maximum**  **total**  cookies obtained by a single child in the distribution.
    ///
    /// Return the **minimum**  unfairness of all distributions.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: cookies = [8,15,10,20,8], k = 2
    /// Output: 31
    /// Explanation: One optimal distribution is [8,15,8] and [10,20]
    /// - The 1^st child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
    /// - The 2^nd child receives [10,20] which has a total of 10 + 20 = 30 cookies.
    /// The unfairness of the distribution is max(31,30) = 31.
    /// It can be shown that there is no distribution with an unfairness less than 31.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: cookies = [6,1,3,2,2,4,1,2], k = 3
    /// Output: 7
    /// Explanation: One optimal distribution is [6,1], [3,2,2], and [4,1,2]
    /// - The 1^st child receives [6,1] which has a total of 6 + 1 = 7 cookies.
    /// - The 2^nd child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
    /// - The 3^rd child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
    /// The unfairness of the distribution is max(7,7,7) = 7.
    /// It can be shown that there is no distribution with an unfairness less than 7.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= cookies.length <= 8`
    /// - `1 <= cookies[i] <= 10^5`
    /// - `2 <= k <= cookies.length`
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        DistributiveCookies::new(&cookies, k as usize).unfairness()
    }
}

struct DistributiveCookies<'a> {
    cookies: &'a Vec<i32>,
    dis: Vec<i32>,
    min_unfairness: i32,
    com: usize,
}

impl<'a> DistributiveCookies<'a> {
    fn new(cookies: &'a Vec<i32>, k: usize) -> Self {
        DistributiveCookies {
            cookies,
            dis: vec![0; k],
            min_unfairness: i32::MAX,
            com: 0,
        }
    }
    fn unfairness(&mut self) -> i32 {
        self.backtrace(0);
        self.min_unfairness
    }
    fn backtrace(&mut self, i: usize) {
        if i == self.cookies.len() {
            let unfairness = *self.dis.iter().max().unwrap();
            self.min_unfairness = unfairness.min(self.min_unfairness);
            return;
        }
        for j in 0..=self.com {
            if j == self.com && self.com < self.dis.len() - 1 {
                self.com += 1;
            } else if self.dis[j] + self.cookies[i] >= self.min_unfairness {
                continue;
            }
            self.dis[j] += self.cookies[i];
            self.backtrace(i + 1);
            self.dis[j] -= self.cookies[i];
            if j == self.com && self.com < self.dis.len() - 1 {
                self.com -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;
        assert_eq!(Solution::distribute_cookies(cookies, k), 31);
    }

    #[test]
    fn example2() {
        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;
        assert_eq!(Solution::distribute_cookies(cookies, k), 7);
    }
}
