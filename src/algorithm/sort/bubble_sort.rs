pub fn bubble_sort<T: Ord>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len()-1-i {
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        bubble_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vec: Vec<i32> = vec![];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}