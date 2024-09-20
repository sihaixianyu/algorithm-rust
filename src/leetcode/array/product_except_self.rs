pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut pre = vec![1; n];
        let mut suf = vec![1; n];

        for i in 1..n {
            pre[i] = pre[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            suf[i] = suf[i + 1] * nums[i + 1];
        }

        pre.into_iter()
            .zip(suf.into_iter())
            .map(|(a, b)| a * b)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let nums = vec![1, 2, 3, 4];
        let ans = Solution::product_except_self(nums);
        let exp = vec![24, 12, 8, 6];
        assert_eq!(ans, exp);
    }

    #[test]
    fn test_case_1() {
        let nums = vec![-1, 1, 0, -3, 3];
        let ans = Solution::product_except_self(nums);
        let exp = vec![0, 0, 9, 0, 0];
        assert_eq!(ans, exp);
    }
}
