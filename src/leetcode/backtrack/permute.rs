pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    let mut used = vec![false; nums.len()];

    backtrack(&nums, &mut res, &mut path, &mut used);

    res
}

fn backtrack(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, used: &mut Vec<bool>) {
    if path.len() == nums.len() {
        res.push(path.clone());
    }

    for i in 0..nums.len() {
        if used[i] == true {
            continue;
        }

        path.push(nums[i]);
        used[i] = true;

        backtrack(nums, res, path, used);

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
        println!("{:?}", res)
    }

    #[test]
    fn test_case2() {
        let nums = vec![0, 1];

        let res = permute(nums);
        println!("{:?}", res)
    }
}
