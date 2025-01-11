pub struct Solution;

impl Solution {
    /// @Prob: First Missing Positive
    /// @Link: https://leetcode.cn/problems/first-missing-positive/description/?envType=study-plan-v2&envId=top-100-liked
    /// @Tags: ["array", "hash"]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();

        for i in 0..nums.len() {
            while nums[i] > 0 && (nums[i] as usize) < len && nums[i] != nums[nums[i] as usize - 1] {
                let tar_idx = (nums[i] - 1) as usize;
                nums.swap(i, tar_idx);
            }
        }

        for i in 0..nums.len() {
            if nums[i] != (i as i32 + 1) {
                return (i + 1) as i32;
            }
        }

        nums.len() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0() {
        let nums = vec![1, 2, 0];
        let ans = 3;

        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_case_1() {
        let nums = vec![3, 4, -1, 1];
        let ans = 2;

        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![7, 8, 9, 11, 12];
        let ans = 1;

        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, ans);
    }
}
