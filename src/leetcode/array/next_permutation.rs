// 下一个排列：https://leetcode.cn/problems/next-permutation/
pub fn next_permutation(nums: &mut Vec<i32>) {
    for i in (1..nums.len()).rev() {
        if nums[i - 1] < nums[i] {
            let mut j = nums.len() - 1;
            while j >= i && nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
            nums[i..].sort();
            return;
        }
    }

    nums.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![1, 2, 3];
        let expected = vec![1, 3, 2];

        next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![3, 2, 1];
        let expected = vec![1, 2, 3];

        next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }
}
