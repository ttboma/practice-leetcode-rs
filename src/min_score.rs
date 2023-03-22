use crate::Solution;

impl Solution {
    /// # 2492. Minimum Score of a Path Between Two Cities
    ///
    /// You are given a positive integer `n` representing `n` cities numbered from
    /// `1` to `n`. You are also given a **2D** array roads where 
    /// `roads[i] = [a_i, b_i, distance_i]` indicates that there is a bidirectional
    /// road between cities `a_i` and `b_i` with a distance equal to `distance_i`. The
    /// cities graph is not necessarily connected. 
    ///
    /// The score of a path between two
    /// cities is defined as the minimum distance of a road in this path.
    /// 
    /// Return the minimum possible score of a path between cities `1` and `n`.
    /// 
    /// Note:
    /// 
    /// - A path is a sequence of roads between two cities.
    /// - It is allowed for a path to contain the same road multiple times, and you
    ///   can visit cities `1` and `n` multiple times along the path.
    /// - The test cases are generated such that there is at least one path between
    ///   `1` and `n`.
    /// 
    /// ## Example 1:
    ///
    /// - **Input:** n = 4, roads = [[1,2,9],[2,3,6],[2,4,5],[1,4,7]]
    /// - **Output:** 5
    /// - **Explanation:** The path from city 1 to 4 with the minimum score is: 
    ///   1 -> 2 -> 4. The score of this path is min(9,5) = 5. It can be shown that
    ///   no other path has less score.
    ///
    /// ## Example 2:
    /// 
    /// - **Input:** n = 4, roads = [[1,2,2],[1,3,4],[3,4,7]]
    /// - **Output:** 2
    /// - **Explanation:** The path from city 1 to 4 with the minimum score is: 
    ///   1 -> 2 -> 1 -> 3 -> 4. The score of this path is min(2,2,4,7) = 2.
    ///
    /// ## Constraints:
    /// 
    /// - 2 <= n <= 10^5
    /// - 1 <= roads.length <= 10^5
    /// - roads[i].length == 3
    /// - 1 <= a_i, b_i <= n
    /// - a_i != b_i
    /// - 1 <= distance_i <= 10^4
    /// - There are no repeated edges.
    /// - There is at least one path between 1 and n.
    ///
    /// ------
    ///
    /// ***Extracted from:***
    /// 
    /// [minimum-score-of-a-path-between-two-cities](https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/)
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut group = vec![None; n as usize];
        let mut min_path = vec![i32::MAX; n as usize];
        for r in roads {
            let mut v = (r[0] - 1) as usize;
            let mut u = (r[1] - 1) as usize;
            let dis = r[2];
            while let Some(p) = group[v] {
                v = p as usize;
            }
            while let Some(p) = group[u] {
                u = p as usize;
            }
            let mut determin_min_dis_of_group = |a: usize, b: usize| {
                group[b] = Some(a);
                min_path[b] = std::cmp::min(dis, min_path[b]);
                min_path[a] = std::cmp::min(min_path[a], min_path[b]);
            };
            if v < u {
                determin_min_dis_of_group(v, u);
            } else if v == u {
                min_path[v] = std::cmp::min(dis, min_path[v]);
            } else {
                determin_min_dis_of_group(u, v);
            }
        }
        min_path[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 4;
        let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
        assert_eq!(Solution::min_score(n, roads), 5);
    }

    #[test]
    fn example2() {
        let n = 4;
        let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
        assert_eq!(Solution::min_score(n, roads), 2);
    }
}
