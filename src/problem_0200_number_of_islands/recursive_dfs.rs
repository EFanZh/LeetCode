pub struct Solution {}

impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        for &(next_i, next_j) in &[(i.wrapping_sub(1), j), (i, j.wrapping_sub(1)), (i, j + 1), (i + 1, j)] {
            if let Some(cell @ '1') = grid.get_mut(next_i).and_then(|row| row.get_mut(next_j)) {
                *cell = 'X';

                Self::dfs(grid, next_i, next_j);
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);

        for i in 0..rows {
            for j in 0..columns {
                if let cell @ '1' = &mut grid[i][j] {
                    *cell = 'X';
                    result += 1;

                    Self::dfs(&mut grid, i, j);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Self::num_islands(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
