pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_min_row = u32::MIN;
        let mut max_min_row_column = 0;

        for row in &matrix {
            let mut min_row = u32::MAX;
            let mut min_row_column = 0;

            for (i, &value) in row.iter().enumerate() {
                let value = value as _;

                if value < min_row {
                    min_row = value;
                    min_row_column = i;
                }
            }

            if min_row > max_min_row {
                max_min_row = min_row;
                max_min_row_column = min_row_column;
            }
        }

        if matrix.into_iter().any(|row| max_min_row < row[max_min_row_column] as _) {
            Vec::new()
        } else {
            vec![max_min_row as _]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        Self::lucky_numbers(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
