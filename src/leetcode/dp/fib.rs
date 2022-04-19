pub fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    
    let (mut dp_1, mut dp_2) = (0, 1);

    for _ in 2..n+1 {
        let temp = dp_1 + dp_2;
        dp_1 = dp_2;
        dp_2 = temp;
    }

    dp_2
}

// pub fn fib(n: i32) -> i32 {
//     if n < 2 {
//         return n
//     }

//     let mut dp = vec![0; (n + 1) as usize];
//     dp[0] = 0;
//     dp[1] = 1;

//     for i in 2..dp.len() {
//         dp[i] = dp[i - 2] + dp[i - 1]
//     }

//     dp[n as usize]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let n = 4;
        let res = fib(n);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case2() {
        let n = 1;
        let res = fib(n);
        assert_eq!(res, 1);
    }
}
