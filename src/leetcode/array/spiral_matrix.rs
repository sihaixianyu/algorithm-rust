pub struct Solution;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    /// @Prob: Spiral Matrix
    /// @Link: https://leetcode.cn/problems/spiral-matrix/submissions/592496213/?envType=study-plan-v2&envId=top-100-liked
    /// @Tags: ["Array", "Simulation"]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mat = matrix;
        let (m, n) = (mat.len(), mat[0].len());
        let mut res = Vec::with_capacity(m * n);

        let mut idx = 0;
        let (mut x, mut y) = (0, 0);
        for _ in 0..m * n {
            res.push(mat[x as usize][y as usize]);
            mat[x as usize][y as usize] = i32::MAX;

            let mut dir = DIRECTIONS[idx % 4];
            let next_x = x + dir.0;
            let next_y = y + dir.1;

            if next_x < 0
                || next_x as usize >= m
                || next_y < 0
                || next_y as usize >= n
                || mat[next_x as usize][next_y as usize] == i32::MAX
            {
                idx += 1;
                dir = DIRECTIONS[idx % 4];
            }

            x = x + dir.0;
            y = y + dir.1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let exp = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];

        let res = Solution::spiral_order(matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_1() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let exp = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];

        let res = Solution::spiral_order(matrix);
        assert_eq!(res, exp);
    }
}
