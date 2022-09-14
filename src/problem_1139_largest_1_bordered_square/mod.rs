pub mod iterative;

pub trait Solution {
    fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 1, 1], [1, 0, 1], [1, 1, 1]] as &dyn Matrix<_>, 9),
            (&[[1, 1, 0, 0]], 1),
            (&[[0, 1], [1, 1]], 1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::largest1_bordered_square(grid.to_vec()), expected);
        }
    }
}
