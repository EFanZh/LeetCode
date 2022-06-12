pub mod bfs;
pub mod dynamic_programming;

pub trait Solution {
    fn max_distance(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 1], [0, 0, 0], [1, 0, 1]] as &dyn Matrix<_>, 2),
            (&[[1, 0, 0], [0, 0, 0], [0, 0, 0]], 4),
            (&[[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]], -1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_distance(grid.to_vec()), expected);
        }
    }
}
