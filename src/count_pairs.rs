use std::usize;

use crate::Solution;

impl Solution {
    /// # 2316. Count Unreachable Pairs of Nodes in an Undirected Graph
    ///
    /// You are given an integer `n`. There is an undirected graph with `n` nodes,
    /// numbered from `0` to `n - 1`. You are given a **2D** integer array edges
    /// where `edges[i] = [a_i, b_i]` denotes that there exists an undirected edge
    /// connecting nodes `a_i` and `b_i`. Return the number of pairs of different
    /// nodes that are unreachable from each other.
    ///
    /// ## Example 1:
    ///
    /// - **Input:** n = 3, edges = [[0,1],[0,2],[1,2]]
    /// - **Output:** 0
    /// - **Explanation:** There are no pairs of nodes that are unreachable from
    ///   each other. Therefore, we return 0.
    /// 
    /// ## Example 2:
    ///
    /// - **Input:** n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
    /// - **Output:** 14
    /// - **Explanation:** here are 14 pairs of nodes that are unreachable from
    ///   each other: [[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],
    ///   [3,5],[3,6],[4,6],[5,6]]. Therefore, we return 14.
    ///
    /// ## Constraints:
    /// - 1 <= n <= 10^5
    /// - 0 <= edges.length <= 2 * 10^5
    /// - edges[i].length == 2
    /// - 0 <= a_i, b_i < n
    /// - a_i != b_i
    /// - There are no repeated edges.
    /// ------
    ///
    /// ***Extracted from:***
    /// 
    /// [count-unreachable-pairs-of-nodes-in-an-undirected-graph](https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/)
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut groups: Vec<Option<usize>> = vec![None; n as usize];
        let mut num = vec![1; n as usize];
        for e in edges {
            let mut find_group = |mut w: usize| -> usize {
                while let Some(i) = groups[w] {
                    if let Some(j) = groups[i] {
                        groups[w] = Some(j);
                        w = i;
                    } else {
                        return i;
                    }
                }
                return w;
            };
            let v = find_group(e[0] as usize);
            let u = find_group(e[1] as usize);
            if v != u {
                groups[u] = Some(v);
                num[v] += num[u];
                num[u] = 0;
            }
        }
        let mut sum = 0;
        let num: Vec<usize> = num
            .iter()
            .filter_map(|&n| {
                if n != 0 {
                    sum += n;
                    Some(n)
                } else {
                    None
                }
            })
            .collect();
        num.iter()
            .map(|n| {
                sum -= n;
                n * sum
            })
            .sum::<usize>() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 3;
        let edges: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        assert_eq!(Solution::count_pairs(n, edges), 0);
    }

    #[test]
    fn example2() {
        let n = 7;
        let edges: Vec<Vec<i32>> = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        assert_eq!(Solution::count_pairs(n, edges), 14);
    }
}
