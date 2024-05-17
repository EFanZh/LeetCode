pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32;
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
                        [1, 1, 1, 0, 0],
                        [0, 1, 1, 1, 1],
                        [0, 0, 0, 0, 0],
                        [1, 0, 0, 0, 0],
                        [1, 1, 0, 1, 1],
                    ] as &dyn Matrix<_>,
                    &[
                        [1, 1, 1, 0, 0],
                        [0, 0, 1, 1, 1],
                        [0, 1, 0, 0, 0],
                        [1, 0, 1, 1, 0],
                        [0, 1, 0, 1, 0],
                    ] as &dyn Matrix<_>,
                ),
                3,
            ),
            (
                (
                    &[
                        [1, 0, 1, 0, 1],
                        [1, 1, 1, 1, 1],
                        [0, 0, 0, 0, 0],
                        [1, 1, 1, 1, 1],
                        [1, 0, 1, 0, 1],
                    ],
                    &[
                        [0, 0, 0, 0, 0],
                        [1, 1, 1, 1, 1],
                        [0, 1, 0, 1, 0],
                        [0, 1, 0, 1, 0],
                        [1, 0, 0, 0, 1],
                    ],
                ),
                2,
            ),
        ];

        for ((grid1, grid2), expected) in test_cases {
            assert_eq!(S::count_sub_islands(grid1.to_vec(), grid2.to_vec()), expected);
        }
    }
}
