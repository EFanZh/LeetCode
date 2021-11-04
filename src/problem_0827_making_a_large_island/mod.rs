pub mod bfs;

pub trait Solution {
    fn largest_island(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0], [0, 1]] as &dyn Matrix<_>, 3),
            (&[[1, 1], [1, 0]], 4),
            (&[[1, 1], [1, 1]], 4),
            (&[[0, 0], [0, 0]], 1),
            (
                &[
                    [0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0],
                    [0, 1, 0, 0, 1, 0, 0],
                    [1, 0, 1, 0, 1, 0, 0],
                    [0, 1, 0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 1, 0, 0],
                ],
                18,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::largest_island(grid.to_vec()), expected);
        }
    }
}
