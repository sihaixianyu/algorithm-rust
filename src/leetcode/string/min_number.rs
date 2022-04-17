pub fn min_number(nums: Vec<i32>) -> String {
    let mut strings: Vec<String> = nums.into_iter().map(|i| i.to_string()).collect();
    strings.sort_by(|a, b| format!("{}{}", a, b).cmp(&format!("{}{}", b, a)));
    strings.into_iter().reduce(|a, b| a + &b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![10, 2];

        let res = min_number(nums);
        assert_eq!("102", res)
    }

    #[test]
    fn test_case2() {
        let nums = vec![3, 30, 34, 5, 9];

        let res = min_number(nums);
        assert_eq!("3033459", res)
    }
}
