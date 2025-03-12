pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx_0 = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(idx_0, i);
                idx_0 += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0() {
        let mut nums = vec![0, 0, 1, 2];
        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 2, 0, 0]);
    }
    #[test]
    fn test_case_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
