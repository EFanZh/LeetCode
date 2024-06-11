pub mod greedy;

pub trait Solution {
    fn grid_game(grid: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 5, 4], [1, 5, 1]] as &dyn Matrix<_>, 4),
            (&[[3, 3, 1], [8, 5, 2]], 4),
            (&[[1, 3, 1, 15], [1, 3, 3, 1]], 7),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::grid_game(grid.to_vec()), expected);
        }
    }
}
