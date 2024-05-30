pub mod dynamic_programming;

pub trait Solution {
    fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 13), 0),
            ((&[[1], [2], [3]], 100), 94),
            ((&[[1, 2, 9, 8, 7]], 6), 1),
            (
                (
                    &[
                        [10, 3, 7, 7, 9, 6, 9, 8, 9, 5],
                        [1, 1, 6, 8, 6, 7, 7, 9, 3, 9],
                        [3, 4, 4, 1, 3, 6, 3, 3, 9, 9],
                        [6, 9, 9, 3, 8, 7, 9, 6, 10, 6],
                    ],
                    5,
                ),
                3,
            ),
            ((&[[15, 15], [5, 15], [2, 15]], 29), 3),
        ];

        for ((mat, target), expected) in test_cases {
            assert_eq!(S::minimize_the_difference(mat.to_vec(), target), expected);
        }
    }
}
