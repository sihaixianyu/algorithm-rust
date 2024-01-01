// 数组中的第K个最大元素：https://leetcode.cn/problems/kth-largest-element-in-an-array/submissions/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len() as i32;
    let mut nums = nums;

    // 寻找第k大的数字，等价于寻找第n-k小的数字
    return quick_find(&mut nums[..], 0, n - 1, n - k);
}

fn quick_find(nums: &mut [i32], left: i32, right: i32, kth: i32) -> i32 {
    if left > right {
        return -1;
    }

    let pivot = nums[left as usize];
    let (mut lp, mut rp) = (left as usize, right as usize);

    while lp < rp {
        while lp < rp && nums[rp] >= pivot {
            rp -= 1;
        }
        while lp < rp && nums[lp] <= pivot {
            lp += 1;
        }
        nums.swap(lp, rp)
    }

    nums.swap(left as usize, lp);

    if kth == lp as i32 {
        nums[lp]
    } else if kth < lp as i32 {
        quick_find(nums, left, lp as i32 - 1, kth)
    } else {
        quick_find(nums, lp as i32 + 1, right, kth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let res = find_kth_largest(nums, k);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_case2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let res = find_kth_largest(nums, k);
        assert_eq!(res, 4);
    }
}
