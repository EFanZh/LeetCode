pub mod bfs;

pub trait Solution {
    fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]] as &dyn Matrix<_>,
                    1,
                ),
                true,
            ),
            (
                (
                    &[
                        [0, 1, 1, 0, 0, 0],
                        [1, 0, 1, 0, 0, 0],
                        [0, 1, 1, 1, 0, 1],
                        [0, 0, 1, 0, 1, 0],
                    ],
                    3,
                ),
                false,
            ),
            ((&[[1, 1, 1], [1, 0, 1], [1, 1, 1]], 5), true),
        ];

        for ((grid, health), expected) in test_cases {
            assert_eq!(S::find_safe_walk(grid.to_vec(), health), expected);
        }
    }
}
