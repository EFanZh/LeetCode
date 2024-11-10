pub mod bfs;

pub trait Solution {
    fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1, 1], [1, 1, 0], [1, 1, 0]] as &dyn Matrix<_>, 2),
            (&[[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::minimum_obstacles(grid.to_vec()), expected);
        }
    }
}
