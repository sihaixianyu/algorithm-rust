// 每日温度：https://leetcode.cn/problems/daily-temperatures/
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stk = vec![];
    let mut ans = vec![0; temperatures.len()];

    for (i, &v) in temperatures.iter().enumerate() {
        while !stk.is_empty() && temperatures[stk[stk.len()-1]] < v {
            ans[stk[stk.len()-1]] = (i - stk[stk.len()-1]) as i32;
            stk.pop();
        }

        stk.push(i);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];

        let res = daily_temperatures(temps);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn test_case2() {
        let temps = vec![30, 40, 50, 60];

        let res = daily_temperatures(temps);
        assert_eq!(res, vec![1, 1, 1, 0]);
    }
}
