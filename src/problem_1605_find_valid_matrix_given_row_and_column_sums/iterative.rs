pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];
        let mut row_iter = result.iter_mut().map(Vec::as_mut_slice).zip(row_sum.iter().copied());
        let mut column_iter = col_sum.iter().copied();
        let mut row = row_iter.next().unwrap();
        let mut column_sum = column_iter.next().unwrap();
        let mut x = 0;

        loop {
            if column_sum < row.1 {
                row.0[x] = column_sum;

                if let Some(next_column_sum) = column_iter.next() {
                    row.1 -= column_sum;
                    column_sum = next_column_sum;
                    x += 1;
                } else {
                    break;
                }
            } else {
                row.0[x] = row.1;

                if let Some(next_row) = row_iter.next() {
                    column_sum -= row.1;
                    row = next_row;
                } else {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        Self::restore_matrix(row_sum, col_sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
