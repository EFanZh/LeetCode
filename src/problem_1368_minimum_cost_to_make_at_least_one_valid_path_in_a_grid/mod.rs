pub mod bfs;

pub trait Solution {
    fn min_cost(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 1, 1, 1], [2, 2, 2, 2], [1, 1, 1, 1], [2, 2, 2, 2]] as &dyn Matrix<_>,
                3,
            ),
            (&[[1, 1, 3], [3, 2, 2], [1, 1, 4]], 0),
            (&[[1, 2], [4, 3]], 1),
            (&[[4]], 0),
            (
                &[
                    [3, 4, 3],
                    [2, 2, 2],
                    [2, 1, 1],
                    [4, 3, 2],
                    [2, 1, 4],
                    [2, 4, 1],
                    [3, 3, 3],
                    [1, 4, 2],
                    [2, 2, 1],
                    [2, 1, 1],
                    [3, 3, 1],
                    [4, 1, 4],
                    [2, 1, 4],
                    [3, 2, 2],
                    [3, 3, 1],
                    [4, 4, 1],
                    [1, 2, 2],
                    [1, 1, 1],
                    [1, 3, 4],
                    [1, 2, 1],
                    [2, 2, 4],
                    [2, 1, 3],
                    [1, 2, 1],
                    [4, 3, 2],
                    [3, 3, 4],
                    [2, 2, 1],
                    [3, 4, 3],
                    [4, 2, 3],
                    [4, 4, 4],
                ],
                18,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::min_cost(grid.to_vec()), expected);
        }
    }
}
