use crate::Solution;

impl Solution {
    /// # [207. Course Schedule](https://leetcode.com/problems/course-schedule/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you **must**  take course `b<sub>i</sub>` first if you want to take course `a<sub>i</sub>`.
    ///
    /// - For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
    ///
    /// Return `true` if you can finish all courses. Otherwise, return `false`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: numCourses = 2, prerequisites = [[1,0]]
    /// Output: true
    /// Explanation: There are a total of 2 courses to take.
    /// To take course 1 you should have finished course 0. So it is possible.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
    /// Output: false
    /// Explanation: There are a total of 2 courses to take.
    /// To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= numCourses <= 2000`
    /// - `0 <= prerequisites.length <= 5000`
    /// - `prerequisites[i].length == 2`
    /// - `0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses`
    /// - All the pairs prerequisites[i] are **unique** .
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![0; num_courses as usize]; // 0: not visited, 1: visiting, 2: visited
        let mut graph = vec![vec![]; num_courses as usize];
        for edge in prerequisites {
            graph[edge[0] as usize].push(edge[1] as usize);
        }

        !(0..num_courses as usize).any(|course| dfs_find_cycle(&graph, &mut visited, course))
    }
}

fn dfs_find_cycle(graph: &Vec<Vec<usize>>, visited: &mut Vec<usize>, course: usize) -> bool {
    match visited[course] {
        0 => {
            visited[course] = 1;
            if graph[course]
                .iter()
                .any(|&next_course| dfs_find_cycle(graph, visited, next_course))
            {
                true
            } else {
                visited[course] = 2;
                false
            }
        }
        1 => true,
        2 => false,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let res = true;
        assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
    }

    #[test]
    fn example2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        let res = false;
        assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
    }
}
