// 搜索旋转数组：
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lp, mut rp) = (0 as usize, nums.len() - 1);

    if target == nums[0] {
        return 0;
    }

    while lp <= rp {
        let mid = lp + (rp - lp) / 2;

        if target == nums[mid] {
            return mid as i32;
        } else if target < nums[mid] {
            if nums[mid] < nums[0] {
                rp = mid - 1;
            } else {
                if target < nums[0] {
                    lp = mid + 1;
                } else if target > nums[0] {
                    rp = mid - 1;
                }
            }
        } else if target > nums[mid] {
            if nums[mid] < nums[0] {
                if target < nums[0] {
                    lp = mid + 1;
                } else if target > nums[0] {
                    rp = mid - 1;
                }
            } else {
                lp = mid + 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn test_case1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let res = search(nums, target);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_case2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let res = search(nums, target);
        assert_eq!(res, -1);
    }
}
