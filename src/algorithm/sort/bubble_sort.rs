pub fn bubble_sort<T: Ord>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len()-1-i {
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];

        bubble_sort(&mut nums[..]);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];
        
        bubble_sort(&mut nums[..]);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}