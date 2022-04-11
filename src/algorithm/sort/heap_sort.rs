pub fn heap_sort<T: Ord + Copy>(vals: &mut [T]) {
    for i in (0..=(vals.len() - 1) / 2).rev() {
        heap_down(vals, i, vals.len())
    }

    for i in (1..vals.len()).rev() {
        vals.swap(0, i);
        heap_down(vals, 0, i)
    }
}

fn heap_down<T: Ord + Copy>(vals: &mut [T], mut root: usize, n: usize) {
    while root * 2 + 1 < n {
        let mut child = root * 2 + 1;
        if child + 1 < n && vals[child] < vals[child + 1] {
            child = child + 1;
        }

        if vals[root] > vals[child] {
            break;
        }

        vals.swap(root, child);
        root = child;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut nums = vec![3, 1, 2, 4];

        heap_sort(&mut nums[..]);
        assert_eq!(vec![1, 2, 3, 4], nums);
    }

    #[test]
    fn test_case2() {
        let mut nums = vec![2, 7, 11, 15];

        heap_sort(&mut nums[..]);
        assert_eq!(vec![2, 7, 11, 15], nums);
    }
}
