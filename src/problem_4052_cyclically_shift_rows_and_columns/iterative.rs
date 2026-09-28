pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    pub fn cyclic_shift(n: i32, grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>) -> Vec<Vec<i32>> {
        let n = n.cast_unsigned() as usize;
        let mut grid = grid;

        grid.iter_mut()
            .zip(row_shift)
            .for_each(|(row, shift)| row.rotate_left(shift.cast_unsigned() as _));

        for (column, shift) in col_shift.into_iter().enumerate() {
            let shift = shift.cast_unsigned() as usize;

            for start_row in 0..Self::gcd(shift, n) {
                let mut row = start_row;
                let saved = grid[start_row][column];

                loop {
                    let mut next_row = row + shift;

                    if next_row >= n {
                        next_row -= n;
                    }

                    if next_row == start_row {
                        break;
                    }

                    grid[row][column] = grid[next_row][column];
                    row = next_row;
                }

                grid[row][column] = saved;
            }
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cyclic_shift(n: i32, grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>) -> Vec<Vec<i32>> {
        Self::cyclic_shift(n, grid, row_shift, col_shift)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
