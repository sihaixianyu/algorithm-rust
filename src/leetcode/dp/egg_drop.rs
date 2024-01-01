use std::cmp::{max, min};

// 鸡蛋掉落：https://leetcode.cn/problems/super-egg-drop/
pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let (uk, un) = (k as usize, n as usize);
    let mut memo = vec![vec![-1; un + 1]; uk + 1];

    dp(k, n, &mut memo)
}

// 参数`n`表示待扫描的楼层数
fn dp(k: i32, n: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
    if n == 0 {
        return 0;
    }

    if k == 1 {
        return n;
    }

    let (uk, un) = (k as usize, n as usize);
    if memo[uk][un] != -1 {
        return memo[uk][un];
    }

    let mut res = i32::MAX;
    for i in 1..=n {
        let case1 = dp(k - 1, i - 1, memo);
        let case2 = dp(k, n - i, memo);
        res = min(res, max(case1, case2) + 1);
    }
    memo[uk][un] = res;

    res
}

pub fn super_egg_drop2(k: i32, n: i32) -> i32 {
    let (uk, un) = (k as usize, n as usize);
    let mut dp = vec![vec![i32::MAX; uk + 1]; un + 1];

    // 当楼层为0，无论多少个鸡蛋都不需要操作
    for j in 0..=uk {
        dp[0][j] = 0;
    }
    // 当楼层为1，当鸡蛋数>=1时值需要操作一次
    for j in 1..=uk {
        dp[1][j] = 1;
    }

    // 当鸡蛋数为0，操作次数只能为0
    for i in 0..=un {
        dp[i][0] = 0;
    }
    // 当鸡蛋数为1，最坏需要从低到高扔i次
    for i in 0..=un {
        dp[i][1] = i as i32;
    }

    for i in 2..=un {
        for j in 2..=uk {
            // 此层的`k`代表选择扔鸡蛋的楼层
            for k in 1..=i {
                dp[i][j] = min(dp[i][j], max(dp[k-1][j-1], dp[i-k][j]) + 1)
            }
        }
    }

    dp[un][uk]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let k = 1;
        let n = 2;

        let res = super_egg_drop2(k, n);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_case2() {
        let k = 2;
        let n = 6;

        let res = super_egg_drop2(k, n);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case3() {
        let k = 7;
        let n = 1000;

        let res = super_egg_drop2(k, n);
        assert_eq!(res, 11);
    }
}
