pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut path = vec![];
    let mut used = vec![false; nums.len()];

    backtrack(&nums, &mut path, &mut used, &mut res);

    res
}

fn backtrack(nums: &[i32], path: &mut Vec<i32>, used: &mut [bool], res: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        res.push(path.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        path.push(nums[i]);
        used[i] = true;

        backtrack(nums, path, used, res);

        path.pop();
        used[i] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 3];
        let res = permute(nums);
        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        )
    }
}
