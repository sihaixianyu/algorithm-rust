pub fn insert_sort<T: Ord + Copy>(vals: &mut [T]) {
    for i in 1..vals.len() {
        let ins_val = vals[i];
        let mut j = i;

        while j > 0 && vals[j - 1] > ins_val {
            vals[j] = vals[j - 1];
            j -= 1;
        }

        vals[j] = ins_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        insert_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vec: Vec<i32> = vec![];
        insert_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        insert_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        insert_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
