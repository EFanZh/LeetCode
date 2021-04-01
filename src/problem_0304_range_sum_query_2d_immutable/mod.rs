pub mod cumulative_sum;

pub trait NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self;
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::NumMatrix;

    pub fn run<N: NumMatrix>() {
        let test_cases = [
            (
                &[
                    &[3, 0, 1, 4, 2] as &[_],
                    &[5, 6, 3, 2, 1],
                    &[1, 2, 0, 1, 5],
                    &[4, 1, 0, 1, 7],
                    &[1, 0, 3, 0, 5],
                ] as &[&[_]],
                &[((2, 1, 4, 3), 8), ((1, 1, 2, 2), 11), ((1, 2, 2, 4), 12)] as &[_],
            ),
            (
                &[&[1], &[-7]],
                &[((0, 0, 0, 0), 1), ((1, 0, 1, 0), -7), ((0, 0, 1, 0), -6)],
            ),
        ];

        for (matrix, sums) in test_cases.iter().copied() {
            let num_matrix = N::new(matrix.iter().copied().map(<[_]>::to_vec).collect());

            for ((row1, col1, row2, col2), expected) in sums.iter().copied() {
                assert_eq!(num_matrix.sum_region(row1, col1, row2, col2), expected);
            }
        }
    }
}
