use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();

    for (i, v) in nums.iter().enumerate() {
        match m.get(&(target-v)) {
            Some(&j) => {
                res.push(i as i32);
                res.push(j as i32);
                break;
            }
            None => {
                m.insert(*v, i);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![3, 2, 4];

        let res = two_sum(nums, 6);
        assert_eq!(vec![2, 1], res);
    }

    #[test]
    fn test_case2() {
        let nums = vec![2, 7, 11, 15];

        let res = two_sum(nums, 9);
        assert_eq!(vec![1, 0], res);
    }
}
