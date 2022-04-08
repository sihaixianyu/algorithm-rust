use std::cmp::min;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut dp = vec![i32::MAX; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..amount + 1 {
        for &c in &coins {
            if i - c >= 0 && dp[(i - c) as usize] != i32::MAX {
                dp[i as usize] = min(dp[i as usize], dp[(i - c) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == i32::MAX {
        return -1;
    }

    dp[amount as usize]
}

// pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
//     if amount == 0 {
//         return 0;
//     }

//     let mut memo = vec![i32::MAX; (amount + 1) as usize];

//     fn dp(coins: &[i32], amount: i32, memo: &mut [i32]) -> i32 {
//         if amount == 0 {
//             return 0;
//         } else if amount < 0 {
//             return -1;
//         }

//         if memo[amount as usize] != i32::MAX {
//             return memo[amount as usize];
//         }

//         let mut res = i32::MAX;
//         for &c in &coins[..] {
//             let sub_res = dp(&coins, amount - c, memo);
//             if sub_res != -1 {
//                 res = min(res, sub_res + 1);
//             }
//         }

//         res = if res == i32::MAX { -1 } else { res };
//         memo[amount as usize] = res;

//         res
//     }

//     dp(&coins[..], amount, &mut memo[..])
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let coins = vec![1, 2, 5];
        let amount = 11;

        let res = coin_change(coins, amount);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case2() {
        let coins = vec![1];
        let amount = 0;

        let res = coin_change(coins, amount);
        assert_eq!(res, 0);
    }
}
