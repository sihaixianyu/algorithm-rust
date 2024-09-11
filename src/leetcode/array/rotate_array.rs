pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;
        let k = (k) % n;

        Self::reverse_bound(nums, 0, n - 1);
        Self::reverse_bound(nums, 0, k - 1);
        Self::reverse_bound(nums, k, n - 1);
    }

    fn reverse_bound(nums: &mut Vec<i32>, lb: i32, rb: i32) {
        let mut lp = lb;
        let mut rp = rb;

        while lp < rp {
            nums.swap(lp as usize, rp as usize);
            lp += 1;
            rp -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;

        Solution::rotate(&mut nums, k);
        let exp = vec![5, 6, 7, 1, 2, 3, 4];

        assert_eq!(nums, exp);
    }

    #[test]
    fn test_case_1() {
        let mut nums = vec![3, 99, -1, -100];
        let k = 2;

        Solution::rotate(&mut nums, k);
        let exp = vec![-1, -100, 3, 99];

        assert_eq!(nums, exp);
    }

    #[test]
    fn test_case_2() {
        let mut nums = vec![-1];
        let k = 0;

        Solution::rotate(&mut nums, k);
        let exp = vec![-1];

        assert_eq!(nums, exp);
    }
}
