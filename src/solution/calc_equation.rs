use crate::Solution;

impl Solution {
    /// # [399. Evaluate Division](https://leetcode.com/problems/evaluate-division/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an array of variable pairs `equations` and an array of real numbers `values`, where `equations[i] = [A<sub>i</sub>, B<sub>i</sub>]` and `values[i]` represent the equation `A<sub>i</sub> / B<sub>i</sub> = values[i]`. Each `A<sub>i</sub>` or `B<sub>i</sub>` is a string that represents a single variable.
    ///
    /// You are also given some `queries`, where `queries[j] = [C<sub>j</sub>, D<sub>j</sub>]` represents the `j^th` query where you must find the answer for `C<sub>j</sub> / D<sub>j</sub> = ?`.
    ///
    /// Return the answers to all queries. If a single answer cannot be determined, return `-1.0`.
    ///
    /// **Note:**  The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.
    ///
    /// **Note:** The variables that do not occur in the list of equations are undefined, so the answer cannot be determined for them.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
    /// Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
    /// Explanation:
    /// Given: a / b = 2.0, b / c = 3.0
    /// queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
    /// return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
    /// note: x is undefined => -1.0```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
    /// Output: [3.75000,0.40000,5.00000,0.20000]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
    /// Output: [0.50000,2.00000,-1.00000,-1.00000]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= equations.length <= 20`
    /// - `equations[i].length == 2`
    /// - `1 <= A<sub>i</sub>.length, B<sub>i</sub>.length <= 5`
    /// - `values.length == equations.length`
    /// - `0.0 < values[i] <= 20.0`
    /// - `1 <= queries.length <= 20`
    /// - `queries[i].length == 2`
    /// - `1 <= C<sub>j</sub>.length, D<sub>j</sub>.length <= 5`
    /// - `A<sub>i</sub>, B<sub>i</sub>, C<sub>j</sub>, D<sub>j</sub>` consist of lower case English letters and digits.
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let ids = equations
            .iter()
            .flat_map(|eq| eq.iter())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<std::collections::HashMap<_, _>>();

        let mut graph: Vec<Vec<(usize, f64)>> = vec![vec![]; ids.len()];
        for (eq, v) in equations.iter().zip(values.iter()) {
            let n = ids[&eq[0]];
            let m = ids[&eq[1]];
            graph[n].push((m, *v));
            graph[m].push((n, 1f64 / *v))
        }

        queries
            .iter()
            .map(|query| {
                let mut visited = vec![false; ids.len()];
                let src = ids.get(&query[0]);
                let dest = ids.get(&query[1]);
                match (src, dest) {
                    (Some(&src), Some(&dest)) => {
                        dfs(src, dest, &mut visited, &graph).unwrap_or(-1.0)
                    }
                    _ => -1.0,
                }
            })
            .collect()
    }
}

fn dfs(
    src: usize,
    dest: usize,
    visited: &mut Vec<bool>,
    graph: &Vec<Vec<(usize, f64)>>,
) -> Option<f64> {
    visited[src] = true;
    if src == dest {
        return Some(1.0);
    }
    for (n, v) in graph[src].iter() {
        if !visited[*n] {
            if let Some(r) = dfs(*n, dest, visited, graph) {
                return Some(r * v);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ];
        let result = vec![6.0, 0.5, -1.0, 1.0, -1.0];
        assert_eq!(Solution::calc_equation(equations, values, queries), result);
    }

    #[test]
    fn example2() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
            vec!["cd".to_string(), "bc".to_string()],
        ];
        let result = vec![3.75, 0.4, 5.0, 0.2];
        assert_eq!(Solution::calc_equation(equations, values, queries), result);
    }
}
