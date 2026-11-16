pub mod dynamic_programming;

pub trait Solution {
    fn max_moves(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]] as &dyn Matrix<_>,
                3,
            ),
            (&[[3, 2, 4], [2, 1, 9], [1, 1, 7]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_moves(grid.to_vec()), expected);
        }
    }
}
