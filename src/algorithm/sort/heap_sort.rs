#[allow(unused)]
pub fn heap_sort<T: PartialOrd + Copy>(vals: &mut [T]) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut vals = vec![3, 1, 2, 4];

        heap_sort(&mut vals[..]);
        assert_eq!(vec![1, 2, 3, 4], vals);
    }

    #[test]
    fn test_case2() {
        let mut vals = vec![2, 7, 11, 15];

        heap_sort(&mut vals[..]);
        assert_eq!(vec![2, 7, 11, 15], vals);
    }
}
