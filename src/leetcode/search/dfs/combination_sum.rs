pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut tmp = vec![];

    let mut candidates: Vec<i32> = candidates;
    candidates.sort();

    backtrack(0, &mut ans, &mut tmp, &candidates, target);

    ans
}

fn backtrack(
    start: usize,
    ans: &mut Vec<Vec<i32>>,
    tmp: &mut Vec<i32>,
    candidate: &Vec<i32>,
    rest: i32,
) {
    if rest == 0 {
        ans.push(tmp.clone());
        return;
    }

    for i in start..candidate.len() {
        let v = candidate[i];

        if v > rest {
            break;
        }

        tmp.push(candidate[i]);
        backtrack(i, ans, tmp, &candidate, rest - v);
        tmp.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::combination_sum;

    #[test]
    fn test_case1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;

        let ans = combination_sum(candidates, target);
        assert_eq!(ans, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_case2() {
        let candidates = vec![2, 3, 5];
        let target = 8;

        let ans = combination_sum(candidates, target);
        assert_eq!(ans, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
