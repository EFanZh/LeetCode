pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut column_diffs = vec![0; columns].into_boxed_slice();

        column_diffs
            .iter_mut()
            .enumerate()
            .for_each(|(i, diff)| *diff = grid.iter().map(|row| row[i]).sum::<i32>() * 2 - rows as i32);

        let mut result = grid;

        for row in &mut result {
            let row_diff = row.iter().sum::<i32>() * 2 - columns as i32;

            row.iter_mut()
                .zip(&*column_diffs)
                .for_each(|(cell, column_diff)| *cell = row_diff + column_diff);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::ones_minus_zeros(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
