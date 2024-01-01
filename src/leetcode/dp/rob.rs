use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    if n == 1 {
        return nums[0];
    }

    let rob_range = |nums: &[i32]| -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 1];

        dp[0] = 0;
        dp[1] = nums[0];

        for i in 2..=n {
            dp[i] = max(dp[i - 1], dp[i - 2] + nums[i - 1]);
        }

        dp[n]
    };

    let res1 = rob_range(&nums[..n - 1]);
    let res2 = rob_range(&nums[1..n]);

    max(res1, res2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![2, 3, 2];

        let res = rob(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 2, 3, 1];

        let res = rob(nums);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_case3() {
        let nums = vec![0];

        let res = rob(nums);
        assert_eq!(res, 0);
    }
}
