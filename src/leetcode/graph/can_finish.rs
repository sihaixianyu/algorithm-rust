use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = HashMap::new();
        for i in 0..num_courses {
            graph.insert(i, HashSet::new());
        }
        prerequisites.iter().for_each(|des_src| {
            let (des, src) = (des_src[0], des_src[1]);
            graph.get_mut(&des).unwrap().insert(src);
        });

        let mut finishable = VecDeque::new();
        graph.iter_mut().for_each(|(des, src_nodes)| {
            if src_nodes.is_empty() {
                finishable.push_back(*des);
            }
        });
        finishable.iter().for_each(|c| {
            graph.remove(c);
        });

        while let Some(c) = finishable.pop_front() {
            graph.iter_mut().for_each(|(des, src_nodes)| {
                src_nodes.remove(&c);
                if src_nodes.is_empty() {
                    finishable.push_back(*des);
                }
            });
            finishable.iter().for_each(|c| {
                graph.remove(c);
            });
        }

        graph.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];

        let res = Solution::can_finish(num_courses, prerequisites);
        assert!(res);
    }

    #[test]
    fn test_case_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];

        let res = Solution::can_finish(num_courses, prerequisites);
        assert!(!res);
    }
}
