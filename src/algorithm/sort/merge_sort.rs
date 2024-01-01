pub fn merge_sort(nums: &mut [i32]) {
    sort(nums, 0, nums.len() - 1);
}

pub fn sort(nums: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid = left + (right - left) / 2;

    sort(nums, left, mid);
    sort(nums, mid + 1, right);
    merge(nums, left, mid, right);
}

fn merge(nums: &mut [i32], left: usize, mid: usize, right: usize) {
    let (mut lp, mut rp) = (left, mid + 1);
    let mut tmp = vec![];

    while lp <= mid && rp <= right {
        if nums[lp] <= nums[rp] {
            tmp.push(nums[lp]);
            lp += 1;
        } else {
            tmp.push(nums[rp]);
            rp += 1;
        }
    }

    if lp == mid + 1 {
        tmp.extend_from_slice(&nums[rp..=right])
    } else {
        tmp.extend_from_slice(&nums[lp..=mid])
    }

    nums[left..=right].copy_from_slice(&tmp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 4, 2, 5, 1];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![5, 4, 3, 2, 1];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case3() {
        let mut nums = vec![1, 2, 3, 4];
        merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }
}
