pub mod iterative;

pub trait Solution {
    fn projection_area(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [3, 4]] as &dyn Matrix<_>, 17),
            (&[[2]], 5),
            (&[[1, 0], [0, 2]], 8),
            (&[[1, 1, 1], [1, 0, 1], [1, 1, 1]], 14),
            (&[[2, 2, 2], [2, 1, 2], [2, 2, 2]], 21),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::projection_area(grid.to_vec()), expected);
        }
    }
}
