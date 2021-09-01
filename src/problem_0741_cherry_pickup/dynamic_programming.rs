pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let first = grid[0][0];

        if first == -1 {
            return 0;
        }

        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut cache = vec![0; rows * rows];

        cache[0] = first;

        for step in 1..rows + columns - 1 {
            let max_y = step.min(rows - 1);
            let mut connected = false;

            for y_1 in (step.saturating_sub(columns - 1)..=max_y).rev() {
                let x_1 = step - y_1;
                let value_1 = grid[y_1][x_1];

                if value_1 == -1 {
                    for x in &mut cache[rows * y_1 + y_1..=rows * y_1 + max_y] {
                        *x = -1;
                    }
                } else {
                    // y_1 < y_2.

                    for y_2 in (y_1 + 1..=max_y).rev() {
                        let x_2 = step - y_2;
                        let value_2 = grid[y_2][x_2];
                        let mut max_count = -1;

                        if value_2 != -1 {
                            for &(prev_y_1, prev_y_2) in &[
                                (y_1.wrapping_sub(1), y_2.wrapping_sub(1)),
                                (y_1.wrapping_sub(1), y_2),
                                (y_1, y_2.wrapping_sub(1)),
                                (y_1, y_2),
                            ] {
                                if prev_y_1 < rows && prev_y_2 < rows.min(step) {
                                    max_count = max_count.max(cache[rows * prev_y_1 + prev_y_2]);
                                }
                            }

                            if max_count != -1 {
                                max_count += value_1 + value_2;
                            }
                        }

                        cache[rows * y_1 + y_2] = max_count;
                        connected |= max_count != -1;
                    }

                    // y_1 == y_2.

                    {
                        let mut max_count = -1;

                        for &(prev_y_1, prev_y_2) in &[
                            (y_1.wrapping_sub(1), y_1.wrapping_sub(1)),
                            (y_1.wrapping_sub(1), y_1),
                            (y_1, y_1),
                        ] {
                            if prev_y_1 < rows && prev_y_2 < rows.min(step) {
                                max_count = max_count.max(cache[rows * prev_y_1 + prev_y_2]);
                            }
                        }

                        if max_count != -1 {
                            max_count += value_1;
                        }

                        cache[rows * y_1 + y_1] = max_count;
                        connected |= max_count != -1;
                    }
                }
            }

            if !connected {
                return 0;
            }
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        Self::cherry_pickup(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
