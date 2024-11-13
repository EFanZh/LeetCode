pub mod iterative;

pub trait Solution {
    fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[2, 0, 0, 1], [0, 3, 1, 0], [0, 5, 2, 0], [4, 0, 0, 2]] as &dyn Matrix<_>,
                true,
            ),
            (&[[5, 7, 0], [0, 3, 1], [0, 5, 0]], false),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::check_x_matrix(grid.to_vec()), expected);
        }
    }
}
