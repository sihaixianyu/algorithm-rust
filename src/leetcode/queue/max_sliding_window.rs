use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
pub struct Pair {
    idx: usize,
    val: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut ans = vec![];
        let mut max_heap: BinaryHeap<Pair> = BinaryHeap::new();

        for i in 0..n {
            let pair = Pair {
                idx: i,
                val: nums[i],
            };
            max_heap.push(pair);

            while !max_heap.is_empty() && max_heap.peek().unwrap().idx + k <= i {
                max_heap.pop();
            }

            if i >= k - 1 {
                let top_val = max_heap.peek().unwrap().val;
                ans.push(top_val);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let res = Solution::max_sliding_window(nums, k);
        let exp = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_1() {
        let nums = vec![1];
        let k = 1;
        let res = Solution::max_sliding_window(nums, k);
        let exp = vec![1];
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, -1];
        let k = 1;
        let res = Solution::max_sliding_window(nums, k);
        let exp = vec![1, -1];
        assert_eq!(res, exp);
    }
}
