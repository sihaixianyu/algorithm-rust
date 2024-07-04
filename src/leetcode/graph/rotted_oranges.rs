use std::cmp;
use std::collections::VecDeque;

static DIRECTIONS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let (num_rows, num_cols) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut locs = VecDeque::new();

        for r in 0..num_rows {
            for c in 0..num_cols {
                if grid[r][c] == 2 {
                    locs.push_back(vec![r, c, 0]);
                }
            }
        }

        let mut elapsed_minutes = 0;

        while let Some(loc) = locs.pop_front() {
            let (r, c, depth) = (loc[0], loc[1], loc[2]);
            elapsed_minutes = cmp::max(elapsed_minutes, depth);
            for d in DIRECTIONS {
                let (nr, nc) = (r as i32 + d[0], c as i32 + d[1]);
                if nr < 0 || nc < 0 || nr >= num_rows as i32 || nc >= num_cols as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == 1 {
                    grid[nr][nc] = 2;
                    locs.push_back(vec![nr, nc, depth + 1])
                }
            }
        }

        if grid
            .iter()
            .flat_map(|row| row.iter())
            .any(|&status| status == 1)
        {
            return -1;
        }

        elapsed_minutes as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let elasped_minutes = Solution::oranges_rotting(grid);
        assert_eq!(elasped_minutes, 4);
    }

    #[test]
    fn test_case_1() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        let elasped_minutes = Solution::oranges_rotting(grid);
        assert_eq!(elasped_minutes, -1);
    }

    #[test]
    fn test_case_2() {
        let grid = vec![vec![0, 2]];
        let elasped_minutes = Solution::oranges_rotting(grid);
        assert_eq!(elasped_minutes, 0);
    }
}
