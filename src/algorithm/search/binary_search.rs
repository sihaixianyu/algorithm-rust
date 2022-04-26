pub fn binary_search<T: Ord>(vals: &[T], tar: T) -> Option<usize> {
    let (mut lp, mut rp) = (0 as usize, vals.len());

    while lp < rp {
        let mid = lp + (rp - lp) / 2;

        if tar == vals[mid] {
            return Some(mid);
        } else if tar < vals[mid] {
            rp = mid;
        } else {
            lp = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let vals = vec![1, 2, 3, 4, 5, 6, 7];

        let res = binary_search(&vals, 4);
        assert_eq!(3, res.unwrap());
    }

    #[test]
    fn test_case2() {
        let vals = vec![1, 3, 5, 7];

        let res = binary_search(&vals, 5);
        assert_eq!(2, res.unwrap());
    }
}
