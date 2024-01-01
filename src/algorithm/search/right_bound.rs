pub fn right_bound(nums: &[i32], tar: i32) -> i32 {
    let mut left = 0 as i32;
    let mut right = (nums.len()-1) as i32;

    while left <= right {
        let mid = (left + (right - left) / 2) as usize;

        if nums[mid] == tar {
            left = mid as i32 + 1;
        } else if nums[mid] < tar {
            left = mid as i32 + 1;
        } else if nums[mid] > tar {
            right = mid as i32 - 1;
        }
    }

    if right == -1 {
        return -1
    }

    if nums[right as usize] != tar {
        return -1
    }

    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let nums = vec![1, 2, 4, 4, 5];
        let tar = 4;
        let res = right_bound(&nums, tar);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case2() {
        let nums = vec![1, 1, 3, 4, 4];
        let tar = 4;
        let res = right_bound(&nums, tar);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_case3() {
        let nums = vec![1, 1, 3, 4, 5];
        let tar = 0;
        let res = right_bound(&nums, tar);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_case4() {
        let nums = vec![1, 1, 3, 4, 5];
        let tar = 6;
        let res = right_bound(&nums, tar);
        assert_eq!(res, -1);
    }
}
