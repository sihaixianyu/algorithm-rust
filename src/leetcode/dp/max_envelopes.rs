use std::cmp::{max, Ordering};

pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![1; envelopes.len()];
    let mut envelopes = envelopes;
    envelopes.sort_by(|a, b| {
        if a[0] < b[0] || (a[0] == b[0] && a[1] < b[1]) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    for i in 0..envelopes.len() {
        for j in (0..i).rev() {
            if envelopes[j][0] < envelopes[i][0] && envelopes[j][1] < envelopes[i][1] {
                dp[i] = max(dp[i], dp[j] + 1)
            }
        }
    }

    dp.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        let res = max_envelopes(envelopes);
        assert_eq!(3, res)
    }

    #[test]
    fn test_case2() {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        let res = max_envelopes(envelopes);
        assert_eq!(1, res)
    }
}
