pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![0];

        Self::dfs(0, &mut path, &mut ans, &graph);

        ans
    }

    fn dfs(cur_node: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, graph: &Vec<Vec<i32>>) {
        if path.len() > graph.len() {
            return;
        }

        if cur_node == graph.len() as i32 - 1 {
            ans.push(path.clone());
            return;
        }

        for &node in &graph[cur_node as usize] {
            path.push(node);
            Self::dfs(node, path, ans, graph);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let ans = Solution::all_paths_source_target(graph);
        assert_eq!(ans, vec![vec![0, 1, 3], vec![0, 2, 3]]);
    }

    #[test]
    fn test_case_1() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let ans = Solution::all_paths_source_target(graph);
        assert_eq!(
            ans,
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ]
        );
    }
}
