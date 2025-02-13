pub struct Solution;

impl Solution {
    /// @Prob: Search Matrix
    /// @Link: https://leetcode.cn/problems/search-a-2d-matrix-ii/description/?envType=study-plan-v2&envId=top-100-liked
    /// @Tags: ["Array", "Matrix"]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let max_row = (matrix.len() - 1) as isize;
        let max_col = (matrix[0].len() - 1) as isize;

        let (mut row, mut col) = (0, max_col);

        while row <= max_row && col >= 0 {
            let cur = matrix[row as usize][col as usize];

            if target == cur {
                return true;
            } else if target > cur {
                row += 1;
            } else if target < cur {
                col -= 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let exp = true;

        let res = Solution::search_matrix(matrix, 5);
        assert_eq!(res, exp);
    }
}
