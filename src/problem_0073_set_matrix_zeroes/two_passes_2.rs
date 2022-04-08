pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if let Some((first_row, rest_rows)) = matrix.split_first_mut() {
            let first_row_has_zero = first_row.iter().any(|x| *x == 0);

            for row in rest_rows.iter_mut() {
                let mut iter = row.iter_mut().zip(first_row.iter_mut());

                if let Some((first_cell, first_cache_cell)) = iter.next() {
                    if *first_cell == 0 {
                        *first_cache_cell = 0;

                        for (cell, cache_cell) in iter {
                            if *cell == 0 {
                                *cache_cell = 0;
                            }
                        }
                    } else {
                        for (cell, cache_cell) in iter {
                            if *cell == 0 {
                                *first_cell = 0;
                                *cache_cell = 0;
                            }
                        }
                    }
                }
            }

            for row in rest_rows {
                let mut iter = first_row.iter_mut().zip(row);

                if let Some((first_cache_cell, first_cell)) = iter.next() {
                    if *first_cell == 0 {
                        for (_, cell) in iter {
                            *cell = 0;
                        }
                    } else {
                        if *first_cache_cell == 0 {
                            *first_cell = 0;
                        }

                        for (cell_cache, cell) in iter {
                            if *cell_cache == 0 {
                                *cell = 0;
                            }
                        }
                    }
                }
            }

            if first_row_has_zero {
                first_row.fill(0);
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
