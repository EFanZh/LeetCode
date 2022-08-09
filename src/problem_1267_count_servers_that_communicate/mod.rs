pub mod iterative;

pub trait Solution {
    fn count_servers(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0], [0, 1]] as &dyn Matrix<_>, 0),
            (&[[1, 0], [1, 1]], 3),
            (&[[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]], 4),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::count_servers(grid.to_vec()), expected);
        }
    }
}
