pub fn merge_sort<T: Ord + Copy>(vals: &mut [T]) {
    if vals.len() == 0 {
        return;
    }

    let len = vals.len();
    sort(&mut vals[..], 0, len - 1);
}

fn sort<T: Ord + Copy>(vals: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mid = (hi - lo) / 2 + lo;
    sort(vals, lo, mid);
    sort(vals, mid + 1, hi);
    merge(vals, lo, mid, hi);
}

fn merge<T: Ord + Copy>(vals: &mut [T], lo: usize, mid: usize, hi: usize) {
    let left = vals[lo..mid + 1].to_vec();
    let right = vals[mid + 1..hi + 1].to_vec();
    let (mut lp, mut rp) = (0, 0);

    for v in &mut vals[lo..hi + 1] {
        if rp == right.len() || (lp < left.len() && left[lp] < right[rp]) {
            *v = left[lp];
            lp += 1;
        } else {
            *v = right[rp];
            rp += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vals = vec![3, 5, 6, 3, 1, 4];

        merge_sort(&mut vals);
        for i in 0..vals.len()-1 {
            assert!(vals[i] <= vals[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vals: Vec<i32> = vec![];

        merge_sort(&mut vals);
        assert_eq!(vals, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vals = vec![6, 5, 4, 3, 2, 1];

        merge_sort(&mut vals);
        for i in 0..vals.len()-1 {
            assert!(vals[i] <= vals[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vals = vec![1, 2, 3, 4, 5, 6];

        merge_sort(&mut vals);
        for i in 0..vals.len()-1 {
            assert!(vals[i] <= vals[i + 1]);
        }
    }
}
