use crate::Solution;

impl Solution {
    /// # [797. All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)
    ///
    /// Given a directed acyclic graph (**DAG** ) of `n` nodes labeled from `0` to `n - 1`, find all possible paths from node `0` to node `n - 1` and return them in **any order** .
    ///
    /// The graph is given as follows: `graph[i]` is a list of all nodes you can visit from node `i` (i.e., there is a directed edge from node `i` to node `graph[i][j]`).
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg" style="width: 242px; height: 242px;">
    ///
    /// ```txt
    /// Input: graph = [[1,2],[3],[3],[]]
    /// Output: [[0,1,3],[0,2,3]]
    /// Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg" style="width: 423px; height: 301px;">
    ///
    /// ```txt
    /// Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
    /// Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == graph.length`
    /// - `2 <= n <= 15`
    /// - `0 <= graph[i][j] < n`
    /// - `graph[i][j] != i` (i.e., there will be no self-loops).
    /// - All the elements of `graph[i]` are **unique** .
    /// - The input graph is **guaranteed**  to be a **DAG** .
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        AllPathsSourceTarget::new(&graph).solve()
    }
}

struct AllPathsSourceTarget<'a> {
    graph: &'a Vec<Vec<i32>>,
    position: Vec<i32>,
    paths: Vec<Vec<i32>>,
}

impl<'a> AllPathsSourceTarget<'a> {
    fn new(graph: &'a Vec<Vec<i32>>) -> Self {
        Self {
            graph,
            position: vec![],
            paths: vec![],
        }
    }

    fn solve(mut self) -> Vec<Vec<i32>> {
        self.back_trace(0);
        self.paths
    }

    fn back_trace(&mut self, i: i32) {
        self.position.push(i);
        if i == self.graph.len() as i32 - 1 {
            self.paths.push(self.position.clone());
            return;
        }
        for v in &self.graph[i as usize] {
            self.back_trace(*v);
            self.position.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let output = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(Solution::all_paths_source_target(graph), output);
    }

    #[test]
    fn example2() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let output = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(Solution::all_paths_source_target(graph), output);
    }
}
