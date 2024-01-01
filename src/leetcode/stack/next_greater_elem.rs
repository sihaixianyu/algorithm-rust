use std::collections::HashMap;

// 下一个更大元素I：https://leetcode.cn/problems/next-greater-element-i/
pub fn next_greater_element_i(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![-1; nums1.len()];
    let mut desc_stk = vec![];
    let mut greaters = HashMap::new();

    for v in nums2.into_iter() {
        while !desc_stk.is_empty() && v > *desc_stk.last().unwrap() {
            greaters.insert(desc_stk.pop().unwrap(), v);
        }
        desc_stk.push(v)
    }

    for (i, v) in nums1.into_iter().enumerate() {
        if let Some(&x) = greaters.get(&v) {
            ans[i] = x;
        }
    }

    ans
}

// 下一个更大元素II：https://leetcode.cn/problems/next-greater-element-ii/
pub fn next_greater_element_ii(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![-1; nums.len()];
    let mut dstk = vec![];

    for i in 0..2 * n {
        while !dstk.is_empty() && nums[i % n] > nums[dstk[dstk.len() - 1]] {
            ans[dstk.pop().unwrap()] = nums[i % n]
        }
        dstk.push(i % n);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_next_greater_element_i {
        use super::next_greater_element_i;

        #[test]
        fn test_case1() {
            let nums1 = vec![4, 1, 2];
            let nums2 = vec![1, 3, 4, 2];

            let ans = next_greater_element_i(nums1, nums2);
            assert_eq!(ans, vec![-1, 3, -1]);
        }
    }

    mod tests_next_greater_element_ii {
        use super::next_greater_element_ii;

        #[test]
        fn test_case2() {
            let nums = vec![1, 2, 1];

            let ans = next_greater_element_ii(nums);
            assert_eq!(ans, vec![2, -1, 2]);
        }
    }
}
