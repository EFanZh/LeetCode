pub mod reduce_to_subarray_sum;

pub trait Solution {
    fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[&[1, 0, 1] as &[_], &[0, -2, 3]] as &[&[_]], 2), 2),
            ((&[&[5, -4, -3, 4], &[-3, -4, 4, 5], &[5, 1, 5, -4]], 10), 10),
            ((&[&[5, -3, 5], &[-4, -4, 1], &[-3, 4, 5], &[4, 5, 4]], 10), 10),
            ((&[&[2, 2, -1]], 3), 3),
        ];

        for ((matrix, k), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::max_sum_submatrix(matrix.iter().map(|row| row.to_vec()).collect(), k),
                expected
            );
        }
    }
}
