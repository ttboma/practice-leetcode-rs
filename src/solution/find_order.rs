use crate::Solution;

impl Solution {
    /// # [210. Course Schedule II](https://leetcode.com/problems/course-schedule-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you **must**  take course `b<sub>i</sub>` first if you want to take course `a<sub>i</sub>`.
    ///
    /// - For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
    ///
    /// Return the ordering of courses you should take to finish all courses. If there are many valid answers, return **any**  of them. If it is impossible to finish all courses, return **an empty array** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: numCourses = 2, prerequisites = [[1,0]]
    /// Output: [0,1]
    /// Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
    /// Output: [0,2,1,3]
    /// Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
    /// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: numCourses = 1, prerequisites = []
    /// Output: [0]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= numCourses <= 2000`
    /// - `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
    /// - `prerequisites[i].length == 2`
    /// - `0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses`
    /// - `a<sub>i</sub> != b<sub>i</sub>`
    /// - All the pairs `[a<sub>i</sub>, b<sub>i</sub>]` are **distinct** .
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut inverse_graph = vec![vec![]; num_courses as usize];
        let mut dependency = vec![0; num_courses as usize];

        prerequisites.into_iter().for_each(|e| {
            dependency[e[0] as usize] += 1;
            inverse_graph[e[1] as usize].push(e[0] as usize);
        });

        let mut queue = dependency
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if *n == 0 { Some(i) } else { None })
            .collect::<Vec<usize>>();

        while let Some(node) = queue.pop() {
            for next in &inverse_graph[node] {
                dependency[*next] -= 1;
                if dependency[*next] == 0 {
                    queue.push(*next);
                }
            }
            result.push(node as i32);
        }
        if result.len() == num_courses as usize {
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 2, 1, 3]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::find_order(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            vec![2, 1, 0]
        );
    }
}
