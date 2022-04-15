pub fn shell_sort<T: Ord + Copy>(vals: &mut [T]) {
    vals.swap(0, vals.len()-1);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];

        shell_sort(&mut nums[..]);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];

        shell_sort(&mut nums[..]);
        assert_eq!(vec![2, 7, 11, 15], nums)
    }
}
