pub mod bfs;

pub trait Solution {
    fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 1, 1], [1, 1, 0], [0, 1, 1]] as &dyn Matrix<_>, 4),
            (&[[2, 1, 1], [0, 1, 1], [1, 0, 1]], -1),
            (&[[0, 2]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::oranges_rotting(grid.to_vec()), expected);
        }
    }
}
