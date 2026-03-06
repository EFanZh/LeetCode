pub mod iterative;

pub trait Solution {
    fn minimum_operations(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[3, 2], [1, 3], [3, 4], [0, 1]] as &dyn Matrix<_>, 15),
            (&[[3, 2, 1], [2, 1, 0], [1, 2, 3]], 12),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::minimum_operations(grid.to_vec()), expected);
        }
    }
}
