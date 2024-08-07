pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut res = vec![];

        for k in 0..(n - 2) {
            if nums[k] > 0 {
                continue;
            }
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }

            let (mut i, mut j) = (k + 1, n - 1);

            while i < j {
                let sum = nums[k] + nums[i] + nums[j];

                if sum < 0 {
                    i += 1;
                    continue;
                }
                if sum > 0 {
                    j -= 1;
                    continue;
                }

                res.push(vec![nums[k], nums[i], nums[j]]);

                while i < j && nums[i] == nums[i + 1] {
                    i += 1;
                }
                while i < j && nums[j] == nums[j - 1] {
                    j -= 1;
                }
                i += 1;
                j -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        let exp = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_1() {
        let nums = vec![0, 1, 1];
        let res = Solution::three_sum(nums);
        let exp: Vec<Vec<i32>> = vec![];
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 0, 0];
        let res = Solution::three_sum(nums);
        let exp = vec![vec![0, 0, 0]];
        assert_eq!(res, exp);
    }
}
