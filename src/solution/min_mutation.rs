use crate::Solution;

impl Solution {
    /// # [433. Minimum Genetic Mutation](https://leetcode.com/problems/minimum-genetic-mutation/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// A gene string can be represented by an 8-character long string, with choices from `'A'`, `'C'`, `'G'`, and `'T'`.
    ///
    /// Suppose we need to investigate a mutation from a gene string `startGene` to a gene string `endGene` where one mutation is defined as one single character changed in the gene string.
    ///
    /// - For example, `"AACCGGTT" --> "AACCGGTA"` is one mutation.
    ///
    /// There is also a gene bank `bank` that records all the valid gene mutations. A gene must be in `bank` to make it a valid gene string.
    ///
    /// Given the two gene strings `startGene` and `endGene` and the gene bank `bank`, return the minimum number of mutations needed to mutate from `startGene` to `endGene`. If there is no such a mutation, return `-1`.
    ///
    /// Note that the starting point is assumed to be valid, so it might not be included in the bank.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
    /// Output: 1
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
    /// Output: 2
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= bank.length <= 10`
    /// - `startGene.length == endGene.length == bank[i].length == 8`
    /// - `startGene`, `endGene`, and `bank[i]` consist of only the characters `['A', 'C', 'G', 'T']`.
    pub fn min_mutation(start_gene: String, end_gene: String, mut bank: Vec<String>) -> i32 {
        bank.push(start_gene);
        let mut visited = vec![0; bank.len()];
        let mut dist = vec![0; bank.len()];
        let mut queue = std::collections::VecDeque::new();
        *visited.last_mut().unwrap() = 1;
        queue.push_back(bank.len() - 1);
        while let Some(idx) = queue.pop_front() {
            let s = &bank[idx];
            for (i, e) in bank.iter().enumerate() {
                if visited[i] == 0 && s.chars().zip(e.chars()).filter(|(a, b)| a != b).count() == 1
                {
                    if e == &end_gene {
                        return dist[idx] + 1;
                    }
                    visited[i] = 1;
                    dist[i] = dist[idx] + 1;
                    queue.push_back(i);
                }
            }
            visited[idx] = 2;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string()];
        let result = 1;

        assert_eq!(Solution::min_mutation(start_gene, end_gene, bank), result);
    }

    #[test]
    fn example2() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ];
        let result = 2;

        assert_eq!(Solution::min_mutation(start_gene, end_gene, bank), result);
    }
}
