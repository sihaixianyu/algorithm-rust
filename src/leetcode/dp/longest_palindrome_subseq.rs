use std::cmp::max;

// 最长回文子序列：https://leetcode.cn/problems/longest-palindromic-subsequence/submissions/
pub fn longest_palindrome_subseq(s: String) -> i32 {
    fn dp(bytes: &[u8], i: i32, j: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
        if j - i <= 0 {
            return j - i + 1;
        }

        let (ui, uj) = (i as usize, j as usize);

        if memo[ui][uj] != -1 {
            return memo[ui][uj];
        }

        if bytes[ui] == bytes[uj] {
            memo[ui][uj] = 2 + dp(bytes, i + 1, j - 1, memo);
        } else {
            memo[ui][uj] = max(dp(bytes, i + 1, j, memo), dp(bytes, i, j - 1, memo))
        }

        memo[ui][uj]
    }

    let bytes = s.as_bytes();
    let len = s.len();
    let mut memo = vec![vec![-1; len]; len];

    dp(bytes, 0, (bytes.len() - 1) as i32, &mut memo)
}

pub fn longest_palindrome_subseq2(s: String) -> i32 {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut dp = vec![vec![0; len]; len];

    // 对角线的所有dp table初始值为2
    for i in 0..len {
        dp[i][i] = 1;
    }

    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            if bytes[i] == bytes[j] {
                dp[i][j] = 2 + dp[i + 1][j - 1];
            } else {
                dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[0][len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "bbbab".to_string();

        let res = longest_palindrome_subseq2(s);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_case2() {
        let s = "cbbd".to_string();

        let res = longest_palindrome_subseq2(s);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_case3() {
        let s = "abcdef".to_string();

        let res = longest_palindrome_subseq2(s);
        assert_eq!(res, 1);
    }
}
