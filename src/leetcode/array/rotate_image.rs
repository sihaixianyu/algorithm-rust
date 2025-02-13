pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();

        for i in 0..(len / 2) {
            for j in 0..(len + 1) / 2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[len - 1 - j][i];
                matrix[len - 1 - j][i] = matrix[len - 1 - i][len - 1 - j];
                matrix[len - 1 - i][len - 1 - j] = matrix[j][len - 1 - i];
                matrix[j][len - 1 - i] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let mut nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let exp = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution::rotate(&mut nums);
        assert_eq!(nums, exp);
    }
}
