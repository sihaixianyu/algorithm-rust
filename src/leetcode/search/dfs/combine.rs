pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut path = vec![];
    let mut res = vec![];

    backtrack(1, n, k, &mut path, &mut res);

    res
}

fn backtrack(cur: i32, n: i32, k: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if path.len() == k as usize {
        res.push(Vec::from(&path[..]));
        return;
    }

    for num in cur..=n {
        path.push(num);
        
        backtrack(num+1, n, k, path, res);
        
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let n = 4;
        let k = 3;
        let res = combine(n, k);
        assert_eq!(
            res,
            vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4],]
        )
    }

    #[test]
    fn test_case2() {
        let n = 3;
        let k = 2;
        let res = combine(n, k);
        assert_eq!(
            res,
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        )
    }
}
