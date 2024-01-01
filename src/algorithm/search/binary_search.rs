pub fn binary_search(nums: &[i32], tar: i32) -> Option<i32> {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == tar {
            return Some(mid as i32);
        } else if nums[mid] < tar {
            left = mid + 1;
        } else if nums[mid] > tar {
            right = mid - 1;
        }
    }

    None
}

pub fn search_left_bound(_nums: &[i32], _tar: i32) -> Option<i32> {
    None
}

pub fn search_right_bound(_nums: &[i32], _tar: i32) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod binary_search_tests {
        use super::binary_search;

        #[test]
        fn test_ok() {
            let nums = vec![1, 2, 3, 4, 5];
            let tar = 5;
            let res = binary_search(&nums, tar);
            assert_eq!(res, Some(4));
        }

        #[test]
        fn test_ok_no_tgt_num() {
            let nums = vec![1, 2, 3, 4, 5];
            let tar = 6;
            let res = binary_search(&nums, tar);
            assert_eq!(res, None);
        }
    }
}
