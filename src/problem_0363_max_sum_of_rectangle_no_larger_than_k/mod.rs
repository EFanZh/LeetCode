pub mod reduce_to_subarray_sum;

pub trait Solution {
    fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[&[1, 0, 1] as &[_], &[0, -2, 3]] as &[&[_]], 2), 2)];

        for ((matrix, k), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::max_sum_submatrix(matrix.iter().map(|row| row.to_vec()).collect(), k),
                expected
            );
        }
    }
}
