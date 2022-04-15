pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = Vec::new();
    let mut board = vec![vec!['.'; n as usize]; n as usize];

    backtrack(&mut res, &mut board, 0);

    res
}

fn backtrack(res: &mut Vec<Vec<String>>, board: &mut Vec<Vec<char>>, row: usize) {
    let n = board.len();

    if row == n {
        let solution = board.iter().map(|v| v.into_iter().collect()).collect();
        res.push(solution);
        return;
    }

    for col in 0..n {
        if !is_valid(board, row, col) {
            continue;
        }

        board[row][col] = 'Q';

        backtrack(res, board, row + 1);

        board[row][col] = '.';
    }
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let n = board.len() as i32;

    let mut i = row as i32;
    while i >= 0 {
        if board[i as usize][col] == 'Q' {
            return false;
        }
        i -= 1;
    }

    let (mut i, mut j) = (row as i32, col as i32);
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    let (mut i, mut j) = (row as i32, col as i32);
    while i >= 0 && j < n {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_case1() {
        let n = 4;

        let res = solve_n_queens(n);
        res.into_iter().for_each(|v| println!("{:?}", v));
    }

    #[test]
    pub fn test_case2() {
        let n = 1;

        let res = solve_n_queens(n);
        res.into_iter().for_each(|v| println!("{:?}", v));
    }

    #[test]
    pub fn test_case3() {
        let n = 5;

        let res = solve_n_queens(n);
        res.into_iter().for_each(|v| println!("{:?}", v));
    }
}
