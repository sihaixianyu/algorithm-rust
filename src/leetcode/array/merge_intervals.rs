use std::cmp::max;

pub struct Solution;

impl Solution {
    /// @Prob: Merge Intervals
    /// @Link: https://leetcode.cn/problems/merge-intervals/?envType=study-plan-v2&envId=top-100-liked
    /// @Tags: ["array", "sorting"]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut intervals = intervals;

        intervals.sort();

        for interval in intervals {
            if let Some(prev) = ans.last_mut() {
                if prev[1] >= interval[0] {
                    prev[1] = max(prev[1], interval[1]);
                    continue;
                }
            }
            ans.push(interval);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let exp = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let ans = Solution::merge(intervals);
        assert_eq!(exp, ans);
    }

    #[test]
    fn test_case_1() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let exp = vec![vec![1, 5]];
        let ans = Solution::merge(intervals);
        assert_eq!(exp, ans);
    }
}
