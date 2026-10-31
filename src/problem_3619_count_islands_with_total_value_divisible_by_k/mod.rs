pub mod dfs;

pub trait Solution {
    fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32;
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
                        [0, 2, 1, 0, 0],
                        [0, 5, 0, 0, 5],
                        [0, 0, 1, 0, 0],
                        [0, 1, 4, 7, 0],
                        [0, 2, 0, 0, 8],
                    ] as &dyn Matrix<_>,
                    5,
                ),
                2,
            ),
            ((&[[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]], 3), 6),
            (
                (
                    &[[0, 0, 0], [0, 0, 1], [11, 0, 6], [0, 10, 2], [0, 0, 0], [8, 0, 0]],
                    19,
                ),
                1,
            ),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::count_islands(grid.to_vec(), k), expected);
        }
    }
}
