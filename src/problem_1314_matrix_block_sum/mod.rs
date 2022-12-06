pub mod cumulative_sum;

pub trait Solution {
    fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 1),
                &[[12, 21, 16], [27, 45, 33], [24, 39, 28]] as &dyn Matrix<_>,
            ),
            (
                (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]], 2),
                &[[45, 45, 45], [45, 45, 45], [45, 45, 45]],
            ),
        ];

        for ((mat, k), expected) in test_cases {
            assert_eq!(S::matrix_block_sum(mat.to_vec(), k), expected);
        }
    }
}
