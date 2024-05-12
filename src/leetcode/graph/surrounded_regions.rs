pub struct Solution;

impl Solution {
    const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            if board[i][0] == 'O' {
                Solution::dfs((i, 0), board);
            }
            if board[i][n - 1] == 'O' {
                Solution::dfs((i, n - 1), board);
            }
        }

        for j in 0..n {
            if board[0][j] == 'O' {
                Solution::dfs((0, j), board);
            }
            if board[m - 1][j] == 'O' {
                Solution::dfs((m - 1, j), board);
            }
        }

        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'O' => board[i][j] = 'X',
                    'A' => board[i][j] = 'O',
                    _ => continue,
                }
            }
        }
    }

    fn dfs(loc: (usize, usize), board: &mut Vec<Vec<char>>) {
        let m = board.len() as isize;
        let n = board[0].len() as isize;
        board[loc.0][loc.1] = 'A';

        let (cur_x, cur_y) = (loc.0 as isize, loc.1 as isize);
        for (mov_x, mov_y) in Solution::DIRECTIONS {
            let (next_x, next_y) = (cur_x + mov_x, cur_y + mov_y);
            if next_x < 0 || next_x >= m || next_y < 0 || next_y >= n {
                continue;
            }
            match board[next_x as usize][next_y as usize] {
                'O' => Solution::dfs((next_x as usize, next_y as usize), board),
                _ => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        Solution::solve(&mut board);
        assert_eq!(board, expected)
    }

    #[test]
    fn test_case_1() {
        let mut board = vec![vec!['X']];
        let expected = vec![vec!['X']];

        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }
}
