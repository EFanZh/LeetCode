pub mod iterative;

pub trait Solution {
    fn min_flips(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 0], [0, 1, 0], [0, 0, 1]] as &dyn Matrix<_>, 3),
            (&[[0, 1], [0, 1], [0, 0]], 2),
            (&[[1], [1]], 2),
            (&[[0], [1], [1]], 2),
            (&[[1], [1], [1]], 3),
            (&[[0, 0]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::min_flips(grid.to_vec()), expected);
        }
    }
}
