pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut rows = matrix.as_slice();
        let mut column_start = 0;
        let mut column_end = rows.first().map_or(0, Vec::len);

        // Right.

        while let Some((first_row, rest_rows)) = rows.split_first() {
            result.extend(&first_row[column_start..column_end]);

            // Down.

            if column_start == column_end {
                break;
            }

            column_end -= 1;

            result.extend(rest_rows.iter().map(|row| row[column_end]));

            // Left

            if let Some((last_row, rest_rows)) = rest_rows.split_last() {
                result.extend(last_row[column_start..column_end].iter().rev());

                // Up.

                if column_start == column_end {
                    break;
                }

                result.extend(rest_rows.iter().rev().map(|row| row[column_start]));

                column_start += 1;
                rows = rest_rows;
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        Self::spiral_order(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
