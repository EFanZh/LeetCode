pub mod iterative;

pub trait Solution {
    fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 8] as &[_], &[4, 7] as &[_]), (&[5, 7, 10], &[8, 6, 8])];

        for (row_sum, col_sum) in test_cases {
            let result = S::restore_matrix(row_sum.to_vec(), col_sum.to_vec());

            assert!(
                result
                    .iter()
                    .map(|row| {
                        assert_eq!(row.len(), col_sum.len());

                        row.iter().sum::<i32>()
                    })
                    .eq(row_sum.iter().copied())
            );

            assert!(
                col_sum
                    .iter()
                    .enumerate()
                    .all(|(i, &sum)| { result.iter().map(|row| row[i]).sum::<i32>() == sum })
            );
        }
    }
}
