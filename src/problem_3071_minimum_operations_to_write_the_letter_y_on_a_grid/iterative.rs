pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut counts = [0_u32; 2];

        for row in &grid {
            for &value in row {
                if let Some(count) = counts.get_mut(value.cast_unsigned() as usize) {
                    *count += 1;
                }
            }
        }

        let mut y_counts = [0_u32; 2];
        let (top, bottom) = grid.split_at(n / 2);

        for (y, row) in top.iter().enumerate() {
            for x in [y, n - 1 - y] {
                if let Some(count) = y_counts.get_mut(row[x].cast_unsigned() as usize) {
                    *count += 1;
                }
            }
        }

        for row in bottom {
            if let Some(count) = y_counts.get_mut(row[n / 2].cast_unsigned() as usize) {
                *count += 1;
            }
        }

        let n_u32 = n as u32;
        let n_squared = n_u32.pow(2);
        let [count_0, count_1] = counts;
        let count_2 = n_squared - count_0 - count_1;
        let [y_count_0, y_count_1] = y_counts;
        let y_count_2 = n_u32 + n_u32 / 2 - y_count_0 - y_count_1;
        let x_count_0 = count_0 - y_count_0;
        let x_count_1 = count_1 - y_count_1;
        let x_count_2 = count_2 - y_count_2;

        (n_squared
            - (x_count_0 + y_count_1.max(y_count_2))
                .max(x_count_1 + y_count_0.max(y_count_2))
                .max(x_count_2 + y_count_0.max(y_count_1)))
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        Self::minimum_operations_to_write_y(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
