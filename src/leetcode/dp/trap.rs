use std::cmp::{max, min};

// 接雨水：https://leetcode.cn/problems/trapping-rain-water/submissions/
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut sum = 0;
    let mut lmax = vec![0; n];
    let mut rmax = vec![0; n];

    lmax[0] = height[0];
    for i in 1..n {
        lmax[i] = max(lmax[i - 1], height[i]);
    }

    rmax[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        rmax[i] = max(rmax[i + 1], height[i]);
    }

    for i in 1..n - 1 {
        let diff = min(lmax[i - 1], rmax[i + 1]) - height[i];
        sum += max(diff, 0);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

        let res = trap(height);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_case2() {
        let height = vec![4, 2, 0, 3, 2, 5];

        let res = trap(height);
        assert_eq!(res, 9);
    }
}
