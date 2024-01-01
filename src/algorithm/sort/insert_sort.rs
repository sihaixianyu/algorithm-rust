pub fn insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let temp = nums[i];
        let mut j = i;
        while j > 0 {
            if nums[j - 1] <= temp {
                break;
            }
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums = vec![3, 4, 2, 5, 1];
        insert_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case_2() {
        let mut nums = vec![5, 4, 3, 2, 1];
        insert_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case_3() {
        let mut nums = vec![1, 2, 3, 4];
        insert_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }
}
