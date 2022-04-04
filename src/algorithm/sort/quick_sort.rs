pub fn quick_sort(nums: &mut [i32], s: i32, e: i32) {
    if s >= e {
        return;
    }

    let pivot = nums[s as usize];
    let (mut lp, mut rp) = (s, e);
    while lp < rp {
        while lp < rp && nums[rp as usize] >= pivot {
            rp -= 1;
        }
        while lp < rp && nums[lp as usize] <= pivot {
            lp += 1
        }
        nums.swap(lp as usize, rp as usize);
    }
    nums.swap(s as usize, lp as usize);

    quick_sort(nums, s, lp - 1);
    quick_sort(nums, lp + 1, e);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];
        let n = nums.len();

        quick_sort(nums.as_mut_slice(), 0, (n - 1) as i32);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];
        let n = nums.len();

        quick_sort(nums.as_mut_slice(), 0, (n - 1) as i32);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}
