pub mod binary_heap;
pub mod quick_select;

pub trait Solution {
    fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32>;
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
                        [1, 1, 0, 0, 0],
                        [1, 1, 1, 1, 0],
                        [1, 0, 0, 0, 0],
                        [1, 1, 0, 0, 0],
                        [1, 1, 1, 1, 1],
                    ] as &dyn Matrix<_>,
                    3,
                ),
                &[2, 0, 3] as &[_],
            ),
            ((&[[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]], 2), &[0, 2]),
            (
                (
                    &[
                        [1, 0, 0],
                        [1, 0, 0],
                        [0, 0, 0],
                        [1, 0, 0],
                        [1, 0, 0],
                        [1, 1, 1],
                        [1, 1, 0],
                    ],
                    7,
                ),
                &[2, 0, 1, 3, 4, 6, 5],
            ),
        ];

        for ((mat, k), expected) in test_cases {
            assert_eq!(S::k_weakest_rows(mat.to_vec(), k), expected);
        }
    }
}
