pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        // Forward pass.
        {
            let (first_row, rest_rows) = grid.split_first_mut().unwrap();
            let mut left = i32::MAX / 2;

            for value in &mut *first_row {
                left = if *value == 0 { left + 1 } else { 0 };
                *value = left;
            }

            let mut prev_row = first_row.as_slice();

            for row in rest_rows {
                let mut left = i32::MAX / 2;

                for (value, &top) in row.iter_mut().zip(prev_row) {
                    left = if *value == 0 { left.min(top) + 1 } else { 0 };
                    *value = left;
                }

                prev_row = row;
            }
        }

        let mut result = 0;

        // Backward pass.
        {
            let (last_row, rest_rows) = grid.split_last_mut().unwrap();
            let mut right = i32::MAX / 2;

            for value in last_row.iter_mut().rev() {
                right = (*value).min(right + 1);
                *value = right;
                result = result.max(right);
            }

            let mut prev_row = last_row.as_slice();

            for row in rest_rows.iter_mut().rev() {
                let mut right = i32::MAX / 2;

                for (value, &bottom) in row.iter_mut().zip(prev_row).rev() {
                    right = (*value).min(right.min(bottom) + 1);
                    *value = right;
                    result = result.max(right);
                }

                prev_row = row;
            }
        }

        if result == 0 || result > i32::MAX / 2 {
            -1
        } else {
            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_distance(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
