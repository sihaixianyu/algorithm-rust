use std::cmp::min;

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = grid[0][0];

    for i in 1..m {
        dp[i][0] = dp[i-1][0] + grid[i][0];
    }
    for j in 1..n {
        dp[0][j] = dp[0][j-1] + grid[0][j];
    }

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j];
        }
    }

    dp[m-1][n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

        let res = min_path_sum(grid);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_case2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let res = min_path_sum(grid);
        assert_eq!(res, 12);
    }
}
