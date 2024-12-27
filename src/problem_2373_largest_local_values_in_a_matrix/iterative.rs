pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(single_use_lifetimes, reason = "false-positive")]
    fn conv_1d<'a>(mut iter: impl Iterator<Item = &'a mut i32>) {
        let mut prev_1 = iter.next().unwrap();
        let mut prev_2 = iter.next().unwrap();

        for value in iter {
            *prev_1 = (*prev_1).max(*prev_2).max(*value);
            prev_1 = prev_2;
            prev_2 = value;
        }
    }

    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let result_rows = grid.len() - 2;
        let result_columns = grid.first().map_or(0, Vec::len) - 2;

        for row in &mut grid {
            Self::conv_1d(row.iter_mut());

            row.truncate(result_columns);
        }

        for column in 0..result_columns {
            Self::conv_1d(grid.iter_mut().map(|row| &mut row[column]));
        }

        grid.truncate(result_rows);

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::largest_local(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
