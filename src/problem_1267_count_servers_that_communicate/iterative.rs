pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut columns = vec![0_u8; grid.len() + grid.first().map_or(0, Vec::len)];
        let (row_counts, column_counts) = columns.split_at_mut(grid.len());

        for (row_count, row) in row_counts.iter_mut().zip(&grid) {
            for (column_count, &cell) in column_counts.iter_mut().zip(row) {
                if cell != 0 {
                    *row_count += 1;
                    *column_count += 1;
                }
            }
        }

        let mut result = 0;

        for (&row_count, row) in row_counts.iter().zip(&grid) {
            for (&column_count, &cell) in column_counts.iter().zip(row) {
                if cell != 0 && (row_count > 1 || column_count > 1) {
                    result += 1;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        Self::count_servers(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
