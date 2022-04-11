pub fn insert_sort<T: Ord + Copy>(vals: &mut [T]) {
    for i in 1..vals.len() {
        let ins_val = vals[i];
        let mut j = i;

        while j > 0 && vals[j - 1] > ins_val {
            vals[j] = vals[j - 1];
            j -= 1;
        }

        vals[j] = ins_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];

        insert_sort(&mut nums[..]);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];

        insert_sort(&mut nums[..]);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}
