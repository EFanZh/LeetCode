pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let row_maxes = grid
            .iter()
            .map(|row| row.iter().copied().max().unwrap())
            .collect::<Vec<_>>();

        let column_maxes = (0..grid.len())
            .map(|i| grid.iter().map(|row| row[i]).max().unwrap())
            .collect::<Vec<_>>();

        let mut result = 0;

        for (row, row_max) in grid.iter().zip(&row_maxes) {
            for (height, column_max) in row.iter().zip(&column_maxes) {
                result += row_max.min(column_max) - height;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_increase_keeping_skyline(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
