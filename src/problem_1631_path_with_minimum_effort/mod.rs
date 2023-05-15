pub mod dijkstra;

pub trait Solution {
    fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 2], [3, 8, 2], [5, 3, 5]] as &dyn Matrix<_>, 2),
            (&[[1, 2, 3], [3, 8, 4], [5, 3, 5]], 1),
            (
                &[
                    [1, 2, 1, 1, 1],
                    [1, 2, 1, 2, 1],
                    [1, 2, 1, 2, 1],
                    [1, 2, 1, 2, 1],
                    [1, 1, 1, 2, 1],
                ],
                0,
            ),
            (
                &[
                    [1, 5, 5, 9, 10, 10, 1, 7],
                    [3, 5, 6, 6, 9, 9, 4, 8],
                    [10, 5, 10, 9, 2, 2, 8, 6],
                    [3, 9, 5, 3, 6, 8, 6, 5],
                ],
                4,
            ),
        ];

        for (heights, expected) in test_cases {
            assert_eq!(S::minimum_effort_path(heights.to_vec()), expected);
        }
    }
}
