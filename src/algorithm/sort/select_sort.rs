pub fn select_sort<T: Ord + Copy>(vals: &mut [T]) {
    for i in 0..vals.len() {
        let mut idx_min = i;
        for j in i+1..vals.len() {
            if vals[j] < vals[idx_min] {
                idx_min = j;
            }
        }
        vals.swap(i, idx_min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];

        select_sort(&mut nums[..]);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];

        select_sort(&mut nums[..]);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}
