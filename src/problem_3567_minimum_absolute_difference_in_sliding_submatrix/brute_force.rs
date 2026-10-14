pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        if k == 1 {
            let mut grid = grid;

            for row in &mut grid {
                row.fill(0);
            }

            grid
        } else {
            let k = k.cast_unsigned() as usize;
            let columns: usize = grid.first().map_or(0, Vec::len);
            let mut sort_buffer = Vec::<i32>::with_capacity(k * k);
            let result_columns = columns - k + 1;

            grid.windows(k)
                .map(|rows| {
                    (0..result_columns)
                        .map(|column| {
                            sort_buffer.extend(rows.iter().flat_map(|row| &row[column..column + k]));
                            sort_buffer.sort_unstable();

                            let mut iter = sort_buffer.iter().copied();
                            let mut prev = iter.next().unwrap();
                            let mut min_diff = u32::MAX;

                            for x in iter {
                                if x != prev {
                                    min_diff = min_diff.min(x.wrapping_sub(prev).cast_unsigned());
                                    prev = x;
                                }
                            }

                            sort_buffer.clear();

                            if min_diff == u32::MAX {
                                min_diff = 0;
                            }

                            min_diff.cast_signed()
                        })
                        .collect()
                })
                .collect()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::min_abs_diff(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
