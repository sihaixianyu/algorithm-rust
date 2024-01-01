use std::cmp::max;

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];
    for i in 0..nums.len() {
        for j in (0..i).rev() {
            if nums[j] < nums[i] {
                dp[i] = max(dp[i], dp[j] + 1)
            }
        }
    }

    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = Vec::from([10, 9, 2, 5, 3, 7, 101, 18]);
        let res = length_of_lis(nums);
        assert_eq!(4, res)
    }

    #[test]
    fn test_case2() {
        let nums = Vec::from([0, 1, 0, 3, 2, 3]);
        let res = length_of_lis(nums);
        assert_eq!(4, res)
    }

    #[test]
    fn test_case3() {
        let nums = Vec::from([7, 7, 7, 7, 7, 7, 7]);
        let res = length_of_lis(nums);
        assert_eq!(1, res)
    }
}
