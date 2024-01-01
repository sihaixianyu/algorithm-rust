pub fn quick_sort(nums: &mut [i32]) {
    let size = nums.len();
    sort(nums, 0, (size-1) as i32);
}

fn sort(nums: &mut [i32], left: i32, right: i32) {
    if left >= right {
        return
    }

    let pivot = nums[left as usize];
    let (mut lp, mut rp) = (left, right);
    
    while lp < rp {
        while lp < rp && nums[rp as usize] >= pivot {
            rp -= 1
        }
        while lp < rp && nums[lp as usize] <= pivot {
            lp += 1
        }
        nums.swap(lp as usize, rp as usize)
    }
    
    nums.swap(left as usize, lp as usize);
    sort(nums, left, lp-1);
    sort(nums, lp+1, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 4, 2, 5, 1];
        quick_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![5, 4, 3, 2, 1];
        quick_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case3() {
        let mut nums = vec![1, 2, 3, 4];
        quick_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }
}
