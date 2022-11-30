pub mod cumulative_sum;

pub trait Solution {
    fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 1, 3, 2, 4, 3, 2], [1, 1, 3, 2, 4, 3, 2], [1, 1, 3, 2, 4, 3, 2]] as &dyn Matrix<_>,
                    4,
                ),
                2,
            ),
            (
                (
                    &[
                        [2, 2, 2, 2, 2],
                        [2, 2, 2, 2, 2],
                        [2, 2, 2, 2, 2],
                        [2, 2, 2, 2, 2],
                        [2, 2, 2, 2, 2],
                    ],
                    1,
                ),
                0,
            ),
        ];

        for ((mat, threshold), expected) in test_cases {
            assert_eq!(S::max_side_length(mat.to_vec(), threshold), expected);
        }
    }
}
