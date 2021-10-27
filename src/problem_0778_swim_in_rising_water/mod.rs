pub mod bfs;
pub mod bidirectional_bfs;

pub trait Solution {
    fn swim_in_water(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 2], [1, 3]] as &dyn Matrix<_>, 3),
            (
                &[
                    [0, 1, 2, 3, 4],
                    [24, 23, 22, 21, 5],
                    [12, 13, 14, 15, 16],
                    [11, 17, 18, 19, 20],
                    [10, 9, 8, 7, 6],
                ],
                16,
            ),
            (
                &[
                    [7, 23, 21, 9, 5],
                    [3, 20, 8, 18, 15],
                    [14, 13, 1, 0, 22],
                    [2, 10, 24, 17, 12],
                    [6, 16, 19, 4, 11],
                ],
                17,
            ),
            (
                &[
                    [3, 19, 34, 15, 7, 21],
                    [31, 25, 8, 0, 16, 27],
                    [4, 1, 13, 18, 28, 17],
                    [6, 2, 24, 23, 10, 35],
                    [20, 5, 22, 12, 32, 29],
                    [33, 11, 9, 14, 30, 26],
                ],
                30,
            ),
            (&[[7]], 7),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::swim_in_water(grid.to_vec()), expected);
        }
    }
}
