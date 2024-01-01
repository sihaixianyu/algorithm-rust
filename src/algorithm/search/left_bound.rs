pub fn left_bound(nums: &[i32], tar: i32) -> i32 {
    let mut left = 0 as i32;
    let mut right = nums.len() as i32;

    while left < right {
        let mid = (left + (right - left) / 2) as usize;

        if nums[mid] == tar {
            right = mid as i32 - 1;
        } else if nums[mid] < tar {
            left = mid as i32 + 1;
        } else if nums[mid] > tar {
            right = mid as i32;
        }
    }

    if left as usize == nums.len() {
        return -1;
    }

    if nums[left as usize] != tar {
        return -1;
    }

    return left;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 3, 3, 4, 5];
        let tar = 3;
        let res = left_bound(&nums, tar);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 1, 3, 4, 5];
        let tar = 1;
        let res = left_bound(&nums, tar);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 1, 3, 4, 5];
        let tar = 0;
        let res = left_bound(&nums, tar);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_case4() {
        let nums = vec![1, 1, 3, 4, 5];
        let tar = 6;
        let res = left_bound(&nums, tar);
        assert_eq!(res, -1);
    }
}
