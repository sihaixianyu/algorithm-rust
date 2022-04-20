pub fn shell_sort<T: Ord + Copy>(vals: &mut [T]) {
    let mut gap = vals.len() / 2;
    while gap > 0 {
        for i in 0..(vals.len() - gap) {
            insert_sort(vals, i, gap)
        }
        gap /= 2;
    }
}

fn insert_sort<T: Ord + Copy>(vals: &mut [T], start: usize, gap: usize) {
    for i in ((start + gap)..vals.len()).step_by(gap) {
        let is_val = vals[i];
        let mut j = i;
        while j >= start + gap && is_val < vals[j - gap] {
            vals[j] = vals[j - gap];
            j -= gap;
        }
        vals[j] = is_val
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vec: Vec<i32> = vec![];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
