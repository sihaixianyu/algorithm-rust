pub fn quick_sort<T: PartialOrd>(vals: &mut [T], s: isize, e: isize) {
    if s >= e {
        return;
    }

    let pivot_idx = s as usize;
    let (mut lp, mut rp) = (s as usize, e as usize);
    while lp < rp {
        while lp < rp && vals[rp] >= vals[pivot_idx] {
            rp -= 1;
        }
        while lp < rp && vals[lp] <= vals[pivot_idx] {
            lp += 1
        }
        vals.swap(lp, rp);
    }
    vals.swap(s as usize, lp);

    quick_sort(vals, s, lp as isize - 1);
    quick_sort(vals, lp as isize + 1, e);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];
        let n = nums.len() as isize;

        quick_sort(nums.as_mut_slice(), 0, n - 1);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];
        let n = nums.len() as isize;

        quick_sort(nums.as_mut_slice(), 0, n - 1);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}
