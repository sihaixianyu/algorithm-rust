pub fn heap_sort<T: Ord + Copy>(vals: &mut [T]) {
    if vals.len() == 0 {
        return
    }

    for i in (0..=(vals.len() - 1) / 2).rev() {
        heap_down(vals, i, vals.len())
    }

    for i in (1..vals.len()).rev() {
        vals.swap(0, i);
        heap_down(vals, 0, i)
    }
}

fn heap_down<T: Ord + Copy>(vals: &mut [T], mut root: usize, end: usize) {
    while root * 2 + 1 < end {
        let mut child = root * 2 + 1;
        if child + 1 < end && vals[child] < vals[child + 1] {
            child = child + 1
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
    fn test_basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        heap_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vec: Vec<i32> = vec![];
        heap_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        heap_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        heap_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
