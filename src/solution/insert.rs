use super::Solution;

impl Solution {
    /// # [57. Insert Interval](https://leetcode.com/problems/insert-interval/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an array of non-overlapping intervals `intervals` where `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]` represent the start and the end of the `i^th` interval and `intervals` is sorted in ascending order by `start<sub>i</sub>`. You are also given an interval `newInterval = [start, end]` that represents the start and end of another interval.
    ///
    /// Insert `newInterval` into `intervals` such that `intervals` is still sorted in ascending order by `start<sub>i</sub>` and `intervals` still does not have any overlapping intervals (merge overlapping intervals if necessary).
    ///
    /// Return `intervals` after the insertion.
    ///
    /// **Note**  that you don't need to modify `intervals` in-place. You can make a new array and return it.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
    /// Output: [[1,5],[6,9]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
    /// Output: [[1,2],[3,10],[12,16]]
    /// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= intervals.length <= 10^4`
    /// - `intervals[i].length == 2`
    /// - `0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10^5`
    /// - `intervals` is sorted by `start<sub>i</sub>` in **ascending**  order.
    /// - `newInterval.length == 2`
    /// - `0 <= start <= end <= 10^5`
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (i, a) = match intervals.binary_search_by(|probe| {
            if std::cmp::Ordering::Less == probe[1].cmp(&new_interval[0]) {
                std::cmp::Ordering::Less
            } else if std::cmp::Ordering::Greater == probe[0].cmp(&new_interval[0]) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(i) => (i, intervals[i][0]),
            Err(i) => (i, new_interval[0]),
        };
        let (j, b) = match intervals.binary_search_by(|probe| {
            if std::cmp::Ordering::Less == probe[1].cmp(&new_interval[1]) {
                std::cmp::Ordering::Less
            } else if std::cmp::Ordering::Greater == probe[0].cmp(&new_interval[1]) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }) {
            Ok(j) => (j + 1, intervals[j][1]),
            Err(j) => (j, new_interval[1]),
        };
        intervals.splice(i..j, vec![vec![a, b]]);
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
