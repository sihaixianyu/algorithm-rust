pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let m = coins.len();
    let n = amount as usize;
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = 1;
    }

    for i in 1..=m {
        for j in 1..=n {
            let balance = j as i32 - coins[i - 1];

            if balance >= 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][balance as usize];
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
        let amount = 5;
        let coins = vec![1, 2, 5];

        let res = change(amount, coins);
        assert_eq!(res, 4)
    }
}
