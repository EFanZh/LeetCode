pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)] // Expected.
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        matrix.reverse();

        for row_plus_1 in 1..n {
            let row = row_plus_1 - 1;

            for column in row_plus_1..n {
                let temp = matrix[row][column];

                matrix[row][column] = matrix[column][row];
                matrix[column][row] = temp;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::rotate(matrix);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
