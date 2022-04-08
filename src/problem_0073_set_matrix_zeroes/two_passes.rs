pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if let Some((first_row, rest_rows)) = matrix.split_first_mut() {
            if let Some((first_row_first_cell, first_row_rest_cells)) = first_row.split_first_mut() {
                let (first_row_has_zero, first_column_has_zero) = if *first_row_first_cell == 0 {
                    (true, true)
                } else {
                    (
                        first_row_rest_cells.iter().any(|x| *x == 0),
                        rest_rows.iter().any(|row| row[0] == 0),
                    )
                };

                for row in rest_rows.iter_mut() {
                    let (first_cell, rest_cells) = row.split_first_mut().unwrap();

                    for (cell, cache_cell) in rest_cells.iter().zip(first_row_rest_cells.iter_mut()) {
                        if *cell == 0 {
                            *cache_cell = 0;
                            *first_cell = 0;
                        }
                    }
                }

                for row in rest_rows {
                    let (first_cell, rest_cells) = row.split_first_mut().unwrap();

                    for (cell, cache_cell) in rest_cells.iter_mut().zip(first_row_rest_cells.iter()) {
                        if *cache_cell == 0 || *first_cell == 0 {
                            *cell = 0;
                        }
                    }
                }

                if first_row_has_zero {
                    first_row.fill(0);
                }

                if first_column_has_zero {
                    for row in matrix {
                        row[0] = 0;
                    }
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        Self::set_zeroes(matrix);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
