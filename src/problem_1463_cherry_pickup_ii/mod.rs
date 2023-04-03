pub mod dynamic_programming;

pub trait Solution {
    fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]] as &dyn Matrix<_>, 24),
            (
                &[
                    [1, 0, 0, 0, 0, 0, 1],
                    [2, 0, 0, 0, 0, 3, 0],
                    [2, 0, 9, 0, 0, 0, 0],
                    [0, 3, 0, 5, 4, 0, 0],
                    [1, 0, 2, 3, 0, 0, 6],
                ],
                28,
            ),
            (
                &[
                    [0, 8, 7, 10, 9, 10, 0, 9, 6],
                    [8, 7, 10, 8, 7, 4, 9, 6, 10],
                    [8, 1, 1, 5, 1, 5, 5, 1, 2],
                    [9, 4, 10, 8, 8, 1, 9, 5, 0],
                    [4, 3, 6, 10, 9, 2, 4, 8, 10],
                    [7, 3, 2, 8, 3, 3, 5, 9, 8],
                    [1, 2, 6, 5, 6, 2, 0, 10, 0],
                ],
                96,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::cherry_pickup(grid.to_vec()), expected);
        }
    }
}
