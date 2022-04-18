pub fn quick_sort<T: Ord + Copy>(vals: &mut [T]) {
    fn helper<T: Ord + Copy>(vals: &mut [T], s: isize, e: isize) {
        if s >= e {
            return;
        }

        let pivot = vals[s as usize];
        let (mut lp, mut rp) = (s as usize, e as usize);
        while lp < rp {
            while lp < rp && vals[rp] >= pivot {
                rp -= 1;
            }
            while lp < rp && vals[lp] <= pivot {
                lp += 1
            }
            vals.swap(lp, rp);
        }
        vals.swap(s as usize, lp);

        helper(vals, s, lp as isize - 1);
        helper(vals, lp as isize + 1, e);
    }

    if vals.len() == 0 {
        return
    }

    helper(vals, 0, (vals.len() - 1) as isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        quick_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_empty() {
        let mut vec: Vec<i32> = vec![];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_reversed() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        quick_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}
