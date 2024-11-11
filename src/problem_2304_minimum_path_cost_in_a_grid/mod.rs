pub mod dynamic_programming;

pub trait Solution {
    fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[5, 3], [4, 0], [2, 1]] as &dyn Matrix<_>,
                    &[[9, 8], [1, 5], [10, 12], [18, 6], [2, 4], [14, 3]] as &dyn Matrix<_>,
                ),
                17,
            ),
            (
                (
                    &[[5, 1, 2], [4, 0, 3]],
                    &[
                        [12, 10, 15],
                        [20, 23, 8],
                        [21, 7, 1],
                        [8, 1, 13],
                        [9, 10, 25],
                        [5, 3, 2],
                    ],
                ),
                6,
            ),
        ];

        for ((grid, move_cost), expected) in test_cases {
            assert_eq!(S::min_path_cost(grid.to_vec(), move_cost.to_vec()), expected);
        }
    }
}
