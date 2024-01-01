// 岛屿数量 => https://leetcode.cn/problems/number-of-islands/description/

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn sink(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() {
            return;
        }
        if j >= grid[0].len() {
            return;
        }
        if grid[i][j] == '0' {
            return;
        }
        grid[i][j] = '0';
        sink(grid, i + 1, j);
        sink(grid, i, j + 1);
        if i >= 1 {
            sink(grid, i - 1, j);
        }
        if j >= 1 {
            sink(grid, i, j - 1);
        }
    }

    let mut mut_grid = grid;
    let mut ans = 0;

    for i in 0..mut_grid.len() {
        for j in 0..mut_grid[0].len() {
            if mut_grid[i][j] == '0' {
                continue;
            }
            sink(&mut mut_grid, i, j);
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::num_islands;

    #[test]
    fn test_case1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let ans = num_islands(grid);
        assert_eq!(ans, 1)
    }

    #[test]
    fn test_case2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let ans = num_islands(grid);
        assert_eq!(ans, 3)
    }
}
