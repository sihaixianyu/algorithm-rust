// 戳气球：https://leetcode.cn/problems/burst-balloons/
pub fn max_coins(_nums: Vec<i32>) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![3, 1, 5, 8];

        let res = max_coins(nums);
        println!("{}", res);
        // assert_eq!(res, 167);
    }
}
