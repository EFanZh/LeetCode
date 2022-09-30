pub mod brute_force;

pub trait Solution {
    fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]] as &dyn Matrix<_>,
                &[[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]] as &dyn Matrix<_>,
            ),
            (
                &[
                    [11, 25, 66, 1, 69, 7],
                    [23, 55, 17, 45, 15, 52],
                    [75, 31, 36, 44, 58, 8],
                    [22, 27, 33, 25, 68, 4],
                    [84, 28, 14, 11, 5, 50],
                ],
                &[
                    [5, 17, 4, 1, 52, 7],
                    [11, 11, 25, 45, 8, 69],
                    [14, 23, 25, 44, 58, 15],
                    [22, 27, 31, 36, 50, 66],
                    [84, 28, 75, 33, 55, 68],
                ],
            ),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::diagonal_sort(mat.to_vec()), expected);
        }
    }
}
