use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut val_idx = HashMap::new();

        for i in 0..nums.len() {
            let need = target - nums[i];

            if let Some(&idx) = val_idx.get(&need) {
                return vec![idx, i as i32];
            }

            val_idx.insert(nums[i], i as i32);
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![0, 1]);
    }
}
