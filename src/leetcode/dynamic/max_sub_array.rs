pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    
    n as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let res = max_sub_array(nums);
        assert_eq!(6, res);
    }
}
