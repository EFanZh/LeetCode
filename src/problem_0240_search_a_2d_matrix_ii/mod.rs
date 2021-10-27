pub mod eliminate_row_or_column;
pub mod eliminate_row_or_column_2;

pub trait Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        [1, 4, 7, 11, 15],
                        [2, 5, 8, 12, 19],
                        [3, 6, 9, 16, 22],
                        [10, 13, 14, 17, 24],
                        [18, 21, 23, 26, 30],
                    ] as &dyn Matrix<_>,
                    5,
                ),
                true,
            ),
            (
                (
                    &[
                        [1, 4, 7, 11, 15],
                        [2, 5, 8, 12, 19],
                        [3, 6, 9, 16, 22],
                        [10, 13, 14, 17, 24],
                        [18, 21, 23, 26, 30],
                    ],
                    20,
                ),
                false,
            ),
            ((&[[-5]], -10), false),
        ];

        for ((matrix, target), expected) in test_cases {
            assert_eq!(S::search_matrix(matrix.to_vec(), target), expected);
        }
    }
}
