use std::cmp::max;

// 0-1背包问题：牛客网
pub fn knapsack(volume: i32, n_items: i32, weights: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (n_items as usize, volume as usize);
    let mut memo = vec![vec![-1; n + 1]; m + 1];

    return dp(0, volume, &mut memo, &weights);
}

fn dp(item: i32, volume: i32, memo: &mut Vec<Vec<i32>>, weights: &Vec<Vec<i32>>) -> i32 {
    if item >= weights.len() as i32 {
        return 0;
    }

    let (ui, uj) = (item as usize, volume as usize);

    if memo[ui][uj] != -1 {
        return memo[ui][uj];
    }

    let weight = weights[ui][0];
    let value = weights[ui][1];

    if volume - weights[ui][0] >= 0 {
        return max(
            value + dp(item + 1, volume - weight, memo, weights),
            dp(item + 1, volume, memo, weights),
        );
    } else {
        memo[ui][uj] = dp(item + 1, volume, memo, weights)
    }

    memo[ui][uj]
}

pub fn knapsack2(volume: i32, n_items: i32, weights: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (n_items as usize, volume as usize);
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            let weight = weights[i - 1][0];
            let value = weights[i - 1][1];

            if j as i32 - weight >= 0 {
                dp[i][j] = max(value + dp[i - 1][j - weight as usize], dp[i - 1][j]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let volume = 10;
        let n_item = 2;
        let weights = vec![vec![1, 3], vec![10, 4]];

        let res = knapsack2(volume, n_item, weights);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_case2() {
        let volume = 10;
        let n_item = 2;
        let weights = vec![vec![1, 3], vec![9, 8]];

        let res = knapsack2(volume, n_item, weights);
        assert_eq!(res, 11);
    }
}
