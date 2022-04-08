pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(grid: &mut [Vec<char>], mut current: (usize, usize), stack: &mut Vec<(usize, usize)>) {
        loop {
            let (i, j) = current;

            for (next_i, next_j) in [(i.wrapping_sub(1), j), (i, j.wrapping_sub(1)), (i, j + 1), (i + 1, j)] {
                if let Some(cell @ '1') = grid.get_mut(next_i).and_then(|row| row.get_mut(next_j)) {
                    *cell = 'X';

                    stack.push((next_i, next_j));
                }
            }

            if let Some(next) = stack.pop() {
                current = next;
            } else {
                break;
            }
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut result = 0;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut stack = Vec::new();

        for i in 0..rows {
            for j in 0..columns {
                if let cell @ '1' = &mut grid[i][j] {
                    *cell = 'X';
                    result += 1;

                    Self::dfs(&mut grid, (i, j), &mut stack);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
